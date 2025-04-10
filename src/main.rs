mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

use std::env;
use std::fs;
use std::path::Path;

fn ensure_directories_exist() {
    let dirs = ["out", "bin"];
    for dir in dirs {
        let path = Path::new(dir);
        if !path.exists() {
            if let Err(e) = fs::create_dir_all(path) {
                eprintln!("❌ Error creando carpeta '{}': {}", dir, e);
            } else {
                println!("📁 Carpeta creada: {}", dir);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: lunaria_compiler <archivo_fuente>");
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("❌ No se pudo leer el archivo");

    // 🧱 Crear carpetas necesarias antes de todo
    ensure_directories_exist();

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
        Err(e) => eprintln!("❌ Error de parseo: {}", e),
    }
}
