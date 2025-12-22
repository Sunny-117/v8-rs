# 需求文档：V8-RS JavaScript 引擎

## 简介

本文档定义了一个基于 Rust 实现的最小可行版本 V8-like JavaScript 引擎的需求。该引擎采用 JIT（Just-In-Time）技术，结合解释执行和编译执行的优势，包含 Ignition 风格的字节码解释器和 TurboFan 风格的优化编译器。

## 术语表

- **Engine（引擎）**: 协调所有组件的主控制器
- **Parser（解析器）**: 将 JavaScript 源代码转换为抽象语法树的组件
- **AST（抽象语法树）**: 源代码的结构化表示
- **Bytecode（字节码）**: 介于 AST 和机器码之间的中间表示
- **Ignition（解释器）**: 执行字节码的栈式解释器
- **Hotspot_Profiler（热点分析器）**: 检测频繁执行代码的监控组件
- **TurboFan（优化编译器）**: 将热点代码编译为优化机器码的 JIT 编译器
- **Deoptimization（反优化）**: 当优化假设失效时回退到解释执行的机制
- **IR（中间表示）**: 优化编译器使用的内部代码表示
- **Execution_Counter（执行计数器）**: 跟踪函数执行次数的计数器
- **Call_Frame（调用帧）**: 存储函数调用上下文的栈帧结构

## 需求

### 需求 1：JavaScript 源码解析

**用户故事：** 作为引擎开发者，我希望将 JavaScript 源代码解析为结构化的 AST，以便后续处理和编译。

#### 验收标准

1. WHEN Parser 接收到有效的 JavaScript 源代码 THEN Parser SHALL 生成对应的 AST
2. THE Parser SHALL 支持数字字面量（i32 和 f64 类型）
3. THE Parser SHALL 支持 let 变量声明语句
4. THE Parser SHALL 支持函数声明语句
5. THE Parser SHALL 支持二元运算表达式（加、减、乘、除）
6. THE Parser SHALL 支持 if 条件语句
7. THE Parser SHALL 支持 for 循环语句
8. THE Parser SHALL 支持函数调用表达式
9. THE Parser SHALL 支持 return 语句
10. WHEN Parser 接收到无效的 JavaScript 源代码 THEN Parser SHALL 返回描述性错误信息
11. THE Parser SHALL 为每个 AST 节点记录源代码位置信息
12. THE Parser SHALL 在解析过程中生成作用域信息

### 需求 2：作用域管理

**用户故事：** 作为引擎开发者，我希望正确管理变量作用域，以便准确解析变量引用和生成字节码。

#### 验收标准

1. THE Engine SHALL 为每个作用域维护变量索引映射
2. WHEN 解析函数声明 THEN Engine SHALL 创建新的函数作用域
3. WHEN 解析块语句 THEN Engine SHALL 创建新的块作用域
4. THE Engine SHALL 支持词法作用域的变量查找
5. WHEN 变量在当前作用域未找到 THEN Engine SHALL 在父作用域中查找

### 需求 3：字节码生成

**用户故事：** 作为引擎开发者，我希望将 AST 转换为字节码，以便解释器执行。

#### 验收标准

1. WHEN Bytecode_Generator 接收到 AST THEN Bytecode_Generator SHALL 生成字节码指令序列
2. THE Bytecode_Generator SHALL 支持 LoadConst 指令（加载常量）
3. THE Bytecode_Generator SHALL 支持 LoadLocal 指令（加载局部变量）
4. THE Bytecode_Generator SHALL 支持 StoreLocal 指令（存储局部变量）
5. THE Bytecode_Generator SHALL 支持 Add、Sub、Mul、Div 算术指令
6. THE Bytecode_Generator SHALL 支持 Call 指令（函数调用）
7. THE Bytecode_Generator SHALL 支持 Return 指令（函数返回）
8. THE Bytecode_Generator SHALL 支持 Jump 和 JumpIfFalse 指令（控制流）
9. THE Bytecode_Generator SHALL 为每个函数生成独立的字节码块
10. THE Bytecode_Generator SHALL 为每个字节码块维护常量池
11. THE Bytecode_Generator SHALL 记录每个函数的局部变量数量

### 需求 4：字节码解释执行

**用户故事：** 作为引擎开发者，我希望解释器能够执行字节码并产生正确结果，以便快速启动程序执行。

#### 验收标准

1. THE Ignition SHALL 使用栈式架构执行字节码
2. THE Ignition SHALL 为每个函数调用维护独立的调用帧
3. THE Ignition SHALL 为每个调用帧维护独立的操作数栈
4. WHEN Ignition 执行字节码指令 THEN Ignition SHALL 按顺序处理指令
5. WHEN Ignition 执行函数 THEN Ignition SHALL 递增该函数的执行计数器
6. THE Ignition SHALL 在执行过程中输出计算结果
7. WHEN 执行遇到错误 THEN Ignition SHALL 返回描述性错误信息

### 需求 5：热点代码检测

**用户故事：** 作为引擎开发者，我希望识别频繁执行的代码，以便对其进行优化编译。

#### 验收标准

