// TurboFan IR (Intermediate Representation)

/// Node ID for IR nodes
pub type NodeId = usize;

/// Type information for IR nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Unknown,
}

/// IR Node representing operations in SSA form
#[derive(Debug, Clone, PartialEq)]
pub enum IRNode {
    /// Constant value
    Constant {
        value: f64,
        id: NodeId,
    },
    /// Addition operation
    Add {
        left: NodeId,
        right: NodeId,
        id: NodeId,
    },
    /// Subtraction operation
    Sub {
        left: NodeId,
        right: NodeId,
        id: NodeId,
    },
    /// Multiplication operation
    Mul {
        left: NodeId,
        right: NodeId,
        id: NodeId,
    },
    /// Division operation
    Div {
        left: NodeId,
        right: NodeId,
        id: NodeId,
    },
    /// Load local variable
    LoadLocal {
        index: usize,
        id: NodeId,
    },
    /// Store local variable
    StoreLocal {
        index: usize,
        value: NodeId,
        id: NodeId,
    },
    /// Function call
    Call {
        callee: NodeId,
        args: Vec<NodeId>,
        id: NodeId,
    },
    /// Return statement
    Return {
        value: NodeId,
        id: NodeId,
    },
    /// Type guard for optimization
    TypeGuard {
        value: NodeId,
        expected_type: Type,
        id: NodeId,
    },
}

impl IRNode {
    /// Get the ID of this node
    pub fn id(&self) -> NodeId {
        match self {
            IRNode::Constant { id, .. } => *id,
            IRNode::Add { id, .. } => *id,
            IRNode::Sub { id, .. } => *id,
            IRNode::Mul { id, .. } => *id,
            IRNode::Div { id, .. } => *id,
            IRNode::LoadLocal { id, .. } => *id,
            IRNode::StoreLocal { id, .. } => *id,
            IRNode::Call { id, .. } => *id,
            IRNode::Return { id, .. } => *id,
            IRNode::TypeGuard { id, .. } => *id,
        }
    }
}

/// IR structure containing all nodes
#[derive(Debug, Clone)]
pub struct IR {
    pub nodes: Vec<IRNode>,
    next_id: NodeId,
}

impl IR {
    /// Create a new empty IR
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
        }
    }
    
    /// Get the next available node ID
    fn next_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    /// Add a constant node
    pub fn add_constant(&mut self, value: f64) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Constant { value, id });
        id
    }
    
    /// Add an addition node
    pub fn add_add(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Add { left, right, id });
        id
    }
    
    /// Add a subtraction node
    pub fn add_sub(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Sub { left, right, id });
        id
    }
    
    /// Add a multiplication node
    pub fn add_mul(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Mul { left, right, id });
        id
    }
    
    /// Add a division node
    pub fn add_div(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Div { left, right, id });
        id
    }
    
    /// Add a load local node
    pub fn add_load_local(&mut self, index: usize) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::LoadLocal { index, id });
        id
    }
    
    /// Add a store local node
    pub fn add_store_local(&mut self, index: usize, value: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::StoreLocal { index, value, id });
        id
    }
    
    /// Add a call node
    pub fn add_call(&mut self, callee: NodeId, args: Vec<NodeId>) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Call { callee, args, id });
        id
    }
    
    /// Add a return node
    pub fn add_return(&mut self, value: NodeId) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::Return { value, id });
        id
    }
    
    /// Add a type guard node
    pub fn add_type_guard(&mut self, value: NodeId, expected_type: Type) -> NodeId {
        let id = self.next_id();
        self.nodes.push(IRNode::TypeGuard { value, expected_type, id });
        id
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&IRNode> {
        self.nodes.iter().find(|n| n.id() == id)
    }
    
    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut IRNode> {
        self.nodes.iter_mut().find(|n| n.id() == id)
    }
}

impl Default for IR {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ir_creation() {
        let ir = IR::new();
        assert_eq!(ir.nodes.len(), 0);
    }
    
    #[test]
    fn test_add_constant() {
        let mut ir = IR::new();
        let id = ir.add_constant(42.0);
        
        assert_eq!(id, 0);
        assert_eq!(ir.nodes.len(), 1);
        
        match &ir.nodes[0] {
            IRNode::Constant { value, id: node_id } => {
                assert_eq!(*value, 42.0);
                assert_eq!(*node_id, 0);
            }
            _ => panic!("Expected Constant node"),
        }
    }
    
    #[test]
    fn test_add_arithmetic() {
        let mut ir = IR::new();
        let left = ir.add_constant(10.0);
        let right = ir.add_constant(20.0);
        let add = ir.add_add(left, right);
        
        assert_eq!(add, 2);
        assert_eq!(ir.nodes.len(), 3);
    }
    
    #[test]
    fn test_node_id() {
        let mut ir = IR::new();
        let id1 = ir.add_constant(1.0);
        let id2 = ir.add_constant(2.0);
        let id3 = ir.add_add(id1, id2);
        
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
    }
    
    #[test]
    fn test_get_node() {
        let mut ir = IR::new();
        let id = ir.add_constant(42.0);
        
        let node = ir.get_node(id).unwrap();
        assert_eq!(node.id(), id);
    }
    
    #[test]
    fn test_type_guard() {
        let mut ir = IR::new();
        let value = ir.add_constant(42.0);
        let guard = ir.add_type_guard(value, Type::Number);
        
        match ir.get_node(guard).unwrap() {
            IRNode::TypeGuard { value: v, expected_type, .. } => {
                assert_eq!(*v, value);
                assert_eq!(*expected_type, Type::Number);
            }
            _ => panic!("Expected TypeGuard node"),
        }
    }
    
    #[test]
    fn test_load_store_local() {
        let mut ir = IR::new();
        let value = ir.add_constant(42.0);
        let store = ir.add_store_local(0, value);
        let load = ir.add_load_local(0);
        
        assert_eq!(ir.nodes.len(), 3);
        assert!(matches!(ir.get_node(store).unwrap(), IRNode::StoreLocal { .. }));
        assert!(matches!(ir.get_node(load).unwrap(), IRNode::LoadLocal { .. }));
    }
}
