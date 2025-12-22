# 第 6 章：字节码指令集

## 本章目标

在本章中，我们将设计字节码指令集，它是：
1. AST 和机器执行之间的桥梁
2. 比 AST 更接近机器，比机器码更抽象
3. 易于解释执行和优化

## 什么是字节码？

字节码是一种中间表示，介于高级语言和机器码之间。它的特点是：
- **紧凑**：比 AST 占用更少空间
- **快速**：比解释 AST 更快
- **可移植**：不依赖特定硬件

**类比：**
- Java 字节码 → JVM
- Python 字节码 → CPython
- JavaScript 字节码 → V8/SpiderMonkey

## 步骤 1：指令集设计

### 1.1 Instruction 枚举

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    LoadConst(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    Add,
    Sub,
    Mul,
    Div,
    Call(usize),
    Return,
    Jump(isize),
    JumpIfFalse(isize),
}
```

### 1.2 指令分类

#### 数据操作指令

```rust
LoadConst(usize)   // 加载常量
LoadLocal(usize)   // 加载局部变量
StoreLocal(usize)  // 存储局部变量
```

**例子：**
```javascript
let x = 10;
```

字节码：
```
LoadConst 0    // 加载常量 10
StoreLocal 0   // 存储到变量 x
```

#### 算术运算指令

```rust
Add  // 加法
Sub  // 减法
Mul  // 乘法
Div  // 除法
```

**例子：**
```javascript
1 + 2 * 3
```

字节码：
```
LoadConst 0  // 1
LoadConst 1  // 2
LoadConst 2  // 3
Mul          // 2 * 3 = 6
Add          // 1 + 6 = 7
```

#### 控制流指令

```rust
Jump(isize)         // 无条件跳转
JumpIfFalse(isize)  // 条件跳转
Return              // 返回
Call(usize)         // 函数调用
```

**例子：**
```javascript
if (x > 0) {
    return 1;
}
return -1;
```

字节码：
```
LoadLocal 0      // x
LoadConst 0      // 0
Greater
JumpIfFalse 3    // 如果 false，跳过 3 条指令
LoadConst 1      // 1
Return
LoadConst 2      // -1
Return
```

## 步骤 2：BytecodeChunk 结构

### 2.1 字节码块

```rust
#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub local_count: usize,
}
```

**字段说明：**

1. **instructions**：指令序列
   - 按顺序执行的指令列表

2. **constants**：常量池
   - 存储所有常量值
   - 指令通过索引引用

3. **local_count**：局部变量数量
   - 用于分配局部变量空间

### 2.2 创建字节码块

```rust
impl BytecodeChunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            local_count: 0,
        }
    }
}
```

### 2.3 添加指令

```rust
pub fn emit(&mut self, instruction: Instruction) {
    self.instructions.push(instruction);
}
```

**例子：**
```rust
let mut chunk = BytecodeChunk::new();
chunk.emit(Instruction::LoadConst(0));
chunk.emit(Instruction::Add);
```

### 2.4 添加常量

```rust
pub fn add_constant(&mut self, value: Value) -> usize {
    self.constants.push(value);
    self.constants.len() - 1
}
```

**为什么返回索引？**
- 指令需要通过索引引用常量
- 避免重复存储相同的常量

**例子：**
```rust
let mut chunk = BytecodeChunk::new();
let idx = chunk.add_constant(Value::Number(42.0));
chunk.emit(Instruction::LoadConst(idx));
```

### 2.5 设置局部变量数量

```rust
pub fn set_local_count(&mut self, count: usize) {
    self.local_count = count;
}
```

## 步骤 3：指令编码

### 3.1 为什么需要编码？

当前实现使用 Rust 枚举，占用空间较大：
```rust
enum Instruction {
    Add,           // 可能占用 16 字节
    LoadConst(usize), // 可能占用 24 字节
}
```

**优化方案：**
```rust
// 使用字节数组
type Bytecode = Vec<u8>;

// 指令编码
const OP_ADD: u8 = 0x01;
const OP_LOAD_CONST: u8 = 0x02;

// 编码 LoadConst(5)
bytecode.push(OP_LOAD_CONST);
bytecode.push(5);
```

### 3.2 编码示例

```rust
pub fn encode(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    for instruction in &self.instructions {
        match instruction {
            Instruction::Add => {
                bytes.push(0x01);
            }
            Instruction::LoadConst(idx) => {
                bytes.push(0x02);
                bytes.extend_from_slice(&idx.to_le_bytes());
            }
            // ... 其他指令
        }
    }
    
    bytes
}
```

## 步骤 4：字节码示例

### 4.1 简单表达式

```javascript
10 + 20
```

字节码：
```
constants: [10.0, 20.0]
instructions:
  0: LoadConst 0
  1: LoadConst 1
  2: Add
