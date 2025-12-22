// Error types for V8-RS

use crate::types::Span;
use std::fmt;

/// Top-level error type for the engine
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Parse error during source code parsing
    ParseError(ParseError),
    /// Runtime error during execution
    RuntimeError(RuntimeError),
    /// Compilation error during JIT compilation
    CompileError(CompileError),
}

/// Errors that occur during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected token encountered
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    /// Unexpected end of file
    UnexpectedEOF,
    /// Invalid syntax
    InvalidSyntax {
        message: String,
        span: Span,
    },
}

/// Errors that occur during runtime execution
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    /// Variable not defined
    UndefinedVariable {
        name: String,
    },
    /// Type mismatch
    TypeError {
        expected: String,
        found: String,
    },
    /// Stack overflow
    StackOverflow,
    /// Division by zero
    DivisionByZero,
}

/// Errors that occur during JIT compilation
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Unsupported feature
    UnsupportedFeature {
        feature: String,
    },
    /// Optimization failed
    OptimizationFailed {
        reason: String,
    },
}

// Display implementations for better error messages

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "Parse error: {}", e),
            Error::RuntimeError(e) => write!(f, "Runtime error: {}", e),
            Error::CompileError(e) => write!(f, "Compile error: {}", e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, span } => {
                write!(f, "Expected '{}', found '{}' at {}:{}", expected, found, span.start, span.end)
            }
            ParseError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
            ParseError::InvalidSyntax { message, span } => {
                write!(f, "{} at {}:{}", message, span.start, span.end)
            }
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable { name } => {
                write!(f, "Undefined variable: {}", name)
            }
            RuntimeError::TypeError { expected, found } => {
                write!(f, "Type error: expected {}, found {}", expected, found)
            }
            RuntimeError::StackOverflow => {
                write!(f, "Stack overflow")
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::UnsupportedFeature { feature } => {
                write!(f, "Unsupported feature: {}", feature)
            }
            CompileError::OptimizationFailed { reason } => {
                write!(f, "Optimization failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for Error {}
impl std::error::Error for ParseError {}
impl std::error::Error for RuntimeError {}
impl std::error::Error for CompileError {}

// Conversion implementations for ergonomic error handling

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

impl From<RuntimeError> for Error {
    fn from(err: RuntimeError) -> Self {
        Error::RuntimeError(err)
    }
}

impl From<CompileError> for Error {
    fn from(err: CompileError) -> Self {
        Error::CompileError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_creation() {
        let err = ParseError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: "number".to_string(),
            span: Span::new(0, 5),
        };
        assert!(matches!(err, ParseError::UnexpectedToken { .. }));
    }

    #[test]
    fn test_runtime_error_creation() {
        let err = RuntimeError::UndefinedVariable {
            name: "x".to_string(),
        };
        assert!(matches!(err, RuntimeError::UndefinedVariable { .. }));
    }

    #[test]
    fn test_error_conversion() {
        let parse_err = ParseError::UnexpectedEOF;
        let err: Error = parse_err.into();
        assert!(matches!(err, Error::ParseError(_)));
    }

    #[test]
    fn test_error_display() {
        let err = Error::RuntimeError(RuntimeError::DivisionByZero);
        let display = format!("{}", err);
        assert!(display.contains("Division by zero"));
    }
}
