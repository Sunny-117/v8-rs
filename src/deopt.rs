// Deoptimization mechanism

use crate::types::{FunctionId, Value};
use crate::bytecode::BytecodeChunk;

/// Deoptimization information
#[derive(Debug, Clone)]
pub struct DeoptInfo {
    /// Function that needs deoptimization
    pub func_id: FunctionId,
    /// Live values at deoptimization point
    pub live_values: Vec<Value>,
    /// Bytecode offset to resume at
    pub bytecode_offset: usize,
    /// Reason for deoptimization
    pub reason: DeoptReason,
}

/// Reason for deoptimization
#[derive(Debug, Clone, PartialEq)]
pub enum DeoptReason {
    /// Type guard failed
    TypeGuardFailed {
        expected: String,
        found: String,
    },
    /// Assumption invalidated
    AssumptionInvalidated {
        assumption: String,
    },
    /// Other reason
    Other {
        message: String,
    },
}

impl DeoptInfo {
    /// Create new deoptimization info
    pub fn new(func_id: FunctionId, reason: DeoptReason) -> Self {
        Self {
            func_id,
            live_values: Vec::new(),
            bytecode_offset: 0,
            reason,
        }
    }
    
    /// Create deoptimization info for type guard failure
    pub fn type_guard_failed(func_id: FunctionId, expected: String, found: String) -> Self {
        Self::new(func_id, DeoptReason::TypeGuardFailed { expected, found })
    }
    
    /// Add a live value
    pub fn add_live_value(&mut self, value: Value) {
        self.live_values.push(value);
    }
    
    /// Set bytecode offset
    pub fn set_bytecode_offset(&mut self, offset: usize) {
        self.bytecode_offset = offset;
    }
}

/// Deoptimization manager
#[derive(Debug)]
pub struct DeoptManager {
    /// Bytecode chunks for functions (for reconstruction)
    bytecode_cache: std::collections::HashMap<FunctionId, BytecodeChunk>,
}

impl DeoptManager {
    /// Create a new deoptimization manager
    pub fn new() -> Self {
        Self {
            bytecode_cache: std::collections::HashMap::new(),
        }
    }
    
    /// Register bytecode for a function
    pub fn register_bytecode(&mut self, func_id: FunctionId, bytecode: BytecodeChunk) {
        self.bytecode_cache.insert(func_id, bytecode);
    }
    
    /// Get bytecode for a function
    pub fn get_bytecode(&self, func_id: FunctionId) -> Option<&BytecodeChunk> {
        self.bytecode_cache.get(&func_id)
    }
    
    /// Trigger deoptimization
    pub fn trigger_deopt(&self, deopt_info: &DeoptInfo) -> Result<DeoptState, String> {
        // Get the bytecode for the function
        let bytecode = self.get_bytecode(deopt_info.func_id)
            .ok_or_else(|| format!("No bytecode found for function {}", deopt_info.func_id))?;
        
        // Create deoptimization state
        Ok(DeoptState {
            func_id: deopt_info.func_id,
            bytecode: bytecode.clone(),
            live_values: deopt_info.live_values.clone(),
            bytecode_offset: deopt_info.bytecode_offset,
            reason: deopt_info.reason.clone(),
        })
    }
}

impl Default for DeoptManager {
    fn default() -> Self {
        Self::new()
    }
}

/// State after deoptimization
#[derive(Debug, Clone)]
pub struct DeoptState {
    /// Function ID
    pub func_id: FunctionId,
    /// Bytecode to resume execution
    pub bytecode: BytecodeChunk,
    /// Live values to restore
    pub live_values: Vec<Value>,
    /// Bytecode offset to resume at
    pub bytecode_offset: usize,
    /// Reason for deoptimization
    pub reason: DeoptReason,
}

impl DeoptState {
    /// Check if type guard should trigger deoptimization
    pub fn check_type_guard(value: &Value, expected_type: &str) -> Option<DeoptReason> {
        let actual_type = match value {
            Value::Number(_) => "number",
            Value::Function(_) => "function",
            Value::Undefined => "undefined",
        };
        
        if actual_type != expected_type {
            Some(DeoptReason::TypeGuardFailed {
                expected: expected_type.to_string(),
                found: actual_type.to_string(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deopt_info_creation() {
        let info = DeoptInfo::new(0, DeoptReason::Other {
            message: "test".to_string(),
        });
        
        assert_eq!(info.func_id, 0);
        assert_eq!(info.live_values.len(), 0);
        assert_eq!(info.bytecode_offset, 0);
    }
    
    #[test]
    fn test_type_guard_failed() {
        let info = DeoptInfo::type_guard_failed(1, "number".to_string(), "undefined".to_string());
        
        assert_eq!(info.func_id, 1);
        assert!(matches!(info.reason, DeoptReason::TypeGuardFailed { .. }));
    }
    
    #[test]
    fn test_add_live_value() {
        let mut info = DeoptInfo::new(0, DeoptReason::Other {
            message: "test".to_string(),
        });
        
        info.add_live_value(Value::Number(42.0));
        assert_eq!(info.live_values.len(), 1);
    }
    
    #[test]
    fn test_deopt_manager() {
        let mut manager = DeoptManager::new();
        let chunk = BytecodeChunk::new();
        
        manager.register_bytecode(0, chunk);
        assert!(manager.get_bytecode(0).is_some());
        assert!(manager.get_bytecode(1).is_none());
    }
    
    #[test]
    fn test_trigger_deopt() {
        let mut manager = DeoptManager::new();
        let chunk = BytecodeChunk::new();
        manager.register_bytecode(0, chunk);
        
        let info = DeoptInfo::new(0, DeoptReason::Other {
            message: "test".to_string(),
        });
        
        let state = manager.trigger_deopt(&info);
        assert!(state.is_ok());
        
        let state = state.unwrap();
        assert_eq!(state.func_id, 0);
    }
    
    #[test]
    fn test_check_type_guard() {
        let value = Value::Number(42.0);
        let reason = DeoptState::check_type_guard(&value, "number");
        assert!(reason.is_none());
        
        let reason = DeoptState::check_type_guard(&value, "function");
        assert!(reason.is_some());
        assert!(matches!(reason.unwrap(), DeoptReason::TypeGuardFailed { .. }));
    }
}
