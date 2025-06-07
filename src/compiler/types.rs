use inkwell::{
    context::Context,
    types::{BasicType, BasicTypeEnum, StructType},
};
use std::fmt::Debug;

use crate::ast::TypeRef;

use super::CodeGen;

pub type DynType<'ctx> = Box<dyn Type<'ctx> + 'ctx>;

pub trait Type<'ctx>: Debug {
    fn llvm_type(&self, codegen: &CodeGen<'ctx>) -> BasicTypeEnum<'ctx>;
}

#[derive(Debug, Clone)]
pub struct TypeLink(usize);

// this is a "shadow" type which pretends to be a real type but actually just proxies for the type
// at the index it has. magic!
impl<'ctx> Type<'ctx> for TypeLink {
    fn llvm_type(&self, codegen: &CodeGen<'ctx>) -> BasicTypeEnum<'ctx> {
        codegen
            .decl_info
            .types
            .get(self.0)
            .unwrap() // if this fails we're screwed
            .llvm_type(codegen)
    }
}

#[derive(Debug, Clone)]
pub struct Struct<'ctx> {
    pub name: String,
    pub llvm_struct: StructType<'ctx>,
    pub fields: StructFields,
}

#[derive(Debug, Clone)]
pub enum StructFields {
    Unresolved(Vec<(String, TypeRef)>),
    Resolved(Vec<(String, TypeLink)>),
}

impl<'ctx> Struct<'ctx> {
    pub fn new(context: &'ctx Context, name: String, fields: Vec<(String, TypeRef)>, generic_args: Option<&Vec<(String, Vec<TypeRef>)>>) -> Self {
        //TODO: support generics
        Self {
            name: name.clone(),
            llvm_struct: context.opaque_struct_type(&name),
            fields: StructFields::Unresolved(fields),
        }
    }
}

impl<'ctx> Type<'ctx> for Struct<'ctx> {
    fn llvm_type(&self, codegen: &CodeGen<'ctx>) -> BasicTypeEnum<'ctx> {
        self.llvm_struct.as_basic_type_enum()
    }
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    F64,
    I64,
}

impl<'ctx> Type<'ctx> for PrimitiveType {
    fn llvm_type(&self, codegen: &CodeGen<'ctx>) -> BasicTypeEnum<'ctx> {
        match self {
            PrimitiveType::F64 => codegen.decl_info.context.f64_type().as_basic_type_enum(),
            PrimitiveType::I64 => codegen.decl_info.context.i64_type().as_basic_type_enum(),
        }
    }
}
