// V8-RS Engine - Main coordinator

use crate::bytecode::BytecodeChunk;
use crate::codegen::BytecodeGenerator;
use crate::error::{Error, ParseError, RuntimeError};
use crate::interpreter::Ignition;
use crate::parser::Parser;
use crate::scope::Scope;
use crate::types::Value;

/// Main engine that coordinates all components
pub struct Engine {
    interpreter: Ignition,
    global_scope: Scope,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            interpreter: Ignition::new(),
            global_scope: Scope::global(),
        }
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
