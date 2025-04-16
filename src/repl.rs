use std::io::{self, Write};
use crate::grimoire::Grimoire;
use crate::ast::Value;

pub fn start_repl(grim: Grimoire) {
    //println!("🌙 Welcome to Lunaria REPL");
    //println!("Type 'exit()' to leave the universe.\n");

    loop {
        print!("🌙> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("⚠️ Failed to read input.");
            continue;
        }

        let input = input.trim();

        if input == "exit()" {
            println!("🌒 Farewell, dreamer.");
            break;
        }

        // Soporte mínimo para: console.out("...")
        if let Some(rest) = input.strip_prefix("console.out(") {
            if let Some(arg) = rest.strip_suffix(");") {
                let arg = arg.trim_matches('"');
                grim.call_function("console", "out", vec![Value::String(arg.to_string())]);
                continue;
            }
        }

        println!("❌ Unknown or unsupported expression.");
    }
}
