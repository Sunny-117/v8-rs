# 第 9 章：Engine 引擎总控

## 本章目标

在本章中，我们将实现 Engine 引擎，它负责：
1. 协调所有组件（Parser、Codegen、Interpreter）
2. 提供统一的执行接口
3. 管理全局状态
4. 处理端到端的执行流程

## 什么是 Engine？

Engine 是整个 JavaScript 引擎的入口点和协调者。它将各个独立的组件串联起来，形成完整的执行流程。

**执行流程：**
```
Source Code → Parser → AST → Codegen → Bytecode → Interpreter → Result
```

## 步骤 1：Engine 结构

```rust
pub struct Engine {
    interpreter: Ignition,
    global_scope: Scope,
}
```

**字段说明：**

1. **interpreter**：字节码解释器
   - 执行生成的字节码
   - 管理运行时状态

2. **global_scope**：全局作用域
   - 存储全局变量
   - 在多次执行间保持状态

## 步骤 2：初始化

```rust
impl Engine {
    pub fn new() -> Self {
        Self {
            interpreter: Ignition::new(),
            global_scope: Scope::global(),
        }
    }
}
```

**为什么需要全局作用域？**
- 支持多次执行
- 保持全局变量的状态
- 例如：
  ```rust
  engine.execute("let x = 10;");
  engine.execute("let y = x + 5;"); // y 可以访问 x
  ```

## 步骤 3：执行流程

### 3.1 主执行方法

```rust
pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
    // 1. 解析源代码
    let ast = self.parse(source)?;
    
    // 2. 生成字节码
    let bytecode = self.generate_bytecode(&ast);
    
    // 3. 解释执行
    let result = self.interpret(bytecode)?;
    
    Ok(result)
}
```

**三个阶段：**
1. **Parse**：源代码 → AST
2. **Codegen**：AST → 字节码
3. **Interpret**：字节码 → 结果

### 3.2 解析阶段

```rust
fn parse(&self, source: &str) -> Result<AST, ParseError> {
    let mut parser = Parser::new(source.to_string());
    parser.parse()
}
```

**错误处理：**
- 语法错误会在这里被捕获
- 返回 ParseError 给调用者

### 3.3 字节码生成阶段

```rust
fn generate_bytecode(&mut self, ast: &AST) -> BytecodeChunk {
    let mut generator = BytecodeGenerator::new(self.global_scope.clone());
    generator.generate(&ast.root)
}
```

**为什么克隆 global_scope？**
- BytecodeGenerator 需要修改作用域
- 但我们希望保持 Engine 的作用域不变
- 生成完成后，作用域的修改会被丢弃

**改进方案：**
```rust
fn generate_bytecode(&mut self, ast: &AST) -> BytecodeChunk {
    let mut generator = BytecodeGenerator::new(self.global_scope.clone());
    let chunk = generator.generate(&ast.root);
    
    // 更新全局作用域
    self.global_scope = generator.scope;
    
    chunk
}
```

### 3.4 解释执行阶段

```rust
fn interpret(&mut self, bytecode: BytecodeChunk) -> Result<Value, RuntimeError> {
    self.interpreter.execute(bytecode)
}
```

**状态管理：**
- 解释器维护自己的状态
- 每次执行都是独立的

## 步骤 4：错误处理

### 4.1 错误转换

```rust
pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
    let ast = self.parse(source)?; // ParseError → Error
    let bytecode = self.generate_bytecode(&ast);
    let result = self.interpret(bytecode)?; // RuntimeError → Error
    Ok(result)
}
```

**自动转换：**
- `?` 操作符自动调用 `From` trait
- ParseError 和 RuntimeError 都可以转换为 Error

### 4.2 错误上下文

```rust
pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
    let ast = self.parse(source).map_err(|e| {
        eprintln!("Parse error in source: {}", source);
        e
    })?;
    
    // ...
}
```

## 步骤 5：使用示例

### 5.1 基本使用

