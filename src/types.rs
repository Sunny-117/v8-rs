// Core data types for V8-RS

use std::fmt;

/// Represents a JavaScript value in the engine
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Numeric value (f64)
    Number(f64),
    /// Function reference by ID
    Function(FunctionId),
    /// Undefined value
    Undefined,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // Format numbers like Node.js/V8
                if n.fract() == 0.0 && n.is_finite() {
                    // Integer-like numbers: print without decimal
                    write!(f, "{}", *n as i64)
                } else {
                    // Floating point numbers: print with decimals
                    write!(f, "{}", n)
                }
            }
            Value::Function(id) => write!(f, "[Function: {}]", id),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

/// Function identifier type
pub type FunctionId = usize;

/// Represents a source code location span
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start position in source code
    pub start: usize,
    /// End position in source code
    pub end: usize,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Create a span covering a single position
    pub fn single(pos: usize) -> Self {
        Self { start: pos, end: pos }
    }

    /// Merge two spans into one covering both
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Undefined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        let num = Value::Number(42.0);
        assert_eq!(num, Value::Number(42.0));

        let func = Value::Function(0);
        assert_eq!(func, Value::Function(0));

        let undef = Value::Undefined;
        assert_eq!(undef, Value::Undefined);
    }

    #[test]
    fn test_span_creation() {
        let span = Span::new(0, 10);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 10);
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 10);
        let merged = span1.merge(span2);
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 10);
    }
}
