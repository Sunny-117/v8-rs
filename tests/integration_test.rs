// Integration tests for V8-RS engine

use v8_rs::{Engine, Value};

#[test]
fn test_simple_number() {
    let mut engine = Engine::new();
    let result = engine.execute("42").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_addition() {
    let mut engine = Engine::new();
    let result = engine.execute("10 + 20").unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_subtraction() {
    let mut engine = Engine::new();
    let result = engine.execute("50 - 15").unwrap();
    assert_eq!(result, Value::Number(35.0));
}

#[test]
fn test_multiplication() {
    let mut engine = Engine::new();
    let result = engine.execute("6 * 7").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_division() {
    let mut engine = Engine::new();
    let result = engine.execute("100 / 4").unwrap();
    assert_eq!(result, Value::Number(25.0));
}

#[test]
fn test_complex_expression() {
    let mut engine = Engine::new();
    let result = engine.execute("(5 + 3) * 2").unwrap();
    assert_eq!(result, Value::Number(16.0));
}

#[test]
fn test_nested_expression() {
    let mut engine = Engine::new();
    let result = engine.execute("((10 + 5) * 2) - 10").unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_let_declaration() {
    let mut engine = Engine::new();
    let result = engine.execute("let x = 42;");
    assert!(result.is_ok());
}

#[test]
fn test_division_by_zero() {
    let mut engine = Engine::new();
    let result = engine.execute("10 / 0");
    assert!(result.is_err());
}

#[test]
fn test_parse_error() {
    let mut engine = Engine::new();
    let result = engine.execute("let = 10");
    assert!(result.is_err());
}

#[test]
fn test_multiple_operations() {
    let mut engine = Engine::new();
    
    let result1 = engine.execute("10 + 5").unwrap();
    assert_eq!(result1, Value::Number(15.0));
    
    let result2 = engine.execute("20 * 2").unwrap();
    assert_eq!(result2, Value::Number(40.0));
    
    let result3 = engine.execute("100 - 50").unwrap();
    assert_eq!(result3, Value::Number(50.0));
}

#[test]
fn test_floating_point() {
    let mut engine = Engine::new();
    let result = engine.execute("3.14 * 2").unwrap();
    assert_eq!(result, Value::Number(6.28));
}

#[test]
fn test_operator_precedence() {
    let mut engine = Engine::new();
    let result = engine.execute("2 + 3 * 4").unwrap();
    assert_eq!(result, Value::Number(14.0));
}

#[test]
fn test_parentheses_override_precedence() {
    let mut engine = Engine::new();
    let result = engine.execute("(2 + 3) * 4").unwrap();
    assert_eq!(result, Value::Number(20.0));
}
