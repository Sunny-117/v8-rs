# 第 2 章：词法分析器（Lexer）

## 本章目标

在本章中，我们将实现词法分析器，它负责：
1. 将源代码字符串分解为 Token 流
2. 识别数字、标识符、关键字和运算符
3. 跳过空白字符
4. 记录每个 Token 的位置信息

## 什么是词法分析？

词法分析是编译器的第一步。它将源代码从字符流转换为 Token 流。

**例子：**
```javascript
let x = 10 + 20;
```

词法分析后变成：
```
[Let, Identifier("x"), Equal, Number(10), Plus, Number(20), Semicolon, Eof]
```

## 步骤 1：设计 Token 类型

### 1.1 TokenKind 枚举

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // 字面量
    Number(f64),
    Identifier(String),
    
    // 关键字
    Let,
    Function,
    If,
    Else,
    For,
    Return,
    
    // 运算符
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Equal,     // =
    EqualEqual, // ==
    
    // 分隔符
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Semicolon,  // ;
    Comma,      // ,
    
    // 特殊
    Eof,
}
```

**设计思考：**

1. **为什么 Number 和 Identifier 包含数据？**
   - 我们需要知道具体的数字值和标识符名称
   - 其他 Token 只需要知道类型即可

2. **为什么需要 Eof？**
   - 标记输入结束
   - 简化解析器逻辑（不需要检查 None）

### 1.2 Token 结构体

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
```

每个 Token 都包含：
- `kind`：Token 的类型和值
- `span`：在源代码中的位置

## 步骤 2：实现 Lexer

### 2.1 Lexer 结构体

```rust
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    current_char: Option<char>,
}
```

**字段说明：**
- `source`：源代码的字符数组（方便索引）
- `position`：当前读取位置
- `current_char`：当前字符（None 表示结束）

### 2.2 初始化

```rust
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
}
```

**为什么转换为 Vec<char>？**
- Rust 的字符串是 UTF-8 编码，不能直接索引
- 转换为字符数组后可以高效访问任意位置

### 2.3 基础操作

```rust
fn advance(&mut self) {
    self.position += 1;
    self.current_char = self.source.get(self.position).copied();
}

fn peek(&self) -> Option<char> {
    self.source.get(self.position + 1).copied()
}
```

**advance**：移动到下一个字符
**peek**：查看下一个字符但不移动

### 2.4 跳过空白

```rust
fn skip_whitespace(&mut self) {
    while let Some(ch) = self.current_char {
        if ch.is_whitespace() {
            self.advance();
        } else {
            break;
        }
    }
}
```

**为什么要跳过空白？**
- 空白字符（空格、制表符、换行）在 JavaScript 中通常没有语义
- 跳过它们简化后续处理

## 步骤 3：扫描不同类型的 Token

### 3.1 扫描数字

```rust
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
```

**算法流程：**
1. 记录起始位置
2. 收集所有数字和小数点
3. 解析为 f64
4. 创建 Token

**支持的格式：**
- 整数：`42`
- 浮点数：`3.14`

### 3.2 扫描标识符和关键字

```rust
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
```

**关键字识别：**
- 先按标识符规则扫描
- 然后检查是否是关键字
- 如果不是，就是普通标识符

**标识符规则：**
- 可以包含字母、数字、下划线
- 不能以数字开头（由调用者保证）

### 3.3 扫描运算符和分隔符

```rust
pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    
    let Some(ch) = self.current_char else {
        return Token::new(TokenKind::Eof, Span::new(self.position, self.position));
    };
    
    let start = self.position;
    
    // 数字
    if ch.is_ascii_digit() {
        return self.scan_number();
    }
    
    // 标识符和关键字
    if ch.is_alphabetic() || ch == '_' {
        return self.scan_identifier();
    }
    
    // 运算符和分隔符
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
        // ... 其他运算符
        _ => {
            self.advance();
            TokenKind::Identifier(ch.to_string())
        }
    };
    
    Token::new(kind, Span::new(start, self.position))
}
```

**特殊处理：== 运算符**
- 看到 `=` 后，检查下一个字符
- 如果也是 `=`，返回 `EqualEqual`
- 否则返回 `Equal`

