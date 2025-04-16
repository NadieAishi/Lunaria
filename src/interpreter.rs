
use std::collections::{HashSet, HashMap};
use rand::seq::SliceRandom;
use crate::ast::Expr;
use crate::ast::Value;

#[derive(Clone)]
pub enum Function {
    Native(fn(Vec<Value>) -> Value),
    UserDefined {
        name: String,
        params: Vec<(String, String)>,
        body: Vec<Expr>,
        return_type: String,
    },
}

pub struct Interpreter {
    pub env: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    pub imported_modules: HashSet<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            env: HashMap::new(),
            functions: HashMap::new(),
            imported_modules: HashSet::new(),
        };
        register_native_functions(&mut interpreter);
        interpreter
    }

    pub fn interpret(&mut self, expressions: Vec<Expr>) {
        for expr in &expressions {
            if matches!(expr, Expr::FunctionDef {..} | Expr::ModuleDef {..}) {
                self.eval(expr.clone());
            }
        }
        for expr in expressions {
            if !matches!(expr, Expr::FunctionDef { .. } | Expr::ModuleDef { .. }) {
                let _ = self.eval(expr);
            }
        }
        self.entry_point();
    }

    pub fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Assignment { name, value, type_hint, mutable } => {
                let val = self.eval_expr(*value);
                if let Some(expected) = &type_hint {
                    let matches = match (expected.as_str(), &val) {
                        ("string", Value::String(_)) => true,
                        ("int", Value::Integer(_)) => true,
                        ("bool", Value::Boolean(_)) => true,
                        ("void", Value::Void) => true,
                        _ => false,
                    };
                    if !matches {
                        println!("❌ Error de tipo: Se esperaba '{}' pero se recibió '{:?}'", expected, val);
                    }
                }
                if mutable {
                    println!("📦 Asignado mutable: {} := {:?}", name, val);
                } else {
                    println!("📦 Asignado: {} := {:?}", name, val);
                }
                self.env.insert(name, val.clone());
                val
            }

            Expr::FunctionDef { name, params, body, return_type } => {
                let func = Function::UserDefined { name: name.clone(), params, body, return_type };
                println!("🧠 Función definida por el usuario: {}", name);
                self.functions.insert(name, func);
                Value::Void
            }

            Expr::FunctionCall { name, args } => {
                if let Some(func) = self.functions.get(&name).cloned() {
                    match func {
                        Function::Native(f) => {
                            let evaluated_args = args.into_iter().map(|a| self.eval_expr(a)).collect::<Vec<_>>();
                            return f(evaluated_args);
                        }
                        Function::UserDefined { params, body, .. } => {
                            if params.len() != args.len() {
                                println!("⚠️ Número incorrecto de argumentos para '{}'", name);
                                return Value::Void;
                            }
                            let mut local_env = HashMap::new();
                            for ((param_name, _), arg) in params.into_iter().zip(args.into_iter()) {
                                let value = self.eval_expr(arg);
                                local_env.insert(param_name, value);
                            }
                            let mut sub = Interpreter {
                                env: local_env,
                                functions: self.functions.clone(),
                                imported_modules: self.imported_modules.clone(),
                            };
                            for expr in body {
                                if let Expr::Return(ret_expr) = expr {
                                    return sub.eval_expr(*ret_expr);
                                } else {
                                    sub.eval(expr);
                                }
                            }
                        }
                    }
                } else {
                    println!("⚠️ Llamada a función desconocida: {}", name);
                }
                Value::Void
            }

            _ => Value::Void
        }
    }

    fn eval_expr(&mut self,expr: Expr) -> Value {
        match expr {
            Expr::Identifier(name) => self.env.get(&name).cloned().unwrap_or(Value::Void),
            Expr::MapLiteral(pairs) =>{
                let mut map = HashMap::new();
                for(k, v_expr) in pairs{
                    let v=
                    self.eval_expr(v_expr);
                        map.insert(k, v);
                }
                Value::Map(map)
            }
            Expr::MapAccess {map, key} =>{
                let m = self.eval_expr(*map);
                let k = self.eval_expr(*key);
                if let (Value::Map(map),
            Value::String(key)) = (m, k) {

                map.get(&key).cloned().unwrap_or(Value::Void)
            }else{
                println!("Invalid map access.");
                Value::Void
                }
            }
            _ => self.expr_to_value(expr),
        }
    }

    fn expr_to_value(&self, expr: Expr) -> Value {
        match expr {
            Expr::String(s) => Value::String(s),
            Expr::Number(n) => { if n.fract() == 0.0{
                Value::Integer(n as i32)
            }else{
                Value::Float(n)
            }
        }
            Expr::Boolean(b) => Value::Boolean(b),
            _ => Value::Void,
        }
    }

    fn entry_point(&mut self) {
        if let Some(Function::UserDefined { params, .. }) = self.functions.get("main") {
            if params.is_empty() {
                println!("📖 Ejecutando desde Grimorium Caelestia...");
               let _= self.eval(Expr::FunctionCall {
                    name: "main".to_string(),
                    args: vec![],
                });
            }
        }
    }
}

