use std::collections::HashMap;

use anyhow::Result;
use inkwell::{builder::Builder, context::Context, module::Module};
use types::{DynType, Struct, TypeLink};
use values::Value;

use crate::ast::{Ast, Def, Name, TypeRef};

mod types;
mod values;

struct CodeGen<'ctx> {
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    decl_info: DeclInfo<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(module_name: &str, decl_info: DeclInfo<'ctx>) -> Self {
        Self {
            module: decl_info.context.create_module(module_name),
            builder: decl_info.context.create_builder(),
            decl_info,
        }
    }

    pub fn resolve_type_ref(&self, type_ref: &TypeRef) -> Result<TypeLink> {
        todo!()
    }
}

struct DeclInfo<'ctx> {
    context: &'ctx Context,
    types: Vec<DynType<'ctx>>,
    values: HashMap<Name, Value<'ctx>>,
}

impl<'ctx> DeclInfo<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            types: Self::init_types(context),
            values: Self::init_values(),
        }
    }

    fn init_types(context: &'ctx Context) -> Vec<DynType<'ctx>> {
        todo!() //TODO: PAIN
    }

    fn init_values() -> HashMap<Name, Value<'ctx>> {
        todo!() //TODO: PAIN
    }

    pub fn populate<'src>(&mut self, ast: &'src Vec<Def>) {
        let mut generic_defs = HashMap::new();

        for def in ast.iter().filter_map(|def| match def {
            Def::Generic(generic_def) => Some(generic_def),
            _ => None,
        }) {
            let def = def.clone();
            generic_defs.insert(def.name, def.args);
        }

        for def in ast.iter().filter_map(|def| match def {
            Def::Type(type_def) => Some(type_def),
            _ => None,
        }) {
            let def = def.clone();
            //TODO: no guarantee of unique names
            self.types.push(Box::new(Struct::new(
                self.context,
                def.name.clone(),
                def.fields,
                generic_defs.get(&def.name),
            )) as DynType<'ctx>)
        }

        for def in ast.iter().filter_map(|def| match def {
            Def::Value(value_def) => Some(value_def),
            _ => None,
        }) {
            //TODO: calculate symbols
        }
    }
}

pub fn compile<'src>(module_name: String, ast: &'src Vec<Def>) -> Result<()> {
    let context = Context::create();
    let mut decl_info = DeclInfo::new(&context);
    decl_info.populate(ast);

    let codegen = CodeGen::new(&module_name, decl_info);

    Ok(())
}
