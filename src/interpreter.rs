// Ignition bytecode interpreter

use crate::bytecode::{BytecodeChunk, Instruction};
use crate::error::RuntimeError;
use crate::types::{FunctionId, Value};
use crate::profiler::HotspotProfiler;
use std::rc::Rc;
use std::cell::RefCell;

/// Call frame for function execution
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub chunk: BytecodeChunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub func_id: FunctionId,
}

impl CallFrame {
    pub fn new(chunk: BytecodeChunk, func_id: FunctionId) -> Self {
        let local_count = chunk.local_count;
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
            locals: vec![Value::Undefined; local_count],
            func_id,
        }
    }
    
    /// Push a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    
    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackOverflow)
    }
    
    /// Peek at the top of the stack
    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }
}

/// Ignition interpreter
pub struct Ignition {
    call_stack: Vec<CallFrame>,
    profiler: Rc<RefCell<HotspotProfiler>>,
}

impl Ignition {
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            profiler: Rc::new(RefCell::new(HotspotProfiler::default())),
        }
    }
    
    /// Create interpreter with a shared profiler
    pub fn with_profiler(profiler: Rc<RefCell<HotspotProfiler>>) -> Self {
        Self {
            call_stack: Vec::new(),
            profiler,
        }
    }
    
    /// Get a reference to the profiler
    pub fn profiler(&self) -> Rc<RefCell<HotspotProfiler>> {
        self.profiler.clone()
    }
    
    /// Execute a bytecode chunk
    pub fn execute(&mut self, chunk: BytecodeChunk) -> Result<Value, RuntimeError> {
        let func_id = 0; // Default function ID for main execution
        self.execute_with_id(chunk, func_id)
    }
    
    /// Execute a bytecode chunk with a specific function ID
    pub fn execute_with_id(&mut self, chunk: BytecodeChunk, func_id: FunctionId) -> Result<Value, RuntimeError> {
        // Record execution in profiler
        self.profiler.borrow_mut().record_execution(func_id);
        
        let frame = CallFrame::new(chunk, func_id);
        self.call_stack.push(frame);
        
        self.run()
    }
    
    /// Main execution loop
    fn run(&mut self) -> Result<Value, RuntimeError> {
        loop {
            let frame = self.call_stack.last_mut()
                .ok_or(RuntimeError::StackOverflow)?;
            
            if frame.ip >= frame.chunk.instructions.len() {
                // End of instructions
                let result = frame.pop().unwrap_or(Value::Undefined);
                self.call_stack.pop();
                
                if self.call_stack.is_empty() {
                    return Ok(result);
                }
                
                // Push result to caller's stack
                if let Some(caller) = self.call_stack.last_mut() {
                    caller.push(result);
                }
                continue;
            }
            
            let instruction = frame.chunk.instructions[frame.ip].clone();
            frame.ip += 1;
            
            self.dispatch(instruction)?;
        }
    }
    
    /// Dispatch a single instruction
    fn dispatch(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
        let frame = self.call_stack.last_mut()
            .ok_or(RuntimeError::StackOverflow)?;
        
        match instruction {
            Instruction::LoadConst(idx) => {
                let value = frame.chunk.constants.get(idx)
                    .cloned()
                    .ok_or(RuntimeError::StackOverflow)?;
                frame.push(value);
            }
            
            Instruction::LoadLocal(idx) => {
                let value = frame.locals.get(idx)
                    .cloned()
                    .ok_or(RuntimeError::UndefinedVariable {
                        name: format!("local_{}", idx),
                    })?;
                frame.push(value);
            }
            
            Instruction::StoreLocal(idx) => {
                let value = frame.pop()?;
                if idx < frame.locals.len() {
                    frame.locals[idx] = value;
                }
            }
            
            Instruction::Add => {
                let right = frame.pop()?;
                let left = frame.pop()?;
                
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        frame.push(Value::Number(l + r));
                    }
                    _ => {
                        return Err(RuntimeError::TypeError {
                            expected: "number".to_string(),
                            found: "other".to_string(),
                        });
                    }
                }
            }
            
            Instruction::Sub => {
                let right = frame.pop()?;
                let left = frame.pop()?;
                
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        frame.push(Value::Number(l - r));
                    }
                    _ => {
                        return Err(RuntimeError::TypeError {
                            expected: "number".to_string(),
                            found: "other".to_string(),
                        });
                    }
                }
            }
            
            Instruction::Mul => {
                let right = frame.pop()?;
                let left = frame.pop()?;
                
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        frame.push(Value::Number(l * r));
                    }
                    _ => {
                        return Err(RuntimeError::TypeError {
                            expected: "number".to_string(),
                            found: "other".to_string(),
                        });
                    }
                }
            }
            
            Instruction::Div => {
                let right = frame.pop()?;
                let left = frame.pop()?;
                
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        if r == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        frame.push(Value::Number(l / r));
                    }
                    _ => {
                        return Err(RuntimeError::TypeError {
                            expected: "number".to_string(),
                            found: "other".to_string(),
                        });
                    }
                }
            }
            
            Instruction::Return => {
                let result = frame.pop().unwrap_or(Value::Undefined);
                self.call_stack.pop();
                
                if let Some(caller) = self.call_stack.last_mut() {
                    caller.push(result);
                }
            }
            
            Instruction::Jump(offset) => {
                let frame = self.call_stack.last_mut().unwrap();
                frame.ip = ((frame.ip as isize) + offset) as usize;
            }
            
            Instruction::JumpIfFalse(offset) => {
                let frame = self.call_stack.last_mut().unwrap();
                let cond = frame.peek().cloned().unwrap_or(Value::Undefined);
                
                // For simplicity, treat 0 as false, everything else as true
                let is_false = match cond {
                    Value::Number(n) => n == 0.0,
                    Value::Undefined => true,
                    _ => false,
                };
                
                if is_false {
                    frame.ip = ((frame.ip as isize) + offset) as usize;
                }
            }
            
            Instruction::Call(_arg_count) => {
                // Simplified: just continue execution
                // Full implementation would handle function calls
            }
        }
        
        Ok(())
    }
}

