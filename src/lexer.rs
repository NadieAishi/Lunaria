#[derive(Debug, Clone, PartialEq)]
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
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();
    
        while let Some(&ch) = chars.peek() {
            match ch {
                c if c.is_whitespace() => {
                    chars.next();
                }
    
                ':' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(":=".to_string()));
                    } else if chars.peek() == Some(&':') {
                        chars.next();
                        tokens.push(Token::Symbol("::".to_string()));
                    }
                }
    
                '"' => {
                    chars.next(); // skip opening quote
                    let mut value = String::new();
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            // Escaped character
                            if let Some(escaped) = chars.next() {
                                match escaped {
                                    'n' => value.push('\n'),
                                    't' => value.push('\t'),
                                    'r' => value.push('\r'),
                                    '\\' => value.push('\\'),
                                    '"' => value.push('"'),
                                    _ => {
                                        value.push('\\');
                                        value.push(escaped);
                                    }
                                }
                            }
                        } else if c == '"' {
                            break;
                        } else {
                            value.push(c);
                        }
                    }                    
                    tokens.push(Token::String(value));
                }
    
                '.' | ',' | '(' | ')' => {
                    tokens.push(Token::Symbol(ch.to_string()));
                    chars.next();
                }
    
                c if c.is_alphanumeric() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
    
                    if ident == "define" {
                        tokens.push(Token::Keyword(ident));
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }
    
                _ => {
                    chars.next(); // skip unrecognized character
                }
            }
        }
    
        tokens
    }
    
}