```rust
fn main() {
    let mut engine = Engine::new();
    
    match engine.execute("10 + 20") {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### 5.2 多次执行

```rust
fn main() {
    let mut engine = Engine::new();
    
    engine.execute("let x = 10;").unwrap();
    engine.execute("let y = 20;").unwrap();
    
    let result = engine.execute("x + y").unwrap();
    println!("Result: {:?}", result); // Number(30.0)
}
```

### 5.3 错误处理

```rust
fn main() {
    let mut engine = Engine::new();
    
    match engine.execute("10 / 0") {
        Ok(result) => println!("Result: {:?}", result),
        Err(Error::RuntimeError(RuntimeError::DivisionByZero)) => {
            eprintln!("Cannot divide by zero!");
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## 步骤 6：扩展功能

### 6.1 添加内置函数

```rust
impl Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            interpreter: Ignition::new(),
            global_scope: Scope::global(),
        };
        
        // 注册内置函数
        engine.register_builtin("print", 0);
        engine.register_builtin("sqrt", 1);
        
        engine
    }
    
    fn register_builtin(&mut self, name: &str, func_id: FunctionId) {
        self.global_scope.declare(name.to_string());
        // 将函数 ID 存储到某个地方
    }
}
```

### 6.2 添加调试模式

```rust
pub struct Engine {
    interpreter: Ignition,
    global_scope: Scope,
    debug_mode: bool,
}

impl Engine {
    pub fn set_debug(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }
    
    pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
        if self.debug_mode {
            println!("Executing: {}", source);
        }
        
        let ast = self.parse(source)?;
        
        if self.debug_mode {
            println!("AST: {:?}", ast);
        }
        
        let bytecode = self.generate_bytecode(&ast);
        
        if self.debug_mode {
            println!("Bytecode: {:?}", bytecode);
        }
        
        let result = self.interpret(bytecode)?;
        
        if self.debug_mode {
            println!("Result: {:?}", result);
        }
        
        Ok(result)
    }
}
```

### 6.3 添加性能统计

```rust
pub struct Engine {
    interpreter: Ignition,
    global_scope: Scope,
    stats: ExecutionStats,
}

struct ExecutionStats {
    parse_time: Duration,
    codegen_time: Duration,
    interpret_time: Duration,
}

impl Engine {
    pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
        let start = Instant::now();
        let ast = self.parse(source)?;
        self.stats.parse_time += start.elapsed();
        
        let start = Instant::now();
        let bytecode = self.generate_bytecode(&ast);
        self.stats.codegen_time += start.elapsed();
        
        let start = Instant::now();
        let result = self.interpret(bytecode)?;
        self.stats.interpret_time += start.elapsed();
        
        Ok(result)
    }
    
    pub fn print_stats(&self) {
        println!("Parse time: {:?}", self.stats.parse_time);
        println!("Codegen time: {:?}", self.stats.codegen_time);
        println!("Interpret time: {:?}", self.stats.interpret_time);
    }
}
```

## 测试 Engine

### 测试基本执行

```rust
#[test]
fn test_execute_number() {
    let mut engine = Engine::new();
    let result = engine.execute("42").unwrap();
    assert_eq!(result, Value::Number(42.0));
}
```

### 测试表达式

```rust
#[test]
fn test_execute_addition() {
    let mut engine = Engine::new();
    let result = engine.execute("10 + 20").unwrap();
    assert_eq!(result, Value::Number(30.0));
}
```

### 测试复杂表达式

```rust
#[test]
fn test_execute_complex_expression() {
    let mut engine = Engine::new();
    let result = engine.execute("(5 + 3) * 2").unwrap();
    assert_eq!(result, Value::Number(16.0));
}
```

### 测试错误处理

```rust
#[test]
fn test_execute_parse_error() {
    let mut engine = Engine::new();
    let result = engine.execute("let = 10");
    assert!(result.is_err());
}

#[test]
fn test_execute_division_by_zero() {
    let mut engine = Engine::new();
    let result = engine.execute("10 / 0");
    assert!(result.is_err());
}
```

## 架构模式

### 1. 管道模式（Pipeline）

```
Input → Stage1 → Stage2 → Stage3 → Output
```

每个阶段独立处理，输出作为下一阶段的输入。

### 2. 外观模式（Facade）

Engine 隐藏了内部复杂性，提供简单的接口：

```rust
// 用户只需要调用一个方法
engine.execute(source)

// 而不是
let ast = parser.parse(source);
let bytecode = codegen.generate(ast);
let result = interpreter.execute(bytecode);
```

### 3. 单例模式（可选）

```rust
lazy_static! {
    static ref GLOBAL_ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
}

pub fn execute(source: &str) -> Result<Value, Error> {
    GLOBAL_ENGINE.lock().unwrap().execute(source)
}
```

## 性能考虑

### 1. 避免不必要的克隆

```rust
// 不好：每次都克隆
fn generate_bytecode(&mut self, ast: &AST) -> BytecodeChunk {
    let generator = BytecodeGenerator::new(self.global_scope.clone());
    // ...
}

// 更好：使用引用
fn generate_bytecode(&mut self, ast: &AST) -> BytecodeChunk {
    let generator = BytecodeGenerator::new(&mut self.global_scope);
    // ...
}
```

### 2. 缓存解析结果

```rust
pub struct Engine {
    // ...
    ast_cache: HashMap<String, AST>,
}

impl Engine {
    pub fn execute(&mut self, source: &str) -> Result<Value, Error> {
        let ast = if let Some(cached) = self.ast_cache.get(source) {
            cached.clone()
        } else {
            let ast = self.parse(source)?;
            self.ast_cache.insert(source.to_string(), ast.clone());
            ast
        };
        
        // ...
    }
}
```

### 3. 字节码缓存

```rust
pub struct Engine {
    // ...
    bytecode_cache: HashMap<String, BytecodeChunk>,
}
```

## 常见问题

### Q1: 如何支持模块系统？

**答：**
```rust
pub struct Engine {
    // ...
    modules: HashMap<String, Module>,
}

impl Engine {
    pub fn load_module(&mut self, name: &str, source: &str) -> Result<(), Error> {
        let ast = self.parse(source)?;
        let bytecode = self.generate_bytecode(&ast);
        
        self.modules.insert(name.to_string(), Module {
            bytecode,
            exports: HashMap::new(),
        });
        
        Ok(())
    }
    
    pub fn import(&mut self, module_name: &str) -> Result<Value, Error> {
        // 加载并执行模块
        // 返回模块的导出
    }
}
```

### Q2: 如何实现 REPL？

**答：**
```rust
fn repl() {
    let mut engine = Engine::new();
    let stdin = io::stdin();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        match engine.execute(&input) {
            Ok(result) => println!("{:?}", result),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
```

### Q3: 如何添加异步支持？

**答：**
```rust
pub async fn execute_async(&mut self, source: &str) -> Result<Value, Error> {
    // 在单独的线程中执行
    let ast = self.parse(source)?;
    let bytecode = self.generate_bytecode(&ast);
    
    tokio::task::spawn_blocking(move || {
        let mut interpreter = Ignition::new();
        interpreter.execute(bytecode)
    }).await.unwrap()
}
```

## 下一步

在下一章中，我们将编写端到端测试，验证整个引擎的功能。

## 练习

1. 添加对多个文件的支持
2. 实现简单的模块系统
3. 添加性能分析工具
4. 实现 REPL 交互式环境

## 完整代码

本章的完整代码在 `src/engine.rs` 文件中。
