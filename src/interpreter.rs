// Aquí va la implementación del intérprete de Lunaria
use crate::parser::Expr;
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Expr>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expressions: Vec<Expr>) {
        for expr in expressions {
            println!("{:?}", expr); // Aquí irá la ejecución real
        }
    }
}