```

### 4.2 变量赋值

```javascript
let x = 10;
let y = 20;
x + y
```

字节码：
```
constants: [10.0, 20.0]
local_count: 2
instructions:
  0: LoadConst 0
  1: StoreLocal 0
  2: LoadConst 1
  3: StoreLocal 1
  4: LoadLocal 0
  5: LoadLocal 1
  6: Add
```

### 4.3 条件语句

```javascript
if (x > 0) {
    y = 1;
} else {
    y = -1;
}
```

字节码：
```
constants: [0.0, 1.0, -1.0]
instructions:
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

### 4.4 循环

```javascript
for (let i = 0; i < 10; i = i + 1) {
    sum = sum + i;
}
```

字节码：
```
constants: [0.0, 10.0, 1.0]
instructions:
  0: LoadConst 0      // 0
  1: StoreLocal 0     // i = 0
  2: LoadLocal 0      // i (循环开始)
  3: LoadConst 1      // 10
  4: Less
  5: JumpIfFalse 8    // 如果 i >= 10，跳出循环
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

## 步骤 5：字节码优化

### 5.1 常量折叠

```javascript
1 + 2
```

未优化：
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

```javascript
if (true) {
    x = 1;
} else {
    x = 2;  // 永远不会执行
}
```

优化后：
```
LoadConst 0
StoreLocal 0
```

### 5.3 窥孔优化

```
LoadLocal 0
StoreLocal 0  // 无意义的操作
```

优化后：
```
// 删除
```

## 测试字节码

### 测试指令创建

```rust
#[test]
fn test_instruction_types() {
    let instructions = vec![
        Instruction::LoadConst(0),
        Instruction::Add,
        Instruction::Return,
    ];
    
    assert_eq!(instructions.len(), 3);
}
```

### 测试字节码块

```rust
#[test]
fn test_bytecode_chunk() {
    let mut chunk = BytecodeChunk::new();
    
    let idx = chunk.add_constant(Value::Number(42.0));
    chunk.emit(Instruction::LoadConst(idx));
    chunk.emit(Instruction::Return);
    
    assert_eq!(chunk.instructions.len(), 2);
    assert_eq!(chunk.constants.len(), 1);
}
```

### 测试常量池

```rust
#[test]
fn test_constant_pool() {
    let mut chunk = BytecodeChunk::new();
    
    let idx1 = chunk.add_constant(Value::Number(10.0));
    let idx2 = chunk.add_constant(Value::Number(20.0));
    
    assert_eq!(idx1, 0);
    assert_eq!(idx2, 1);
    assert_eq!(chunk.constants[0], Value::Number(10.0));
    assert_eq!(chunk.constants[1], Value::Number(20.0));
}
```

## 字节码 vs 机器码

### 字节码
- **抽象**：不依赖硬件
- **可移植**：跨平台
- **解释执行**：需要虚拟机
- **较慢**：有解释开销

### 机器码
- **具体**：特定于 CPU
- **不可移植**：平台相关
- **直接执行**：CPU 直接运行
- **快速**：无解释开销

## 常见问题

### Q1: 为什么不直接生成机器码？

**答：**
1. 机器码生成复杂
2. 需要支持多个平台
3. 字节码更易于优化
4. 可以先解释执行，热点代码再编译

### Q2: 如何选择指令集？

**答：** 考虑因素：
1. **简单性**：易于实现和理解
2. **紧凑性**：占用更少空间
3. **效率**：执行速度快
4. **可扩展性**：易于添加新指令

### Q3: 栈式 vs 寄存器式？

**栈式（我们的选择）：**
- 指令简单
- 代码紧凑
- 易于实现

**寄存器式：**
- 指令复杂
- 代码较大
- 执行更快

## 下一步

在下一章中，我们将实现字节码生成器，将 AST 转换为字节码。

## 练习

1. 添加比较运算指令（Less, Greater, Equal）
2. 添加逻辑运算指令（And, Or, Not）
3. 实现字节码的序列化和反序列化
4. 设计一个反汇编器，将字节码转换为可读格式

## 完整代码

本章的完整代码在 `src/bytecode.rs` 文件中。
