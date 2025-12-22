# 第 1 章：项目初始化和基础结构

## 本章目标

在本章中，我们将：
1. 创建 Rust 项目结构
2. 配置依赖项（用于属性测试）
3. 定义核心数据类型（Value、Span、Error）

## 为什么从这里开始？

在构建任何复杂系统之前，我们需要先建立坚实的基础。对于 JavaScript 引擎来说，这个基础包括：
- **数据类型**：如何表示 JavaScript 的值？
- **位置信息**：如何追踪代码在源文件中的位置？
- **错误处理**：如何优雅地处理各种错误？

## 步骤 1：创建 Rust 项目

```bash
cargo init --name v8_rs
```

这会创建一个标准的 Rust 项目结构：
```
v8-rs/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs (我们将创建)
```

### 配置 Cargo.toml

```toml
[package]
name = "v8_rs"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
quickcheck = "1.0"
quickcheck_macros = "1.0"
```

**为什么选择这些依赖？**
- `quickcheck`：属性测试框架，用于生成随机测试用例
- `quickcheck_macros`：简化属性测试的宏

## 步骤 2：设计核心数据类型

### 2.1 Value 类型 (src/types.rs)

JavaScript 是动态类型语言，一个变量可以存储不同类型的值。我们需要一个枚举来表示所有可能的值类型：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Function(FunctionId),
    Undefined,
}
```

**设计思考：**

1. **为什么用 f64？**
   - JavaScript 的所有数字都是 64 位浮点数
   - 这简化了实现，避免整数和浮点数的转换

2. **为什么 Function 只存储 ID？**
   - 函数可能很大，直接存储会导致大量复制
   - 使用 ID 引用，实际函数存储在其他地方

3. **为什么需要 Undefined？**
   - JavaScript 中未初始化的变量值为 undefined
   - 函数没有返回值时返回 undefined

**派生的 trait：**
- `Debug`：方便调试打印
- `Clone`：值需要在栈上复制
- `PartialEq`：比较两个值是否相等

### 2.2 Span 类型

当解析器遇到错误时，我们需要告诉用户错误发生在哪里。Span 记录了代码片段在源文件中的位置：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```

**实用方法：**

```rust
impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
```

**merge 方法的用途：**
当我们解析一个表达式如 `1 + 2` 时：
- `1` 的 span 是 `Span { start: 0, end: 1 }`
- `2` 的 span 是 `Span { start: 4, end: 5 }`
- 整个表达式的 span 是 `Span { start: 0, end: 5 }`

## 步骤 3：设计错误类型 (src/error.rs)

一个好的错误系统应该：
1. **分类清晰**：不同阶段的错误分开
2. **信息丰富**：包含足够的上下文
3. **易于使用**：支持 `?` 操作符

### 3.1 顶层错误类型

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    CompileError(CompileError),
}
```

这个设计将错误分为三类：
- **ParseError**：源代码语法错误
- **RuntimeError**：执行时错误
- **CompileError**：JIT 编译错误

### 3.2 ParseError

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    UnexpectedEOF,
    InvalidSyntax {
        message: String,
        span: Span,
    },
}
```

**示例错误信息：**
```
Expected 'identifier', found 'number' at 4:8
```

### 3.3 RuntimeError

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    UndefinedVariable { name: String },
    TypeError { expected: String, found: String },
    StackOverflow,
    DivisionByZero,
}
```

**为什么需要这些错误？**
- `UndefinedVariable`：访问未声明的变量
- `TypeError`：类型不匹配（如字符串 + 数字）
- `StackOverflow`：递归太深
- `DivisionByZero`：除以零

### 3.4 Display 实现

为了让错误信息更友好，我们实现 `Display` trait：

```rust
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, span } => {
                write!(f, "Expected '{}', found '{}' at {}:{}", 
                       expected, found, span.start, span.end)
            }
            // ...
        }
    }
}
```

### 3.5 错误转换

为了支持 `?` 操作符，我们实现 `From` trait：

```rust
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}
```

这样我们就可以写：
```rust
fn parse() -> Result<AST, Error> {
    let token = lexer.next_token()?; // ParseError 自动转换为 Error
    // ...
}
```

## 步骤 4：模块组织 (src/lib.rs)

```rust
pub mod types;
pub mod error;

pub use types::{Value, Span};
pub use error::{Error, ParseError, RuntimeError, CompileError};
```

**为什么要 re-export？**
- 用户可以写 `use v8_rs::Value` 而不是 `use v8_rs::types::Value`
- 简化 API，隐藏内部模块结构

## 测试驱动开发

### 测试 Value 类型

```rust
#[test]
fn test_value_creation() {
    let num = Value::Number(42.0);
    assert_eq!(num, Value::Number(42.0));
    
    let func = Value::Function(0);
    assert_eq!(func, Value::Function(0));
}
```

### 测试 Span 合并

```rust
#[test]
fn test_span_merge() {
    let span1 = Span::new(0, 5);
    let span2 = Span::new(3, 10);
    let merged = span1.merge(span2);
    assert_eq!(merged.start, 0);
    assert_eq!(merged.end, 10);
}
```

### 测试错误转换

```rust
#[test]
fn test_error_conversion() {
    let parse_err = ParseError::UnexpectedEOF;
    let err: Error = parse_err.into();
    assert!(matches!(err, Error::ParseError(_)));
}
```

## 运行测试

```bash
cargo test --lib
```

你应该看到所有测试通过：
```
running 7 tests
test types::tests::test_value_creation ... ok
test types::tests::test_span_creation ... ok
test types::tests::test_span_merge ... ok
test error::tests::test_parse_error_creation ... ok
test error::tests::test_runtime_error_creation ... ok
test error::tests::test_error_conversion ... ok
test error::tests::test_error_display ... ok

test result: ok. 7 passed; 0 failed
```

## 设计原则总结

1. **类型安全**：使用 Rust 的类型系统防止错误
2. **明确性**：每个类型都有清晰的用途
3. **可扩展性**：容易添加新的值类型或错误类型
4. **人性化**：错误信息包含位置和上下文

## 下一步

在下一章中，我们将实现词法分析器（Lexer），将源代码字符串转换为 Token 流。

## 练习

1. 添加一个新的 Value 类型：`Boolean(bool)`
2. 添加一个新的 RuntimeError：`IndexOutOfBounds`
3. 为 Span 添加一个方法 `contains(pos: usize) -> bool`，检查位置是否在 span 内

## 完整代码

本章的完整代码可以在以下文件中找到：
- `src/types.rs`
- `src/error.rs`
- `src/lib.rs`
