#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]


use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    String(String),
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

    fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let tok = &self.tokens[self.position];
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();

        while self.position < self.tokens.len() {
            // println!("🧩 Analizando token: {:?}", self.tokens.get(self.position));

            match self.tokens.get(self.position) {
                Some(Token::Keyword(keyword)) if keyword == "define" => {
                    // println!("📌 Encontrado: define");
                    self.advance(); // consume 'define'

                    let name = if let Some(Token::Identifier(id)) = self.tokens.get(self.position) {
                        let id = id.clone();
                        self.advance();
                        id
                    } else {
                        return Err("❌ Expected identifier after 'define'".to_string());
                    };

                    if !matches!(self.tokens.get(self.position), Some(Token::Symbol(s)) if s == "::") {
                        return Err("❌ Expected '::' after identifier".to_string());
                    }
                    self.advance();

                    if matches!(self.tokens.get(self.position), Some(Token::Identifier(_))) {
                        self.advance(); // skip type
                    } else {
                        return Err("❌ Expected type after '::'".to_string());
                    }

                    if !matches!(self.tokens.get(self.position), Some(Token::Operator(op)) if op == ":=") {
                        return Err("❌ Expected ':=' after type".to_string());
                    }
                    self.advance();

                    if let Some(Token::String(value)) = self.tokens.get(self.position) {
                        let value = value.clone();
                        self.advance();
                        // println!("📦 Asignación de valor: {value}");
                        expressions.push(Expr::Assignment {
                            name,
                            value: Box::new(Expr::String(value)),
                        });
                    } else {
                        return Err("❌ Expected string value after ':='".to_string());
                    }
                }

                Some(Token::Identifier(id)) => {
                    println!("⚙️ Llamada potencial: {id}");
                    let name = id.clone();
                    self.advance();

                    if matches!(self.tokens.get(self.position), Some(Token::Symbol(s)) if s == ".") {
                        self.advance();
                        if let Some(Token::Identifier(method)) = self.tokens.get(self.position) {
                            let call_name = format!("{}.{}", name, method);
                            self.advance();

                            if !matches!(self.tokens.get(self.position), Some(Token::Symbol(s)) if s == "(") {
                                return Err("❌ Expected '(' after method name".to_string());
                            }
                            self.advance();

                            let mut args = Vec::new();
                            while !matches!(self.tokens.get(self.position), Some(Token::Symbol(s)) if s == ")") {
                                match self.tokens.get(self.position) {
                                    Some(Token::String(s)) => {
                                        println!("📨 Argumento string: {s}");
                                        args.push(Expr::String(s.clone()));
                                        self.advance();
                                    }
                                    Some(Token::Identifier(id)) => {
                                        println!("📨 Argumento identificador: {id}");
                                        args.push(Expr::Identifier(id.clone()));
                                        self.advance();
                                    }
                                    _ => break,
                                }

                                if matches!(self.tokens.get(self.position), Some(Token::Symbol(s)) if s == ",") {
                                    self.advance(); // skip comma
                                }
                            }

                            self.advance(); // skip ')'
                            // println!("📞 Generando llamada a función: {call_name}");
                            expressions.push(Expr::Call {
                                name: call_name,
                                args,
                            });
                        }
                    }
                }

                _ => {
                    // println!("⚠️ Token ignorado");
                    self.advance();
                }
            }
        }

        Ok(expressions)
    }
}
