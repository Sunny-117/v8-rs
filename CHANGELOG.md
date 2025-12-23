# Changelog / 更新日志

All notable changes to this project will be documented in this file.

本文件记录项目的所有重要更改。

---

## [0.1.0] - 2024-12-23

### Added / 新增

#### Core Engine / 核心引擎
- ✅ Complete lexical analyzer (Lexer) supporting numbers, identifiers, keywords, operators
- ✅ 完整的词法分析器，支持数字、标识符、关键字、运算符
- ✅ Recursive descent parser with operator precedence
- ✅ 递归下降解析器，支持运算符优先级
- ✅ Abstract Syntax Tree (AST) with expression and statement nodes
- ✅ 抽象语法树（AST），包含表达式和语句节点
- ✅ Bytecode instruction set (LoadConst, LoadLocal, StoreLocal, Add, Sub, Mul, Div, Print, etc.)
- ✅ 字节码指令集（LoadConst、LoadLocal、StoreLocal、Add、Sub、Mul、Div、Print 等）
- ✅ Stack-based bytecode interpreter (Ignition)
- ✅ 栈式字节码解释器（Ignition）
- ✅ Lexical scope management with variable declaration and lookup
- ✅ 词法作用域管理，支持变量声明和查找
- ✅ Main engine coordinator integrating all components
- ✅ 主引擎协调器，集成所有组件

#### Built-in Functions / 内置函数
- ✅ `print()` function for output
- ✅ `print()` 函数用于输出
- ✅ Proper handling of undefined values
- ✅ 正确处理 undefined 值

#### Execution Modes / 执行模式
- ✅ REPL mode (interactive shell) - expressions automatically displayed
- ✅ REPL 模式（交互式 shell）- 表达式自动显示
- ✅ File execution mode - runs JavaScript files
- ✅ 文件执行模式 - 运行 JavaScript 文件
- ✅ Library mode - can be used as a Rust library
- ✅ 库模式 - 可作为 Rust 库使用

#### Output Formatting / 输出格式
- ✅ Node.js-compatible output format
- ✅ 与 Node.js 兼容的输出格式
- ✅ Integers display without decimal point (e.g., `42` not `42.0`)
- ✅ 整数显示不带小数点（如 `42` 而非 `42.0`）
- ✅ Floating point numbers display with decimals (e.g., `3.14`)
- ✅ 浮点数显示带小数（如 `3.14`）

#### Error Handling / 错误处理
- ✅ Parse errors with position information
- ✅ 解析错误，包含位置信息
- ✅ Runtime errors (division by zero, undefined variables, etc.)
- ✅ 运行时错误（除以零、未定义变量等）
- ✅ Clear error messages
- ✅ 清晰的错误消息

#### Testing / 测试
- ✅ 123 test cases covering all components
- ✅ 123 个测试用例，覆盖所有组件
- ✅ Unit tests for lexer, parser, bytecode, interpreter, engine
- ✅ 词法分析器、解析器、字节码、解释器、引擎的单元测试
- ✅ Integration tests for end-to-end functionality
- ✅ 端到端功能的集成测试
- ✅ 100% test pass rate
- ✅ 100% 测试通过率

#### Documentation / 文档
- ✅ Complete Chinese README (default)
- ✅ 完整的中文 README（默认）
- ✅ Complete English README
- ✅ 完整的英文 README
- ✅ 10-chapter tutorial (5000+ lines)
- ✅ 10 章教程（5000+ 行）
- ✅ Quick start guide
- ✅ 快速开始指南
- ✅ Detailed usage guide
- ✅ 详细使用指南
- ✅ Differences from Node.js document
- ✅ 与 Node.js 差异文档
- ✅ Example files with documentation
- ✅ 示例文件及文档

#### Examples / 示例
- ✅ `hello.js` - Basic arithmetic
- ✅ `hello.js` - 基本算术
- ✅ `arithmetic.js` - Complex expressions
- ✅ `arithmetic.js` - 复杂表达式
- ✅ `fibonacci.js` - Fibonacci sequence
- ✅ `fibonacci.js` - 斐波那契数列
- ✅ `print_test.js` - Multiple print statements
- ✅ `print_test.js` - 多个打印语句
- ✅ `error.js` - Error handling demo
- ✅ `error.js` - 错误处理演示
- ✅ `basic.rs` - Rust library usage example
- ✅ `basic.rs` - Rust 库使用示例

### Supported JavaScript Features / 支持的 JavaScript 特性

