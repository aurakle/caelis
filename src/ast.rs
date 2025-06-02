use std::collections::HashMap;

pub type DynExpr = Box<dyn Expr>;

pub trait Expr {
    fn debug_text(&self) -> String;
}

impl Expr for i64 {
    fn debug_text(&self) -> String {
        format!("Int64: {}", self)
    }
}

impl Expr for f64 {
    fn debug_text(&self) -> String {
        format!("Float64: {}", self)
    }
}

impl Expr for char {
    fn debug_text(&self) -> String {
        format!("Char: \'{}\'", self)
    }
}

impl Expr for String {
    fn debug_text(&self) -> String {
        format!("String: \"{}\"", self)
    }
}

pub struct Assignment {
    pub name: String,
    pub body: DynExpr,
}

impl Expr for Assignment {
    fn debug_text(&self) -> String {
        format!("Assignment: {{ Name: {}, Body: {} }}", self.name, self.body.debug_text())
    }
}

pub struct FnDef {
    pub arg_name: String,
    pub arg_type: String,
    pub ret_type: Option<String>,
    pub body: DynExpr,
}

impl Expr for FnDef {
    fn debug_text(&self) -> String {
        format!("FnDef: {{ ArgName: {}, ArgType: {}, RetType: {}, Body: {} }}", self.arg_name, self.arg_type, self.ret_type.clone().unwrap_or(String::from("null")), self.body.debug_text())
    }
}

// pub struct TypeDef {
//     pub name: String,
//     pub fields: HashMap<String, DynExpr>,
// }
//
// impl Expr for TypeDef {
//
// }

pub struct FnCall {
    pub func: DynExpr,
    pub arg: DynExpr,
}

impl Expr for FnCall {
    fn debug_text(&self) -> String {
        format!("FnCall: {{ Func: {}, Arg: {} }}", self.func.debug_text(), self.arg.debug_text())
    }
}
