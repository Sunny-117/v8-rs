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
pub mod ir;
pub mod turbofan;
pub mod codegen_backend;
pub mod deopt;
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
pub use ir::{IR, IRNode, NodeId, Type as IRType};
pub use turbofan::TurboFan;
pub use codegen_backend::{CodeGenerator, CodegenBackend, CompiledFunction};
pub use deopt::{DeoptInfo, DeoptManager, DeoptReason, DeoptState};
pub use engine::Engine;
