// Recursive descent parser for JavaScript

use crate::ast::{AST, ASTNode, BinOp};
use crate::error::ParseError;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::types::Span;

/// Parser for converting tokens into AST
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    eof_token: Token,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Self {
            tokens,
            position: 0,
            eof_token: Token::new(TokenKind::Eof, Span::new(0, 0)),
        }
    }
    
    /// Get the current token
    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&self.eof_token)
    }
    
    /// Peek at the next token
    fn peek(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&self.eof_token)
    }
    
    /// Advance to the next token
    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        self.position += 1;
        token
    }
    
    /// Check if current token matches the expected kind
    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
    }
    
    /// Consume a token if it matches, otherwise error
    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found: format!("{:?}", self.current().kind),
                span: self.current().span,
            })
        }
    }
    
    /// Parse the entire program
    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let mut statements = Vec::new();
        
        while !matches!(self.current().kind, TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }
        
        Ok(AST::new(ASTNode::Program(statements)))
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current().kind {
            TokenKind::Let => self.parse_let_decl(),
            TokenKind::Function => self.parse_function_decl(),
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::For => self.parse_for_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::LeftBrace => self.parse_block_stmt(),
            _ => {
                // Expression statement
                let expr = self.parse_expression()?;
                if matches!(self.current().kind, TokenKind::Semicolon) {
                    self.advance();
                }
                Ok(expr)
            }
        }
    }
    
    /// Parse let declaration: let x = expr;
    fn parse_let_decl(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::Let)?;
        
        let name = match &self.current().kind {
            TokenKind::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.current().kind),
                    span: self.current().span,
                });
            }
        };
        
        self.expect(TokenKind::Equal)?;
        let init = Box::new(self.parse_expression()?);
        
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
        }
        
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::LetDecl {
            name,
            init,
            span: Span::new(start, end),
        })
    }
    
    /// Parse function declaration: function name(params) { body }
    fn parse_function_decl(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::Function)?;
        
        let name = match &self.current().kind {
            TokenKind::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.current().kind),
                    span: self.current().span,
                });
            }
        };
        
        self.expect(TokenKind::LeftParen)?;
        
        let mut params = Vec::new();
        while !matches!(self.current().kind, TokenKind::RightParen) {
            if let TokenKind::Identifier(param) = &self.current().kind {
                params.push(param.clone());
                self.advance();
                
                if matches!(self.current().kind, TokenKind::Comma) {
                    self.advance();
                }
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "parameter".to_string(),
                    found: format!("{:?}", self.current().kind),
                    span: self.current().span,
                });
            }
        }
        
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_block_stmt()?);
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::FunctionDecl {
            name,
            params,
            body,
            span: Span::new(start, end),
        })
    }
    
    /// Parse if statement: if (cond) { then } else { else }
    fn parse_if_stmt(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::If)?;
        
        self.expect(TokenKind::LeftParen)?;
        let cond = Box::new(self.parse_expression()?);
        self.expect(TokenKind::RightParen)?;
        
        let then_branch = Box::new(self.parse_block_stmt()?);
        
        let else_branch = if matches!(self.current().kind, TokenKind::Else) {
            self.advance();
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };
        
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::IfStmt {
            cond,
            then_branch,
            else_branch,
            span: Span::new(start, end),
        })
    }
    
    /// Parse for loop: for (init; cond; update) { body }
    fn parse_for_stmt(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::For)?;
        
        self.expect(TokenKind::LeftParen)?;
        
        let init = Box::new(self.parse_statement()?);
        let cond = Box::new(self.parse_expression()?);
        self.expect(TokenKind::Semicolon)?;
        let update = Box::new(self.parse_expression()?);
        
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_block_stmt()?);
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::ForStmt {
            init,
            cond,
            update,
            body,
            span: Span::new(start, end),
        })
    }
    
    /// Parse return statement: return expr;
    fn parse_return_stmt(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::Return)?;
        
        let value = Box::new(self.parse_expression()?);
        
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
        }
        
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::ReturnStmt {
            value,
            span: Span::new(start, end),
        })
    }
    
    /// Parse block statement: { statements }
    fn parse_block_stmt(&mut self) -> Result<ASTNode, ParseError> {
        let start = self.current().span.start;
        self.expect(TokenKind::LeftBrace)?;
        
        let mut statements = Vec::new();
        while !matches!(self.current().kind, TokenKind::RightBrace | TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }
        
        self.expect(TokenKind::RightBrace)?;
        let end = self.tokens.get(self.position.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        
        Ok(ASTNode::BlockStmt {
            statements,
            span: Span::new(start, end),
        })
    }
    
    /// Parse expression
    fn parse_expression(&mut self) -> Result<ASTNode, ParseError> {
        self.parse_additive()
    }
    
    /// Parse additive expression: term ((+|-) term)*
    fn parse_additive(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_multiplicative()?;
        
        while matches!(self.current().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.current().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_multiplicative()?;
            let span = left.span().merge(right.span());
            
            left = ASTNode::BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    /// Parse multiplicative expression: primary ((*|/) primary)*
    fn parse_multiplicative(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_call()?;
        
        while matches!(self.current().kind, TokenKind::Star | TokenKind::Slash) {
            let op = match self.current().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_call()?;
            let span = left.span().merge(right.span());
            
            left = ASTNode::BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    /// Parse call expression: primary(args)
    fn parse_call(&mut self) -> Result<ASTNode, ParseError> {
        let mut expr = self.parse_primary()?;
        
        while matches!(self.current().kind, TokenKind::LeftParen) {
            self.advance();
            
            let mut args = Vec::new();
            while !matches!(self.current().kind, TokenKind::RightParen) {
                args.push(self.parse_expression()?);
                
                if matches!(self.current().kind, TokenKind::Comma) {
                    self.advance();
                }
            }
            
            let end_span = self.current().span;
            self.expect(TokenKind::RightParen)?;
            
            let span = expr.span().merge(end_span);
            expr = ASTNode::CallExpr {
                callee: Box::new(expr),
                args,
                span,
            };
        }
        
        Ok(expr)
    }
    
    /// Parse primary expression: number | identifier | (expr)
    fn parse_primary(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current().kind {
            TokenKind::Number(n) => {
                let value = *n;
                let span = self.current().span;
                self.advance();
                Ok(ASTNode::NumberLiteral { value, span })
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                let span = self.current().span;
                self.advance();
                Ok(ASTNode::Identifier { name, span })
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: format!("{:?}", self.current().kind),
                span: self.current().span,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("42".to_string());
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(stmts) = ast.root {
            assert_eq!(stmts.len(), 1);
            assert!(matches!(stmts[0], ASTNode::NumberLiteral { value: 42.0, .. }));
        } else {
            panic!("Expected Program node");
        }
    }
    
    #[test]
    fn test_parse_binary_expr() {
        let mut parser = Parser::new("1 + 2".to_string());
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(stmts) = ast.root {
            assert_eq!(stmts.len(), 1);
            assert!(matches!(stmts[0], ASTNode::BinaryExpr { op: BinOp::Add, .. }));
        } else {
            panic!("Expected Program node");
        }
    }
    
    #[test]
    fn test_parse_let_decl() {
        let mut parser = Parser::new("let x = 10;".to_string());
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(stmts) = ast.root {
            assert_eq!(stmts.len(), 1);
            if let ASTNode::LetDecl { name, .. } = &stmts[0] {
                assert_eq!(name, "x");
            } else {
                panic!("Expected LetDecl node");
            }
        } else {
            panic!("Expected Program node");
        }
    }
    
    #[test]
    fn test_parse_function_decl() {
        let mut parser = Parser::new("function add(a, b) { return a + b; }".to_string());
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(stmts) = ast.root {
            assert_eq!(stmts.len(), 1);
            if let ASTNode::FunctionDecl { name, params, .. } = &stmts[0] {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            } else {
                panic!("Expected FunctionDecl node");
            }
        } else {
            panic!("Expected Program node");
        }
    }
    
    #[test]
    fn test_parse_call_expr() {
        let mut parser = Parser::new("foo(1, 2)".to_string());
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(stmts) = ast.root {
            assert_eq!(stmts.len(), 1);
            if let ASTNode::CallExpr { args, .. } = &stmts[0] {
                assert_eq!(args.len(), 2);
            } else {
                panic!("Expected CallExpr node");
            }
        } else {
            panic!("Expected Program node");
        }
    }
    
    #[test]
    fn test_parse_error_unexpected_token() {
        let mut parser = Parser::new("let = 10".to_string());
        let result = parser.parse();
        
        assert!(result.is_err());
        if let Err(ParseError::UnexpectedToken { expected, .. }) = result {
            assert!(expected.contains("identifier"));
        } else {
            panic!("Expected UnexpectedToken error");
        }
    }
    
    #[test]
    fn test_parse_error_invalid_syntax() {
        let mut parser = Parser::new("function { }".to_string());
        let result = parser.parse();
        
        assert!(result.is_err());
    }
}
