use chumsky::{input::ValueInput, pratt::*, prelude::*, recursive::Indirect};

use crate::{ast::{self, DynDef, DynExpr, TypeRef}, lexer::Token, util::Span};

pub(crate) fn create<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, Vec<DynDef>, extra::Err<Rich<'src, Token<'src>>>> {
    choice((
        generic_definition(),
        definition(),
    )).repeated().collect()
}

fn generic_definition<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynDef, extra::Err<Rich<'src, Token<'src>>>> {
    name()
        .then_ignore(just(Token::DollarSign))
        .then(generic_arg_def().separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::Semicolon))
        .map(|r| {
            Box::new(ast::GenericAssignment {
                name: r.0,
                args: r.1,
            }) as DynDef
        })
        .labelled("generic definition")
}

fn generic_arg_def<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, (String, Vec<TypeRef>), extra::Err<Rich<'src, Token<'src>>>> {
    name()
        .then(type_ref().separated_by(just(Token::Ampersand)).collect())
        .labelled("generic argument")
}

fn definition<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynDef, extra::Err<Rich<'src, Token<'src>>>>
{
    name()
        .then_ignore(just(Token::Equal))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map(|r| {
            Box::new(ast::Assignment {
                name: r.0,
                body: r.1,
            }) as DynDef
        })
        .labelled("name definition")
}

fn expr<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> {
    let mut expr = Recursive::declare();
    expr.define(if_then_else(expr.clone()).or(non_call_expr(expr.clone()).pratt((
        postfix(2, non_call_expr(expr.clone()), |func, arg, _| {
            Box::new(ast::FnCall {
                func,
                arg,
            }) as DynExpr
        }),
        infix(left(1), just(Token::PipeInto), |arg, _, func, _| {
            Box::new(ast::FnCall {
                func,
                arg,
            }) as DynExpr
        }),
    ))).labelled("expression"));

    expr
}

fn non_call_expr<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    expr: Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    choice((
        fn_def(expr.clone()),
        constant(),
        literal(),
        just(Token::PipeFrom).ignore_then(expr.clone()),
        expr.clone().delimited_by(
            just(Token::OpenParen),
            just(Token::CloseParen),
        ),
    ))
}

fn fn_def<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    expr: Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .then(type_ref())
        .then_ignore(just(Token::Arrow))
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
        .labelled("function definition")
}

fn if_then_else<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    expr: Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    just(Token::If)
        .ignore_then(expr.clone())
        .then_ignore(just(Token::Then))
        .then(expr.clone())
        .then_ignore(just(Token::Else))
        .then(expr)
        .map(|r| {
            let ((condition_expr, then_expr), else_expr) = r;

            Box::new(ast::IfThenElse {
                condition_expr,
                then_expr,
                else_expr,
            }) as DynExpr
        })
        .labelled("branching expression")
}

fn constant<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name().map(|name| Box::new(ast::Constant { name }) as DynExpr).labelled("name reference")
}

fn literal<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    //TODO: add support for other literals
    choice((
        float(),
        int(),
    )).labelled("literal")
}

fn int<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::IntLiteral(i) => Box::new(i) as DynExpr }.labelled("int literal")
}

fn float<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::FloatLiteral(f) => Box::new(f) as DynExpr }.labelled("float literal")
}

fn type_ref<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    just(Token::Colon).ignore_then(name().map(|r| TypeRef::Named(r)).or(inner_type_ref())).labelled("type reference")
}

fn inner_type_ref<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    todo() //TODO: functions and named type refs with generic args
}

fn name<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>() -> impl Parser<'src, I, String, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::Name(name) => name }.map(|r| r.to_string()).labelled("name")
}
