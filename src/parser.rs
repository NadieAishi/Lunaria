// Aquí va la implementación del parser de Lunaria
use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        Ok(vec![]) // Aquí irá la lógica real
    }
}
