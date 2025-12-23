// V8-RS Engine - Main coordinator

use crate::bytecode::BytecodeChunk;
use crate::codegen::BytecodeGenerator;
use crate::codegen_backend::{CodeGenerator, CompiledFunction};
use crate::deopt::{DeoptInfo, DeoptManager, DeoptState};
use crate::error::{Error, ParseError, RuntimeError};
use crate::interpreter::Ignition;
use crate::parser::Parser;
use crate::profiler::HotspotProfiler;
use crate::scope::Scope;
use crate::turbofan::TurboFan;
use crate::types::{FunctionId, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Main engine that coordinates all components
pub struct Engine {
    interpreter: Ignition,
    global_scope: Scope,
    profiler: Rc<RefCell<HotspotProfiler>>,
    jit: TurboFan,
    codegen: CodeGenerator,
    deopt_manager: DeoptManager,
    compiled_functions: HashMap<FunctionId, CompiledFunction>,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Self {
        let profiler = Rc::new(RefCell::new(HotspotProfiler::default()));
        Self {
            interpreter: Ignition::with_profiler(profiler.clone()),
            global_scope: Scope::global(),
            profiler,
            jit: TurboFan::new(),
            codegen: CodeGenerator::mock(),
            deopt_manager: DeoptManager::new(),
            compiled_functions: HashMap::new(),
        }
    }
    
    /// Get a reference to the profiler
    pub fn profiler(&self) -> Rc<RefCell<HotspotProfiler>> {
        self.profiler.clone()
    }
    
    /// Check if a function should be optimized
    pub fn should_optimize(&self, func_id: FunctionId) -> bool {
        self.profiler.borrow().is_hot(func_id)
    }
    
    /// Optimize a function
    pub fn optimize(&mut self, func_id: FunctionId, bytecode: &BytecodeChunk) -> Option<CompiledFunction> {
        // Compile bytecode to optimized IR
        let ir = self.jit.compile(bytecode, func_id);
        
        // Generate machine code
        let compiled = self.codegen.generate(&ir, func_id);
        
        // Store compiled function
        self.compiled_functions.insert(func_id, compiled.clone());
        
        // Register bytecode for potential deoptimization
        self.deopt_manager.register_bytecode(func_id, bytecode.clone());
        
        Some(compiled)
    }
    
    /// Deoptimize a function
    pub fn deoptimize(&mut self, deopt_info: DeoptInfo) -> Result<(), String> {
        // Trigger deoptimization
        let deopt_state = self.deopt_manager.trigger_deopt(&deopt_info)?;
        
        // Remove compiled function
        self.compiled_functions.remove(&deopt_state.func_id);
        
        // Unmark as hot
        self.profiler.borrow_mut().unmark_hot(deopt_state.func_id);
        
        // Continue execution in interpreter
        // (In a real implementation, this would restore the interpreter state)
        
        Ok(())
    }
    
    /// Execute JavaScript source code
    pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
        // Parse source code to AST
        let ast = self.parse(source)?;
        
        // Generate bytecode from AST
        let bytecode = self.generate_bytecode(&ast);
        
        // Interpret bytecode
        let result = self.interpret(bytecode)?;
        
        Ok(result)
    }
    
    /// Parse source code into AST
    fn parse(&self, source: &str) -> Result<crate::ast::AST, ParseError> {
        let mut parser = Parser::new(source.to_string());
        parser.parse()
    }
    
    /// Generate bytecode from AST
    fn generate_bytecode(&mut self, ast: &crate::ast::AST) -> BytecodeChunk {
        let mut generator = BytecodeGenerator::new(self.global_scope.clone());
        generator.generate(&ast.root)
    }
    
    /// Interpret bytecode
    fn interpret(&mut self, bytecode: BytecodeChunk) -> Result<Value, RuntimeError> {
        self.interpreter.execute(bytecode)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_creation() {
        let engine = Engine::new();
        assert!(true); // Engine created successfully
    }
    
    #[test]
    fn test_execute_number() {
        let mut engine = Engine::new();
        let result = engine.execute("42").unwrap();
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_execute_addition() {
        let mut engine = Engine::new();
        let result = engine.execute("10 + 20").unwrap();
        assert_eq!(result, Value::Number(30.0));
    }
    
    #[test]
    fn test_execute_complex_expression() {
        let mut engine = Engine::new();
        let result = engine.execute("(5 + 3) * 2").unwrap();
        assert_eq!(result, Value::Number(16.0));
    }
    
    #[test]
    fn test_execute_let_declaration() {
        let mut engine = Engine::new();
        let result = engine.execute("let x = 42;").unwrap();
        // Let declaration returns undefined or the value
        assert!(matches!(result, Value::Number(_) | Value::Undefined));
    }
    
    #[test]
    fn test_execute_parse_error() {
        let mut engine = Engine::new();
        let result = engine.execute("let = 10");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_execute_division_by_zero() {
        let mut engine = Engine::new();
        let result = engine.execute("10 / 0");
        assert!(result.is_err());
    }
}
