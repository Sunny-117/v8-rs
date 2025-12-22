// V8-RS JavaScript Engine
// Core library modules

pub mod types;
pub mod error;

// Re-export commonly used types
pub use types::{Value, Span};
pub use error::{Error, ParseError, RuntimeError, CompileError};
