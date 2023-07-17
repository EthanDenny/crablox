pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use crate::vm::InterpretResult;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]\n");
        process::exit(64);
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Could not read line");
        interpret(line);
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path)
        .expect("Could not read file");

    let result = interpret(source);
  
    if result == InterpretResult::CompileError { process::exit(65); }
    if result == InterpretResult::RuntimeError { process::exit(70); }
}

fn interpret(source: String) -> InterpretResult {
    compiler::compile(source);
    InterpretResult::Ok
}
