# V8-RS 使用指南 / Usage Guide

[中文](#中文) | [English](#english)

---

## 中文

### 安装和构建

#### 1. 克隆项目

```bash
git clone <repo-url>
cd v8-rs
```

#### 2. 构建项目

```bash
# 开发版本（包含调试信息）
cargo build

# 发布版本（优化性能）
cargo build --release
```

构建完成后，可执行文件位于：
- 开发版本：`./target/debug/v8_rs`
- 发布版本：`./target/release/v8_rs`

### 使用方式

V8-RS 支持两种使用方式：

#### 1. 交互式 REPL 模式

不带参数运行，启动交互式 JavaScript 解释器：

```bash
$ ./target/release/v8_rs
V8-RS JavaScript Engine v0.1.0
Type JavaScript code or 'exit' to quit

> 
```

在 REPL 中可以：
- 输入 JavaScript 表达式并立即查看结果
- 声明变量并在后续使用
- 输入 `exit` 或 `quit` 退出
- 按 Ctrl+D (Unix) 或 Ctrl+Z (Windows) 退出

**示例会话：**

```bash
> 42
Number(42.0)

> let x = 10
Number(10.0)

> let y = 20
Number(20.0)

> x + y
Number(30.0)

> (x + y) * 2
Number(60.0)

> exit
Goodbye!
```

#### 2. 文件执行模式

传入 JavaScript 文件路径，执行文件中的代码：

```bash
$ ./target/release/v8_rs script.js
```

**示例：**

创建文件 `example.js`：
```javascript
let a = 10;
let b = 20;
a + b
```

执行：
```bash
$ ./target/release/v8_rs example.js
Number(30.0)
```

### 支持的 JavaScript 特性

当前版本支持以下 JavaScript 特性：

#### ✅ 数字字面量
```javascript
42
3.14
0
```

#### ✅ 算术运算
```javascript
10 + 20        // 加法
30 - 10        // 减法
5 * 6          // 乘法
20 / 4         // 除法
```

#### ✅ 变量声明和赋值
```javascript
let x = 10;
let y = 20;
let z = x + y;
```

#### ✅ 复杂表达式
```javascript
(5 + 3) * 2
let x = 10;
let y = x * 2 + 5;
```

### 错误处理

V8-RS 提供清晰的错误信息：

#### 语法错误
```bash
$ echo "let = 10" > error.js
$ ./target/release/v8_rs error.js
Error: Parse error: Expected 'identifier', found 'Assign' at 0:4
```

#### 运行时错误
```bash
$ echo "10 / 0" > error.js
$ ./target/release/v8_rs error.js
Error: Runtime error: Division by zero
```

#### 文件不存在
```bash
$ ./target/release/v8_rs nonexistent.js
Error reading file 'nonexistent.js': No such file or directory (os error 2)
```

### 作为 Rust 库使用

V8-RS 也可以作为 Rust 库集成到你的项目中：

#### 1. 添加依赖

在 `Cargo.toml` 中添加：
```toml
[dependencies]
v8_rs = { path = "../v8-rs" }
```

#### 2. 使用示例

```rust
use v8_rs::Engine;

fn main() {
    // 创建引擎实例
    let mut engine = Engine::new();
    
    // 执行 JavaScript 代码
    match engine.execute("let x = 10; x * 2") {
        Ok(result) => println!("结果: {:?}", result),
        Err(err) => eprintln!("错误: {}", err),
    }
}
```

### 系统安装（可选）

如果想在系统范围内使用 V8-RS：

#### Linux / macOS

```bash
# 构建发布版本
cargo build --release

# 复制到系统路径
sudo cp target/release/v8_rs /usr/local/bin/

# 现在可以在任何地方使用
v8_rs script.js
```

#### 添加到 PATH

或者将构建目录添加到 PATH：

```bash
# 在 ~/.bashrc 或 ~/.zshrc 中添加
export PATH="$PATH:/path/to/v8-rs/target/release"
```

### 性能提示

- 使用 `--release` 构建以获得最佳性能
- 发布版本比开发版本快 10-100 倍
- 对于生产使用，始终使用发布版本

### 限制和已知问题

当前版本的限制：

- ❌ 不支持注释（`//` 和 `/* */`）
- ❌ 不支持字符串类型
- ❌ 不支持布尔类型
- ❌ 不支持函数调用
- ❌ 不支持对象和数组
- ❌ 不支持控制流（if/while/for）

这些特性将在未来版本中添加。

---

## English

### Installation and Building

#### 1. Clone the Repository

```bash
git clone <repo-url>
cd v8-rs
```

#### 2. Build the Project

```bash
# Development build (with debug info)
cargo build

# Release build (optimized)
cargo build --release
```

After building, the executable is located at:
- Development: `./target/debug/v8_rs`
- Release: `./target/release/v8_rs`

### Usage Modes

V8-RS supports two usage modes:

#### 1. Interactive REPL Mode

Run without arguments to start the interactive JavaScript interpreter:

```bash
$ ./target/release/v8_rs
V8-RS JavaScript Engine v0.1.0
Type JavaScript code or 'exit' to quit

> 
```

In the REPL you can:
- Enter JavaScript expressions and see results immediately
- Declare variables and use them later
- Type `exit` or `quit` to exit
- Press Ctrl+D (Unix) or Ctrl+Z (Windows) to exit

**Example Session:**

```bash
> 42
Number(42.0)

> let x = 10
Number(10.0)

> let y = 20
Number(20.0)

> x + y
Number(30.0)

> (x + y) * 2
Number(60.0)

> exit
Goodbye!
```

#### 2. File Execution Mode

Pass a JavaScript file path to execute the code in the file:

```bash
$ ./target/release/v8_rs script.js
```

**Example:**

Create file `example.js`:
```javascript
let a = 10;
let b = 20;
a + b
```

Execute:
```bash
$ ./target/release/v8_rs example.js
Number(30.0)
```

### Supported JavaScript Features

Current version supports the following JavaScript features:

#### ✅ Number Literals
```javascript
42
3.14
0
```

#### ✅ Arithmetic Operations
```javascript
10 + 20        // Addition
30 - 10        // Subtraction
5 * 6          // Multiplication
20 / 4         // Division
```

#### ✅ Variable Declaration and Assignment
```javascript
let x = 10;
let y = 20;
let z = x + y;
```

#### ✅ Complex Expressions
```javascript
(5 + 3) * 2
let x = 10;
let y = x * 2 + 5;
```

### Error Handling

V8-RS provides clear error messages:

#### Syntax Errors
```bash
$ echo "let = 10" > error.js
$ ./target/release/v8_rs error.js
Error: Parse error: Expected 'identifier', found 'Assign' at 0:4
```

#### Runtime Errors
```bash
$ echo "10 / 0" > error.js
$ ./target/release/v8_rs error.js
Error: Runtime error: Division by zero
```

#### File Not Found
```bash
$ ./target/release/v8_rs nonexistent.js
Error reading file 'nonexistent.js': No such file or directory (os error 2)
```

### Using as a Rust Library

V8-RS can also be integrated into your Rust projects as a library:

#### 1. Add Dependency

In your `Cargo.toml`:
```toml
[dependencies]
v8_rs = { path = "../v8-rs" }
```

#### 2. Usage Example

```rust
use v8_rs::Engine;

fn main() {
    // Create engine instance
    let mut engine = Engine::new();
    
    // Execute JavaScript code
    match engine.execute("let x = 10; x * 2") {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### System-wide Installation (Optional)

To use V8-RS system-wide:

#### Linux / macOS

```bash
# Build release version
cargo build --release

# Copy to system path
sudo cp target/release/v8_rs /usr/local/bin/

# Now you can use it anywhere
v8_rs script.js
```

#### Add to PATH

Or add the build directory to PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/v8-rs/target/release"
```

### Performance Tips

- Use `--release` build for best performance
- Release builds are 10-100x faster than debug builds
- Always use release builds for production

### Limitations and Known Issues

Current version limitations:

- ❌ No comment support (`//` and `/* */`)
- ❌ No string type
- ❌ No boolean type
- ❌ No function calls
- ❌ No objects and arrays
- ❌ No control flow (if/while/for)

These features will be added in future versions.

---

<div align="center">

**📚 更多文档 / More Documentation**

[完整教程 / Full Tutorial](./README_CN.md) | [快速开始 / Quick Start](./QUICKSTART.md)

</div>
