use crate::lexer::Token;
use crate::ast::Expr;

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

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let mut mutable = false;
        if matches!(self.peek(), Some(Token::Keyword(k)) if k == "mut") {
            self.advance();
            mutable = true;
        }
    
        let name = if let Some(Token::Identifier(id)) = self.advance() {
            id.clone()
        } else {
            return Err("❌ Expected identifier after 'define'".to_string());
        };
    
        if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "::") {
            return Err("❌ Expected '::' after identifier".to_string());
        }
    
        let type_hint = if let Some(Token::Identifier(t)) = self.advance() {
            Some(t.clone())
        } else {
            return Err("❌ Expected type after '::'".to_string());
        };
    
        if !matches!(self.advance(), Some(Token::Operator(op)) if op == ":=") {
            return Err("❌ Expected ':=' after type".to_string());
        }
    
        // 💡 Aquí permitimos parsear expresiones completas (incluyendo llamadas a funciones)
        let value = self.parse_expression()?;
    
        if matches!(self.peek(), Some(Token::Symbol(s)) if s == ";") {
            self.advance();
        }
    
        Ok(Expr::Assignment {
            name,
            value: Box::new(value),
            type_hint,
            mutable,
        })
    }
    

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();

        while self.position < self.tokens.len() {
            match self.tokens.get(self.position) {
                Some(Token::Keyword(kw)) if kw == "fn" => {
                    let func = self.parse_single_function()?;
                    expressions.push(func);
                }
                Some(Token::Keyword(kw)) if kw == "summon" => {
                    self.advance();
                
                    let name = if let Some(Token::Identifier(id)) = self.advance() {
                        id.clone()
                    } else {
                        return Err("❌ Expected module name after 'summon'".to_string());
                    };
                
                    if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "{") {
                        return Err("❌ Expected '{{' to start module body".to_string());
                    }
                
                    let mut body = Vec::new();
                
                    while !matches!(self.peek(), Some(Token::Symbol(s)) if s == "}") {
                        if matches!(self.peek(), Some(Token::Keyword(k)) if k == "fn") {
                            self.advance(); // consume 'fn'
                
                            let fname = if let Some(Token::Identifier(id)) = self.advance() {
                                id.clone()
                            } else {
                                return Err("❌ Expected function name".to_string());
                            };
                
                            if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "(") {
                                return Err("❌ Expected '(' after function name".to_string());
                            }
                
                            let mut params = Vec::new();
                            while !matches!(self.peek(), Some(Token::Symbol(s)) if s == ")") {
                                let param_name = if let Some(Token::Identifier(id)) = self.advance() {
                                    id.clone()
                                } else {
                                    return Err("❌ Expected parameter name".to_string());
                                };
                
                                if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "::") {
                                    return Err("❌ Expected '::' after parameter name".to_string());
                                }
                
                                let param_type = if let Some(Token::Identifier(typ)) = self.advance() {
                                    typ.clone()
                                } else {
                                    return Err("❌ Expected parameter type".to_string());
                                };
                
                                params.push((param_name, param_type));
                
                                if matches!(self.peek(), Some(Token::Symbol(s)) if s == ",") {
                                    self.advance();
                                }
                            }
                
                            self.advance(); // consume ')'
                
                            if !matches!(self.advance(), Some(Token::Operator(op)) if op == "->") {
                                return Err("❌ Expected '->' after parameters".to_string());
                            }
                
                            let return_type = if let Some(Token::Identifier(rt)) = self.advance() {
                                rt.clone()
                            } else {
                                return Err("❌ Expected return type".to_string());
                            };
                
                            if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "{") {
                                return Err("❌ Expected '{{' to start function body".to_string());
                            }
                
                            let mut func_body = Vec::new();
                            while !matches!(self.peek(), Some(Token::Symbol(s)) if s == "}") {
                                let expr = self.parse_expression()?;
                                if !matches!(expr, Expr::Empty) {
                                    func_body.push(expr);
                                }
                            }
                
                            self.advance(); // consume '}'
                
                            body.push(Expr::FunctionDef {
                                name: fname,
                                params,
                                return_type,
                                body: func_body,
                            });
                        } else {
                            let expr = self.parse_expression()?;
                            if !matches!(expr, Expr::Empty) {
                                println!("⚠️ Ignorado en módulo '{}': {:?}", name, expr);
                            }
                        }
                    }
                
                    self.advance(); // consume '}'
                    expressions.push(Expr::ModuleDef { name, body });
                }
                