impl Default for Ignition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_call_frame_creation() {
        let chunk = BytecodeChunk::new();
        let frame = CallFrame::new(chunk, 0);
        
        assert_eq!(frame.ip, 0);
        assert_eq!(frame.stack.len(), 0);
        assert_eq!(frame.func_id, 0);
    }
    
    #[test]
    fn test_call_frame_push_pop() {
        let chunk = BytecodeChunk::new();
        let mut frame = CallFrame::new(chunk, 0);
        
        frame.push(Value::Number(42.0));
        assert_eq!(frame.stack.len(), 1);
        
        let value = frame.pop().unwrap();
        assert_eq!(value, Value::Number(42.0));
        assert_eq!(frame.stack.len(), 0);
    }
    
    #[test]
    fn test_execute_load_const() {
        let mut chunk = BytecodeChunk::new();
        let idx = chunk.add_constant(Value::Number(42.0));
        chunk.emit(Instruction::LoadConst(idx));
        
        let mut interpreter = Ignition::new();
        let result = interpreter.execute(chunk).unwrap();
        
        assert_eq!(result, Value::Number(42.0));
    }
    
    #[test]
    fn test_execute_add() {
        let mut chunk = BytecodeChunk::new();
        let idx1 = chunk.add_constant(Value::Number(10.0));
        let idx2 = chunk.add_constant(Value::Number(20.0));
        chunk.emit(Instruction::LoadConst(idx1));
        chunk.emit(Instruction::LoadConst(idx2));
        chunk.emit(Instruction::Add);
        
        let mut interpreter = Ignition::new();
        let result = interpreter.execute(chunk).unwrap();
        
        assert_eq!(result, Value::Number(30.0));
    }
    
    #[test]
    fn test_execute_arithmetic() {
        let mut chunk = BytecodeChunk::new();
        // (10 + 5) * 2
        let idx1 = chunk.add_constant(Value::Number(10.0));
        let idx2 = chunk.add_constant(Value::Number(5.0));
        let idx3 = chunk.add_constant(Value::Number(2.0));
        
        chunk.emit(Instruction::LoadConst(idx1));
        chunk.emit(Instruction::LoadConst(idx2));
        chunk.emit(Instruction::Add);
        chunk.emit(Instruction::LoadConst(idx3));
        chunk.emit(Instruction::Mul);
        
        let mut interpreter = Ignition::new();
        let result = interpreter.execute(chunk).unwrap();
        
        assert_eq!(result, Value::Number(30.0));
    }
    
    #[test]
    fn test_execute_division_by_zero() {
        let mut chunk = BytecodeChunk::new();
        let idx1 = chunk.add_constant(Value::Number(10.0));
        let idx2 = chunk.add_constant(Value::Number(0.0));
        
        chunk.emit(Instruction::LoadConst(idx1));
        chunk.emit(Instruction::LoadConst(idx2));
        chunk.emit(Instruction::Div);
        
        let mut interpreter = Ignition::new();
        let result = interpreter.execute(chunk);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RuntimeError::DivisionByZero));
    }
}
