use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;
use std::path::Path;

use crate::parser::Expr;

#[derive(Default)]
pub struct Interpreter {
    variables: HashMap<String, Expr>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, statements: Vec<Expr>) {
        for stmt in statements {
            println!("Ejecutando instrucción: {:?}", stmt);
            match stmt {
                Expr::Assignment { name, value } => {
                    let evaluated = self.evaluate_expr(*value);
                    self.variables.insert(name, evaluated);
                }

                Expr::Call { name, args } => {
                    if name == "fs.out" && args.len() == 2 {
                        let path_eval = self.evaluate_expr(args[0].clone());
                        let content_eval = self.evaluate_expr(args[1].clone());

                        if let (Expr::String(path), Expr::String(content)) = (path_eval, content_eval) {
                            let path = path.replace("\\", std::path::MAIN_SEPARATOR_STR);
                            let path_obj = Path::new(&path);

                            if let Some(parent) = path_obj.parent() {
                                if let Err(e) = create_dir_all(parent) {
                                    println!("❌ Error creating directories: {e}");
                                    continue;
                                }
                            }

                            println!("📝 Writing file to: {path}");

                            match File::create(&path) {
                                Ok(mut file) => {
                                    if let Err(e) = file.write_all(content.as_bytes()) {
                                        println!("❌ Error writing file: {e}");
                                    }
                                }
                                Err(e) => println!("❌ Error creating file: {e}"),
                            }
                        }
                    }

                    else if name == "console.out" && args.len() == 1 {
                        let msg = self.evaluate_expr(args[0].clone());
                        if let Expr::String(msg) = msg {
                            println!("{msg}");
                        }
                    }

                    else if name == "shell.run" && args.len() == 1 {
                        let cmd = self.evaluate_expr(args[0].clone());
                        if let Expr::String(cmd) = cmd {
                            println!("💻 Running command: {cmd}");
                            let output = if cfg!(target_os = "windows") {
                                Command::new("cmd").args(["/C", &cmd]).output()
                            } else {
                                Command::new("sh").arg("-c").arg(&cmd).output()
                            };

                            match output {
                                Ok(out) => {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    if !stdout.trim().is_empty() {
                                        println!("📤 Output:\n{stdout}");
                                    }
                                    if !stderr.trim().is_empty() {
                                        eprintln!("⚠️ Error:\n{stderr}");
                                    }
                                }
                                Err(e) => println!("❌ Failed to run command: {e}"),
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    fn evaluate_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::String(s) => Expr::String(s),
            Expr::Assignment { .. } => expr,
            Expr::Call { .. } => expr,
            Expr::Identifier(name) => {
                self.variables
                    .get(&name)
                    .cloned()
                    .unwrap_or(Expr::String(format!("{{undefined:{name}}}")))
            }
            _ => expr,
        }
    }
}
