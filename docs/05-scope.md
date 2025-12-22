# 第 5 章：作用域系统

## 本章目标

在本章中，我们将实现作用域管理系统，它负责：
1. 管理变量的声明和查找
2. 实现词法作用域
3. 支持嵌套作用域
4. 为字节码生成提供变量索引

## 什么是作用域？

作用域定义了变量的可见性范围。JavaScript 使用词法作用域（静态作用域），变量的作用域在编写代码时就确定了。

**例子：**
```javascript
let x = 10;        // 全局作用域

function foo() {
    let y = 20;    // 函数作用域
    {
        let z = 30; // 块作用域
        console.log(x, y, z); // 都可见
    }
    // z 不可见
}
// y 不可见
```

## 步骤 1：设计作用域类型

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
}
```

**三种作用域：**
- **Global**：全局作用域，程序顶层
- **Function**：函数作用域，函数内部
- **Block**：块作用域，`{}` 内部

## 步骤 2：Scope 结构体

```rust
#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, usize>,
    scope_type: ScopeType,
    next_index: usize,
}
```

**字段说明：**

1. **parent**：父作用域
   - 用于实现作用域链
   - None 表示全局作用域

2. **variables**：变量映射
   - 键：变量名
   - 值：变量索引（用于字节码）

3. **scope_type**：作用域类型
   - 用于区分不同的作用域

4. **next_index**：下一个可用索引
   - 每声明一个变量，索引递增
   - 用于分配局部变量槽位

## 步骤 3：创建作用域

### 3.1 创建全局作用域

```rust
impl Scope {
    pub fn new(scope_type: ScopeType, parent: Option<Box<Scope>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
            scope_type,
            next_index: 0,
        }
    }
    
    pub fn global() -> Self {
        Self::new(ScopeType::Global, None)
    }
}
```

### 3.2 创建子作用域

```rust
pub fn function_scope(&self) -> Self {
    Self::new(ScopeType::Function, Some(Box::new(self.clone())))
}

pub fn block_scope(&self) -> Self {
    Self::new(ScopeType::Block, Some(Box::new(self.clone())))
}
```

**为什么要 clone？**
- 作用域需要独立存在
- 子作用域可能比父作用域活得更久
- Rust 的所有权系统要求明确的所有权

## 步骤 4：变量操作

### 4.1 声明变量

```rust
pub fn declare(&mut self, name: String) -> usize {
    let index = self.next_index;
    self.variables.insert(name, index);
    self.next_index += 1;
    index
}
```

**返回索引的用途：**
- 字节码生成时需要知道变量的位置
- 例如：`LoadLocal(0)` 加载第 0 个局部变量

**例子：**
```rust
let mut scope = Scope::global();
let x_index = scope.declare("x".to_string()); // 返回 0
let y_index = scope.declare("y".to_string()); // 返回 1
```

### 4.2 查找变量

```rust
pub fn lookup(&self, name: &str) -> Option<usize> {
    if let Some(&index) = self.variables.get(name) {
        Some(index)
    } else if let Some(ref parent) = self.parent {
        parent.lookup(name)
    } else {
        None
    }
}
```

**查找算法：**
1. 在当前作用域查找
2. 如果找到，返回索引
3. 如果没找到，在父作用域查找
4. 递归直到找到或到达全局作用域
5. 如果都没找到，返回 None

**例子：**
```rust
let mut global = Scope::global();
global.declare("x".to_string());

let mut func = global.function_scope();
func.declare("y".to_string());

assert_eq!(func.lookup("y"), Some(0)); // 在当前作用域
assert_eq!(func.lookup("x"), Some(0)); // 在父作用域
assert_eq!(func.lookup("z"), None);    // 不存在
```

## 步骤 5：作用域链

### 5.1 作用域链的概念

作用域链是一系列嵌套的作用域，从内到外：

```
Block Scope (z)
    ↓
Function Scope (y)
    ↓
Global Scope (x)
```

### 5.2 实现示例

```rust
let mut global = Scope::global();
global.declare("a".to_string());

let mut func = global.function_scope();
func.declare("b".to_string());

let mut block = func.block_scope();
block.declare("c".to_string());

// block 可以访问所有变量
assert!(block.lookup("a").is_some());
assert!(block.lookup("b").is_some());
assert!(block.lookup("c").is_some());

// func 不能访问 block 的变量
assert!(func.lookup("c").is_none());
```

## 步骤 6：变量索引的含义

### 6.1 为什么需要索引？

在字节码中，我们使用索引而不是名称来引用变量：

```javascript
let x = 10;
let y = 20;
let z = x + y;
```

字节码：
```
LoadConst 0    // 10
StoreLocal 0   // x = 10
LoadConst 1    // 20
StoreLocal 1   // y = 20
LoadLocal 0    // 加载 x
LoadLocal 1    // 加载 y
Add
StoreLocal 2   // z = x + y
```

### 6.2 索引分配策略

每个作用域独立分配索引：

```rust
// 全局作用域
let x = 10;  // index 0
let y = 20;  // index 1

