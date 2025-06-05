use std::collections::HashMap;
use std::fmt::Debug;

pub type DynDef = Box<dyn TopLevelDef>;
pub type DynExpr = Box<dyn Expr>;

pub trait TopLevelDef: Debug {
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub body: DynExpr,
}

impl TopLevelDef for Assignment {
}

#[derive(Debug, Clone)]
pub struct GenericAssignment {
    pub name: String,
    pub args: HashMap<String, Vec<TypeRef>>,
}

impl TopLevelDef for GenericAssignment {

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
pub struct FnDef {
    pub arg_name: String,
    pub arg_type: TypeRef,
    pub ret_type: Option<TypeRef>,
    pub body: DynExpr,
}

impl Expr for FnDef {
    fn boxed(&self) -> DynExpr {
        Box::new(self.clone()) as DynExpr
    }
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub func: DynExpr,
    pub arg: DynExpr,
}

impl Expr for FnCall {
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
pub enum TypeRef {
    Named(String),
    // Function(),
}
