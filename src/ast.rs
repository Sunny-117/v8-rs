// Abstract Syntax Tree definitions

use crate::types::Span;

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Less,
    Greater,
}

/// AST Node types
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    /// Program root containing statements
    Program(Vec<ASTNode>),
    
    /// Function declaration
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
        span: Span,
    },
    
    /// Let variable declaration
    LetDecl {
        name: String,
        init: Box<ASTNode>,
        span: Span,
    },
    
    /// For loop statement
    ForStmt {
        init: Box<ASTNode>,
        cond: Box<ASTNode>,
        update: Box<ASTNode>,
        body: Box<ASTNode>,
        span: Span,
    },
    
    /// If statement
    IfStmt {
        cond: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
        span: Span,
    },
    
    /// Return statement
    ReturnStmt {
        value: Box<ASTNode>,
        span: Span,
    },
    
    /// Block statement (multiple statements)
    BlockStmt {
        statements: Vec<ASTNode>,
        span: Span,
    },
    
    /// Binary expression
    BinaryExpr {
        op: BinOp,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        span: Span,
    },
    
    /// Function call expression
    CallExpr {
        callee: Box<ASTNode>,
        args: Vec<ASTNode>,
        span: Span,
    },
    
    /// Identifier
    Identifier {
        name: String,
        span: Span,
    },
    
    /// Number literal
    NumberLiteral {
        value: f64,
        span: Span,
    },
}

impl ASTNode {
    /// Get the span of this node
    pub fn span(&self) -> Span {
        match self {
            ASTNode::Program(_) => Span::new(0, 0),
            ASTNode::FunctionDecl { span, .. } => *span,
            ASTNode::LetDecl { span, .. } => *span,
            ASTNode::ForStmt { span, .. } => *span,
            ASTNode::IfStmt { span, .. } => *span,
            ASTNode::ReturnStmt { span, .. } => *span,
            ASTNode::BlockStmt { span, .. } => *span,
            ASTNode::BinaryExpr { span, .. } => *span,
            ASTNode::CallExpr { span, .. } => *span,
            ASTNode::Identifier { span, .. } => *span,
            ASTNode::NumberLiteral { span, .. } => *span,
        }
    }
}

/// Complete AST with root node
#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub root: ASTNode,
}

impl AST {
    pub fn new(root: ASTNode) -> Self {
        Self { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_number_literal() {
        let node = ASTNode::NumberLiteral {
            value: 42.0,
            span: Span::new(0, 2),
        };
        assert_eq!(node.span(), Span::new(0, 2));
    }
    
    #[test]
    fn test_identifier() {
        let node = ASTNode::Identifier {
            name: "x".to_string(),
            span: Span::new(0, 1),
        };
        assert_eq!(node.span(), Span::new(0, 1));
    }
    
    #[test]
    fn test_binary_expr() {
        let left = Box::new(ASTNode::NumberLiteral {
            value: 1.0,
            span: Span::new(0, 1),
        });
        let right = Box::new(ASTNode::NumberLiteral {
            value: 2.0,
            span: Span::new(4, 5),
        });
        
        let node = ASTNode::BinaryExpr {
            op: BinOp::Add,
            left,
            right,
            span: Span::new(0, 5),
        };
        
        assert_eq!(node.span(), Span::new(0, 5));
    }
    
    #[test]
    fn test_let_decl() {
        let init = Box::new(ASTNode::NumberLiteral {
            value: 10.0,
            span: Span::new(8, 10),
        });
        
        let node = ASTNode::LetDecl {
            name: "x".to_string(),
            init,
            span: Span::new(0, 10),
        };
        
        assert_eq!(node.span(), Span::new(0, 10));
    }
    
    #[test]
    fn test_function_decl() {
        let body = Box::new(ASTNode::BlockStmt {
            statements: vec![],
            span: Span::new(20, 22),
        });
        
        let node = ASTNode::FunctionDecl {
            name: "foo".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body,
            span: Span::new(0, 22),
        };
        
        assert_eq!(node.span(), Span::new(0, 22));
    }
    
    #[test]
    fn test_ast_creation() {
        let root = ASTNode::Program(vec![
            ASTNode::NumberLiteral {
                value: 42.0,
                span: Span::new(0, 2),
            }
        ]);
        
        let ast = AST::new(root);
        assert!(matches!(ast.root, ASTNode::Program(_)));
    }
}
