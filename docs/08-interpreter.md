# 第 8 章：Ignition 解释器

## 本章目标

在本章中，我们将实现 Ignition 字节码解释器，它负责：
1. 执行字节码指令
2. 管理调用栈和操作数栈
3. 处理运行时错误
4. 实现栈式虚拟机

## 什么是字节码解释器？

字节码解释器是一个虚拟机，它逐条执行字节码指令。类似于 CPU 执行机器码，但运行在软件层面。

**优点：**
- 比直接解释 AST 更快
- 比编译为机器码更简单
- 便于优化和调试

## 步骤 1：调用帧（CallFrame）

### 1.1 CallFrame 结构

```rust
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub chunk: BytecodeChunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub func_id: FunctionId,
}
```

**字段说明：**

1. **chunk**：当前执行的字节码块
2. **ip**：指令指针（Instruction Pointer）
   - 指向下一条要执行的指令
3. **stack**：操作数栈
   - 存储临时计算结果
4. **locals**：局部变量数组
   - 存储函数的局部变量
5. **func_id**：函数 ID
   - 用于调试和性能分析

### 1.2 创建调用帧

```rust
impl CallFrame {
    pub fn new(chunk: BytecodeChunk, func_id: FunctionId) -> Self {
        let local_count = chunk.local_count;
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
            locals: vec![Value::Undefined; local_count],
            func_id,
        }
    }
}
```

**初始化：**
- IP 从 0 开始
- 栈为空
- 局部变量初始化为 Undefined

### 1.3 栈操作

```rust
pub fn push(&mut self, value: Value) {
    self.stack.push(value);
}

pub fn pop(&mut self) -> Result<Value, RuntimeError> {
    self.stack.pop().ok_or(RuntimeError::StackOverflow)
}

pub fn peek(&self) -> Option<&Value> {
    self.stack.last()
}
```

**为什么 pop 返回 Result？**
- 栈可能为空
- 需要处理栈下溢错误

## 步骤 2：Ignition 解释器

### 2.1 Ignition 结构

```rust
pub struct Ignition {
    call_stack: Vec<CallFrame>,
}

impl Ignition {
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
        }
    }
}
```

**call_stack**：调用栈
- 每个函数调用创建一个新的 CallFrame
- 支持递归和嵌套调用

### 2.2 执行入口

```rust
pub fn execute(&mut self, chunk: BytecodeChunk) -> Result<Value, RuntimeError> {
    let frame = CallFrame::new(chunk, 0);
    self.call_stack.push(frame);
    
    self.run()
}
```

## 步骤 3：主执行循环

```rust
fn run(&mut self) -> Result<Value, RuntimeError> {
    loop {
        let frame = self.call_stack.last_mut()
            .ok_or(RuntimeError::StackOverflow)?;
        
        // 检查是否执行完毕
        if frame.ip >= frame.chunk.instructions.len() {
            let result = frame.pop().unwrap_or(Value::Undefined);
            self.call_stack.pop();
            
            if self.call_stack.is_empty() {
                return Ok(result);
            }
            
            // 将结果推送到调用者的栈
            if let Some(caller) = self.call_stack.last_mut() {
                caller.push(result);
            }
            continue;
        }
        
        // 获取并执行指令
        let instruction = frame.chunk.instructions[frame.ip].clone();
        frame.ip += 1;
        
        self.dispatch(instruction)?;
    }
}
```

**执行流程：**
1. 获取当前调用帧
2. 检查是否执行完毕
3. 如果完毕，返回结果
4. 否则，获取下一条指令
5. 执行指令
6. 重复

## 步骤 4：指令分发

### 4.1 LoadConst - 加载常量

```rust
Instruction::LoadConst(idx) => {
    let value = frame.chunk.constants.get(idx)
        .cloned()
        .ok_or(RuntimeError::StackOverflow)?;
    frame.push(value);
}
```

**例子：**
```
LoadConst 0  // 加载常量池中索引 0 的值
```

### 4.2 LoadLocal / StoreLocal - 局部变量

