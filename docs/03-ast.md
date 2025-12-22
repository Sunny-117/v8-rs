# 第 3 章：抽象语法树（AST）

## 本章目标

在本章中，我们将：
1. 设计 AST 节点类型
2. 表示 JavaScript 的语法结构
3. 为每个节点添加位置信息

## 什么是 AST？

抽象语法树（Abstract Syntax Tree）是源代码的树形表示。它抽象掉了语法细节（如括号、分号），只保留语义信息。

**例子：**
```javascript
let x = 10 + 20;
```

AST 表示：
```
Program
└── LetDecl
    ├── name: "x"
    └── init: BinaryExpr
        ├── op: Add
        ├── left: NumberLiteral(10)
        └── right: NumberLiteral(20)
```

## 步骤 1：设计二元运算符

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Equal,  // ==
    Less,   // <
    Greater, // >
}
```

**为什么单独定义？**
- 运算符会在多个地方使用（解析、编译、优化）
- 便于添加新运算符
- 类型安全：不会混淆不同的运算符

## 步骤 2：设计 AST 节点

### 2.1 ASTNode 枚举

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
        span: Span,
    },
    
    LetDecl {
        name: String,
        init: Box<ASTNode>,
        span: Span,
    },
    
    // ... 其他节点类型
}
```

**设计原则：**

1. **使用 Box 包装子节点**
   - AST 节点可能很大，直接嵌套会导致栈溢出
   - Box 将数据存储在堆上

2. **每个节点都有 Span**
   - 用于错误报告
   - 用于调试和源码映射

3. **使用具名字段**
   - 比元组更清晰
   - 便于理解和维护

### 2.2 表达式节点

```rust
// 二元表达式：left op right
BinaryExpr {
    op: BinOp,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
    span: Span,
},

// 函数调用：callee(args)
CallExpr {
    callee: Box<ASTNode>,
    args: Vec<ASTNode>,
    span: Span,
},

// 标识符：变量名
Identifier {
    name: String,
    span: Span,
},

// 数字字面量：42, 3.14
NumberLiteral {
    value: f64,
    span: Span,
},
```

**表达式的特点：**
- 有值（可以求值）
- 可以嵌套
- 可以作为其他表达式的一部分

### 2.3 语句节点

```rust
// Let 声明：let x = expr;
LetDecl {
    name: String,
    init: Box<ASTNode>,
    span: Span,
},

// Return 语句：return expr;
ReturnStmt {
    value: Box<ASTNode>,
    span: Span,
},

// If 语句：if (cond) { then } else { else }
IfStmt {
    cond: Box<ASTNode>,
    then_branch: Box<ASTNode>,
    else_branch: Option<Box<ASTNode>>,
    span: Span,
},

// For 循环：for (init; cond; update) { body }
ForStmt {
    init: Box<ASTNode>,
    cond: Box<ASTNode>,
    update: Box<ASTNode>,
    body: Box<ASTNode>,
    span: Span,
},

// 块语句：{ statements }
BlockStmt {
    statements: Vec<ASTNode>,
    span: Span,
},
```

**语句的特点：**
- 执行动作
- 通常不返回值
- 控制程序流程

## 步骤 3：实用方法

### 3.1 获取节点的 Span

```rust
impl ASTNode {
    pub fn span(&self) -> Span {
        match self {
            ASTNode::Program(_) => Span::new(0, 0),
            ASTNode::FunctionDecl { span, .. } => *span,
            ASTNode::LetDecl { span, .. } => *span,
            ASTNode::BinaryExpr { span, .. } => *span,
            // ... 其他节点
        }
    }
}
```

**用途：**
- 错误报告时显示位置
- 合并多个节点的位置

### 3.2 AST 包装器

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub root: ASTNode,
}

