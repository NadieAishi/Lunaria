// Aquí va la implementación del lexer de Lunaria
#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Operator(String),
    Symbol(String),
    Keyword(String),
    Newline,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        // Aquí irá la lógica del lexer
        vec![]
    }
}
