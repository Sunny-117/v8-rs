# V8-RS vs Node.js Differences
# V8-RS 与 Node.js 的差异

This document explains the key differences between V8-RS and Node.js/V8.

本文档说明 V8-RS 与 Node.js/V8 之间的主要差异。

---

## Output Behavior / 输出行为

### REPL Mode / REPL 模式

**Both V8-RS and Node.js automatically print expression results in REPL mode.**

**V8-RS 和 Node.js 在 REPL 模式下都会自动打印表达式结果。**

#### V8-RS
```bash
$ ./target/release/v8_rs
> 42
42
> let x = 10
10
> x + 20
30
```

#### Node.js
```bash
$ node
> 42
42
> let x = 10
undefined
> x + 20
30
```

✅ **Similar behavior / 行为相似**

---

### File Mode / 文件模式

**In file mode, expressions are NOT automatically printed.**

**在文件模式下，表达式不会自动打印。**

#### Example File / 示例文件
```javascript
// test.js
let x = 10;
x * 2
```

#### V8-RS
```bash
$ ./target/release/v8_rs test.js
(no output / 无输出)
```

#### Node.js
```bash
$ node test.js
(no output / 无输出)
```

✅ **Identical behavior / 行为一致**

---

## Print Function / 打印函数

### V8-RS: `print()` function

V8-RS provides a built-in `print()` function for output.

V8-RS 提供内置的 `print()` 函数用于输出。

```javascript
let x = 10;
print(x);        // Prints: 10
print(x * 2);    // Prints: 20
```

### Node.js: `console.log()` function

Node.js uses `console.log()` for output.

Node.js 使用 `console.log()` 进行输出。

```javascript
let x = 10;
console.log(x);        // Prints: 10
console.log(x * 2);    // Prints: 20
```

⚠️ **Different API / API 不同**

**Why `print()` instead of `console.log()`?**

**为什么使用 `print()` 而不是 `console.log()`？**

- Simpler to implement (no object system needed)
- 更简单的实现（不需要对象系统）
- Common in educational languages (Python, Ruby, etc.)
- 在教学语言中很常见（Python、Ruby 等）
- Focuses on core engine concepts
- 专注于核心引擎概念

---

## Supported Features / 支持的特性

### V8-RS (Current Version)

✅ Supported / 支持：
- Numbers / 数字
- Arithmetic operations / 算术运算
- Variables (`let`) / 变量声明
- Expressions / 表达式
- `print()` function / 打印函数

❌ Not Yet Supported / 尚不支持：
- Strings / 字符串
- Booleans / 布尔值
- Objects / 对象
- Arrays / 数组
- Functions (user-defined) / 函数（用户定义）
- Control flow (if/while/for) / 控制流
- `console.log()` / console.log()
- Comments / 注释

### Node.js/V8

✅ Full JavaScript ES2024 support
✅ 完整的 JavaScript ES2024 支持

---

## Error Handling / 错误处理

### Division by Zero / 除以零

#### V8-RS
```bash
$ echo "print(10 / 0)" > test.js
$ ./target/release/v8_rs test.js
Error: Runtime error: Division by zero
```

#### Node.js
```bash
$ echo "console.log(10 / 0)" > test.js
$ node test.js
Infinity
```

⚠️ **Different behavior / 行为不同**

V8-RS treats division by zero as an error for educational purposes.

V8-RS 出于教学目的将除以零视为错误。

---

## Migration Guide / 迁移指南

### From Node.js to V8-RS / 从 Node.js 迁移到 V8-RS

Replace `console.log()` with `print()`:

将 `console.log()` 替换为 `print()`：

**Node.js:**
```javascript
let x = 10;
let y = 20;
console.log(x + y);
```

**V8-RS:**
```javascript
let x = 10;
let y = 20;
print(x + y);
```

### From V8-RS to Node.js / 从 V8-RS 迁移到 Node.js

Replace `print()` with `console.log()`:

将 `print()` 替换为 `console.log()`：

**V8-RS:**
```javascript
let result = 42;
print(result);
```

**Node.js:**
```javascript
let result = 42;
console.log(result);
```

---

## Summary / 总结

V8-RS is designed as an educational JavaScript engine that:

V8-RS 是一个教学用 JavaScript 引擎，它：

- ✅ Follows JavaScript semantics for core features
- ✅ 遵循核心特性的 JavaScript 语义
- ✅ Uses simplified APIs (`print()` vs `console.log()`)
- ✅ 使用简化的 API（`print()` 而非 `console.log()`）
- ✅ Provides clear error messages
- ✅ 提供清晰的错误消息
- ✅ Focuses on understanding engine internals
- ✅ 专注于理解引擎内部机制

It's perfect for learning how JavaScript engines work, but not a replacement for production engines like V8 or SpiderMonkey.

它非常适合学习 JavaScript 引擎的工作原理，但不能替代 V8 或 SpiderMonkey 等生产引擎。
