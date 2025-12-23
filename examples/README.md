# V8-RS Examples / 示例

This directory contains example JavaScript files that demonstrate V8-RS capabilities.

本目录包含演示 V8-RS 功能的 JavaScript 示例文件。

## Running Examples / 运行示例

```bash
# Build the project first / 首先构建项目
cargo build --release

# Run any example / 运行任意示例
./target/release/v8_rs examples/<filename>.js
```

## Available Examples / 可用示例

### 1. hello.js
Basic variable declaration and arithmetic.
基本变量声明和算术运算。

```bash
$ ./target/release/v8_rs examples/hello.js
84
```

### 2. arithmetic.js
Multiple variables and complex arithmetic expressions.
多个变量和复杂算术表达式。

```bash
$ ./target/release/v8_rs examples/arithmetic.js
750
```

### 3. fibonacci.js
Computing Fibonacci sequence numbers using variables.
使用变量计算斐波那契数列。

```bash
$ ./target/release/v8_rs examples/fibonacci.js
8
```

### 4. error.js
Demonstrates runtime error handling (division by zero).
演示运行时错误处理（除以零）。

```bash
$ ./target/release/v8_rs examples/error.js
Error: Runtime error: Division by zero
```

### 5. basic.rs
Rust example showing how to use V8-RS as a library.
Rust 示例，展示如何将 V8-RS 用作库。

```bash
$ cargo run --example basic
```

## Creating Your Own Examples / 创建自己的示例

Create a new `.js` file with JavaScript code:

创建一个包含 JavaScript 代码的新 `.js` 文件：

```javascript
let x = 10;
let y = 20;
x + y
```

Run it:

运行它：

```bash
./target/release/v8_rs your_example.js
```

## Supported Features / 支持的特性

Current examples demonstrate:
当前示例演示：

- ✅ Number literals / 数字字面量
- ✅ Variable declarations (`let`) / 变量声明
- ✅ Arithmetic operations (`+`, `-`, `*`, `/`) / 算术运算
- ✅ Complex expressions / 复杂表达式
- ✅ Error handling / 错误处理

## Limitations / 限制

Current version does not support:
当前版本不支持：

- ❌ Comments (`//`, `/* */`) / 注释
- ❌ Strings / 字符串
- ❌ Booleans / 布尔值
- ❌ Functions / 函数
- ❌ Control flow (if, while, for) / 控制流
- ❌ Objects and arrays / 对象和数组

These features will be added in future versions.
这些特性将在未来版本中添加。
