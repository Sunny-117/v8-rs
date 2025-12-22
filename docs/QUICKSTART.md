# 快速开始指南

本指南将帮助你在 10 分钟内运行 V8-RS 引擎并理解其基本工作原理。

## 前置要求

- Rust 1.70 或更高版本
- 基本的命令行知识

## 步骤 1：克隆或查看项目

```bash
cd v8-rs
```

## 步骤 2：构建项目

```bash
cargo build
```

预期输出：
```
   Compiling v8_rs v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.5s
```

## 步骤 3：运行示例

```bash
cargo run
```

你将看到：
```
V8-RS JavaScript Engine
Version 0.1.0

Executing: 42
Result: Number(42.0)

Executing: 10 + 20
Result: Number(30.0)

Executing: (5 + 3) * 2
Result: Number(16.0)

Executing: 100 / 4
Result: Number(25.0)

Executing: let x = 15;
Result: Undefined
```

## 步骤 4：运行测试

```bash
cargo test
```

你将看到所有 67 个测试通过：
```
running 53 tests (unit tests)
...
test result: ok. 53 passed

running 14 tests (integration tests)
...
test result: ok. 14 passed
```

## 步骤 5：尝试交互式使用

创建一个新文件 `examples/repl.rs`：

```rust
use v8_rs::Engine;
use std::io::{self, Write};

fn main() {
    let mut engine = Engine::new();
    
    println!("V8-RS REPL");
    println!("Type JavaScript expressions (Ctrl+C to exit)\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match engine.execute(&input.trim()) {
            Ok(result) => println!("{:?}", result),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
```

运行：
```bash
cargo run --example repl
```

尝试输入：
```
> 10 + 20
Number(30.0)
> let x = 5;
Undefined
> x * 2
Number(10.0)
```

## 理解执行流程

让我们追踪一个简单表达式的执行：

### 输入
```javascript
10 + 20
```

### 1. 词法分析（Lexer）
```
Token { kind: Number(10.0), span: Span { start: 0, end: 2 } }
Token { kind: Plus, span: Span { start: 3, end: 4 } }
Token { kind: Number(20.0), span: Span { start: 5, end: 7 } }
Token { kind: Eof, span: Span { start: 7, end: 7 } }
```

### 2. 语法分析（Parser）
```
Program
└── BinaryExpr
    ├── op: Add
    ├── left: NumberLiteral(10.0)
    └── right: NumberLiteral(20.0)
```

### 3. 字节码生成（Codegen）
```
Constants: [10.0, 20.0]
Instructions:
  0: LoadConst 0
  1: LoadConst 1
  2: Add
```

### 4. 解释执行（Interpreter）
```
执行 LoadConst 0:
  stack: [10.0]

执行 LoadConst 1:
  stack: [10.0, 20.0]

执行 Add:
  pop 20.0, pop 10.0
  push 30.0
  stack: [30.0]

返回: Number(30.0)
```

## 添加调试输出

修改 `src/main.rs` 来查看内部过程：

```rust
use v8_rs::{Engine, Parser, BytecodeGenerator, Scope};

fn main() {
    let source = "10 + 20";
    println!("Source: {}\n", source);
    
    // 1. 解析
    let mut parser = Parser::new(source.to_string());
    let ast = parser.parse().unwrap();
    println!("AST: {:#?}\n", ast);
    
    // 2. 生成字节码
    let mut codegen = BytecodeGenerator::new(Scope::global());
    let bytecode = codegen.generate(&ast.root);
    println!("Bytecode:");
    println!("  Constants: {:?}", bytecode.constants);
    println!("  Instructions:");
    for (i, inst) in bytecode.instructions.iter().enumerate() {
        println!("    {}: {:?}", i, inst);
    }
    println!();
    
    // 3. 执行
    let mut engine = Engine::new();
    let result = engine.execute(source).unwrap();
    println!("Result: {:?}", result);
}
```

## 支持的语法

### ✅ 已实现

```javascript
// 数字
42
3.14

// 算术运算
10 + 20
5 * 6
100 / 4
50 - 15

// 括号
(5 + 3) * 2

// 变量
let x = 10;
let y = x + 5;

// 运算符优先级
2 + 3 * 4  // 结果是 14，不是 20
```

### ❌ 未实现

```javascript
// 字符串
"hello"

// 对象
{ x: 10 }

// 数组
[1, 2, 3]

// 函数调用（部分实现）
foo(1, 2)

// 控制流（已有 AST，但执行有限）
if (x > 0) { ... }
for (let i = 0; i < 10; i++) { ... }
```

## 常见问题

### Q: 为什么有些语法解析成功但执行失败？

**A:** 解析器实现了完整的语法，但解释器只实现了基本功能。例如：
```javascript
function foo() { return 42; }  // 解析成功
foo()  // 执行失败（函数调用未完全实现）
```

### Q: 如何添加新的运算符？

**A:** 需要修改三个地方：
1. `src/lexer.rs` - 添加 Token 类型
2. `src/ast.rs` - 添加到 BinOp 枚举
3. `src/interpreter.rs` - 添加执行逻辑

### Q: 性能如何？

**A:** 当前实现优先考虑清晰性而非性能：
- 解析：~10,000 行/秒
- 执行：~1,000,000 指令/秒

对于学习和原型开发足够快。

## 下一步

1. **阅读教程**：从[第 1 章](./01-project-setup.md)开始深入学习
2. **查看测试**：`tests/integration_test.rs` 有更多示例
3. **尝试修改**：添加新功能或优化现有代码
4. **阅读源码**：理解每个模块的实现细节

## 获取帮助

- 📖 查看[完整教程](./README.md)
- 🐛 遇到问题？查看测试用例
- 💡 有想法？欢迎提 Issue

---

**准备好深入学习了吗？开始阅读[教程目录](./README.md)！** 🚀
