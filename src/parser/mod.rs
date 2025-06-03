use chumsky::{extra::ParserExtra, prelude::*, recursive::Indirect};
use once_cell::sync::OnceCell;

use crate::ast::{self, DynExpr, Expr};

pub(crate) fn parser<'src>() -> impl Parser<'src, &'src str, Vec<ast::Assignment>> {
    assignment().padded_by(whitespace()).repeated().collect()
}

fn assignment<'src>() -> impl Parser<'src, &'src str, ast::Assignment> {
    name()
        .then(just("=").padded_by(whitespace()))
        .then(expr())
        .map(|r| {
            let name = r.0 .0;
            let body = r.1;

            ast::Assignment {
                name,
                body
            }
        })
}

fn expr<'src>() -> impl Parser<'src, &'src str, DynExpr> {
    recursive(|expr|
        choice((
            fn_def(expr.clone()),
            fn_call(expr.clone()),
            constant(),
            expr.delimited_by(just('('), just(')'))
        ))
    )
}

fn fn_def<'src>(expr: Recursive<dyn Parser<'src, &'src str, DynExpr>>) -> impl Parser<'src, &'src str, DynExpr> + Clone {
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

fn fn_call<'src>(expr: Recursive<dyn Parser<'src, &'src str, DynExpr>>) -> impl Parser<'src, &'src str, DynExpr> + Clone {
    todo()
}

fn constant<'src>() -> impl Parser<'src, &'src str, DynExpr> + Clone {
    todo()
}

fn type_ref<'src>() -> impl Parser<'src, &'src str, String> {
    just(':')
        .ignore_then(name())
        .then_ignore(whitespace())
}

fn name<'src>() -> impl Parser<'src, &'src str, String> {
    just('_')
        .or(alphabetical())
        .repeated()
        .collect::<String>()
        .then(
            just('_')
                .or(alphanumerical())
                .repeated()
                .collect::<String>(),
        )
        .map(|r| format!("{}{}", r.0, r.1))
}

fn alphanumerical<'src>() -> impl Parser<'src, &'src str, char> {
    alphabetical().or(numerical())
}

fn alphabetical<'src>() -> impl Parser<'src, &'src str, char> {
    one_of('a'..='z').or(one_of('A'..='Z'))
}

fn numerical<'src>() -> impl Parser<'src, &'src str, char> {
    one_of('0'..='9')
}

fn whitespace<'src>() -> impl Parser<'src, &'src str, ()> {
    indent().or(linebreak()).repeated().at_least(1)
}

fn indent<'src>() -> impl Parser<'src, &'src str, char> {
    one_of(" 	")
}

fn linebreak<'src>() -> impl Parser<'src, &'src str, char> {
    one_of("\r\n")
}