function foo() {
    // 函数作用域（重新从 0 开始）
    let a = 1;  // index 0
    let b = 2;  // index 1
}
```

## 步骤 7：实用方法

### 7.1 获取局部变量数量

```rust
pub fn local_count(&self) -> usize {
    self.next_index
}
```

**用途：**
- 字节码生成时需要知道需要多少局部变量槽位
- 用于分配调用帧的局部变量数组

### 7.2 获取作用域类型

```rust
pub fn scope_type(&self) -> &ScopeType {
    &self.scope_type
}
```

## 测试作用域系统

### 测试变量声明

```rust
#[test]
fn test_declare_variable() {
    let mut scope = Scope::global();
    let index = scope.declare("x".to_string());
    assert_eq!(index, 0);
    
    let index2 = scope.declare("y".to_string());
    assert_eq!(index2, 1);
}
```

### 测试变量查找

```rust
#[test]
fn test_lookup_variable() {
    let mut scope = Scope::global();
    scope.declare("x".to_string());
    
    assert_eq!(scope.lookup("x"), Some(0));
    assert_eq!(scope.lookup("y"), None);
}
```

### 测试嵌套作用域

```rust
#[test]
fn test_nested_scope_lookup() {
    let mut global = Scope::global();
    global.declare("x".to_string());
    
    let mut func = global.function_scope();
    func.declare("y".to_string());
    
    // 可以找到 x（父作用域）和 y（当前作用域）
    assert_eq!(func.lookup("x"), Some(0));
    assert_eq!(func.lookup("y"), Some(0));
    assert_eq!(func.lookup("z"), None);
}
```

### 测试作用域链

```rust
#[test]
fn test_scope_chain() {
    let mut global = Scope::global();
    global.declare("a".to_string());
    
    let mut func = global.function_scope();
    func.declare("b".to_string());
    
    let mut block = func.block_scope();
    block.declare("c".to_string());
    
    // 块作用域可以看到所有变量
    assert_eq!(block.lookup("a"), Some(0));
    assert_eq!(block.lookup("b"), Some(0));
    assert_eq!(block.lookup("c"), Some(0));
}
```

## 作用域与闭包

### 闭包的挑战

```javascript
function outer() {
    let x = 10;
    return function inner() {
        return x; // 访问外部变量
    };
}
```

**问题：**
- `inner` 需要访问 `outer` 的变量 `x`
- 但 `outer` 已经返回，其作用域应该被销毁

**解决方案：**
1. **闭包捕获**：将外部变量复制到闭包中
2. **上值（Upvalue）**：保持对外部变量的引用

我们的简化实现暂不支持闭包。

## 作用域与字节码生成

### 集成示例

```rust
pub struct BytecodeGenerator {
    chunk: BytecodeChunk,
    scope: Scope,
}

impl BytecodeGenerator {
    fn compile_let_decl(&mut self, name: &str, init: &ASTNode) {
        // 编译初始化表达式
        self.compile_node(init);
        
        // 声明变量并获取索引
        let index = self.scope.declare(name.to_string());
        
        // 生成存储指令
        self.chunk.emit(Instruction::StoreLocal(index));
    }
    
    fn compile_identifier(&mut self, name: &str) {
        // 查找变量索引
        if let Some(index) = self.scope.lookup(name) {
            // 生成加载指令
            self.chunk.emit(Instruction::LoadLocal(index));
        } else {
            // 变量未定义错误
            panic!("Undefined variable: {}", name);
        }
    }
}
```

## 常见问题

### Q1: 为什么不使用全局变量表？

**答：** 
- 每个作用域独立管理变量，更清晰
- 支持变量遮蔽（shadowing）
- 便于实现块作用域

### Q2: 如何处理变量遮蔽？

```javascript
let x = 10;
{
    let x = 20; // 遮蔽外部的 x
    console.log(x); // 20
}
console.log(x); // 10
```

**答：** 当前实现自动支持，因为内部作用域的查找优先于外部。

### Q3: 如何实现 const？

**答：** 在 Scope 中添加一个 `constants` 集合：

```rust
pub struct Scope {
    // ...
    constants: HashSet<String>,
}

pub fn declare_const(&mut self, name: String) -> usize {
    let index = self.declare(name.clone());
    self.constants.insert(name);
    index
}

pub fn is_const(&self, name: &str) -> bool {
    self.constants.contains(name)
}
```

## 性能优化

### 使用 HashMap 而不是 Vec

```rust
// 慢：O(n) 查找
variables: Vec<(String, usize)>

// 快：O(1) 查找
variables: HashMap<String, usize>
```

### 避免不必要的克隆

```rust
// 不好：每次都克隆
pub fn function_scope(&self) -> Self {
    Self::new(ScopeType::Function, Some(Box::new(self.clone())))
}

// 更好：使用 Rc 共享
pub struct Scope {
    parent: Option<Rc<Scope>>,
    // ...
}
```

## 下一步

在下一章中，我们将实现字节码指令集，为字节码生成做准备。

## 练习

1. 添加对 const 声明的支持
2. 实现变量重复声明检查
3. 添加作用域的可视化打印
4. 实现简单的闭包支持

## 完整代码

本章的完整代码在 `src/scope.rs` 文件中。
