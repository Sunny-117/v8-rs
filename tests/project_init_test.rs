// Project initialization tests for V8-RS engine
// Tests project structure correctness and dependency loading

use v8_rs::{
    Engine, Value, Error, ParseError, RuntimeError, CompileError,
    Lexer, ASTNode, BinOp, Parser,
    Scope, ScopeType, Instruction, BytecodeChunk,
    BytecodeGenerator, Ignition, Span,
};

/// Test that all core modules are accessible
#[test]
fn test_module_structure() {
    // This test verifies that all expected modules are properly exported
    // and can be imported without errors
    
    // Test types module
    let _value = Value::Number(42.0);
    let _span = Span::new(0, 10);
    
    // Test error module
    let _parse_err = ParseError::UnexpectedEOF;
    let _runtime_err = RuntimeError::DivisionByZero;
    let _compile_err = CompileError::UnsupportedFeature {
        feature: "test".to_string(),
    };
    
    // Test that Error enum can hold all error types
    let _err: Error = ParseError::UnexpectedEOF.into();
    
    assert!(true); // All modules accessible
}

/// Test that the Engine can be instantiated
#[test]
fn test_engine_instantiation() {
    let engine = Engine::new();
    // Engine should be created without panicking
    drop(engine);
    assert!(true);
}

/// Test that the Engine default implementation works
#[test]
fn test_engine_default() {
    let engine = Engine::default();
    drop(engine);
    assert!(true);
}

/// Test that Parser can be instantiated
#[test]
fn test_parser_instantiation() {
    let parser = Parser::new("42".to_string());
    drop(parser);
    assert!(true);
}

/// Test that Lexer can be instantiated
#[test]
fn test_lexer_instantiation() {
    let lexer = Lexer::new("42".to_string());
    drop(lexer);
    assert!(true);
}

/// Test that Scope can be created
#[test]
fn test_scope_creation() {
    let global_scope = Scope::global();
    drop(global_scope);
    
    let global = Scope::global();
    let function_scope = global.function_scope();
    drop(function_scope);
    
    let global2 = Scope::global();
    let block_scope = global2.block_scope();
    drop(block_scope);
    
    assert!(true);
}

/// Test that BytecodeGenerator can be instantiated
#[test]
fn test_bytecode_generator_instantiation() {
    let scope = Scope::global();
    let generator = BytecodeGenerator::new(scope);
    drop(generator);
    assert!(true);
}

/// Test that Ignition interpreter can be instantiated
#[test]
fn test_interpreter_instantiation() {
    let interpreter = Ignition::new();
    drop(interpreter);
    assert!(true);
}

/// Test that BytecodeChunk can be created
#[test]
fn test_bytecode_chunk_creation() {
    let chunk = BytecodeChunk {
        instructions: vec![],
        constants: vec![],
        local_count: 0,
    };
    drop(chunk);
    assert!(true);
}

/// Test that all Instruction variants can be created
#[test]
fn test_instruction_variants() {
    let instructions = vec![
        Instruction::LoadConst(0),
        Instruction::LoadLocal(0),
        Instruction::StoreLocal(0),
        Instruction::Add,
        Instruction::Sub,
        Instruction::Mul,
        Instruction::Div,
        Instruction::Call(0),
        Instruction::Return,
        Instruction::Jump(0),
        Instruction::JumpIfFalse(0),
    ];
    
    assert_eq!(instructions.len(), 11);
}

/// Test that all Value variants can be created
#[test]
fn test_value_variants() {
    let num = Value::Number(42.0);
    let func = Value::Function(0);
    let undef = Value::Undefined;
    
    assert!(matches!(num, Value::Number(_)));
    assert!(matches!(func, Value::Function(_)));
    assert!(matches!(undef, Value::Undefined));
}

/// Test that all Error variants can be created
#[test]
fn test_error_variants() {
    let parse_err = Error::ParseError(ParseError::UnexpectedEOF);
    let runtime_err = Error::RuntimeError(RuntimeError::DivisionByZero);
    let compile_err = Error::CompileError(CompileError::UnsupportedFeature {
        feature: "test".to_string(),
    });
    
    assert!(matches!(parse_err, Error::ParseError(_)));
    assert!(matches!(runtime_err, Error::RuntimeError(_)));
    assert!(matches!(compile_err, Error::CompileError(_)));
}

/// Test that ASTNode variants can be created
#[test]
fn test_ast_node_variants() {
    let span = Span::new(0, 1);
    
    let _program = ASTNode::Program(vec![]);
    let _number = ASTNode::NumberLiteral { value: 42.0, span };
    let _identifier = ASTNode::Identifier { name: "x".to_string(), span };
    let _binary = ASTNode::BinaryExpr {
        op: BinOp::Add,
        left: Box::new(ASTNode::NumberLiteral { value: 1.0, span }),
        right: Box::new(ASTNode::NumberLiteral { value: 2.0, span }),
        span,
    };
    
    assert!(true);
}

/// Test that BinOp variants exist
#[test]
fn test_binop_variants() {
    let ops = vec![BinOp::Add, BinOp::Sub, BinOp::Mul, BinOp::Div];
    assert_eq!(ops.len(), 4);
}

/// Test that ScopeType variants exist
#[test]
fn test_scope_type_variants() {
    let _global = ScopeType::Global;
    let _function = ScopeType::Function;
    let _block = ScopeType::Block;
    assert!(true);
}

/// Test that Span utility methods work
#[test]
fn test_span_utilities() {
    let span1 = Span::new(0, 5);
    let span2 = Span::single(10);
    let merged = span1.merge(span2);
    
    assert_eq!(span1.start, 0);
    assert_eq!(span1.end, 5);
    assert_eq!(span2.start, 10);
    assert_eq!(span2.end, 10);
    assert_eq!(merged.start, 0);
    assert_eq!(merged.end, 10);
}

/// Test that quickcheck dependency is available
#[test]
fn test_quickcheck_dependency() {
    // This test verifies that quickcheck is properly loaded
    // by using a simple quickcheck test
    use quickcheck::quickcheck;
    
    fn prop_identity(x: i32) -> bool {
        x == x
    }
    
    quickcheck(prop_identity as fn(i32) -> bool);
    assert!(true);
}

/// Test that the engine can execute a minimal program
#[test]
fn test_minimal_execution() {
    let mut engine = Engine::new();
    let result = engine.execute("42");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(42.0));
}

/// Test that the engine properly handles errors
#[test]
fn test_error_handling() {
    let mut engine = Engine::new();
    
    // Test parse error
    let parse_result = engine.execute("let = 10");
    assert!(parse_result.is_err());
    
    // Test runtime error (division by zero)
    let runtime_result = engine.execute("10 / 0");
    assert!(runtime_result.is_err());
}

/// Test that multiple engine instances can coexist
#[test]
fn test_multiple_engines() {
    let mut engine1 = Engine::new();
    let mut engine2 = Engine::new();
    
    let result1 = engine1.execute("10 + 20").unwrap();
    let result2 = engine2.execute("30 + 40").unwrap();
    
    assert_eq!(result1, Value::Number(30.0));
    assert_eq!(result2, Value::Number(70.0));
}

/// Test that the project compiles with all features
#[test]
fn test_project_compilation() {
    // If this test runs, it means the project compiled successfully
    // with all dependencies and modules properly linked
    assert!(true);
}