fn register_native_functions(interpreter: &mut Interpreter) {
    fn stringify(val: &Value) -> String {
        match val {
            Value::String(s) => s.clone(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Void => "(void)".to_string(),
            Value::Map(map) => {
                let items = map.iter()
                    .map(|(k, v)| format!("'{}': {}", k, stringify(v)))
                    .collect::<Vec<_>>();
                format!("{{ {} }}", items.join(", "))
            }
            Value::List(list) => {
                let items = list.iter().map(stringify).collect::<Vec<_>>();
                format!("[{}]", items.join(", "))
            }
        }
    }    
    
    fn console_out(args: Vec<Value>) -> Value {
        for val in args {
                print!("{}", stringify(&val));

        }
        println!();
        Value::Void
    }

    fn add(args: Vec<Value>) -> Value {
        if args.len() != 2 {
            println!("🔍 Argumentos recibidos en add: {:?}", args);
            match (&args[0], &args[1]) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                (Value::Integer(a), Value::Float(b)) => Value::Float(*a as f64 + b),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a + *b as f64),
                _ => {
                    println!("❌ 'add' espera números válidos");
                    Value::Void
                }
            }
        } else {
            println!("❌ 'add' espera 2 argumentos");
            Value::Void
        }
    }
    fn sub(args: Vec<Value>) -> Value {
        if args.len() == 2 {
            match (&args[0], &args[1]) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                (Value::Integer(a), Value::Float(b)) => Value::Float(*a as f64 - b),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a - *b as f64),
                _ => {
                    println!("❌ 'sub' espera números válidos");
                    Value::Void
                }
            }
        } else {
            println!("❌ 'sub' espera 2 argumentos");
            Value::Void
        }
    }

fn mul(args: Vec<Value>) -> Value {
    if args.len() == 2 {
        match (&args[0], &args[1]) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(*a as f64 * b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a * *b as f64),
            _ => {
                println!("❌ 'mul' espera números válidos");
                Value::Void
            }
        }
    } else {
        println!("❌ 'mul' espera 2 argumentos");
        Value::Void
    }
}
    fn div(args: Vec<Value>) -> Value{
        if args.len() == 2{
            match(&args[0], &args[1]){
                (Value::Integer(_),Value::Float(b)) if *b == 0.0 =>{
                    println!("❌ División por cero no permitida");
                    Value::Void
                }
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(*a as f64 / b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a / *b as f64),
            _ => {
                println!("❌ 'div' espera números válidos");
                Value::Void
            }
        }
    } else{
        println!("❌ 'div' espera 2 argumentos");
        Value::Void
    }
    }

    interpreter.functions.insert("console.out".to_string(), Function::Native(console_out));
    interpreter.functions.insert("math.add".to_string(), Function::Native(add));
    interpreter.functions.insert("math.sub".to_string(),Function::Native(sub));
    interpreter.functions.insert("math.mul".to_string(),Function::Native(mul));
    interpreter.functions.insert("math.div".to_string(),Function::Native(div));
}
