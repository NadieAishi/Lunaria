#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Operator(String),
    Symbol(String),
    Keyword(String),
    Boolean(bool),
    Comment(String),
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

                '~' => {
                    chars.next();
                    match chars.peek() {
                        Some('>') => {
                            chars.next();
                            let mut comment = String::new();
                            while let Some(&c) = chars.peek() {
                                if c == '\n' {
                                    break;
                                }
                                comment.push(c);
                                chars.next();
                            }
                            tokens.push(Token::Comment(comment));
                        }
                        Some('<') => {
                            chars.next();
                            let mut comment = String::new();
                            while let Some(c) = chars.next() {
                                if c == '>' && chars.peek() == Some(&'~') {
                                    chars.next();
                                    break;
                                }
                                comment.push(c);
                            }
                            tokens.push(Token::Comment(comment));
                        }
                        _ => {}
                    }
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
                // -> symbol
                '-' =>{
                    chars.next();
                    if chars.peek() == Some(&'>'){
                        chars.next();
                    tokens.push(Token::Operator("->".to_string()));
                    }else{
                        tokens.push(Token::Operator("-".to_string()));
                    }
                }

                '"' | '\'' => {
                    let quote = chars.next().unwrap();
                    let mut value = String::new();
                    while let Some(c) = chars.next() {
                        if c == '\\' && quote == '"' {
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
                        } else if c == quote {
                            break;
                        } else {
                            value.push(c);
                        }
                    }
                    tokens.push(Token::String(value));
                }

                '.' | ',' | '(' | ')' | ';' | '{' | '}' => {
                    tokens.push(Token::Symbol(ch.to_string()));
                    chars.next();
                }

                c if c.is_ascii_digit() => {
                    let mut num_str = String::new();
                    let mut is_float = false;
                
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            chars.next();
                        } else if c == '.' && !is_float {
                            is_float = true;
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                
                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    } else {
                        println!("⚠️ Número mal formado: {}", num_str);
                    }
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

                    match ident.as_str() {
                       "fn" | "define" | "mut" | "return" | "evoke" | "summon" | "as" => {
                            tokens.push(Token::Keyword(ident));
                        }
                        "true" => tokens.push(Token::Boolean(true)),
                        "false" => tokens.push(Token::Boolean(false)),
                        "and" | "or" | "not" | "if" | "else" | "elif" | "match" => {
                            tokens.push(Token::Operator(ident));
                        }
                        _ => tokens.push(Token::Identifier(ident)),
                    }
                }

                _ => {
                    chars.next(); // skip unrecognized
                }
            }
        }

        tokens
    }
}
