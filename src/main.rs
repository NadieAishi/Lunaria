mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lunaria_compiler <source_file>");
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize();
    println!("--- Tokens ---");
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("--- AST generated ---");
            let mut interpreter = Interpreter::new();
            interpreter.interpret(ast);
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
