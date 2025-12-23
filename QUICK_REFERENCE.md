# V8-RS Quick Reference / 快速参考

## 🚀 快速开始

```bash
# 构建
cargo build --release

# REPL 模式
./target/release/v8_rs

# 执行文件
./target/release/v8_rs script.js

# 运行测试
cargo test
```

---

## 📝 支持的语法

### 数字
```javascript
42          // 整数
3.14        // 浮点数
```

### 变量
```javascript
let x = 10;     // 声明
x = 20;         // 赋值（暂不支持）
```

### 运算符
```javascript
10 + 20     // 加法
30 - 10     // 减法
5 * 6       // 乘法
20 / 4      // 除法
(5 + 3) * 2 // 括号
```

### 打印
```javascript
print(42);          // 打印数字
print(x + y);       // 打印表达式
```

---

## 💻 使用模式

### REPL 模式
```bash
$ ./target/release/v8_rs
> let x = 10
10
> x + 20
30
> print(x)
10
> exit
```

### 文件模式
```javascript
// script.js
let x = 10;
let y = 20;
print(x + y);
```

```bash
$ ./target/release/v8_rs script.js
30
```

### 库模式
```rust
use v8_rs::Engine;

let mut engine = Engine::new();
let result = engine.execute("10 + 20").unwrap();
println!("{}", result); // 30
```

---

## ⚠️ 重要提示

### ✅ 支持
- 数字（整数和浮点数）
- 算术运算（+ - * /）
- 变量声明（let）
- print() 函数
- 括号表达式

### ❌ 不支持
- 注释（// 或 /* */）
- 字符串
- 布尔值
- 控制流（if/while/for）
- 函数定义
- 对象和数组
- console.log()（使用 print()）

---

## 🔧 常见问题

### Q: 为什么没有输出？
A: 文件模式下需要使用 `print()` 函数。

```javascript
// ❌ 错误 - 无输出
let x = 42;
x * 2

// ✅ 正确 - 有输出
let x = 42;
print(x * 2);
```

### Q: 如何打印多个值？
A: 多次调用 `print()`。

```javascript
let x = 10;
let y = 20;
print(x);
print(y);
print(x + y);
```

### Q: 支持注释吗？
A: 暂不支持。请删除所有注释。

```javascript
// ❌ 错误 - 会报错
// let x = 10;

// ✅ 正确
let x = 10;
```

### Q: 除以零会怎样？
A: 会报错（与 Node.js 不同）。

```javascript
print(10 / 0);  // Error: Division by zero
```

---

## 📚 文档链接

- [完整 README](./README.md)
- [English README](./README_EN.md)
- [使用指南](./docs/USAGE.md)
- [与 Node.js 的差异](./docs/DIFFERENCES.md)
- [完整教程](./docs/README_CN.md)
- [更新日志](./CHANGELOG.md)
- [项目总结](./PROJECT_SUMMARY.md)

---

## 🎯 示例

### 基本算术
```javascript
let a = 10;
let b = 20;
print(a + b);  // 30
```

### 复杂表达式
```javascript
let x = 5;
let y = 3;
print((x + y) * (x - y));  // 16
```

### 斐波那契
```javascript
let a = 0;
let b = 1;
let c = a + b;
let d = b + c;
let e = c + d;
print(e);  // 3
```

---

## 🐛 错误处理

### 语法错误
```bash
$ echo "let = 10" > error.js
$ ./target/release/v8_rs error.js
Error: Parse error: Expected 'identifier', found 'Assign' at 0:4
```

### 运行时错误
```bash
$ echo "print(10 / 0)" > error.js
$ ./target/release/v8_rs error.js
Error: Runtime error: Division by zero
```

### 文件错误
```bash
$ ./target/release/v8_rs nonexistent.js
Error reading file 'nonexistent.js': No such file or directory
```

---

## 🔄 与 Node.js 对比

| 特性 | V8-RS | Node.js |
|------|-------|---------|
| 打印 | `print(x)` | `console.log(x)` |
| 整数输出 | `42` | `42` |
| 浮点输出 | `3.14` | `3.14` |
| 除以零 | 错误 | `Infinity` |
| 文件模式 | 不自动打印 | 不自动打印 |
| REPL 模式 | 自动打印 | 自动打印 |

---

## 💡 最佳实践

1. **使用 print() 输出**
   ```javascript
   print(result);  // ✅ 正确
   result;         // ❌ 文件模式无输出
   ```

2. **避免除以零**
   ```javascript
   if (y != 0) {  // ❌ 暂不支持 if
       print(x / y);
   }
   ```

3. **删除注释**
   ```javascript
   // 删除所有注释
   let x = 10;
   ```

4. **使用发布版本**
   ```bash
   cargo build --release  # 更快
   ```

---

<div align="center">

**快速参考完成！开始使用 V8-RS 吧！**

[返回主页](./README.md)

</div>
