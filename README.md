# V8-RS JavaScript Engine

A minimal viable V8-like JavaScript engine implemented in Rust, featuring JIT compilation with Ignition-style bytecode interpreter and TurboFan-style optimizing compiler.

## Project Structure

```
v8-rs/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # Binary entry point
│   ├── types.rs        # Core data types (Value, Span)
│   └── error.rs        # Error types (ParseError, RuntimeError, CompileError)
├── Cargo.toml          # Project configuration
└── README.md           # This file
```

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

```bash
cargo test
```

## Dependencies

- **quickcheck** - Property-based testing framework
- **quickcheck_macros** - Macros for quickcheck

## Requirements

This implementation satisfies requirements 10.5 and 10.6:
- Memory management using Rust's ownership system (Rc/RefCell)
- Component coordination structure

## Next Steps

The following components will be implemented:
1. Parser (JavaScript source → AST)
2. Scope management (Variable scoping)
3. Bytecode Generator (AST → Bytecode)
4. Ignition Interpreter (Bytecode execution)
5. Hotspot Profiler (Hot code detection)
6. TurboFan JIT Compiler (Optimization)
7. Deoptimization (Fallback mechanism)
8. Engine integration (Component coordination)