这种技术叫做"向前看"（lookahead）。

## 步骤 4：完整的词法分析

```rust
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
```

这个方法将整个源代码转换为 Token 数组。

## 测试驱动开发

### 测试数字

```rust
#[test]
fn test_tokenize_numbers() {
    let mut lexer = Lexer::new("42 3.14".to_string());
    let tokens = lexer.tokenize();
    
    assert_eq!(tokens.len(), 3); // 2 numbers + EOF
    assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
    assert_eq!(tokens[1].kind, TokenKind::Number(3.14));
}
```

### 测试关键字

```rust
#[test]
fn test_tokenize_keywords() {
    let mut lexer = Lexer::new("let function if".to_string());
    let tokens = lexer.tokenize();
    
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Function);
    assert_eq!(tokens[2].kind, TokenKind::If);
}
```

### 测试完整表达式

```rust
#[test]
fn test_tokenize_expression() {
    let mut lexer = Lexer::new("let x = 10 + 20;".to_string());
    let tokens = lexer.tokenize();
    
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::Equal);
    assert_eq!(tokens[3].kind, TokenKind::Number(10.0));
    assert_eq!(tokens[4].kind, TokenKind::Plus);
    assert_eq!(tokens[5].kind, TokenKind::Number(20.0));
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
    assert_eq!(tokens[7].kind, TokenKind::Eof);
}
```

## 常见问题

### Q1: 为什么不用正则表达式？

**答：** 虽然正则表达式可以简化词法分析，但：
1. 手写词法分析器更快
2. 错误处理更灵活
3. 更容易理解和调试

### Q2: 如何处理注释？

**答：** 在 `skip_whitespace` 中添加注释处理：

```rust
fn skip_whitespace(&mut self) {
    while let Some(ch) = self.current_char {
        if ch.is_whitespace() {
            self.advance();
        } else if ch == '/' && self.peek() == Some('/') {
            // 跳过单行注释
            while self.current_char != Some('\n') && self.current_char.is_some() {
                self.advance();
            }
        } else {
            break;
        }
    }
}
```

### Q3: 如何处理字符串字面量？

**答：** 添加一个 `scan_string` 方法：

```rust
fn scan_string(&mut self) -> Token {
    let start = self.position;
    self.advance(); // 跳过开始的引号
    
    let mut string = String::new();
    while let Some(ch) = self.current_char {
        if ch == '"' {
            self.advance(); // 跳过结束的引号
            break;
        }
        string.push(ch);
        self.advance();
    }
    
    Token::new(TokenKind::String(string), Span::new(start, self.position))
}
```

## 性能优化

### 使用字符数组而不是字符串切片

```rust
// 慢：每次都要重新计算 UTF-8 边界
let ch = source[position..position+1].chars().next();

// 快：直接索引
let ch = source[position];
```

### 预分配 Token 数组

```rust
pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(self.source.len() / 4);
    // 估计平均每 4 个字符一个 Token
    // ...
}
```

## 调试技巧

### 打印 Token 流

```rust
fn main() {
    let mut lexer = Lexer::new("let x = 10;".to_string());
    let tokens = lexer.tokenize();
    
    for token in tokens {
        println!("{:?}", token);
    }
}
```

输出：
```
Token { kind: Let, span: Span { start: 0, end: 3 } }
Token { kind: Identifier("x"), span: Span { start: 4, end: 5 } }
Token { kind: Equal, span: Span { start: 6, end: 7 } }
Token { kind: Number(10.0), span: Span { start: 8, end: 10 } }
Token { kind: Semicolon, span: Span { start: 10, end: 11 } }
Token { kind: Eof, span: Span { start: 11, end: 11 } }
```

## 下一步

在下一章中，我们将定义抽象语法树（AST），为解析器做准备。

## 练习

1. 添加对十六进制数字的支持（如 `0xFF`）
2. 添加对科学计数法的支持（如 `1e10`）
3. 实现字符串字面量的词法分析
4. 添加对多行注释的支持（`/* ... */`）

## 完整代码

本章的完整代码在 `src/lexer.rs` 文件中。
