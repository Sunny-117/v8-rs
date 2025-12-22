// Scope management for variable resolution

use std::collections::HashMap;

/// Type of scope
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
}

/// Scope for managing variable bindings
#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, usize>,
    scope_type: ScopeType,
    next_index: usize,
}

impl Scope {
    /// Create a new scope
    pub fn new(scope_type: ScopeType, parent: Option<Box<Scope>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
            scope_type,
            next_index: 0,
        }
    }
    
    /// Create a global scope
    pub fn global() -> Self {
        Self::new(ScopeType::Global, None)
    }
    
    /// Create a function scope with this scope as parent
    pub fn function_scope(&self) -> Self {
        Self::new(ScopeType::Function, Some(Box::new(self.clone())))
    }
    
    /// Create a block scope with this scope as parent
    pub fn block_scope(&self) -> Self {
        Self::new(ScopeType::Block, Some(Box::new(self.clone())))
    }
    
    /// Declare a new variable in this scope
    pub fn declare(&mut self, name: String) -> usize {
        let index = self.next_index;
        self.variables.insert(name, index);
        self.next_index += 1;
        index
    }
    
    /// Look up a variable in this scope or parent scopes
    pub fn lookup(&self, name: &str) -> Option<usize> {
        if let Some(&index) = self.variables.get(name) {
            Some(index)
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
    
    /// Get the scope type
    pub fn scope_type(&self) -> &ScopeType {
        &self.scope_type
    }
    
    /// Get the number of variables in this scope
    pub fn local_count(&self) -> usize {
        self.next_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_global_scope() {
        let scope = Scope::global();
        assert_eq!(scope.scope_type(), &ScopeType::Global);
    }
    
    #[test]
    fn test_declare_variable() {
        let mut scope = Scope::global();
        let index = scope.declare("x".to_string());
        assert_eq!(index, 0);
        
        let index2 = scope.declare("y".to_string());
        assert_eq!(index2, 1);
    }
    
    #[test]
    fn test_lookup_variable() {
        let mut scope = Scope::global();
        scope.declare("x".to_string());
        
        assert_eq!(scope.lookup("x"), Some(0));
        assert_eq!(scope.lookup("y"), None);
    }
    
    #[test]
    fn test_nested_scope_lookup() {
        let mut global = Scope::global();
        global.declare("x".to_string());
        
        let mut func = global.function_scope();
        func.declare("y".to_string());
        
        // Can find both x (from parent) and y (from current)
        assert_eq!(func.lookup("x"), Some(0));
        assert_eq!(func.lookup("y"), Some(0));
        assert_eq!(func.lookup("z"), None);
    }
    
    #[test]
    fn test_scope_chain() {
        let mut global = Scope::global();
        global.declare("a".to_string());
        
        let mut func = global.function_scope();
        func.declare("b".to_string());
        
        let mut block = func.block_scope();
        block.declare("c".to_string());
        
        // Block scope can see all variables
        assert_eq!(block.lookup("a"), Some(0));
        assert_eq!(block.lookup("b"), Some(0));
        assert_eq!(block.lookup("c"), Some(0));
    }
    
    #[test]
    fn test_local_count() {
        let mut scope = Scope::global();
        assert_eq!(scope.local_count(), 0);
        
        scope.declare("x".to_string());
        assert_eq!(scope.local_count(), 1);
        
        scope.declare("y".to_string());
        assert_eq!(scope.local_count(), 2);
    }
}
