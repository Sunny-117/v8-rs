# V8-RS

<div align="center">

**一个用 Rust 实现的 JavaScript 引擎**

[![Tests](https://img.shields.io/badge/tests-123%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-2021-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

[English](./README_EN.md) | 中文

</div>

---

## 📖 项目简介

V8-RS 是一个用 Rust 编写的简化版 JavaScript 引擎，旨在帮助开发者理解现代 JavaScript 引擎（如 V8）的工作原理。

本项目实现了完整的编译器前端和字节码解释器，包含词法分析、语法分析、字节码生成和虚拟机执行等核心组件。

## ✨ 特性

- 🚀 **完整的编译流程** - 从源代码到字节码的完整转换
- 🔍 **词法和语法分析** - 支持 JavaScript 核心语法
- 📦 **字节码虚拟机** - 栈式虚拟机执行引擎
- 🎯 **作用域管理** - 完整的词法作用域实现
- 🖨️ **内置打印函数** - 支持 `print()` 函数输出
- ✅ **全面测试** - 123 个测试用例，100% 通过率
- 📚 **详细教程** - 10 章完整教程，5000+ 行文档

## 🎓 学习资源

本项目提供了完整的中文教程，帮助你从零开始理解 JavaScript 引擎的实现：

- **[📘 完整教程](./docs/README_CN.md)** - 10 章系统教程
- **[⚡ 快速开始](./docs/QUICKSTART.md)** - 10 分钟快速上手
- **[📖 使用指南](./docs/USAGE.md)** - 详细使用说明
- **[� 与 No]de.js 的差异](./docs/DIFFERENCES.md)** - 了解 V8-RS 与 Node.js 的区别

## 🚀 快速开始

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
./target/release/v8_rs examples/hello.js
```

## 💡 使用示例

### 作为独立可执行文件

#### REPL 模式（交互式）

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

#### 文件执行模式

创建 JavaScript 文件：
```javascript
// script.js
let x = 5;
let y = 10;
print(x * y);
```

执行文件：
```bash
$ ./target/release/v8_rs script.js
50
```

### 作为 Rust 库使用

```rust
use v8_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // 执行 JavaScript 代码
    let result = engine.execute("(5 + 3) * 2").unwrap();
    println!("结果: {:?}", result); // Number(16.0)
}
```

## 📝 支持的 JavaScript 特性

当前版本支持：

- ✅ 数字字面量（整数和浮点数）
- ✅ 算术运算（`+`, `-`, `*`, `/`）
- ✅ 变量声明（`let`）
- ✅ 复杂表达式
- ✅ 内置 `print()` 函数

计划支持：

- � 字符串类型建
- � 布尔文类型
- 🔜 控制流（if/while/for）
- 🔜 函数定义和调用
- � 对象和数组
- 🔜 注释支持

## 🎯 适合人群

- 想要理解 JavaScript 引擎工作原理的开发者
- 对编译器和虚拟机感兴趣的学习者
- 希望深入学习 Rust 系统编程的工程师
- 准备为 V8、SpiderMonkey 等开源项目贡献的开发者

## 📊 项目状态

```
✅ 词法分析器      ✅ 语法分析器      ✅ 字节码生成
✅ 虚拟机解释器    ✅ 作用域管理      ✅ 错误处理
✅ Print 函数      ✅ 完整测试套件    ✅ 详细文档
```

## � *项目结构

```
v8-rs/
├── src/
│   ├── lexer.rs          # 词法分析器
│   ├── parser.rs         # 语法分析器
│   ├── ast.rs            # 抽象语法树
│   ├── bytecode.rs       # 字节码定义
│   ├── codegen.rs        # 字节码生成器
│   ├── interpreter.rs    # 解释器（Ignition）
│   ├── engine.rs         # 引擎主协调器
│   └── ...
├── docs/                 # 完整教程文档
├── examples/             # JavaScript 示例
└── tests/                # 测试套件
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🌏 翻译文档

## 📄 许可证

本项目采用 MIT 许可证，详见 [LICENSE](LICENSE) 文件。

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给一个 Star！**

Made with ❤️ by Rust and JavaScript enthusiasts

</div>
