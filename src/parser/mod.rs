use chumsky::{prelude::*, recursive::Indirect};

use crate::ast::{self, DynExpr};

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

            ast::Assignment { name, body }
        })
}

fn expr<'src>() -> impl Parser<'src, &'src str, DynExpr> {
    let mut expr = Recursive::declare();
    expr.define(choice((
        fn_def(expr.clone()),
        fn_call(expr.clone()),
        constant(),
        expr.clone().delimited_by(just('('), just(')')),
    )));

    expr
}

fn fn_def<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Default>>,
) -> impl Parser<'src, &'src str, DynExpr> + Clone {
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
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Default>>,
) -> impl Parser<'src, &'src str, DynExpr> + Clone {
    todo()
}

fn constant<'src>() -> impl Parser<'src, &'src str, DynExpr> + Clone {
    todo()
}

fn type_ref<'src>() -> impl Parser<'src, &'src str, String> + Clone {
    just(':').ignore_then(name()).then_ignore(whitespace())
}

fn name<'src>() -> impl Parser<'src, &'src str, String> + Clone {
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

fn alphanumerical<'src>() -> impl Parser<'src, &'src str, char> + Clone {
    alphabetical().or(numerical())
}

fn alphabetical<'src>() -> impl Parser<'src, &'src str, char> + Clone {
    one_of('a'..='z').or(one_of('A'..='Z'))
}

fn numerical<'src>() -> impl Parser<'src, &'src str, char> + Clone {
    one_of('0'..='9')
}

fn whitespace<'src>() -> impl Parser<'src, &'src str, String> + Clone {
    indent().or(linebreak()).repeated().at_least(1).collect()
}

fn indent<'src>() -> impl Parser<'src, &'src str, char> + Clone {
    one_of(" 	")
}

fn linebreak<'src>() -> impl Parser<'src, &'src str, char> + Clone {
    one_of("\r\n")
}
