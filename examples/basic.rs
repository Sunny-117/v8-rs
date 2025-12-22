// Basic example of using V8-RS engine

use v8_rs::Engine;

fn main() {
    println!("=== V8-RS Basic Examples ===\n");
    
    let mut engine = Engine::new();
    
    // Example 1: Simple numbers
    println!("Example 1: Simple number");
    match engine.execute("42") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 2: Basic arithmetic
    println!("Example 2: Basic arithmetic");
    match engine.execute("10 + 20 * 2") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 3: Parentheses
    println!("Example 3: Parentheses");
    match engine.execute("(10 + 20) * 2") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 4: Division
    println!("Example 4: Division");
    match engine.execute("100 / 4") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 5: Variable declaration
    println!("Example 5: Variable declaration");
    match engine.execute("let x = 15;") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 6: Complex expression
    println!("Example 6: Complex expression");
    match engine.execute("((5 + 3) * 2) - 6") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 7: Floating point
    println!("Example 7: Floating point");
    match engine.execute("3.14 * 2.0") {
        Ok(result) => println!("Result: {:?}\n", result),
        Err(err) => println!("Error: {}\n", err),
    }
    
    println!("=== All examples completed ===");
}
