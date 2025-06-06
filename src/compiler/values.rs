use inkwell::context::Context;

use crate::ast::DynExpr;

use super::types::DynType;

//TODO: big pain (type inference go brrr)
pub struct Value<'ctx> {
    todo: &'ctx str,
}

impl<'ctx> Value<'ctx> {}
