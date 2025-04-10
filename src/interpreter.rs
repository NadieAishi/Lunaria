use std::collections::HashMap;
use std::process::Command;
use std::fs::create_dir_all;

use crate::parser::Expr;

pub struct Interpreter {
    pub env: HashMap<String, Expr>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expressions: Vec<Expr>) {
        for expr in expressions {
            self.eval(expr);
        }
    }

    fn eval(&mut self, expr: Expr) -> Expr {
        match expr.clone() {
            Expr::Assignment { name, value } => {
                self.env.insert(name, *value);
                Expr::String(String::new())
            }

            Expr::Call { name, args } => {
                match name.as_str() {
                    "fs.out" => {
                        if let (Some(Expr::String(path)), Some(value)) = (args.get(0), args.get(1)) {
                            let dir_path = std::path::Path::new(path)
                                .parent()
                                .map(|p| p.to_str().unwrap_or(""))
                                .unwrap_or("");

                            if !dir_path.is_empty() {
                                if let Err(e) = create_dir_all(dir_path) {
                                    eprintln!("❌ Error al crear carpeta '{}': {}", dir_path, e);
                                } else {
                                    println!("📁 Carpeta creada: {}", dir_path);
                                }
                            }

                            if let Some(Expr::String(val)) = self.env.get(&value.clone_identifier_name()) {
                                if let Err(e) = std::fs::write(path, val) {
                                    eprintln!("❌ Error escribiendo en archivo '{}': {}", path, e);
                                } else {
                                    println!("📝 Writing file to: {}", path);
                                }
                            }
                        }
                        Expr::String(String::new())
                    }

                    "console.out" => {
                        if let Some(value) = args.get(0) {
                            match value {
                                Expr::String(s) => println!("{}", s),
                                Expr::Identifier(name) => {
                                    if let Some(Expr::String(val)) = self.env.get(name.as_str()) {
                                        println!("{}", val);
                                    }
                                }
                                _ => {}
                            }
                        }
                        Expr::String(String::new())
                    }

                    "shell.run" => {
                        if let Some(Expr::String(cmd)) = args.get(0) {
                            println!("💻 Running command: {}", cmd);

                            #[cfg(target_os = "windows")]
                            let output = Command::new("cmd")
                                .args(["/C", &cmd])
                                .output();

                            #[cfg(not(target_os = "windows"))]
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg(cmd)
                                .output();

                            match output {
                                Ok(output) => {
                                    if output.status.success() {
                                        let stdout = String::from_utf8_lossy(&output.stdout);
                                        if !stdout.trim().is_empty() {
                                            println!("📤 Output:\n{}", stdout);
                                        }
                                    } else {
                                        let stderr = String::from_utf8_lossy(&output.stderr);
                                        eprintln!("⚠️ Error:\n{}", stderr);
                                    }
                                }
                                Err(e) => eprintln!("⚠️ Error ejecutando comando: {}", e),
                            }
                        }
                        Expr::String(String::new())
                    }

                    _ => Expr::String(String::new()),
                }
            }

            Expr::Identifier(name) => {
                self.env.get(&name).cloned().unwrap_or_else(|| Expr::String(String::new()))
            }

            Expr::String(_) => expr,

            _ => Expr::String(String::new()), // por si acaso
        }
    }
}

// Helper method to extract identifier name from an Expr::Identifier
trait ExtractIdentifier {
    fn clone_identifier_name(&self) -> String;
}

impl ExtractIdentifier for Expr {
    fn clone_identifier_name(&self) -> String {
        if let Expr::Identifier(name) = self {
            name.clone()
        } else {
            String::new()
        }
    }
}
