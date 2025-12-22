# 设计文档：V8-RS JavaScript 引擎

## 概述

V8-RS 是一个基于 Rust 实现的最小可行版本 JavaScript 引擎，采用 JIT（Just-In-Time）编译技术。该引擎结合了解释执行和编译执行的优势，通过 Ignition 风格的字节码解释器快速启动，并通过 TurboFan 风格的优化编译器提升热点代码的执行性能。

### 设计目标

1. **架构清晰性**：优先考虑代码的可读性和可维护性
2. **功能完整性**：实现完整的 JIT 执行流程（解析 → 字节码 → 解释执行 → 优化编译 → 反优化）
3. **性能权衡**：通过混合执行策略平衡启动速度和运行性能
4. **模块化设计**：各组件独立且可测试

### 技术栈

- **语言**：Rust
- **代码生成后端**：Cranelift 或 Dynasm
- **内存管理**：引用计数（Rc/RefCell）或内存池（Arena）

## 架构

### 系统架构图

```
┌─────────────────────────────────────────────────────────────┐
│                         Engine                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Parser  │→ │ Bytecode │→ │ Ignition │→ │ Hotspot  │   │
│  │          │  │Generator │  │          │  │ Profiler │   │
│  └──────────┘  └──────────┘  └──────────┘  └────┬─────┘   │
│                                                   │          │
│                                                   ↓          │
│                                            ┌──────────┐     │
│                                            │ TurboFan │     │
│                                            │   JIT    │     │
│                                            └────┬─────┘     │
│                                                 │          │
│                                                 ↓          │
│                                          ┌──────────┐     │
│                                          │ Machine  │     │
│                                          │   Code   │     │
│                                          └──────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### 执行流程

1. **解析阶段**：Parser 将 JavaScript 源代码转换为 AST 和作用域信息
2. **字节码生成**：Bytecode Generator 将 AST 转换为字节码指令序列
3. **解释执行**：Ignition 解释器执行字节码并收集执行统计信息
4. **热点检测**：Hotspot Profiler 监控函数执行频率，识别热点代码
5. **优化编译**：TurboFan 将热点代码编译为优化的机器码
6. **优化执行**：Engine 使用优化后的机器码替代解释执行
7. **反优化**：当优化假设失效时，回退到解释执行

## 组件和接口

### 1. Engine（引擎）

**职责**：协调所有组件，管理执行流程

**接口**：

```rust
pub struct Engine {
    parser: Parser,
    bytecode_gen: BytecodeGenerator,
    interpreter: Ignition,
    profiler: HotspotProfiler,
    jit: TurboFan,
    heap: Heap,
    stack: Stack,
    global_context: GlobalContext,
}

impl Engine {
    pub fn new() -> Self;
    pub fn execute(&mut self, source: &str) -> Result<Value, Error>;
    fn parse(&self, source: &str) -> Result<AST, ParseError>;
    fn generate_bytecode(&self, ast: &AST) -> BytecodeChunk;
    fn interpret(&mut self, bytecode: &BytecodeChunk) -> Result<Value, RuntimeError>;
    fn should_optimize(&self, func_id: FunctionId) -> bool;
    fn optimize(&mut self, func_id: FunctionId) -> CompiledFunction;
    fn deoptimize(&mut self, func_id: FunctionId);
}
```

### 2. Parser（解析器）

**职责**：将 JavaScript 源代码解析为 AST

**接口**：

```rust
pub struct Parser {
    source: String,
    position: usize,
}

pub struct AST {
    pub root: ASTNode,
    pub scope: Scope,
}

