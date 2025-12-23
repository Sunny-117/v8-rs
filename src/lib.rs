// V8-RS JavaScript Engine
// Core library modules

pub mod types;
pub mod error;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod scope;
pub mod bytecode;
pub mod codegen;
pub mod interpreter;
pub mod profiler;
pub mod engine;

// Re-export commonly used types
pub use types::{Value, Span, FunctionId};
pub use error::{Error, ParseError, RuntimeError, CompileError};
pub use lexer::{Lexer, Token, TokenKind};
pub use ast::{AST, ASTNode, BinOp};
pub use parser::Parser;
pub use scope::{Scope, ScopeType};
pub use bytecode::{Instruction, BytecodeChunk};
pub use codegen::BytecodeGenerator;
pub use interpreter::{Ignition, CallFrame};
pub use profiler::HotspotProfiler;
pub use engine::Engine;