1. THE Hotspot_Profiler SHALL 为每个函数维护执行计数器
2. WHEN 函数执行次数超过阈值 THEN Hotspot_Profiler SHALL 将该函数标记为热点代码
3. WHEN 函数被标记为热点代码 THEN Hotspot_Profiler SHALL 触发 TurboFan 编译
4. THE Hotspot_Profiler SHALL 支持配置热点检测阈值
5. THE Hotspot_Profiler SHALL 记录每个函数的热点状态

### 需求 6：中间表示生成

**用户故事：** 作为引擎开发者，我希望将字节码转换为优化友好的中间表示，以便执行优化变换。

#### 验收标准

1. WHEN TurboFan 接收到字节码 THEN TurboFan SHALL 生成 SSA 形式的 IR
2. THE TurboFan SHALL 支持 Constant IR 节点（常量）
3. THE TurboFan SHALL 支持 Add、Sub、Mul、Div IR 节点（算术运算）
4. THE TurboFan SHALL 支持 LoadLocal 和 StoreLocal IR 节点（变量访问）
5. THE TurboFan SHALL 支持 Return IR 节点（函数返回）
6. THE TurboFan SHALL 支持 Call IR 节点（函数调用）
7. THE TurboFan SHALL 为 IR 节点附加类型反馈信息

### 需求 7：代码优化

**用户故事：** 作为引擎开发者，我希望对中间表示执行优化，以便生成更高效的机器码。

#### 验收标准

1. THE TurboFan SHALL 执行常量折叠优化
2. THE TurboFan SHALL 执行小函数内联优化
3. THE TurboFan SHALL 执行冗余加载消除优化
4. THE TurboFan SHALL 执行数字类型特化优化
5. THE TurboFan SHALL 在优化代码中插入类型假设保护
6. WHEN 优化基于类型假设 THEN TurboFan SHALL 插入运行时类型检查

### 需求 8：机器码生成

**用户故事：** 作为引擎开发者，我希望将优化后的 IR 编译为可执行的机器码，以便提高执行性能。

#### 验收标准

1. WHEN TurboFan 完成优化 THEN TurboFan SHALL 生成本地机器码
2. THE TurboFan SHALL 支持算术运算的机器码生成
3. THE TurboFan SHALL 支持函数调用的机器码生成
4. THE TurboFan SHALL 存储生成的函数入口指针
5. WHEN 机器码生成完成 THEN Engine SHALL 使用机器码替代解释执行
6. THE TurboFan SHALL 使用 Cranelift 或 Dynasm 作为代码生成后端

### 需求 9：反优化机制

**用户故事：** 作为引擎开发者，我希望在优化假设失效时能够安全回退到解释执行，以便保证程序正确性。

#### 验收标准

1. WHEN 优化代码中的类型保护失败 THEN Engine SHALL 触发反优化
2. WHEN 触发反优化 THEN Engine SHALL 捕获当前的活跃值
3. WHEN 触发反优化 THEN Engine SHALL 重建解释器调用帧
4. WHEN 触发反优化 THEN Engine SHALL 在 Ignition 中恢复执行
5. THE Engine SHALL 确保反优化后程序状态的正确性
6. WHEN 反优化完成 THEN Engine SHALL 继续使用解释执行该函数

### 需求 10：引擎初始化

**用户故事：** 作为引擎开发者，我希望引擎能够正确初始化运行环境，以便执行 JavaScript 代码。

#### 验收标准

1. WHEN Engine 启动 THEN Engine SHALL 初始化堆内存空间
2. WHEN Engine 启动 THEN Engine SHALL 初始化栈内存空间
3. WHEN Engine 启动 THEN Engine SHALL 创建全局执行上下文
4. WHEN Engine 启动 THEN Engine SHALL 创建全局作用域
5. THE Engine SHALL 使用引用计数（Rc）或内存池（Arena）管理内存
6. THE Engine SHALL 协调 Parser、Ignition、Hotspot_Profiler 和 TurboFan 组件

### 需求 11：端到端执行流程

**用户故事：** 作为引擎用户，我希望能够提交 JavaScript 代码并获得执行结果，以便运行我的程序。

#### 验收标准

1. WHEN 用户提交 JavaScript 源代码 THEN Engine SHALL 解析源代码生成 AST
2. WHEN AST 生成完成 THEN Engine SHALL 生成字节码
3. WHEN 字节码生成完成 THEN Engine SHALL 使用 Ignition 解释执行
4. WHILE 解释执行 THEN Hotspot_Profiler SHALL 监控执行频率
5. WHEN 检测到热点代码 THEN TurboFan SHALL 编译为优化机器码
6. WHEN 优化机器码可用 THEN Engine SHALL 使用机器码执行
7. IF 优化假设失效 THEN Engine SHALL 执行反优化并回退到解释执行
8. WHEN 执行完成 THEN Engine SHALL 返回执行结果

## 约束条件

1. 本项目专注于架构清晰性而非功能完整性
2. 仅支持 JavaScript 子集：数字、函数、循环
3. 不实现垃圾回收器（使用 Rc/RefCell/Arena）
4. 不支持完整的 JavaScript 规范
5. 简化对象模型（不实现完整的 Hidden Class）
6. 代码必须模块化且可测试
7. 使用 Rust 语言实现所有组件