pub enum ASTNode {
    Program(Vec<ASTNode>),
    FunctionDecl { name: String, params: Vec<String>, body: Box<ASTNode>, span: Span },
    LetDecl { name: String, init: Box<ASTNode>, span: Span },
    ForStmt { init: Box<ASTNode>, cond: Box<ASTNode>, update: Box<ASTNode>, body: Box<ASTNode>, span: Span },
    IfStmt { cond: Box<ASTNode>, then_branch: Box<ASTNode>, else_branch: Option<Box<ASTNode>>, span: Span },
    ReturnStmt { value: Box<ASTNode>, span: Span },
    BinaryExpr { op: BinOp, left: Box<ASTNode>, right: Box<ASTNode>, span: Span },
    CallExpr { callee: Box<ASTNode>, args: Vec<ASTNode>, span: Span },
    Identifier { name: String, span: Span },
    NumberLiteral { value: f64, span: Span },
}

pub enum BinOp {
    Add, Sub, Mul, Div,
}

pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Parser {
    pub fn new(source: String) -> Self;
    pub fn parse(&mut self) -> Result<AST, ParseError>;
}
```

### 3. Scope（作用域）

**职责**：管理变量的词法作用域

**接口**：

```rust
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, usize>, // 变量名 -> 索引
    scope_type: ScopeType,
}

pub enum ScopeType {
    Global,
    Function,
    Block,
}

impl Scope {
    pub fn new(scope_type: ScopeType, parent: Option<Box<Scope>>) -> Self;
    pub fn declare(&mut self, name: String) -> usize;
    pub fn lookup(&self, name: &str) -> Option<usize>;
}
```

### 4. Bytecode Generator（字节码生成器）

**职责**：将 AST 转换为字节码

**接口**：

```rust
pub struct BytecodeGenerator {
    chunk: BytecodeChunk,
    scope: Scope,
}

pub struct BytecodeChunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub local_count: usize,
}

pub enum Instruction {
    LoadConst(usize),      // 加载常量池中的常量
    LoadLocal(usize),      // 加载局部变量
    StoreLocal(usize),     // 存储局部变量
    Add,                   // 加法
    Sub,                   // 减法
    Mul,                   // 乘法
    Div,                   // 除法
    Call(usize),           // 函数调用（参数数量）
    Return,                // 返回
    Jump(isize),           // 无条件跳转
    JumpIfFalse(isize),    // 条件跳转
}

impl BytecodeGenerator {
    pub fn new(scope: Scope) -> Self;
    pub fn generate(&mut self, ast: &AST) -> BytecodeChunk;
}
```

### 5. Ignition（解释器）

**职责**：执行字节码并收集执行统计信息

**接口**：

```rust
pub struct Ignition {
    call_stack: Vec<CallFrame>,
    profiler: HotspotProfiler,
}

pub struct CallFrame {
    pub chunk: BytecodeChunk,
    pub ip: usize,              // 指令指针
    pub stack: Vec<Value>,      // 操作数栈
    pub locals: Vec<Value>,     // 局部变量
    pub func_id: FunctionId,
}

pub type FunctionId = usize;

impl Ignition {
    pub fn new(profiler: HotspotProfiler) -> Self;
    pub fn execute(&mut self, chunk: BytecodeChunk) -> Result<Value, RuntimeError>;
    fn dispatch(&mut self, instruction: &Instruction) -> Result<(), RuntimeError>;
}
```

### 6. Hotspot Profiler（热点分析器）

**职责**：检测频繁执行的函数

**接口**：

```rust
pub struct HotspotProfiler {
    execution_counts: HashMap<FunctionId, usize>,
    hotspot_threshold: usize,
    hot_functions: HashSet<FunctionId>,
}

impl HotspotProfiler {
    pub fn new(threshold: usize) -> Self;
    pub fn record_execution(&mut self, func_id: FunctionId);
    pub fn is_hot(&self, func_id: FunctionId) -> bool;
    pub fn mark_hot(&mut self, func_id: FunctionId);
}
```

### 7. TurboFan（优化编译器）

**职责**：将字节码编译为优化的机器码

**接口**：

```rust
pub struct TurboFan {
    backend: CodegenBackend,
}

pub enum CodegenBackend {
    Cranelift,
    Dynasm,
}

