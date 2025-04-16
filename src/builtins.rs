use crate::grimoire::{Grimoire, Module, Function};
use crate::ast::Value;

pub fn load_builtin_modules(grim: &mut Grimoire) {
    let mut console = Module::new("console");

    console.insert("out", Function::Native(|args| {
        if let Some(Value::String(s)) = args.get(0) {
            println!("{}", s);
        }
        Value::Void
    }));

    grim.add_module(console);
}