//

                Some(Token::Keyword(kw)) if kw == "evoke" => {
                    self.advance();

                    let module_name = if let Some(Token::Identifier(id)) = self.advance() {
                        id.clone()
                    } else {
                        return Err("❌ Expected module name after 'evoke'".to_string());
                    };

                    if matches!(self.peek(), Some(Token::Symbol(s)) if s == ";") {
                        self.advance();
                    }

                    expressions.push(Expr::ModuleImport(module_name));
                }

                Some(Token::Keyword(kw)) if kw == "define" => {
                    self.advance();
                    let assignment = self.parse_assignment()?;
                    expressions.push(assignment);
                }

                _ => {
                    let expr = self.parse_expression()?;
                    expressions.push(expr);
                }
            }
        }

        Ok(expressions)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.tokens.get(self.position) {
            Some(Token::Keyword(kw)) if kw == "return" => {
                self.advance(); // consume 'return'

                if matches!(self.peek(), Some(Token::Symbol(s)) if s == ";") {
                    self.advance();
                    return Ok(Expr::Return(Box::new(Expr::Empty)));
                }

                let expr = self.parse_expression()?;
                Ok(Expr::Return(Box::new(expr)))
            }

            Some(Token::Identifier(first)) => {
                let mut name = first.clone();
                self.advance();

                while let Some(Token::Symbol(dot)) = self.peek() {
                    if dot == "." {
                        self.advance(); // consume '.'

                        match self.peek() {
                            Some(Token::Identifier(next)) => {
                                let next = next.clone();
                                self.advance(); // consume identifier
                                name = format!("{}.{}", name, next);
                            }
                            other => {
                                return Err(format!(
                                    "❌ Expected identifier after '.', found: {:?}",
                                    other
                                ));
                            }
                        }
                    } else {
                        break;
                    }
                }

                if matches!(self.peek(), Some(Token::Symbol(s)) if s == "(") {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Some(Token::Symbol(s)) if s == ")") {
                        match self.peek() {
                            Some(Token::String(s)) => {
                                args.push(Expr::String(s.clone()));
                                self.advance();
                            }
                            Some(Token::Identifier(id)) => {
                                args.push(Expr::Identifier(id.clone()));
                                self.advance();
                            }
                            Some(Token::Number(n)) => {
                                args.push(Expr::Number(*n));
                                self.advance();
                            }
                            Some(Token::Boolean(b)) => {
                                args.push(Expr::Boolean(*b));
                                self.advance();
                            }
                            _ => break,
                        }

                        if matches!(self.peek(), Some(Token::Symbol(s)) if s == ",") {
                            self.advance();
                        }
                    }
                    self.advance(); // consume ')'
                   return Ok(Expr::FunctionCall { name, args })
                
                }
                // Comprobar si hay acceso con corchetes: agenda["Juan"]
if matches!(self.peek(), Some(Token::Symbol(s)) if s == "[") {
    self.advance(); // consume '['

    let key_expr = match self.advance() {
        Some(Token::String(s)) => Expr::String(s.clone()),
        Some(Token::Identifier(id)) => Expr::Identifier(id.clone()),
        other => return Err(format!("❌ Expected string or identifier as map key, found: {:?}", other)),
    };

    if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "]") {
        return Err("❌ Expected ']' after map key".to_string());
    }

    return Ok(Expr::MapAccess {
        map: Box::new(Expr::Identifier(name)),
        key: Box::new(key_expr),
    });
}
                else {
                    Ok(Expr::Identifier(name))
                }
            }

            _ => {
                self.advance();
                Ok(Expr::Empty)
            }
        }
    }

    // ✅ Función auxiliar: parsea una única función sin recursión infinita
    fn parse_single_function(&mut self) -> Result<Expr, String> {
        self.advance(); // consume 'fn'

        let name = if let Some(Token::Identifier(id)) = self.advance() {
            id.clone()
        } else {
            return Err("❌ Expected function name".to_string());
        };

        if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "(") {
            return Err("❌ Expected '(' after function name".to_string());
        }

        let mut params = Vec::new();
        while !matches!(self.peek(), Some(Token::Symbol(s)) if s == ")") {
            let param_name = if let Some(Token::Identifier(id)) = self.advance() {
                id.clone()
            } else {
                return Err("❌ Expected parameter name".to_string());
            };

            if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "::") {
                return Err("❌ Expected '::' after parameter name".to_string());
            }

            let param_type = if let Some(Token::Identifier(typ)) = self.advance() {
                typ.clone()
            } else {
                return Err("❌ Expected parameter type".to_string());
            };

            params.push((param_name, param_type));

            if matches!(self.peek(), Some(Token::Symbol(s)) if s == ",") {
                self.advance();
            }
        }

        self.advance(); // consume ')'

        if !matches!(self.advance(), Some(Token::Operator(op)) if op == "->") {
            return Err("❌ Expected '->' after parameters".to_string());
        }

        let return_type = if let Some(Token::Identifier(rt)) = self.advance() {
            rt.clone()
        } else {
            return Err("❌ Expected return type".to_string());
        };

        if !matches!(self.advance(), Some(Token::Symbol(s)) if s == "{") {
            return Err("❌ Expected '{' to start function body".to_string());
        }

        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Token::Symbol(s)) if s == "}") {
            if self.peek().is_none() {
                return Err("❌ Unexpected end of function body".to_string());
            }
            let expr = self.parse_expression()?;
            body.push(expr);
        }

        self.advance(); // consume '}'

        Ok(Expr::FunctionDef {
            name,
            params,
            return_type,
            body,
        })
    }
}