pub struct IR {
    pub nodes: Vec<IRNode>,
}

pub enum IRNode {
    Constant { value: f64, id: NodeId },
    Add { left: NodeId, right: NodeId, id: NodeId },
    Sub { left: NodeId, right: NodeId, id: NodeId },
    Mul { left: NodeId, right: NodeId, id: NodeId },
    Div { left: NodeId, right: NodeId, id: NodeId },
    LoadLocal { index: usize, id: NodeId },
    StoreLocal { index: usize, value: NodeId, id: NodeId },
    Call { callee: NodeId, args: Vec<NodeId>, id: NodeId },
    Return { value: NodeId, id: NodeId },
    TypeGuard { value: NodeId, expected_type: Type, id: NodeId },
}

pub type NodeId = usize;

pub enum Type {
    Number,
    Unknown,
}

pub struct CompiledFunction {
    pub entry_point: *const u8,
    pub func_id: FunctionId,
}

impl TurboFan {
    pub fn new(backend: CodegenBackend) -> Self;
    pub fn compile(&mut self, bytecode: &BytecodeChunk) -> CompiledFunction;
    fn lower_to_ir(&self, bytecode: &BytecodeChunk) -> IR;
    fn optimize(&self, ir: &mut IR);
    fn generate_code(&self, ir: &IR) -> CompiledFunction;
}
```

### 8. Deoptimization（反优化）

**职责**：当优化假设失效时回退到解释执行

**接口**：

```rust
pub struct DeoptInfo {
    pub func_id: FunctionId,
    pub live_values: Vec<Value>,
    pub bytecode_offset: usize,
}

impl Engine {
    fn deoptimize(&mut self, deopt_info: DeoptInfo) {
        // 1. 捕获活跃值
        let live_values = deopt_info.live_values;
        
        // 2. 重建解释器调用帧
        let frame = CallFrame {
            chunk: self.get_bytecode(deopt_info.func_id),
            ip: deopt_info.bytecode_offset,
            stack: live_values,
            locals: vec![],
            func_id: deopt_info.func_id,
        };
        
        // 3. 在 Ignition 中恢复执行
        self.interpreter.call_stack.push(frame);
        
        // 4. 标记函数不再使用优化代码
        self.profiler.hot_functions.remove(&deopt_info.func_id);
    }
}
```

## 数据模型

### Value（值类型）

```rust
pub enum Value {
    Number(f64),
    Function(FunctionId),
    Undefined,
}
```

### 内存管理

使用 Rust 的引用计数（Rc）和内部可变性（RefCell）来管理内存，避免实现完整的垃圾回收器：

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub type HeapValue = Rc<RefCell<Value>>;
```

## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的形式化陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*

### 属性 1：解析往返一致性

*对于任何*有效的 JavaScript 源代码（在支持的子集内），解析生成 AST 后，AST 应该包含源代码的所有语义信息，且每个 AST 节点都应该有源代码位置信息。

**验证：需求 1.1, 1.11**

### 属性 2：解析器语法支持完整性

*对于任何*支持的 JavaScript 语法元素（数字字面量、let 声明、函数声明、二元表达式、if 语句、for 循环、函数调用、return 语句），解析器应该生成相应类型的 AST 节点。

**验证：需求 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9**

### 属性 3：解析错误处理

*对于任何*无效的 JavaScript 源代码，解析器应该返回描述性错误信息而不是崩溃。

**验证：需求 1.10**

### 属性 4：作用域链查找正确性

*对于任何*嵌套作用域结构和变量名，如果变量在当前作用域未找到，则应该在父作用域链中查找，直到找到或到达全局作用域。

**验证：需求 2.4, 2.5**

### 属性 5：字节码生成完整性

*对于任何*有效的 AST，字节码生成器应该生成相应的字节码指令序列，且每个函数应该有独立的字节码块、常量池和局部变量计数。

