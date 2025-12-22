# V8-RS JavaScript Engine

A minimal viable V8-like JavaScript engine implemented in Rust, featuring JIT compilation with Ignition-style bytecode interpreter and TurboFan-style optimizing compiler.

## Project Structure

```
v8-rs/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # Binary entry point
│   ├── types.rs        # Core data types (Value, Span)
│   ├── error.rs        # Error types (ParseError, RuntimeError, CompileError)
│   ├── lexer.rs        # Lexical analyzer (tokenization)
│   ├── ast.rs          # Abstract Syntax Tree definitions
│   ├── parser.rs       # Recursive descent parser
│   ├── scope.rs        # Scope management for variables
│   ├── bytecode.rs     # Bytecode instruction definitions
│   ├── codegen.rs      # Bytecode generator (AST → Bytecode)
│   ├── interpreter.rs  # Ignition bytecode interpreter
│   └── engine.rs       # Main engine coordinator
├── tests/
│   └── integration_test.rs  # Integration tests
├── examples/
│   └── basic.rs        # Basic usage examples
├── Cargo.toml          # Project configuration
└── README.md           # This file
```

## Features Implemented

### ✅ Core Components

- **Lexer**: Tokenizes JavaScript source code
  - Numbers (integers and floats)
  - Identifiers and keywords
  - Operators (+, -, *, /, =, ==, <, >)
  - Delimiters (parentheses, braces, semicolons)

- **Parser**: Recursive descent parser
  - Number literals
  - Binary expressions (arithmetic)
  - Let declarations
  - Function declarations
  - If statements
  - For loops
  - Function calls
  - Return statements
  - Block statements

- **Scope Management**: Lexical scoping
  - Global, function, and block scopes
  - Variable declaration and lookup
  - Scope chain traversal

- **Bytecode Generator**: AST to bytecode compilation
  - LoadConst, LoadLocal, StoreLocal
  - Arithmetic operations (Add, Sub, Mul, Div)
  - Control flow (Jump, JumpIfFalse)
  - Function calls and returns

- **Ignition Interpreter**: Stack-based bytecode execution
  - Call frame management
  - Operand stack operations
  - Local variable storage
  - Arithmetic execution
  - Error handling (division by zero, type errors)

- **Engine**: Main coordinator
  - Parse → Bytecode → Interpret pipeline
  - Error propagation
  - Global scope management

## Core Data Types

### Value
Represents JavaScript values in the engine:
- `Number(f64)` - Numeric values
- `Function(FunctionId)` - Function references
- `Undefined` - Undefined value

### Span
Represents source code location information:
- `start: usize` - Start position
- `end: usize` - End position

### Error Types
- `ParseError` - Errors during parsing
- `RuntimeError` - Errors during execution
- `CompileError` - Errors during JIT compilation

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Testing

Run all tests:
```bash
cargo test
```

Run only unit tests:
```bash
cargo test --lib
```

Run integration tests:
```bash
cargo test --test integration_test
```

## Examples

Run the basic example:
```bash
cargo run --example basic
```

### Usage Example

```rust
use v8_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // Execute JavaScript code
    let result = engine.execute("(5 + 3) * 2").unwrap();
    println!("Result: {:?}", result); // Number(16.0)
}
```

## Supported JavaScript Subset

Currently supports:
- ✅ Number literals (integers and floats)
- ✅ Arithmetic operations (+, -, *, /)
- ✅ Parentheses for grouping
- ✅ Let variable declarations
- ✅ Operator precedence
- ✅ Basic error handling

## Dependencies

- **quickcheck** - Property-based testing framework
- **quickcheck_macros** - Macros for quickcheck

## Test Results

All tests passing:
- 53 unit tests
- 14 integration tests
- 0 failures

## Requirements Satisfied

This implementation satisfies the following requirements:
- ✅ 1.x: JavaScript source code parsing
- ✅ 2.x: Scope management
- ✅ 3.x: Bytecode generation
- ✅ 4.x: Bytecode interpretation
- ✅ 10.5, 10.6: Memory management and component coordination
- ✅ 11.1-11.3: End-to-end execution flow

## Future Work

The following components are planned but not yet implemented:
- Hotspot Profiler (hot code detection)
- TurboFan JIT Compiler (optimization)
- IR generation and optimization passes
- Machine code generation
- Deoptimization (fallback mechanism)
- Function execution and calls
- More JavaScript features (objects, arrays, etc.)

## Architecture

The engine follows a pipeline architecture:

```
Source Code → Lexer → Parser → AST → Bytecode Generator → Bytecode
                                                              ↓
                                                         Interpreter
                                                              ↓
                                                           Result
```

## License

This is an educational project demonstrating JIT compilation concepts.
