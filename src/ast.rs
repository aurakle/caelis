use std::fmt::Debug;
use std::hash::Hash;

use arcstr::Substr;

pub trait Ast: Debug + Clone {
    fn text(&self) -> &Substr;
}

#[derive(Debug, Clone)]
pub struct Root {
    pub text: Substr,
    // pub imports: Vec<Import>,
    pub defs: Vec<Def>,
}

impl Ast for Root {
    fn text(&self) -> &Substr {
        &self.text
    }
}

#[derive(Debug, Clone)]
pub enum Def {
    Generic(GenericDef),
    Value(ValueDef),
    Type(TypeDef),
}

impl Ast for Def {
    fn text(&self) -> &Substr {
        match self {
            Def::Generic(generic_def) => &generic_def.text,
            Def::Value(value_def) => &value_def.text,
            Def::Type(type_def) => &type_def.text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericDef {
    pub text: Substr,
    pub name: Name,
    pub args: Vec<(Name, Vec<TypeRef>)>,
}

#[derive(Debug, Clone)]
pub struct ValueDef {
    pub text: Substr,
    pub name: Name,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub text: Substr,
    pub name: Name,
    pub fields: Vec<(Name, TypeRef)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name(pub Substr);

impl Ast for Name {
    fn text(&self) -> &Substr {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum TypeRef {
    Named(Substr, Name, Vec<TypeRef>),
    Function(Substr, Box<TypeRef>, Box<TypeRef>),
}

impl Ast for TypeRef {
    fn text(&self) -> &Substr {
        match self {
            TypeRef::Named(substr, _, _) => substr,
            TypeRef::Function(substr, _, _) => substr,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    SymbolRef(Substr, Name),
    Func(Substr, Name, TypeRef, Option<TypeRef>, Box<Expr>),
    Call(Substr, Box<Expr>, Box<Expr>),
    IfThenElse(Substr, Box<Expr>, Box<Expr>, Box<Expr>),
    LetIn(Substr, Vec<ValueDef>, Box<Expr>),
    Float(Substr, f64),
    Int(Substr, f64),
}

impl Ast for Expr {
    fn text(&self) -> &Substr {
        match self {
            Expr::SymbolRef(substr, _) => substr,
            Expr::Func(substr, _, _, _, _) => substr,
            Expr::Call(substr, _, _) => substr,
            Expr::IfThenElse(substr, _, _, _) => substr,
            Expr::LetIn(substr, _, _) => substr,
            Expr::Float(substr, _) => substr,
            Expr::Int(substr, _) => substr,
        }
    }
}