**验证：需求 3.1, 3.9, 3.10, 3.11**

### 属性 6：字节码指令支持完整性

*对于任何*支持的 AST 节点类型（常量、变量访问、算术运算、函数调用、控制流），字节码生成器应该生成相应的字节码指令。

**验证：需求 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8**

### 属性 7：解释执行隔离性

*对于任何*函数调用序列，每个函数调用应该有独立的调用帧和操作数栈，互不干扰。

**验证：需求 4.2, 4.3**

### 属性 8：执行计数器单调性

*对于任何*函数，每次执行该函数时，其执行计数器应该递增。

**验证：需求 4.5, 5.1**

### 属性 9：热点检测阈值正确性

*对于任何*函数，当且仅当其执行次数超过配置的阈值时，应该被标记为热点代码并触发 JIT 编译。

**验证：需求 5.2, 5.3**

### 属性 10：IR 生成完整性

*对于任何*字节码指令，TurboFan 应该生成相应的 SSA 形式 IR 节点，且 IR 节点应该包含类型反馈信息。

**验证：需求 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 6.7**

### 属性 11：常量折叠正确性

*对于任何*包含常量表达式的 IR（如 `2 + 3`），优化后应该被折叠为单个常量节点（如 `5`）。

**验证：需求 7.1**

### 属性 12：优化代码类型保护

*对于任何*基于类型假设的优化，生成的机器码应该包含运行时类型检查保护。

**验证：需求 7.5, 7.6**

### 属性 13：机器码生成正确性

*对于任何*优化后的 IR，TurboFan 应该生成可执行的本地机器码，且存储函数入口指针。

**验证：需求 8.1, 8.4**

### 属性 14：反优化状态一致性

*对于任何*触发反优化的优化代码，反优化后重建的解释器状态应该能够继续正确执行，且执行结果应该与从未优化过的解释执行结果一致。

**验证：需求 9.1, 9.2, 9.3, 9.4, 9.5, 9.6**

### 属性 15：端到端执行正确性

*对于任何*有效的 JavaScript 程序（在支持的子集内），引擎应该能够完成完整的执行流程（解析 → 字节码 → 解释执行 → 可选的优化编译），并返回正确的执行结果。

**验证：需求 11.1, 11.2, 11.3, 11.4, 11.5, 11.6, 11.7, 11.8**

## 错误处理

### 错误类型

```rust
pub enum Error {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    CompileError(CompileError),
}

pub enum ParseError {
    UnexpectedToken { expected: String, found: String, span: Span },
    UnexpectedEOF,
    InvalidSyntax { message: String, span: Span },
}

pub enum RuntimeError {
    UndefinedVariable { name: String },
    TypeError { expected: String, found: String },
    StackOverflow,
    DivisionByZero,
}

pub enum CompileError {
    UnsupportedFeature { feature: String },
    OptimizationFailed { reason: String },
}
```

### 错误处理策略

1. **解析错误**：返回详细的错误信息，包括错误位置和期望的语法
2. **运行时错误**：捕获错误，清理调用栈，返回错误信息
3. **编译错误**：回退到解释执行，记录编译失败原因
4. **反优化**：当优化假设失效时，安全地回退到解释执行

## 测试策略

### 双重测试方法

本项目采用单元测试和基于属性的测试相结合的方法：

- **单元测试**：验证特定示例、边界情况和错误条件
- **属性测试**：验证跨所有输入的通用属性

两者是互补的，对于全面覆盖都是必要的。

### 单元测试

单元测试专注于：
- 演示正确行为的特定示例
- 组件之间的集成点
- 边界情况和错误条件

示例：
```rust
#[test]
fn test_parse_number_literal() {
    let mut parser = Parser::new("42".to_string());
    let ast = parser.parse().unwrap();
    assert!(matches!(ast.root, ASTNode::NumberLiteral { value: 42.0, .. }));
}

#[test]
fn test_parse_invalid_syntax() {
    let mut parser = Parser::new("let = 42".to_string());
    assert!(parser.parse().is_err());
}
```

