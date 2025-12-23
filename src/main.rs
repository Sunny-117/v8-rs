use v8_rs::Engine;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            // 无参数：启动 REPL
            run_repl();
        }
        2 => {
            // 一个参数：执行文件
            let filename = &args[1];
            run_file(filename);
        }
        _ => {
            // 多个参数：显示用法
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn run_repl() {
    println!("V8-RS JavaScript Engine v0.1.0");
    println!("Type JavaScript code or 'exit' to quit\n");
    
    let mut engine = Engine::new();
    let stdin = io::stdin();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                if input == "exit" || input == "quit" {
                    println!("Goodbye!");
                    break;
                }
                
                match engine.execute(input) {
                    Ok(result) => println!("{}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn run_file(filename: &str) {
    // 读取文件内容
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };
    
    // 执行代码
    let mut engine = Engine::new();
    match engine.execute(&source) {
        Ok(result) => {
            // 只在非 Undefined 时打印结果
            if !matches!(result, v8_rs::Value::Undefined) {
                println!("{}", result);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {}              Start REPL (interactive mode)", program);
    eprintln!("  {} <file.js>    Execute JavaScript file", program);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {}              # Start interactive shell", program);
    eprintln!("  {} script.js    # Run script.js", program);
}