```rust
Instruction::LoadLocal(idx) => {
    let value = frame.locals.get(idx)
        .cloned()
        .ok_or(RuntimeError::UndefinedVariable {
            name: format!("local_{}", idx),
        })?;
    frame.push(value);
}

Instruction::StoreLocal(idx) => {
    let value = frame.pop()?;
    if idx < frame.locals.len() {
        frame.locals[idx] = value;
    }
}
```

**例子：**
```
LoadConst 0   // 加载 10
StoreLocal 0  // x = 10
LoadLocal 0   // 加载 x
```

### 4.3 算术运算

```rust
Instruction::Add => {
    let right = frame.pop()?;
    let left = frame.pop()?;
    
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            frame.push(Value::Number(l + r));
        }
        _ => {
            return Err(RuntimeError::TypeError {
                expected: "number".to_string(),
                found: "other".to_string(),
            });
        }
    }
}
```

**栈的变化：**
```
初始：[10, 20]
pop right: [10]
pop left: []
push result: [30]
```

### 4.4 除法（带错误检查）

```rust
Instruction::Div => {
    let right = frame.pop()?;
    let left = frame.pop()?;
    
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if r == 0.0 {
                return Err(RuntimeError::DivisionByZero);
            }
            frame.push(Value::Number(l / r));
        }
        _ => {
            return Err(RuntimeError::TypeError {
                expected: "number".to_string(),
                found: "other".to_string(),
            });
        }
    }
}
```

**为什么检查除零？**
- 虽然 f64 的除零返回 Infinity
- 但我们希望提供更明确的错误信息

### 4.5 跳转指令

```rust
Instruction::Jump(offset) => {
    let frame = self.call_stack.last_mut().unwrap();
    frame.ip = ((frame.ip as isize) + offset) as usize;
}

Instruction::JumpIfFalse(offset) => {
    let frame = self.call_stack.last_mut().unwrap();
    let cond = frame.peek().cloned().unwrap_or(Value::Undefined);
    
    let is_false = match cond {
        Value::Number(n) => n == 0.0,
        Value::Undefined => true,
        _ => false,
    };
    
    if is_false {
        frame.ip = ((frame.ip as isize) + offset) as usize;
    }
}
```

**跳转的用途：**
- 实现 if 语句
- 实现循环
- 实现短路求值

### 4.6 Return 指令

```rust
Instruction::Return => {
    let result = frame.pop().unwrap_or(Value::Undefined);
    self.call_stack.pop();
    
    if let Some(caller) = self.call_stack.last_mut() {
        caller.push(result);
    }
}
```

**返回流程：**
1. 弹出返回值
2. 移除当前调用帧
3. 将返回值推送到调用者的栈

## 步骤 5：执行示例

### 5.1 简单表达式

```javascript
10 + 20
```

字节码：
```
LoadConst 0  // 10
LoadConst 1  // 20
Add
```

执行过程：
```
1. LoadConst 0
   stack: [10]

2. LoadConst 1
   stack: [10, 20]

3. Add
   pop 20, pop 10
   push 30
   stack: [30]

4. 结束，返回 30
```

### 5.2 变量赋值

```javascript
let x = 10;
let y = 20;
x + y
```

字节码：
```
LoadConst 0   // 10
StoreLocal 0  // x = 10
LoadConst 1   // 20
StoreLocal 1  // y = 20
LoadLocal 0   // 加载 x
LoadLocal 1   // 加载 y
Add
```

执行过程：
```
1. LoadConst 0, StoreLocal 0
   locals: [10, Undefined]
   stack: []

2. LoadConst 1, StoreLocal 1
   locals: [10, 20]
   stack: []

3. LoadLocal 0, LoadLocal 1
   stack: [10, 20]

4. Add
   stack: [30]
```

### 5.3 条件语句

```javascript
if (x > 0) {
    return 1;
} else {
    return -1;
}
```

字节码：
```
LoadLocal 0      // x
LoadConst 0      // 0
Greater
JumpIfFalse 3    // 如果 false，跳到 else
LoadConst 1      // 1
Return
LoadConst 2      // -1
Return
```

## 测试解释器

### 测试加载常量

```rust
#[test]
fn test_execute_load_const() {
    let mut chunk = BytecodeChunk::new();
    let idx = chunk.add_constant(Value::Number(42.0));
    chunk.emit(Instruction::LoadConst(idx));
    
    let mut interpreter = Ignition::new();
    let result = interpreter.execute(chunk).unwrap();
    
    assert_eq!(result, Value::Number(42.0));
}
```