### 基于属性的测试

使用 **quickcheck** 或 **proptest** 库进行基于属性的测试。

**配置要求**：
- 每个属性测试最少运行 100 次迭代
- 每个测试必须引用其设计文档属性
- 标签格式：`// Feature: v8-rs, Property N: [property text]`

示例：
```rust
use quickcheck::{quickcheck, TestResult};

// Feature: v8-rs, Property 1: 解析往返一致性
#[quickcheck]
fn prop_parse_roundtrip(source: ValidJSSource) -> TestResult {
    let mut parser = Parser::new(source.0);
    match parser.parse() {
        Ok(ast) => {
            // 验证 AST 包含所有语义信息
            // 验证每个节点都有位置信息
            TestResult::passed()
        }
        Err(_) => TestResult::discard(),
    }
}

// Feature: v8-rs, Property 8: 执行计数器单调性
#[quickcheck]
fn prop_execution_counter_monotonic(func: ValidFunction, n: usize) -> bool {
    let mut engine = Engine::new();
    let func_id = engine.register_function(func);
    
    let count_before = engine.profiler.execution_counts.get(&func_id).copied().unwrap_or(0);
    
    for _ in 0..n {
        engine.execute_function(func_id);
    }
    
    let count_after = engine.profiler.execution_counts.get(&func_id).copied().unwrap_or(0);
    
    count_after == count_before + n
}
```

### 测试数据生成

为属性测试实现智能生成器：

```rust
use quickcheck::{Arbitrary, Gen};

// 生成有效的 JavaScript 源代码
#[derive(Clone, Debug)]
struct ValidJSSource(String);

impl Arbitrary for ValidJSSource {
    fn arbitrary(g: &mut Gen) -> Self {
        // 生成支持的 JavaScript 子集
        // 确保生成的代码语法正确
        ValidJSSource(generate_valid_js(g))
    }
}

// 生成有效的函数
#[derive(Clone, Debug)]
struct ValidFunction {
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,
}

impl Arbitrary for ValidFunction {
    fn arbitrary(g: &mut Gen) -> Self {
        ValidFunction {
            name: generate_identifier(g),
            params: generate_params(g),
            body: generate_statements(g),
        }
    }
}
```

### 测试覆盖目标

1. **解析器**：所有支持的语法元素，错误情况
2. **字节码生成**：所有 AST 节点类型到字节码的转换
3. **解释器**：所有字节码指令的执行，调用栈管理
4. **热点检测**：阈值配置，热点标记
5. **优化编译**：IR 生成，优化 passes，代码生成
6. **反优化**：类型保护失败，状态重建
7. **端到端**：完整执行流程，包括优化和反优化

## 实现注意事项

### 性能考虑

1. **解释器优化**：使用直接线程化（direct threading）或计算 goto 优化字节码分发
2. **内存分配**：使用内存池减少频繁的小对象分配
3. **热点检测**：使用低开销的计数器，避免影响解释执行性能
4. **JIT 编译**：异步编译，不阻塞解释执行

### 安全性考虑

1. **内存安全**：利用 Rust 的所有权系统保证内存安全
2. **类型安全**：在优化代码中插入运行时类型检查
3. **栈溢出保护**：限制调用栈深度
4. **整数溢出**：使用 checked 算术操作

### 可扩展性

1. **模块化设计**：各组件通过清晰的接口交互
2. **可配置性**：热点阈值、优化级别等参数可配置
3. **后端抽象**：代码生成后端可切换（Cranelift/Dynasm）
4. **未来扩展**：预留接口支持更多 JavaScript 特性

## 参考资料

1. V8 引擎架构文档
2. Ignition 字节码解释器设计
3. TurboFan 优化编译器论文
4. Cranelift 代码生成器文档
5. Property-Based Testing 最佳实践
