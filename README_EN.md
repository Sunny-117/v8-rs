# V8-RS

<div align="center">

**A JavaScript Engine Written in Rust**

[![Tests](https://img.shields.io/badge/tests-123%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-2021-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

English | [中文](./README.md)

</div>

---

## 📖 About

V8-RS is a simplified JavaScript engine written in Rust, designed to help developers understand how modern JavaScript engines (like V8) work under the hood.

This project implements a complete compiler frontend and bytecode interpreter, including lexical analysis, parsing, bytecode generation, and virtual machine execution.

## ✨ Features

- 🚀 **Complete Compilation Pipeline** - Full transformation from source to bytecode
- 🔍 **Lexer & Parser** - Support for core JavaScript syntax
- 📦 **Bytecode VM** - Stack-based virtual machine execution engine
- 🎯 **Scope Management** - Complete lexical scoping implementation
- 🖨️ **Built-in Print Function** - Support for `print()` function output
- ✅ **Comprehensive Testing** - 123 test cases with 100% pass rate
- 📚 **Detailed Tutorial** - 10-chapter tutorial with 5000+ lines of documentation

## 🎓 Learning Resources

This project provides a complete tutorial to help you understand JavaScript engine implementation from scratch:

- **[📘 Full Tutorial](./docs/README.md)** - 10-chapter systematic tutorial
- **[⚡ Quick Start](./docs/QUICKSTART.md)** - Get started in 10 minutes
- **[📖 Usage Guide](./docs/USAGE.md)** - Detailed usage instructions
- **[🔄 Differences from Node.js](./docs/DIFFERENCES.md)** - Understand V8-RS vs Node.js

## 🚀 Quick Start

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
./target/release/v8_rs examples/hello.js
```

## 💡 Usage Examples

### As a Standalone Executable

#### REPL Mode (Interactive)

```bash
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
```

#### File Execution Mode

Create a JavaScript file:
```javascript
// script.js
let x = 5;
let y = 10;
print(x * y);
```

Execute the file:
```bash
$ ./target/release/v8_rs script.js
50
```

### As a Rust Library

```rust
use v8_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // Execute JavaScript code
    let result = engine.execute("(5 + 3) * 2").unwrap();
    println!("Result: {:?}", result); // Number(16.0)
}
```

## 📝 Supported JavaScript Features

Currently supported:

- ✅ Number literals (integers and floats)
- ✅ Arithmetic operations (`+`, `-`, `*`, `/`)
- ✅ Variable declarations (`let`)
- ✅ Complex expressions
- ✅ Built-in `print()` function

Planned features:

- 🔜 String type
- 🔜 Boolean type
- 🔜 Control flow (if/while/for)
- 🔜 Function definitions and calls
- 🔜 Objects and arrays
- 🔜 Comment support

## 🎯 Target Audience

- Developers who want to understand how JavaScript engines work
- Learners interested in compilers and virtual machines
- Engineers looking to dive deep into Rust systems programming
- Developers preparing to contribute to open-source projects like V8 or SpiderMonkey

## 📊 Project Status

```
✅ Lexer             ✅ Parser            ✅ Bytecode Generator
✅ VM Interpreter    ✅ Scope Manager     ✅ Error Handling
✅ Print Function    ✅ Test Suite        ✅ Documentation
```

## 📂 Project Structure

```
v8-rs/
├── src/
│   ├── lexer.rs          # Lexical analyzer
│   ├── parser.rs         # Parser
│   ├── ast.rs            # Abstract syntax tree
│   ├── bytecode.rs       # Bytecode definitions
│   ├── codegen.rs        # Bytecode generator
│   ├── interpreter.rs    # Interpreter (Ignition)
│   ├── engine.rs         # Main engine coordinator
│   └── ...
├── docs/                 # Complete tutorial documentation
├── examples/             # JavaScript examples
└── tests/                # Test suite
```

## 🤝 Contributing

Issues and Pull Requests are welcome!

- 🐛 Report bugs
- 💡 Suggest new features
- 📝 Improve documentation
- 🌏 Translate documentation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**⭐ If this project helps you, please give it a Star!**

Made with ❤️ by Rust and JavaScript enthusiasts

</div>
