#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]


mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("❌ No se pudo leer el archivo .lna");

    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("❌ Error al parsear el código");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast);
}

fn run_repl() {
    let mut interpreter = Interpreter::new();

    loop {
        print!("🌙> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("❌ Error al leer la entrada.");
            continue;
        }

        if input.trim().is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(ast) => interpreter.interpret(ast),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        run_file(filename);
    } else {
        run_repl();
    }
}
