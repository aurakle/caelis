use chumsky::{input::ValueInput, pratt::*, prelude::*, recursive::Indirect};

use crate::{
    ast::{self, DynDef, DynExpr, TypeRef},
    lexer::Token,
    util::Span,
};

pub(crate) fn create<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, Vec<DynDef>, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    choice((generic_definition(), definition(expr()).map(|def| Box::new(def) as DynDef), type_definition()))
        .repeated()
        .collect()
}

fn generic_definition<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynDef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .then_ignore(just(Token::DollarSign))
        .then(generic_arg_def().separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::Semicolon))
        .map(|(name, args)| Box::new(ast::GenericDef { name, args }) as DynDef)
        .labelled("generic definition")
}

fn generic_arg_def<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, (String, Vec<TypeRef>), extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .then(type_ref().separated_by(just(Token::Ampersand)).collect())
        .labelled("generic type argument")
}

fn definition<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    expr: Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, ast::ValueDef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .then_ignore(just(Token::Equal))
        .then(expr)
        .then_ignore(just(Token::Semicolon))
        .map(|(name, body)| ast::ValueDef { name, body })
        .labelled("value definition")
}

fn type_definition<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynDef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .then_ignore(just(Token::Pipe))
        .then(field_def().separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::Semicolon))
        .map(|(name, fields)| Box::new(ast::TypeDef { name, fields }) as DynDef)
        .labelled("type definition")
}

fn field_def<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, (String, TypeRef), extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name().then(type_ref()).labelled("field definition")
}

fn expr<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>> {
    let mut expr = Recursive::declare();
    expr.define(
        if_then_else(expr.clone())
            .or(let_in(expr.clone()))
            .or(non_call_expr(expr.clone()).pratt((
                postfix(2, non_call_expr(expr.clone()), |func, arg, _| {
                    Box::new(ast::Call { func, arg }) as DynExpr
                }),
                infix(left(1), just(Token::PipeInto), |arg, _, func, _| {
                    Box::new(ast::Call { func, arg }) as DynExpr
                }),
            )))
            .labelled("expression"),
    );

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
        expr.clone()
            .delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
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
        .map(|(((arg_name, arg_type), ret_type), body)| {
            Box::new(ast::Func {
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
        .map(|((condition_expr, then_expr), else_expr)| {
            Box::new(ast::IfThenElse {
                condition_expr,
                then_expr,
                else_expr,
            }) as DynExpr
        })
        .labelled("branching expression")
}

fn let_in<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    expr: Recursive<Indirect<'src, 'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    just(Token::Let)
        .ignore_then(definition(expr.clone()).repeated().collect())
        .then_ignore(just(Token::In))
        .then(expr)
        .map(|(defs, body)| {
            Box::new(ast::LetIn {
                defs,
                body,
            }) as DynExpr
        })
        .labelled("let expression")
}

fn constant<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    name()
        .map(|name| Box::new(ast::Constant { name }) as DynExpr)
        .labelled("name reference")
}

fn literal<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    //TODO: add support for other literals
    choice((float(), int())).labelled("literal")
}

fn int<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::IntLiteral(i) => Box::new(i) as DynExpr }.labelled("int literal")
}

fn float<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, DynExpr, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::FloatLiteral(f) => Box::new(f) as DynExpr }.labelled("float literal")
}

fn type_ref<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    just(Token::Colon)
        .ignore_then(
            name()
                .map(|r| TypeRef::Named(r, Vec::new()))
                .or(inner_type_ref().delimited_by(just(Token::OpenParen), just(Token::CloseParen))),
        )
        .labelled("type reference")
}

fn inner_type_ref<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    let mut inner_type_ref = Recursive::declare();
    inner_type_ref.define(non_fn_inner_type_ref(inner_type_ref.clone()).pratt((infix(
        right(1),
        just(Token::Arrow),
        |arg, _, ret, _| TypeRef::Function(Box::new(arg), Box::new(ret)),
    ),)));

    inner_type_ref
}

fn non_fn_inner_type_ref<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
    type_ref: Recursive<Indirect<'src, 'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>>>,
) -> impl Parser<'src, I, TypeRef, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    recursive(|this| {
        choice((
            name()
                .then(this.clone().repeated().collect())
                .map(|(name, type_args)| TypeRef::Named(name, type_args)),
            type_ref
                .clone()
                .delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
        ))
    })
}

fn name<'src, I: ValueInput<'src, Token = Token<'src>, Span = Span>>(
) -> impl Parser<'src, I, String, extra::Err<Rich<'src, Token<'src>>>> + Clone {
    select! { Token::Name(name) => name }
        .map(|r| r.to_string())
        .labelled("name")
}