- ✅ Number literals (integers and floats)
- ✅ 数字字面量（整数和浮点数）
- ✅ Arithmetic operators: `+`, `-`, `*`, `/`
- ✅ 算术运算符：`+`、`-`、`*`、`/`
- ✅ Variable declarations: `let`
- ✅ 变量声明：`let`
- ✅ Variable assignment
- ✅ 变量赋值
- ✅ Identifiers
- ✅ 标识符
- ✅ Parenthesized expressions
- ✅ 括号表达式
- ✅ Built-in `print()` function
- ✅ 内置 `print()` 函数

### Known Limitations / 已知限制

- ❌ No comment support (`//` or `/* */`)
- ❌ 不支持注释（`//` 或 `/* */`）
- ❌ No string type
- ❌ 不支持字符串类型
- ❌ No boolean type
- ❌ 不支持布尔类型
- ❌ No control flow (if/while/for)
- ❌ 不支持控制流（if/while/for）
- ❌ No user-defined functions
- ❌ 不支持用户定义函数
- ❌ No objects or arrays
- ❌ 不支持对象或数组
- ❌ No `console.log()` (use `print()` instead)
- ❌ 不支持 `console.log()`（请使用 `print()`）

### Technical Details / 技术细节

#### Architecture / 架构
- Lexer → Parser → AST → Bytecode Generator → Interpreter
- 词法分析器 → 解析器 → AST → 字节码生成器 → 解释器

#### Components / 组件
- **Lexer** (`src/lexer.rs`): Tokenizes source code
- **词法分析器** (`src/lexer.rs`)：将源代码标记化
- **Parser** (`src/parser.rs`): Builds AST from tokens
- **解析器** (`src/parser.rs`)：从标记构建 AST
- **AST** (`src/ast.rs`): Abstract syntax tree definitions
- **AST** (`src/ast.rs`)：抽象语法树定义
- **Bytecode** (`src/bytecode.rs`): Instruction set definitions
- **字节码** (`src/bytecode.rs`)：指令集定义
- **Codegen** (`src/codegen.rs`): Converts AST to bytecode
- **代码生成** (`src/codegen.rs`)：将 AST 转换为字节码
- **Interpreter** (`src/interpreter.rs`): Executes bytecode
- **解释器** (`src/interpreter.rs`)：执行字节码
- **Engine** (`src/engine.rs`): Main coordinator
- **引擎** (`src/engine.rs`)：主协调器
- **Scope** (`src/scope.rs`): Variable scope management
- **作用域** (`src/scope.rs`)：变量作用域管理

#### Performance / 性能
- Stack-based VM for efficient execution
- 栈式虚拟机，高效执行
- Release build optimizations enabled
- 启用发布版本优化
- Minimal memory footprint
- 最小内存占用

---

## Future Plans / 未来计划

### Version 0.2.0 (Planned)
- 🔜 String type support
- 🔜  字符串类型支持
- 🔜 Boolean type support
- 🔜  布尔类型支持
- 🔜 Comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- 🔜 比较运算符（`==`、`!=`、`<`、`>`、`<=`、`>=`）
- 🔜 Logical operators (`&&`, `||`, `!`)
- 🔜 逻辑运算符（`&&`、`||`、`!`）
- 🔜 Comment support (`//` and `/* */`)
- 🔜 注释支持（`//` 和 `/* */`）

### Version 0.3.0 (Planned)
- 🔜 Control flow: `if`/`else`
- 🔜 控制流：`if`/`else`
- 🔜 Control flow: `while` loops
- 🔜 控制流：`while` 循环
- 🔜 Control flow: `for` loops
- 🔜 控制流：`for` 循环

### Version 0.4.0 (Planned)
- 🔜 User-defined functions
- 🔜 用户定义函数
- 🔜 Function calls with arguments
- 🔜 带参数的函数调用
- 🔜 Return statements
- 🔜 返回语句
- 🔜 Closures
- 🔜 闭包

### Version 0.5.0 (Planned)
- 🔜 Object literals
- 🔜 对象字面量
- 🔜 Array literals
- 🔜 数组字面量
- 🔜 Property access
- 🔜 属性访问
- 🔜 Array indexing
- 🔜 数组索引

---

## Contributors / 贡献者

Thanks to all contributors who helped make this project possible!

感谢所有帮助实现此项目的贡献者！

---

<div align="center">

**Made with ❤️ by Rust and JavaScript enthusiasts**

</div>
