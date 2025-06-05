use std::fmt::Debug;

pub type DynExpr = Box<dyn Expr>;

#[derive(Debug, Clone)]
pub enum Def {
    Generic(GenericDef),
    Value(ValueDef),
    Type(TypeDef),
}

#[derive(Debug, Clone)]
pub struct GenericDef {
    pub name: String,
    pub args: Vec<(String, Vec<TypeRef>)>,
}

#[derive(Debug, Clone)]
pub struct ValueDef {
    pub name: String,
    pub body: DynExpr,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub fields: Vec<(String, TypeRef)>,
}

pub trait Expr: Debug {
    fn boxed(&self) -> DynExpr;
}

impl Clone for DynExpr {
    fn clone(&self) -> Self {
        self.boxed()
    }
}

impl Expr for i64 {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for f64 {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for char {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

impl Expr for String {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
}

impl Expr for Constant {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub arg_name: String,
    pub arg_type: TypeRef,
    pub ret_type: Option<TypeRef>,
    pub body: DynExpr,
}

impl Expr for Func {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub func: DynExpr,
    pub arg: DynExpr,
}

impl Expr for Call {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct IfThenElse {
    pub condition_expr: DynExpr,
    pub then_expr: DynExpr,
    pub else_expr: DynExpr,
}

impl Expr for IfThenElse {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct LetIn {
    pub defs: Vec<ValueDef>,
    pub body: DynExpr,
}

impl Expr for LetIn {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub enum TypeRef {
    Named(String, Vec<TypeRef>),
    Function(Box<TypeRef>, Box<TypeRef>),
}
