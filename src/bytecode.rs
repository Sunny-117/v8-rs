// Bytecode definitions and generation

use crate::types::Value;

/// Bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Load a constant from the constant pool
    LoadConst(usize),
    /// Load a local variable
    LoadLocal(usize),
    /// Store to a local variable
    StoreLocal(usize),
    /// Add two values
    Add,
    /// Subtract two values
    Sub,
    /// Multiply two values
    Mul,
    /// Divide two values
    Div,
    /// Print the top value on the stack (for console.log/print)
    Print,
    /// Call a function with N arguments
    Call(usize),
    /// Return from function
    Return,
    /// Unconditional jump
    Jump(isize),
    /// Jump if top of stack is false
    JumpIfFalse(isize),
}

/// A chunk of bytecode with constants and metadata
#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub local_count: usize,
}

impl BytecodeChunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            local_count: 0,
        }
    }
    
    /// Add an instruction
    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    
    /// Add a constant and return its index
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    /// Set the number of local variables
    pub fn set_local_count(&mut self, count: usize) {
        self.local_count = count;
    }
}

impl Default for BytecodeChunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bytecode_chunk_creation() {
        let chunk = BytecodeChunk::new();
        assert_eq!(chunk.instructions.len(), 0);
        assert_eq!(chunk.constants.len(), 0);
        assert_eq!(chunk.local_count, 0);
    }
    
    #[test]
    fn test_emit_instruction() {
        let mut chunk = BytecodeChunk::new();
        chunk.emit(Instruction::Add);
        chunk.emit(Instruction::Return);
        
        assert_eq!(chunk.instructions.len(), 2);
        assert_eq!(chunk.instructions[0], Instruction::Add);
        assert_eq!(chunk.instructions[1], Instruction::Return);
    }
    
    #[test]
    fn test_add_constant() {
        let mut chunk = BytecodeChunk::new();
        let idx1 = chunk.add_constant(Value::Number(42.0));
        let idx2 = chunk.add_constant(Value::Number(3.14));
        
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(chunk.constants.len(), 2);
    }
    
    #[test]
    fn test_set_local_count() {
        let mut chunk = BytecodeChunk::new();
        chunk.set_local_count(5);
        assert_eq!(chunk.local_count, 5);
    }
    
    #[test]
    fn test_instruction_types() {
        let instructions = vec![
            Instruction::LoadConst(0),
            Instruction::LoadLocal(1),
            Instruction::StoreLocal(2),
            Instruction::Add,
            Instruction::Sub,
            Instruction::Mul,
            Instruction::Div,
            Instruction::Call(3),
            Instruction::Return,
            Instruction::Jump(10),
            Instruction::JumpIfFalse(-5),
        ];
        
        assert_eq!(instructions.len(), 11);
    }
}
