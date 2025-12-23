// Bytecode generation from AST

use crate::ast::{ASTNode, BinOp};
use crate::bytecode::{BytecodeChunk, Instruction};
use crate::scope::Scope;
use crate::types::Value;

/// Bytecode generator
pub struct BytecodeGenerator {
    chunk: BytecodeChunk,
    scope: Scope,
}

impl BytecodeGenerator {
    pub fn new(scope: Scope) -> Self {
        Self {
            chunk: BytecodeChunk::new(),
            scope,
        }
    }
    
    /// Generate bytecode from AST
    pub fn generate(&mut self, ast: &ASTNode) -> BytecodeChunk {
        self.compile_node(ast);
        self.chunk.set_local_count(self.scope.local_count());
        self.chunk.clone()
    }
    
    /// Compile a single AST node
    fn compile_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(stmts) => {
                for stmt in stmts {
                    self.compile_node(stmt);
                }
            }
            
            ASTNode::NumberLiteral { value, .. } => {
                let idx = self.chunk.add_constant(Value::Number(*value));
                self.chunk.emit(Instruction::LoadConst(idx));
            }
            
            ASTNode::Identifier { name, .. } => {
                if let Some(idx) = self.scope.lookup(name) {
                    self.chunk.emit(Instruction::LoadLocal(idx));
                }
            }
            
            ASTNode::BinaryExpr { op, left, right, .. } => {
                self.compile_node(left);
                self.compile_node(right);
                
                match op {
                    BinOp::Add => self.chunk.emit(Instruction::Add),
                    BinOp::Sub => self.chunk.emit(Instruction::Sub),
                    BinOp::Mul => self.chunk.emit(Instruction::Mul),
                    BinOp::Div => self.chunk.emit(Instruction::Div),
                    _ => {}
                }
            }
            
            ASTNode::LetDecl { name, init, .. } => {
                // Compile the initializer
                self.compile_node(init);
                
                // Declare the variable and store
                let idx = self.scope.declare(name.clone());
                self.chunk.emit(Instruction::StoreLocal(idx));
            }
            
            ASTNode::CallExpr { callee, args, .. } => {
                // Check if this is a call to the built-in print() function
                if let ASTNode::Identifier { name, .. } = &**callee {
                    if name == "print" && args.len() == 1 {
                        // Special handling for print(arg)
                        self.compile_node(&args[0]);
                        self.chunk.emit(Instruction::Print);
                        return;
                    }
                }
                
                // General function call handling
                // Compile callee
                self.compile_node(callee);
                
                // Compile arguments
                for arg in args {
                    self.compile_node(arg);
                }
                
                // Emit call instruction
                self.chunk.emit(Instruction::Call(args.len()));
            }
            
            ASTNode::ReturnStmt { value, .. } => {
                self.compile_node(value);
                self.chunk.emit(Instruction::Return);
            }
            
            ASTNode::BlockStmt { statements, .. } => {
                for stmt in statements {
                    self.compile_node(stmt);
                }
            }
            
            ASTNode::IfStmt { cond, then_branch, else_branch, .. } => {
                // Compile condition
                self.compile_node(cond);
                
                // Jump if false (placeholder)
                let jump_if_false_idx = self.chunk.instructions.len();
                self.chunk.emit(Instruction::JumpIfFalse(0));
                
                // Compile then branch
                self.compile_node(then_branch);
                
                // Jump over else (placeholder)
                let jump_idx = self.chunk.instructions.len();
                self.chunk.emit(Instruction::Jump(0));
                
                // Patch jump_if_false
                let else_start = self.chunk.instructions.len();
                let jump_if_false_offset = (else_start as isize) - (jump_if_false_idx as isize) - 1;
                self.chunk.instructions[jump_if_false_idx] = Instruction::JumpIfFalse(jump_if_false_offset);
                
                // Compile else branch if present
                if let Some(else_br) = else_branch {
                    self.compile_node(else_br);
                }
                
                // Patch jump
                let end = self.chunk.instructions.len();
                let jump_offset = (end as isize) - (jump_idx as isize) - 1;
                self.chunk.instructions[jump_idx] = Instruction::Jump(jump_offset);
            }
            
            ASTNode::ForStmt { init, cond, update, body, .. } => {
                // Compile init
                self.compile_node(init);
                
                // Loop start
                let loop_start = self.chunk.instructions.len();
                
                // Compile condition
                self.compile_node(cond);
                
                // Jump if false (exit loop)
                let jump_if_false_idx = self.chunk.instructions.len();
                self.chunk.emit(Instruction::JumpIfFalse(0));
                
                // Compile body
                self.compile_node(body);
                
                // Compile update
                self.compile_node(update);
                
                // Jump back to loop start
                let current = self.chunk.instructions.len();
                let jump_back_offset = (loop_start as isize) - (current as isize) - 1;
                self.chunk.emit(Instruction::Jump(jump_back_offset));
                
                // Patch jump_if_false
                let end = self.chunk.instructions.len();
                let jump_if_false_offset = (end as isize) - (jump_if_false_idx as isize) - 1;
                self.chunk.instructions[jump_if_false_idx] = Instruction::JumpIfFalse(jump_if_false_offset);
            }
            
            ASTNode::FunctionDecl { name, params, body, .. } => {
                // For now, we'll skip function declarations in bytecode generation
                // They would need to be compiled separately and stored
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    
    #[test]
    fn test_compile_number() {
        let mut parser = Parser::new("42".to_string());
        let ast = parser.parse().unwrap();
        
        let mut gen = BytecodeGenerator::new(Scope::global());
        let chunk = gen.generate(&ast.root);
        
        assert_eq!(chunk.instructions.len(), 1);
        assert_eq!(chunk.instructions[0], Instruction::LoadConst(0));
        assert_eq!(chunk.constants[0], Value::Number(42.0));
    }
    
    #[test]
    fn test_compile_binary_expr() {
        let mut parser = Parser::new("1 + 2".to_string());
        let ast = parser.parse().unwrap();
        
        let mut gen = BytecodeGenerator::new(Scope::global());
        let chunk = gen.generate(&ast.root);
        
        // Should have: LoadConst(1), LoadConst(2), Add
        assert!(chunk.instructions.len() >= 3);
        assert_eq!(chunk.instructions[chunk.instructions.len() - 1], Instruction::Add);
    }
    
    #[test]
    fn test_compile_let_decl() {
        let mut parser = Parser::new("let x = 10;".to_string());
        let ast = parser.parse().unwrap();
        
        let mut gen = BytecodeGenerator::new(Scope::global());
        let chunk = gen.generate(&ast.root);
        
        // Should have: LoadConst(10), StoreLocal(0)
        assert!(chunk.instructions.len() >= 2);
        assert!(matches!(chunk.instructions[chunk.instructions.len() - 1], Instruction::StoreLocal(0)));
    }
    
    #[test]
    fn test_compile_call_expr() {
        let mut parser = Parser::new("foo(1, 2)".to_string());
        let ast = parser.parse().unwrap();
        
        let mut gen = BytecodeGenerator::new(Scope::global());
        let chunk = gen.generate(&ast.root);
        
        // Should end with Call(2)
        assert!(matches!(chunk.instructions[chunk.instructions.len() - 1], Instruction::Call(2)));
    }
}
