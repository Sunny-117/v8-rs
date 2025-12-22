# 第 7 章：字节码生成器

## 本章目标

在本章中，我们将实现字节码生成器，它负责：
1. 遍历 AST 并生成字节码
2. 处理控制流（if、for、跳转）
3. 管理变量和作用域
4. 生成优化的字节码

## 什么是字节码生成？

字节码生成是编译器的后端，将高层的 AST 转换为低层的字节码指令。这个过程也叫"代码生成"或"lowering"。

**转换示例：**
```
AST: BinaryExpr(Add, Number(1), Number(2))
  ↓
Bytecode:
  LoadConst 0  // 1
  LoadConst 1  // 2
  Add
```

## 步骤 1：BytecodeGenerator 结构

```rust
pub struct BytecodeGenerator {
    chunk: BytecodeChunk,
    scope: Scope,
}
```

**字段说明：**
- **chunk**：正在生成的字节码块
- **scope**：当前作用域（用于变量解析）

### 初始化

```rust
impl BytecodeGenerator {
    pub fn new(scope: Scope) -> Self {
        Self {
            chunk: BytecodeChunk::new(),
            scope,
        }
    }
}
```

## 步骤 2：生成入口

```rust
pub fn generate(&mut self, ast: &ASTNode) -> BytecodeChunk {
    self.compile_node(ast);
    self.chunk.set_local_count(self.scope.local_count());
    self.chunk.clone()
}
```

**流程：**
1. 编译 AST 节点
2. 设置局部变量数量
3. 返回生成的字节码块

## 步骤 3：编译不同类型的节点

### 3.1 Program 节点

```rust
ASTNode::Program(stmts) => {
    for stmt in stmts {
        self.compile_node(stmt);
    }
}
```

**简单遍历**：依次编译每个语句

### 3.2 数字字面量

```rust
ASTNode::NumberLiteral { value, .. } => {
    let idx = self.chunk.add_constant(Value::Number(*value));
    self.chunk.emit(Instruction::LoadConst(idx));
}
```

**步骤：**
1. 将数字添加到常量池
2. 生成 LoadConst 指令

**例子：**
```javascript
42
```
生成：
```
LoadConst 0  // constants[0] = 42.0
```

### 3.3 标识符

```rust
ASTNode::Identifier { name, .. } => {
    if let Some(idx) = self.scope.lookup(name) {
        self.chunk.emit(Instruction::LoadLocal(idx));
    } else {
        panic!("Undefined variable: {}", name);
    }
}
```

**步骤：**
1. 在作用域中查找变量
2. 生成 LoadLocal 指令
3. 如果未找到，报错

**例子：**
```javascript
x
```
生成：
```
LoadLocal 0  // 假设 x 是第 0 个局部变量
```

### 3.4 二元表达式

```rust
ASTNode::BinaryExpr { op, left, right, .. } => {
    // 先编译左操作数
    self.compile_node(left);
    
    // 再编译右操作数
    self.compile_node(right);
    
    // 最后生成运算指令
    match op {
        BinOp::Add => self.chunk.emit(Instruction::Add),
        BinOp::Sub => self.chunk.emit(Instruction::Sub),
        BinOp::Mul => self.chunk.emit(Instruction::Mul),
        BinOp::Div => self.chunk.emit(Instruction::Div),
        _ => {}
    }
}
```

**后序遍历**：先处理子节点，再处理当前节点

**例子：**
```javascript
1 + 2
```
生成：
```
LoadConst 0  // 1
LoadConst 1  // 2
Add
```

**复杂例子：**
```javascript
(1 + 2) * 3
```
生成：
```
LoadConst 0  // 1
LoadConst 1  // 2
Add          // (1 + 2)
LoadConst 2  // 3
Mul          // (1 + 2) * 3
```

### 3.5 Let 声明

```rust
ASTNode::LetDecl { name, init, .. } => {
    // 编译初始化表达式
    self.compile_node(init);
    
    // 声明变量并获取索引
    let idx = self.scope.declare(name.clone());
    
    // 生成存储指令
    self.chunk.emit(Instruction::StoreLocal(idx));
}
```

**例子：**
```javascript
let x = 10;
```
生成：
```
LoadConst 0   // 10
StoreLocal 0  // x = 10
```

## 步骤 4：控制流编译

### 4.1 If 语句

```javascript
if (condition) {
    thenBranch
} else {
    elseBranch
}
```

**字节码结构：**
```
    编译 condition
    JumpIfFalse else_label
    编译 thenBranch
    Jump end_label
else_label:
    编译 elseBranch
end_label:
    ...
```

