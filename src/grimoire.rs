use std::collections::HashMap;
use crate::ast::Value;

pub enum Function {
    Native(fn(Vec<Value>) -> Value),
    // Future: UserDefined(UserFunction)
}

pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Function>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            functions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, function_name: &str, function: Function) {
        self.functions.insert(function_name.to_string(), function);
    }

    pub fn get(&self, function_name: &str) -> Option<&Function> {
        self.functions.get(function_name)
    }
}

pub struct Grimoire {
    pub modules: HashMap<String, Module>,
    pub variables: HashMap<String, Value>,
}

impl Grimoire {
    pub fn new() -> Self {
        Grimoire {
            modules: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.name.clone(), module);
    }

    pub fn call_function(&self, module: &str, function: &str, args: Vec<Value>) -> Option<Value> {
        self.modules.get(module).and_then(|m| {
            m.get(function).map(|f| match f {
                Function::Native(func) => func(args),
                // Future: UserDefined(f) => f.run(args)
            })
        })
    }
    // Nueva función
    pub fn call_global_function(&self, function: &str, args: Vec<Value>) -> Option<Value>{
        for (_name, module) in &self.modules{
            if let Some(func) = module.get(function){
                return match func{
                    Function::Native(f) => Some(f(args)),
                };
            }
        }
        None
    }
}
