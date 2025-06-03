use chumsky::{prelude::*, recursive::Indirect, text::ident};

use crate::ast::{self, DynExpr, Field};

pub(crate) fn create<'src>() -> impl Parser<'src, &'src str, Vec<ast::Assignment>, extra::Err<Rich<'src, char>>> {
    assignment().padded_by(whitespace().or_not()).repeated().collect()
}

fn assignment<'src>() -> impl Parser<'src, &'src str, ast::Assignment, extra::Err<Rich<'src, char>>> {
    name()
        .then(just("=").padded_by(whitespace()))
        .then(expr())
        .map(|r| {
            let name = r.0 .0;
            let body = r.1;

            ast::Assignment { name, body }
        })
}

fn expr<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> {
    let mut expr = Recursive::declare();
    expr.define(fn_call(expr.clone()).or(non_call_expr(expr.clone())));

    expr
}

fn non_call_expr<'src>(expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    choice((
        fn_def(expr.clone()),
        field(),
        constant(),
        expr.clone().delimited_by(just('('), just(')')),
    ))
}

fn fn_def<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    name()
        .then_ignore(whitespace())
        .then(type_ref())
        .then_ignore(just("->"))
        .then_ignore(whitespace().or_not())
        .then(type_ref().or_not())
        .then(expr)
        .map(|r| {
            let (((arg_name, arg_type), ret_type), body) = r;

            Box::new(ast::FnDef {
                arg_name,
                arg_type,
                ret_type,
                body,
            }) as DynExpr
        })
}

fn fn_call<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    non_call_expr(expr.clone())
        .then_ignore(whitespace())
        .then(expr)
        .map(|r| {
            Box::new(ast::FnCall {
                func: r.0,
                arg: r.1,
            }) as DynExpr
        })
}

fn field<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    name()
        .map(|r| {
            Box::new(Field {
                name: r
            }) as DynExpr
        })
}

fn constant<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    //TODO: add support for other constants
    text::int(10)
        .from_str::<i64>()
        .unwrapped() // should never fail
        .map(|r| Box::new(r) as DynExpr)
}

fn type_ref<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    just(':').ignore_then(name()).then_ignore(whitespace())
}

fn name<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    ident().map(|r| String::from(r))
}

fn alphanumerical<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    alphabetical().or(numerical())
}

fn alphabetical<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of('a'..='z').or(one_of('A'..='Z'))
}

fn numerical<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of('0'..='9')
}

fn whitespace<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    indent().or(linebreak()).repeated().at_least(1).collect()
}

fn indent<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of(" 	")
}

fn linebreak<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of("\r\n")
}
