#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

use std::env;
use std::fs;
use std::io;

mod ast;
mod lexer;
mod parser;
mod interpreter;
mod grimoire;
mod builtins;
mod repl;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::grimoire::Grimoire;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file = args.get(1).cloned().unwrap_or("main.lna".to_string());

    match fs::read_to_string(&target_file) {
        Ok(code) => {
            if code.trim().is_empty() {
                println!("🌑 El grimorio está en blanco... nada que conjurar.");
            } else {
                let mut lexer = Lexer::new(&code);
                let tokens = lexer.tokenize();

                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => {
                        let mut interpreter = Interpreter::new();
                        interpreter.interpret(ast);
                    }
                    Err(e) => {
                        println!("❌ Error al interpretar '{}': {}", target_file, e);
                    }
                }
            }
        }
        Err(_) => {
            println!("⚠️ No se pudo encontrar '{}'.", target_file);
        }
    }

    // Inicia REPL solo si no se proporcionó un archivo
    if args.get(1).is_none() {
        println!("\n🌙 Welcome to Lunaria REPL");
        println!("Type 'exit()' to leave the universe.\n");
        // Start the repl environment
        repl::start_repl(Grimoire::new());
    }
}
