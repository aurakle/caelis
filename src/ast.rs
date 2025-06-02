use std::collections::HashMap;

pub type DynExpr = Box<dyn Expr>;

pub trait Expr {
}

pub struct Name(pub String);

impl Expr for Name {

}

pub struct Call(pub Name, pub DynExpr);

impl Expr for Call {

}
pub struct FnDef(pub Name, pub Option<TypeRef>, pub ArgDef, pub DynExpr);

impl Expr for FnDef {

}

pub struct ArgDef(pub Name, pub TypeRef);

impl Expr for ArgDef {

}

pub struct TypeDef(pub Name, pub HashMap<Name, DynExpr>);

impl Expr for TypeDef {

}

pub struct TypeRef(pub Name);

impl Expr for TypeRef {

}

impl Expr for i32 {

}

impl Expr for i64 {

}

impl Expr for f32 {

}

impl Expr for f64 {

}

impl Expr for char {

}

impl Expr for String {

}