impl AST {
    pub fn new(root: ASTNode) -> Self {
        Self { root }
    }
}
```

**为什么需要包装器？**
- 提供统一的入口点
- 未来可以添加元数据（如源文件名）
- 便于序列化和反序列化

## 步骤 4：AST 的构建示例

### 手动构建 AST

```rust
// 表示：let x = 10;
let ast = ASTNode::LetDecl {
    name: "x".to_string(),
    init: Box::new(ASTNode::NumberLiteral {
        value: 10.0,
        span: Span::new(8, 10),
    }),
    span: Span::new(0, 11),
};
```

### 构建复杂表达式

```rust
// 表示：1 + 2
let expr = ASTNode::BinaryExpr {
    op: BinOp::Add,
    left: Box::new(ASTNode::NumberLiteral {
        value: 1.0,
        span: Span::new(0, 1),
    }),
    right: Box::new(ASTNode::NumberLiteral {
        value: 2.0,
        span: Span::new(4, 5),
    }),
    span: Span::new(0, 5),
};
```

## AST 的遍历

### 递归遍历

```rust
fn visit_node(node: &ASTNode) {
    match node {
        ASTNode::BinaryExpr { left, right, .. } => {
            visit_node(left);
            visit_node(right);
        }
        ASTNode::LetDecl { init, .. } => {
            visit_node(init);
        }
        ASTNode::BlockStmt { statements, .. } => {
            for stmt in statements {
                visit_node(stmt);
            }
        }
        _ => {}
    }
}
```

### 访问者模式

```rust
trait Visitor {
    fn visit_binary_expr(&mut self, op: &BinOp, left: &ASTNode, right: &ASTNode);
    fn visit_let_decl(&mut self, name: &str, init: &ASTNode);
    // ... 其他方法
}
```

## 测试 AST

### 测试节点创建

```rust
#[test]
fn test_number_literal() {
    let node = ASTNode::NumberLiteral {
        value: 42.0,
        span: Span::new(0, 2),
    };
    assert_eq!(node.span(), Span::new(0, 2));
}
```

### 测试复杂结构

```rust
#[test]
fn test_binary_expr() {
    let left = Box::new(ASTNode::NumberLiteral {
        value: 1.0,
        span: Span::new(0, 1),
    });
    let right = Box::new(ASTNode::NumberLiteral {
        value: 2.0,
        span: Span::new(4, 5),
    });
    
    let node = ASTNode::BinaryExpr {
        op: BinOp::Add,
        left,
        right,
        span: Span::new(0, 5),
    };
    
    assert_eq!(node.span(), Span::new(0, 5));
}
```

## AST vs Parse Tree

### Parse Tree（解析树）
- 包含所有语法细节
- 包括括号、分号等
- 直接对应语法规则

### AST（抽象语法树）
- 只保留语义信息
- 去掉了语法噪音
- 更适合后续处理

**例子：**
```javascript
(1 + 2) * 3
```

Parse Tree:
```
Expression
├── (
├── Expression
│   ├── Number(1)
│   ├── +
│   └── Number(2)
├── )
├── *
└── Number(3)
```

AST:
```
BinaryExpr(Mul)
├── BinaryExpr(Add)
│   ├── Number(1)
│   └── Number(2)
└── Number(3)
```

## 设计考虑

### 1. 为什么不用 trait object？

```rust
// 不推荐
trait ASTNode {}
struct BinaryExpr { ... }
impl ASTNode for BinaryExpr {}

// 推荐
enum ASTNode {
    BinaryExpr { ... }
}
```

**原因：**
- 枚举更高效（无动态分发）
- 模式匹配更方便
- 编译器可以检查完整性

### 2. 为什么使用 Option<Box<ASTNode>>？

```rust
IfStmt {
    else_branch: Option<Box<ASTNode>>,
    // ...
}
```

**原因：**
- else 分支是可选的
- Option 明确表达了这一点
- 避免使用特殊值（如空节点）

## 常见问题

### Q1: 为什么 Program 包含 Vec<ASTNode> 而不是 Vec<Statement>？

**答：** 为了简化设计。在 JavaScript 中，顶层可以有表达式语句，所以统一使用 ASTNode。

### Q2: 如何表示运算符优先级？

**答：** 优先级在解析阶段处理，AST 中已经体现了正确的结构。例如 `1 + 2 * 3` 的 AST 是：
```
Add
├── 1
└── Mul
    ├── 2
    └── 3
```

### Q3: 如何添加类型信息？

**答：** 可以在节点中添加类型字段：
```rust
BinaryExpr {
    op: BinOp,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
    span: Span,
    ty: Option<Type>, // 类型推导后填充
}
```

## 下一步

在下一章中，我们将实现递归下降解析器，将 Token 流转换为 AST。

## 练习

1. 添加一个 `UnaryExpr` 节点表示一元运算（如 `-x`）
2. 添加一个 `WhileStmt` 节点表示 while 循环
3. 实现一个函数打印 AST 的树形结构
4. 实现一个函数计算 AST 的深度

## 完整代码

本章的完整代码在 `src/ast.rs` 文件中。
