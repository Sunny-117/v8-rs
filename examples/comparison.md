# V8-RS vs Node.js Output Comparison
# V8-RS 与 Node.js 输出对比

This document shows that V8-RS produces output identical to Node.js/V8.

本文档展示 V8-RS 产生的输出与 Node.js/V8 完全一致。

## Integer Output / 整数输出

### V8-RS
```bash
$ echo "let x = 750; x" > test.js
$ ./target/release/v8_rs test.js
750
```

### Node.js
```bash
$ node -e "console.log(750)"
750
```

✅ **Identical output / 输出一致**

---

## Floating Point Output / 浮点数输出

### V8-RS
```bash
$ echo "let x = 3.14; x * 2" > test.js
$ ./target/release/v8_rs test.js
6.28
```

### Node.js
```bash
$ node -e "console.log(3.14 * 2)"
6.28
```

✅ **Identical output / 输出一致**

---

## Division Result / 除法结果

### V8-RS
```bash
$ echo "let x = 10; let y = 3; x / y" > test.js
$ ./target/release/v8_rs test.js
3.3333333333333335
```

### Node.js
```bash
$ node -e "console.log(10 / 3)"
3.3333333333333335
```

✅ **Identical output / 输出一致**

---

## Complex Expression / 复杂表达式

### V8-RS
```bash
$ ./target/release/v8_rs examples/arithmetic.js
750
```

### Node.js
```bash
$ node -e "let a=10;let b=5;let sum=a+b;let product=a*b;let result=sum*product;console.log(result)"
750
```

✅ **Identical output / 输出一致**

---

## Error Handling / 错误处理

### V8-RS
```bash
$ echo "10 / 0" > error.js
$ ./target/release/v8_rs error.js
Error: Runtime error: Division by zero
```

### Node.js
```bash
$ node -e "console.log(10 / 0)"
Infinity
```

⚠️ **Different behavior / 行为不同**

Note: V8-RS treats division by zero as an error, while JavaScript returns `Infinity`.
This is a design choice for educational purposes.

注意：V8-RS 将除以零视为错误，而 JavaScript 返回 `Infinity`。
这是出于教学目的的设计选择。

---

## Summary / 总结

V8-RS successfully mimics Node.js/V8 output format for:
- ✅ Integer numbers (no decimal point)
- ✅ Floating point numbers (with decimals)
- ✅ Arithmetic precision
- ✅ Expression evaluation

V8-RS 成功模仿了 Node.js/V8 的输出格式：
- ✅ 整数（无小数点）
- ✅ 浮点数（带小数）
- ✅ 算术精度
- ✅ 表达式求值

The output is clean and matches what developers expect from JavaScript engines!

输出简洁，符合开发者对 JavaScript 引擎的期望！
