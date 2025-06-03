use std::collections::HashMap;

pub type DynExpr = Box<dyn Expr>;

pub trait Expr {
    //TODO: use serde_json?
    fn debug_text(&self) -> String;
    fn boxed(&self) -> DynExpr;
}

impl Clone for DynExpr {
    fn clone(&self) -> Self {
        self.boxed()
    }
}

impl Expr for i64 {
    fn debug_text(&self) -> String {
        format!("Int64: {}", self)
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for f64 {
    fn debug_text(&self) -> String {
        format!("Float64: {}", self)
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for char {
    fn debug_text(&self) -> String {
        format!("Char: \'{}\'", self)
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for String {
    fn debug_text(&self) -> String {
        format!("String: \"{}\"", self)
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Clone)]
pub struct Field {
    pub name: String,
}

impl Expr for Field {
    fn debug_text(&self) -> String {
        format!("Field: {{ Name: {} }}", self.name)
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Clone)]
pub struct Assignment {
    pub name: String,
    pub body: DynExpr,
}

impl Expr for Assignment {
    fn debug_text(&self) -> String {
        format!(
            "Assignment: {{ Name: {}, Body: {} }}",
            self.name,
            self.body.debug_text()
        )
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Clone)]
pub struct FnDef {
    pub arg_name: String,
    pub arg_type: String,
    pub ret_type: Option<String>,
    pub body: DynExpr,
}

impl Expr for FnDef {
    fn debug_text(&self) -> String {
        format!(
            "FnDef: {{ ArgName: {}, ArgType: {}, RetType: {}, Body: {} }}",
            self.arg_name,
            self.arg_type,
            self.ret_type.clone().unwrap_or(String::from("null")),
            self.body.debug_text()
        )
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

// #[derive(Clone)]
// pub struct TypeDef {
//     pub name: String,
//     pub fields: HashMap<String, DynExpr>,
// }
//
// impl Expr for TypeDef {
//
// }

#[derive(Clone)]
pub struct FnCall {
    pub func: DynExpr,
    pub arg: DynExpr,
}

impl Expr for FnCall {
    fn debug_text(&self) -> String {
        format!(
            "FnCall: {{ Func: {}, Arg: {} }}",
            self.func.debug_text(),
            self.arg.debug_text()
        )
    }

    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}
