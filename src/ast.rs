use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    String(String),
    Boolean(bool),
    Number(f64),
    List(Vec<Expr>),
    Assignment {
        name: String,
        value: Box<Expr>,
        type_hint: Option<String>, // :: Support.
        mutable: bool,
    },
    FunctionDef{
        name: String,
        params: Vec<(String,String)>, //name, type
        body: Vec<Expr>,
        return_type: String,
    },
    // End FunctionDef block.
     // --------------------
    // Begin FunctionCall.
    FunctionCall{
        name: String,
        args: Vec<Expr>,
    },
    Comment,
    Empty,

MapLiteral(HashMap<String,Expr>),
MapAccess{
    map: Box<Expr>,
    key: Box<Expr>,
},
    
    Import{
        module: String,
        alias: Option<String>,
    },
    Return(Box<Expr>),
    ModuleDef{
        name:String,
        body:Vec<Expr>,
    },
    ModuleImport(String),

}
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Void,
    Map(HashMap<String, Value>),
    List(Vec<Value>),
}