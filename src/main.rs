use v8_rs::Value;

fn main() {
    println!("V8-RS JavaScript Engine");
    println!("Version 0.1.0");
    
    // Basic initialization test
    let value = Value::Number(42.0);
    println!("Created value: {:?}", value);
}
