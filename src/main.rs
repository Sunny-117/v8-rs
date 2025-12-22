use v8_rs::Engine;

fn main() {
    println!("V8-RS JavaScript Engine");
    println!("Version 0.1.0\n");
    
    let mut engine = Engine::new();
    
    // Test examples
    let examples = vec![
        "42",
        "10 + 20",
        "(5 + 3) * 2",
        "100 / 4",
        "let x = 15;",
    ];
    
    for example in examples {
        println!("Executing: {}", example);
        match engine.execute(example) {
            Ok(result) => println!("Result: {:?}\n", result),
            Err(err) => println!("Error: {}\n", err),
        }
    }
}
