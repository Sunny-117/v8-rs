# 第 4 章：递归下降解析器

## 本章目标

在本章中，我们将实现递归下降解析器，它负责：
1. 将 Token 流转换为 AST
2. 处理运算符优先级
3. 解析各种语句和表达式
4. 提供清晰的错误信息

## 什么是递归下降解析？

递归下降是一种自顶向下的解析技术。每个语法规则对应一个函数，函数之间相互调用形成递归结构。

**优点：**
- 易于理解和实现
- 错误处理灵活
- 调试方便

**缺点：**
- 不能处理左递归
- 可能需要回溯

## 步骤 1：Parser 结构

```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    eof_token: Token,
}
```

**字段说明：**
- `tokens`：Token 数组
- `position`：当前读取位置
- `eof_token`：预先创建的 EOF token（避免生命周期问题）

### 初始化

```rust
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
}
```

**为什么在 Parser 中创建 Lexer？**
- 简化 API：用户只需要调用 `Parser::new`
- 词法分析和语法分析紧密相关

## 步骤 2：基础操作

### 2.1 查看当前 Token

```rust
fn current(&self) -> &Token {
    self.tokens.get(self.position).unwrap_or(&self.eof_token)
}
```

### 2.2 前进到下一个 Token

```rust
fn advance(&mut self) -> Token {
    let token = self.current().clone();
    self.position += 1;
    token
}
```

### 2.3 检查 Token 类型

```rust
fn check(&self, kind: &TokenKind) -> bool {
    std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
}
```

**为什么使用 discriminant？**
- 只比较枚举的变体，不比较内部数据
- 例如：`check(&TokenKind::Number(0.0))` 会匹配任何数字

### 2.4 期望特定 Token

```rust
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
```

**用途：**
- 解析固定的语法元素（如分号、括号）
- 自动生成错误信息

## 步骤 3：解析表达式

### 3.1 运算符优先级

JavaScript 的运算符优先级（从低到高）：
1. 赋值：`=`
2. 比较：`==`, `<`, `>`
3. 加减：`+`, `-`
4. 乘除：`*`, `/`
5. 一元：`-`, `!`
6. 调用：`()`
7. 基本：数字、标识符、括号

### 3.2 解析加法表达式

```rust
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
```

**算法解释：**

1. 解析左操作数（更高优先级）
2. 循环：
   - 如果看到 `+` 或 `-`
   - 解析右操作数
   - 创建二元表达式节点
   - 将结果作为新的左操作数
3. 返回最终结果

**例子：** `1 + 2 + 3`

```
第一次循环：
left = 1
看到 +
right = 2
left = (1 + 2)

第二次循环：
left = (1 + 2)
看到 +
right = 3
left = ((1 + 2) + 3)
```

### 3.3 解析乘法表达式

```rust
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
```

**为什么乘法调用 parse_call？**
- 因为函数调用的优先级更高
- 例如：`foo() * 2` 应该先调用 `foo()`

### 3.4 解析函数调用

```rust
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
```

**支持链式调用：**
```javascript
foo()()  // 调用 foo 的返回值
```

### 3.5 解析基本表达式

```rust
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
```

**处理括号：**
- 括号改变优先级
- 递归调用 `parse_expression` 解析括号内的表达式

## 步骤 4：解析语句

### 4.1 Let 声明

```rust
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
```

**语法：** `let identifier = expression;`

### 4.2 If 语句

```rust
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
```

**语法：** `if (condition) { ... } else { ... }`

### 4.3 For 循环

```rust
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
```

**语法：** `for (init; condition; update) { ... }`

### 4.4 块语句

```rust
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
```

**语法：** `{ statement1; statement2; ... }`

## 步骤 5：顶层解析

```rust
pub fn parse(&mut self) -> Result<AST, ParseError> {
    let mut statements = Vec::new();
    
    while !matches!(self.current().kind, TokenKind::Eof) {
        statements.push(self.parse_statement()?);
    }
    
    Ok(AST::new(ASTNode::Program(statements)))
}
```

