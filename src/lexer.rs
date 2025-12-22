// Lexer for tokenizing JavaScript source code

use crate::types::Span;

/// Token types supported by the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    Identifier(String),
    
    // Keywords
    Let,
    Function,
    If,
    Else,
    For,
    Return,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Less,
    Greater,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    
    // Special
    Eof,
}

/// A token with its kind and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Lexer for scanning source code into tokens
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let current_char = chars.get(0).copied();
        Self {
            source: chars,
            position: 0,
            current_char,
        }
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.source.get(self.position).copied();
    }
    
    /// Peek at the next character without advancing
    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Scan a number literal
    fn scan_number(&mut self) -> Token {
        let start = self.position;
        let mut num_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let value = num_str.parse::<f64>().unwrap_or(0.0);
        Token::new(TokenKind::Number(value), Span::new(start, self.position))
    }
    
    /// Scan an identifier or keyword
    fn scan_identifier(&mut self) -> Token {
        let start = self.position;
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "function" => TokenKind::Function,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier(ident),
        };
        
        Token::new(kind, Span::new(start, self.position))
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let Some(ch) = self.current_char else {
            return Token::new(TokenKind::Eof, Span::new(self.position, self.position));
        };
        
        let start = self.position;
        
        // Numbers
        if ch.is_ascii_digit() {
            return self.scan_number();
        }
        
        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.scan_identifier();
        }
        
        // Operators and delimiters
        let kind = match ch {
            '+' => {
                self.advance();
                TokenKind::Plus
            }
            '-' => {
                self.advance();
                TokenKind::Minus
            }
            '*' => {
                self.advance();
                TokenKind::Star
            }
            '/' => {
                self.advance();
                TokenKind::Slash
            }
            '=' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                self.advance();
                TokenKind::Less
            }
            '>' => {
                self.advance();
                TokenKind::Greater
            }
            '(' => {
                self.advance();
                TokenKind::LeftParen
            }
            ')' => {
                self.advance();
                TokenKind::RightParen
            }
            '{' => {
                self.advance();
                TokenKind::LeftBrace
            }
            '}' => {
                self.advance();
                TokenKind::RightBrace
            }
            ';' => {
                self.advance();
                TokenKind::Semicolon
            }
            ',' => {
                self.advance();
                TokenKind::Comma
            }
            _ => {
                self.advance();
                // For unsupported characters, return an identifier with the char
                TokenKind::Identifier(ch.to_string())
            }
        };
        
        Token::new(kind, Span::new(start, self.position))
    }
    
    /// Tokenize the entire source into a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("42 3.14".to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 3); // 2 numbers + EOF
        assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[1].kind, TokenKind::Number(3.14));
    }
    
    #[test]
    fn test_tokenize_identifiers() {
        let mut lexer = Lexer::new("foo bar_123".to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Identifier("foo".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("bar_123".to_string()));
    }
    
    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("let function if else for return".to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Function);
        assert_eq!(tokens[2].kind, TokenKind::If);
        assert_eq!(tokens[3].kind, TokenKind::Else);
        assert_eq!(tokens[4].kind, TokenKind::For);
        assert_eq!(tokens[5].kind, TokenKind::Return);
    }
    
    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / = == < >".to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::Equal);
        assert_eq!(tokens[5].kind, TokenKind::EqualEqual);
    }
    
    #[test]
    fn test_tokenize_expression() {
        let mut lexer = Lexer::new("let x = 10 + 20;".to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 8); // 7 tokens + EOF
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[3].kind, TokenKind::Number(10.0));
        assert_eq!(tokens[4].kind, TokenKind::Plus);
        assert_eq!(tokens[5].kind, TokenKind::Number(20.0));
        assert_eq!(tokens[6].kind, TokenKind::Semicolon);
        assert_eq!(tokens[7].kind, TokenKind::Eof);
    }
}