**实现：**
```rust
ASTNode::IfStmt { cond, then_branch, else_branch, .. } => {
    // 编译条件
    self.compile_node(cond);
    
    // JumpIfFalse（占位符）
    let jump_if_false_idx = self.chunk.instructions.len();
    self.chunk.emit(Instruction::JumpIfFalse(0));
    
    // 编译 then 分支
    self.compile_node(then_branch);
    
    // Jump（占位符）
    let jump_idx = self.chunk.instructions.len();
    self.chunk.emit(Instruction::Jump(0));
    
    // 回填 JumpIfFalse
    let else_start = self.chunk.instructions.len();
    let jump_if_false_offset = (else_start as isize) - (jump_if_false_idx as isize) - 1;
    self.chunk.instructions[jump_if_false_idx] = 
        Instruction::JumpIfFalse(jump_if_false_offset);
    
    // 编译 else 分支
    if let Some(else_br) = else_branch {
        self.compile_node(else_br);
    }
    
    // 回填 Jump
    let end = self.chunk.instructions.len();
    let jump_offset = (end as isize) - (jump_idx as isize) - 1;
    self.chunk.instructions[jump_idx] = Instruction::Jump(jump_offset);
}
```

**回填技术：**
1. 先生成跳转指令（偏移量为 0）
2. 编译目标代码
3. 计算实际偏移量
4. 回填跳转指令

**例子：**
```javascript
if (x > 0) {
    y = 1;
} else {
    y = -1;
}
```

生成：
```
0: LoadLocal 0      // x
1: LoadConst 0      // 0
2: Greater
3: JumpIfFalse 3    // 跳到指令 7
4: LoadConst 1      // 1
5: StoreLocal 1     // y = 1
6: Jump 2           // 跳到指令 9
7: LoadConst 2      // -1
8: StoreLocal 1     // y = -1
9: ...
```

### 4.2 For 循环

```javascript
for (init; condition; update) {
    body
}
```

**字节码结构：**
```
    编译 init
loop_start:
    编译 condition
    JumpIfFalse loop_end
    编译 body
    编译 update
    Jump loop_start
loop_end:
    ...
```

**实现：**
```rust
ASTNode::ForStmt { init, cond, update, body, .. } => {
    // 编译初始化
    self.compile_node(init);
    
    // 循环开始
    let loop_start = self.chunk.instructions.len();
    
    // 编译条件
    self.compile_node(cond);
    
    // JumpIfFalse（占位符）
    let jump_if_false_idx = self.chunk.instructions.len();
    self.chunk.emit(Instruction::JumpIfFalse(0));
    
    // 编译循环体
    self.compile_node(body);
    
    // 编译更新
    self.compile_node(update);
    
    // 跳回循环开始
    let current = self.chunk.instructions.len();
    let jump_back_offset = (loop_start as isize) - (current as isize) - 1;
    self.chunk.emit(Instruction::Jump(jump_back_offset));
    
    // 回填 JumpIfFalse
    let end = self.chunk.instructions.len();
    let jump_if_false_offset = (end as isize) - (jump_if_false_idx as isize) - 1;
    self.chunk.instructions[jump_if_false_idx] = 
        Instruction::JumpIfFalse(jump_if_false_offset);
}
```

**例子：**
```javascript
for (let i = 0; i < 10; i = i + 1) {
    sum = sum + i;
}
```

生成：
```
 0: LoadConst 0      // 0
 1: StoreLocal 0     // i = 0
 2: LoadLocal 0      // i (循环开始)
 3: LoadConst 1      // 10
 4: Less
 5: JumpIfFalse 8    // 跳到指令 14
 6: LoadLocal 1      // sum
 7: LoadLocal 0      // i
 8: Add
 9: StoreLocal 1     // sum = sum + i
10: LoadLocal 0      // i
11: LoadConst 2      // 1
12: Add
13: StoreLocal 0     // i = i + 1
14: Jump -13         // 跳回指令 2
15: ...
```

### 4.3 Return 语句

```rust
ASTNode::ReturnStmt { value, .. } => {
    self.compile_node(value);
    self.chunk.emit(Instruction::Return);
}
```

**例子：**
```javascript
return x + 1;
```

生成：
```
LoadLocal 0   // x
LoadConst 0   // 1
Add
Return
```

### 4.4 函数调用

```rust
ASTNode::CallExpr { callee, args, .. } => {
    // 编译被调用者
    self.compile_node(callee);
    
    // 编译参数
    for arg in args {
        self.compile_node(arg);
    }
    
    // 生成调用指令
    self.chunk.emit(Instruction::Call(args.len()));
}
```

**例子：**
```javascript
foo(1, 2)
```

生成：
```
LoadLocal 0   // foo
LoadConst 0   // 1
LoadConst 1   // 2
Call 2        // 调用，2 个参数
```

