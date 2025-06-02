use nom::{branch::alt, bytes::tag, character::{complete::{crlf, line_ending, space1}, one_of}, combinator::{map, opt}, multi::{many0, many1}, IResult, Parser};

use crate::ast::{self, DynExpr, Expr};

pub(crate) fn parse(input: &str) -> IResult<&str, Vec<DynExpr>> {
    many0(alt((
        fn_def,
        type_def
    ))).parse(input)
}

fn expr(input: &str) -> IResult<&str, DynExpr> {
    alt((
        fn_def,
        fn_call,
        constant
    )).parse(input)
}

fn constant(input: &str) -> IResult<&str, DynExpr> {
    alt((
        i32,
        i64,
        f32,
        f64,
        char,
        string
    )).parse(input)
}

fn fn_call(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn type_def(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn fn_def(input: &str) -> IResult<&str, DynExpr> {
    map(
        (tag("def"), whitespace, name, whitespace, opt((type_ref, whitespace)), arg_def, whitespace, tag("->"), whitespace, expr),
        |r| {
            let name = r.2;
            let ret_type = r.4.map(|v| v.0);
            let arg_def = r.5;
            let body = r.9;

            Box::new(ast::FnDef(name, ret_type, arg_def, body)) as DynExpr
        }
    ).parse(input)
}

fn arg_def(input: &str) -> IResult<&str, ast::ArgDef> {
    todo!()
}

fn type_ref(input: &str) -> IResult<&str, ast::TypeRef> {
    map(
        (tag(":"), name, whitespace),
        |r| ast::TypeRef(r.1)
    ).parse(input)
}

fn name(input: &str) -> IResult<&str, ast::Name> {
    map(
        (many1(one_of("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")),
            many0(one_of("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890$"))),
        |r| {
            let mut v = Vec::new();
            v.extend(r.1.iter().cloned());
            ast::Name(v.iter().collect())
        }
    ).parse(input)
}

fn i32(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn i64(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn f32(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn f64(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn char(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn string(input: &str) -> IResult<&str, DynExpr> {
    todo!()
}

fn whitespace(input: &str) -> IResult<&str, ()> {
    map(
        many1(alt((crlf, line_ending, space1))),
        |r| ()
    ).parse(input)
}