## 错误处理

### 同步点

当遇到错误时，解析器需要恢复到已知状态：

```rust
fn synchronize(&mut self) {
    self.advance();
    
    while !matches!(self.current().kind, TokenKind::Eof) {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            return;
        }
        
        match self.current().kind {
            TokenKind::Let | TokenKind::Function | TokenKind::If | TokenKind::For => {
                return;
            }
            _ => {}
        }
        
        self.advance();
    }
}
```

### 错误恢复

```rust
fn parse_statement(&mut self) -> Result<ASTNode, ParseError> {
    match self.parse_statement_inner() {
        Ok(stmt) => Ok(stmt),
        Err(err) => {
            eprintln!("Parse error: {}", err);
            self.synchronize();
            Err(err)
        }
    }
}
```

## 测试解析器

### 测试简单表达式

```rust
#[test]
fn test_parse_number() {
    let mut parser = Parser::new("42".to_string());
    let ast = parser.parse().unwrap();
    
    if let ASTNode::Program(stmts) = ast.root {
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], ASTNode::NumberLiteral { value: 42.0, .. }));
    }
}
```

### 测试二元表达式

```rust
#[test]
fn test_parse_binary_expr() {
    let mut parser = Parser::new("1 + 2".to_string());
    let ast = parser.parse().unwrap();
    
    if let ASTNode::Program(stmts) = ast.root {
        assert!(matches!(stmts[0], ASTNode::BinaryExpr { op: BinOp::Add, .. }));
    }
}
```

### 测试错误处理

```rust
#[test]
fn test_parse_error() {
    let mut parser = Parser::new("let = 10".to_string());
    let result = parser.parse();
    
    assert!(result.is_err());
    if let Err(ParseError::UnexpectedToken { expected, .. }) = result {
        assert!(expected.contains("identifier"));
    }
}
```

## 调试技巧

### 打印解析过程

```rust
fn parse_expression(&mut self) -> Result<ASTNode, ParseError> {
    println!("Parsing expression at token: {:?}", self.current());
    let result = self.parse_additive()?;
    println!("Parsed expression: {:?}", result);
    Ok(result)
}
```

### 可视化 AST

```rust
fn print_ast(node: &ASTNode, indent: usize) {
    let prefix = "  ".repeat(indent);
    match node {
        ASTNode::BinaryExpr { op, left, right, .. } => {
            println!("{}BinaryExpr({:?})", prefix, op);
            print_ast(left, indent + 1);
            print_ast(right, indent + 1);
        }
        ASTNode::NumberLiteral { value, .. } => {
            println!("{}Number({})", prefix, value);
        }
        // ... 其他节点
    }
}
```

## 常见问题

### Q1: 如何处理左递归？

**答：** 将左递归改写为循环。例如：
```
// 左递归（不行）
expr := expr + term | term

// 改写为循环（可以）
expr := term (+ term)*
```

### Q2: 如何添加新的运算符？

**答：** 
1. 在 Lexer 中添加 Token 类型
2. 在 BinOp 中添加运算符
3. 在相应优先级的解析函数中处理

### Q3: 如何支持三元运算符？

**答：**
```rust
fn parse_ternary(&mut self) -> Result<ASTNode, ParseError> {
    let cond = self.parse_logical_or()?;
    
    if matches!(self.current().kind, TokenKind::Question) {
        self.advance();
        let then_expr = self.parse_expression()?;
        self.expect(TokenKind::Colon)?;
        let else_expr = self.parse_ternary()?;
        
        // 创建 TernaryExpr 节点
    }
    
    Ok(cond)
}
```

## 下一步

在下一章中，我们将实现作用域系统，为变量解析做准备。

## 练习

1. 添加对一元运算符（`-x`, `!x`）的支持
2. 实现 while 循环的解析
3. 添加对数组字面量的支持（`[1, 2, 3]`）
4. 实现更好的错误恢复机制

## 完整代码

本章的完整代码在 `src/parser.rs` 文件中。
