# V8-RS

<div align="center">

**一个用 Rust 实现的 JavaScript 引擎**

[![Tests](https://img.shields.io/badge/tests-67%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-2021-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

[English](#english) | [中文](#中文)

</div>

---

## 中文

### 📖 项目简介

V8-RS 是一个用 Rust 编写的简化版 JavaScript 引擎，旨在帮助开发者理解现代 JavaScript 引擎（如 V8）的工作原理。

本项目实现了完整的编译器前端和字节码解释器，包含词法分析、语法分析、字节码生成和虚拟机执行等核心组件。

### ✨ 特性

- 🚀 **完整的编译流程** - 从源代码到字节码的完整转换
- 🔍 **词法和语法分析** - 支持 JavaScript 核心语法
- 📦 **字节码虚拟机** - 栈式虚拟机执行引擎
- 🎯 **作用域管理** - 完整的词法作用域实现
- ✅ **全面测试** - 67 个测试用例，100% 通过率
- 📚 **详细教程** - 10 章完整教程，5000+ 行文档

### 🎓 学习资源

本项目提供了完整的中文教程，帮助你从零开始理解 JavaScript 引擎的实现：

- **[📘 完整教程](./docs/README_CN.md)** - 10 章系统教程
- **[⚡ 快速开始](./docs/QUICKSTART.md)** - 10 分钟快速上手
- **[📖 使用指南](./docs/USAGE.md)** - 详细使用说明

### 🚀 快速开始

```bash
# 克隆项目
git clone <repo-url>
cd v8-rs

# 构建项目
cargo build --release

# 运行测试
cargo test

# 启动 REPL（交互式模式）
./target/release/v8_rs

# 执行 JavaScript 文件
./target/release/v8_rs test.js
```

### 💡 使用示例

#### 作为独立可执行文件

```bash
# 启动交互式 REPL
$ ./target/release/v8_rs
V8-RS JavaScript Engine v0.1.0
Type JavaScript code or 'exit' to quit

> let x = 10
10
> let y = 20
20
> x + y
30
> print(x + y)
30
> exit
Goodbye!

# 执行 JavaScript 文件
$ cat > script.js << 'EOF'
let x = 5;
let y = 10;
print(x * y);
EOF
$ ./target/release/v8_rs script.js
50
```

#### 作为 Rust 库使用

```rust
use v8_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // 执行 JavaScript 代码
    let result = engine.execute("(5 + 3) * 2").unwrap();
    println!("结果: {:?}", result); // Number(16.0)
}
```

### 🎯 适合人群

- 想要理解 JavaScript 引擎工作原理的开发者
- 对编译器和虚拟机感兴趣的学习者
- 希望深入学习 Rust 系统编程的工程师
- 准备为 V8、SpiderMonkey 等开源项目贡献的开发者

### 📊 项目状态

```
✅ 词法分析器      ✅ 语法分析器      ✅ 字节码生成
✅ 虚拟机解释器    ✅ 作用域管理      ✅ 错误处理
✅ 完整测试套件    ✅ 详细文档        ✅ 示例代码
```

### 🤝 贡献

欢迎提交 Issue 和 Pull Request！

- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🌏 翻译文档

### 📄 许可证

本项目采用 MIT 许可证，详见 [LICENSE](LICENSE) 文件。

---

## English

### 📖 About

V8-RS is a JavaScript engine written in Rust, designed to help developers understand how modern JavaScript engines (like V8) work under the hood.

This project implements a complete compiler frontend and bytecode interpreter, including lexical analysis, parsing, bytecode generation, and virtual machine execution.

### ✨ Features

- 🚀 **Complete Compilation Pipeline** - Full transformation from source to bytecode
- 🔍 **Lexer & Parser** - Support for core JavaScript syntax
- 📦 **Bytecode VM** - Stack-based virtual machine execution engine
- 🎯 **Scope Management** - Complete lexical scoping implementation
- ✅ **Comprehensive Testing** - 67 test cases with 100% pass rate
- 📚 **Detailed Tutorial** - 10-chapter tutorial with 5000+ lines of documentation

### 🎓 Learning Resources

This project provides a complete tutorial to help you understand JavaScript engine implementation from scratch:

- **[📘 Full Tutorial](./docs/README.md)** - 10-chapter systematic tutorial
- **[⚡ Quick Start](./docs/QUICKSTART.md)** - Get started in 10 minutes
- **[📖 Usage Guide](./docs/USAGE.md)** - Detailed usage instructions

### 🚀 Quick Start

```bash
# Clone the repository
git clone <repo-url>
cd v8-rs

# Build the project
cargo build --release

# Run tests
cargo test

# Start REPL (interactive mode)
./target/release/v8_rs

# Execute JavaScript file
./target/release/v8_rs test.js
```

### 💡 Usage Example

#### As a Standalone Executable

```bash
# Start interactive REPL
$ ./target/release/v8_rs
V8-RS JavaScript Engine v0.1.0
Type JavaScript code or 'exit' to quit

> let x = 10
10
> let y = 20
20
> x + y
30
> print(x + y)
30
> exit
Goodbye!

# Execute JavaScript file
$ cat > script.js << 'EOF'
let x = 5;
let y = 10;
print(x * y);
EOF
$ ./target/release/v8_rs script.js
50
```

#### As a Rust Library

```rust
use v8_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // Execute JavaScript code
    let result = engine.execute("(5 + 3) * 2").unwrap();
    println!("Result: {:?}", result); // Number(16.0)
}
```

### 🎯 Target Audience

- Developers who want to understand how JavaScript engines work
- Learners interested in compilers and virtual machines
- Engineers looking to dive deep into Rust systems programming
- Developers preparing to contribute to open-source projects like V8 or SpiderMonkey

### 📊 Project Status

```
✅ Lexer             ✅ Parser            ✅ Bytecode Generator
✅ VM Interpreter    ✅ Scope Manager     ✅ Error Handling
✅ Test Suite        ✅ Documentation     ✅ Examples
```

### 🤝 Contributing

Issues and Pull Requests are welcome!

- 🐛 Report bugs
- 💡 Suggest new features
- 📝 Improve documentation
- 🌏 Translate documentation

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给一个 Star！**

**⭐ If this project helps you, please give it a Star!**

</div>