### 测试算术运算

```rust
#[test]
fn test_execute_add() {
    let mut chunk = BytecodeChunk::new();
    let idx1 = chunk.add_constant(Value::Number(10.0));
    let idx2 = chunk.add_constant(Value::Number(20.0));
    chunk.emit(Instruction::LoadConst(idx1));
    chunk.emit(Instruction::LoadConst(idx2));
    chunk.emit(Instruction::Add);
    
    let mut interpreter = Ignition::new();
    let result = interpreter.execute(chunk).unwrap();
    
    assert_eq!(result, Value::Number(30.0));
}
```

### 测试错误处理

```rust
#[test]
fn test_execute_division_by_zero() {
    let mut chunk = BytecodeChunk::new();
    let idx1 = chunk.add_constant(Value::Number(10.0));
    let idx2 = chunk.add_constant(Value::Number(0.0));
    chunk.emit(Instruction::LoadConst(idx1));
    chunk.emit(Instruction::LoadConst(idx2));
    chunk.emit(Instruction::Div);
    
    let mut interpreter = Ignition::new();
    let result = interpreter.execute(chunk);
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RuntimeError::DivisionByZero));
}
```

## 性能优化

### 1. 直接线程化（Direct Threading）

```rust
// 当前实现：间接跳转
match instruction {
    Instruction::Add => { /* ... */ }
    Instruction::Sub => { /* ... */ }
}

// 优化：直接跳转（需要 unsafe）
static DISPATCH_TABLE: [fn(&mut Ignition); 256] = [
    op_add, op_sub, // ...
];
```

### 2. 栈缓存

```rust
// 将栈顶几个值缓存在寄存器中
let mut stack_top = Value::Undefined;
let mut stack_top_1 = Value::Undefined;
```

### 3. 指令融合

```rust
// 将常见的指令序列合并
LoadConst 0
LoadConst 1
Add

// 合并为
AddConsts 0 1
```

## 调试技巧

### 打印执行过程

```rust
fn dispatch(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
    println!("Executing: {:?}", instruction);
    println!("Stack before: {:?}", self.call_stack.last().unwrap().stack);
    
    // 执行指令...
    
    println!("Stack after: {:?}", self.call_stack.last().unwrap().stack);
    Ok(())
}
```

### 栈追踪

```rust
fn print_stack_trace(&self) {
    println!("Call stack:");
    for (i, frame) in self.call_stack.iter().enumerate() {
        println!("  Frame {}: func_id={}, ip={}", i, frame.func_id, frame.ip);
    }
}
```

## 常见问题

### Q1: 为什么使用栈式虚拟机？

**答：**
- 简单：指令不需要指定操作数位置
- 紧凑：字节码更小
- 易于实现：栈操作很直观

### Q2: 如何实现函数调用？

**答：**
```rust
Instruction::Call(arg_count) => {
    // 1. 从栈中弹出参数
    let mut args = Vec::new();
    for _ in 0..arg_count {
        args.push(frame.pop()?);
    }
    
    // 2. 弹出函数
    let func = frame.pop()?;
    
    // 3. 创建新的调用帧
    let new_frame = CallFrame::new(func_chunk, func_id);
    
    // 4. 将参数复制到局部变量
    for (i, arg) in args.iter().enumerate() {
        new_frame.locals[i] = arg.clone();
    }
    
    // 5. 推送新帧
    self.call_stack.push(new_frame);
}
```

### Q3: 如何限制栈深度？

**答：**
```rust
const MAX_CALL_DEPTH: usize = 1000;

fn run(&mut self) -> Result<Value, RuntimeError> {
    if self.call_stack.len() > MAX_CALL_DEPTH {
        return Err(RuntimeError::StackOverflow);
    }
    // ...
}
```

## 下一步

在下一章中，我们将实现 Engine 引擎总控，整合所有组件。

## 练习

1. 添加对字符串连接的支持
2. 实现 print 指令用于调试
3. 添加断点支持
4. 实现单步执行模式

## 完整代码

本章的完整代码在 `src/interpreter.rs` 文件中。
