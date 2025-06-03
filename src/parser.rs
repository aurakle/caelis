use chumsky::{input::ValueInput, prelude::*, recursive::Indirect, text::{ident, keyword}};

use crate::ast::{self, DynExpr};

pub(crate) fn create<'src>(
) -> impl Parser<'src, &'src str, Vec<ast::Assignment>, extra::Err<Rich<'src, char>>> {
    assignment().padded().repeated().collect()
}

fn assignment<'src>() -> impl Parser<'src, &'src str, ast::Assignment, extra::Err<Rich<'src, char>>>
{
    name()
        .then_ignore(just('=').padded())
        .then(expr().padded())
        .then_ignore(just(';').padded())
        .map(|r| {
            let name = r.0;
            let body = r.1;

            ast::Assignment { name, body }
        })
        .labelled("definition")
}

fn expr<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> {
    let mut expr = Recursive::declare();
    expr.define(choice((
        fn_def(expr.clone()),
        fn_call(expr.clone()),
        non_call_expr(expr.clone()),
    )).labelled("expression"));

    expr
}

fn non_call_expr<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    choice((
        constant(),
        literal(),
        expr.clone().delimited_by(
            just('(').then(whitespace().or_not()),
            whitespace().or_not().then(just(')')),
        ),
    ))
}

fn fn_def<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    name()
        .then_ignore(whitespace())
        .then(type_ref())
        .then_ignore(just("->").padded())
        .then(type_ref().or_not())
        .then(expr.padded())
        .map(|r| {
            let (((arg_name, arg_type), ret_type), body) = r;

            Box::new(ast::FnDef {
                arg_name,
                arg_type,
                ret_type,
                body,
            }) as DynExpr
        })
        .labelled("function definition")
}

fn fn_call<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    non_call_expr(expr.clone()).foldl(
        whitespace().ignore_then(non_call_expr(expr)).repeated(),
        |left, right| {
            Box::new(ast::FnCall {
                func: left,
                arg: right,
            }) as DynExpr
        },
    ).labelled("function application")
}

fn constant<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    name().map(|r| Box::new(ast::Constant { name: r }) as DynExpr)
}

fn literal<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    //TODO: add support for other constants
    choice((
        float(),
        int(),
    ))
}

fn int<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    text::int(10)
        .padded()
        .from_str::<i64>()
        .unwrapped() // should never fail
        .map(|r| Box::new(r) as DynExpr)
}

fn float<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    text::int(10)
        .then(just('.')
        .then(text::digits(10)).or_not())
        .to_slice()
        .from_str::<f64>()
        .unwrapped() // should never fail
        .map(|r| Box::new(r) as DynExpr)
}

fn type_ref<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    just(':').ignore_then(name()).then_ignore(whitespace())
}

fn name<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone {
    ident().map(|r| String::from(r))
}

fn whitespace<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> + Clone
{
    indent().or(linebreak()).repeated().at_least(1).collect()
}

fn indent<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of(" 	")
}

fn linebreak<'src>() -> impl Parser<'src, &'src str, char, extra::Err<Rich<'src, char>>> + Clone {
    one_of("\r\n")
}
