use chumsky::{input::ValueInput, pratt::*, prelude::*, recursive::Indirect, text::*};

use crate::ast::{self, DynDef, DynExpr, TypeRef};

pub(crate) fn create<'src>(
) -> impl Parser<'src, &'src str, Vec<DynDef>, extra::Err<Rich<'src, char>>> {
    choice((
        generic_definition(),
        definition(),
    )).padded().repeated().collect()
}

fn definition<'src>() -> impl Parser<'src, &'src str, DynDef, extra::Err<Rich<'src, char>>>
{
    name()
        .then_ignore(just('=').padded())
        .then(expr().padded())
        .then_ignore(just(';').padded())
        .map(|r| {
            Box::new(ast::Assignment {
                name: r.0,
                body: r.1,
            }) as DynDef
        })
        .labelled("definition")
}

fn generic_definition<'src>() -> impl Parser<'src, &'src str, DynDef, extra::Err<Rich<'src, char>>> {
    name()
        .then_ignore(just('$').padded())
        .then(generic_arg_def().padded().separated_by(just(',')).collect())
        .then_ignore(just(';').padded())
        .map(|r| {
            Box::new(ast::GenericAssignment {
                name: r.0,
                args: r.1,
            }) as DynDef
        })
        .labelled("generic definition")
}

fn generic_arg_def<'src>() -> impl Parser<'src, &'src str, (String, Vec<TypeRef>), extra::Err<Rich<'src, char>>> {
    name()
        .then(type_ref().padded().separated_by(just('&')).collect())
        .labelled("generic argument")
}

fn expr<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> {
    let mut expr = Recursive::declare();
    expr.define(non_call_expr(expr.clone()).pratt((
        infix(left(2), whitespace(), |func, _, arg, _| {
            Box::new(ast::FnCall {
                func,
                arg,
            }) as DynExpr
        }),
        infix(left(1), just("|>").padded(), |arg, _, func, _| {
            Box::new(ast::FnCall {
                func,
                arg,
            }) as DynExpr
        }),
    )).labelled("expression"));

    expr
}

fn non_call_expr<'src>(
    expr: Recursive<Indirect<'src, 'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>>>,
) -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    choice((
        fn_def(expr.clone()),
        constant(),
        literal(),
        just("<|").ignore_then(whitespace().or_not()).ignore_then(expr.clone()),
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

fn constant<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    name().map(|name| Box::new(ast::Constant { name }) as DynExpr)
}

fn literal<'src>() -> impl Parser<'src, &'src str, DynExpr, extra::Err<Rich<'src, char>>> + Clone {
    //TODO: add support for other literals
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

fn type_ref<'src>() -> impl Parser<'src, &'src str, TypeRef, extra::Err<Rich<'src, char>>> + Clone {
    just(':').ignore_then(name().map(|r| TypeRef::Named(r)).or(inner_type_ref()))
}

fn inner_type_ref<'src>() -> impl Parser<'src, &'src str, TypeRef, extra::Err<Rich<'src, char>>> + Clone {
    todo() //TODO: functions and named type refs with generic args
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