## 步骤 5：优化技术

### 5.1 常量折叠

```rust
fn fold_constants(&mut self, node: &ASTNode) -> Option<Value> {
    match node {
        ASTNode::BinaryExpr { op: BinOp::Add, left, right, .. } => {
            if let (Some(Value::Number(l)), Some(Value::Number(r))) = 
                (self.fold_constants(left), self.fold_constants(right)) {
                return Some(Value::Number(l + r));
            }
        }
        ASTNode::NumberLiteral { value, .. } => {
            return Some(Value::Number(*value));
        }
        _ => {}
    }
    None
}
```

**例子：**
```javascript
1 + 2
```

优化前：
```
LoadConst 0  // 1
LoadConst 1  // 2
Add
```

优化后：
```
LoadConst 0  // 3
```

### 5.2 死代码消除

```rust
fn is_constant_true(node: &ASTNode) -> bool {
    matches!(node, ASTNode::NumberLiteral { value, .. } if *value != 0.0)
}

// 在编译 if 语句时
if is_constant_true(cond) {
    // 只编译 then 分支
    self.compile_node(then_branch);
} else {
    // 正常编译
}
```

## 测试字节码生成

### 测试数字编译

```rust
#[test]
fn test_compile_number() {
    let mut parser = Parser::new("42".to_string());
    let ast = parser.parse().unwrap();
    
    let mut gen = BytecodeGenerator::new(Scope::global());
    let chunk = gen.generate(&ast.root);
    
    assert_eq!(chunk.instructions.len(), 1);
    assert_eq!(chunk.instructions[0], Instruction::LoadConst(0));
    assert_eq!(chunk.constants[0], Value::Number(42.0));
}
```

### 测试表达式编译

```rust
#[test]
fn test_compile_binary_expr() {
    let mut parser = Parser::new("1 + 2".to_string());
    let ast = parser.parse().unwrap();
    
    let mut gen = BytecodeGenerator::new(Scope::global());
    let chunk = gen.generate(&ast.root);
    
    // 应该有：LoadConst, LoadConst, Add
    assert!(chunk.instructions.len() >= 3);
    assert_eq!(chunk.instructions[chunk.instructions.len() - 1], Instruction::Add);
}
```

### 测试变量编译

```rust
#[test]
fn test_compile_let_decl() {
    let mut parser = Parser::new("let x = 10;".to_string());
    let ast = parser.parse().unwrap();
    
    let mut gen = BytecodeGenerator::new(Scope::global());
    let chunk = gen.generate(&ast.root);
    
    // 应该有：LoadConst, StoreLocal
    assert!(chunk.instructions.len() >= 2);
    assert!(matches!(
        chunk.instructions[chunk.instructions.len() - 1],
        Instruction::StoreLocal(0)
    ));
}
```

## 常见问题

### Q1: 为什么需要回填？

**答：** 因为在生成跳转指令时，我们还不知道目标位置。只有编译完目标代码后，才能计算偏移量。

### Q2: 如何处理嵌套的控制流？

**答：** 递归编译。每个控制流结构独立处理自己的跳转。

### Q3: 如何优化跳转链？

**答：**
```rust
// 跳转链：Jump → Jump → Jump → target
// 优化为：Jump → target

fn optimize_jumps(&mut self) {
    for i in 0..self.chunk.instructions.len() {
        if let Instruction::Jump(offset) = self.chunk.instructions[i] {
            let target = (i as isize + offset + 1) as usize;
            if let Instruction::Jump(next_offset) = self.chunk.instructions[target] {
                // 直接跳到最终目标
                let final_offset = offset + next_offset + 1;
                self.chunk.instructions[i] = Instruction::Jump(final_offset);
            }
        }
    }
}
```

## 调试技巧

### 反汇编字节码

```rust
fn disassemble(chunk: &BytecodeChunk) {
    println!("=== Bytecode ===");
    println!("Constants: {:?}", chunk.constants);
    println!("Local count: {}", chunk.local_count);
    println!("\nInstructions:");
    
    for (i, instruction) in chunk.instructions.iter().enumerate() {
        println!("{:04} {:?}", i, instruction);
    }
}
```

### 追踪编译过程

```rust
fn compile_node(&mut self, node: &ASTNode) {
    println!("Compiling: {:?}", node);
    
    // 编译逻辑...
    
    println!("Generated: {:?}", self.chunk.instructions.last());
}
```

## 下一步

在下一章中，我们将实现 Ignition 解释器，执行生成的字节码。

## 练习

1. 实现常量折叠优化
2. 添加对 while 循环的支持
3. 实现窥孔优化
4. 添加字节码验证器

## 完整代码

本章的完整代码在 `src/codegen.rs` 文件中。
