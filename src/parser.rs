use arcstr::Substr;
use chumsky::{pratt::*, prelude::*};

use crate::ast::{self, Ast, Def, Expr, TypeRef};

macro_rules! parser_shell {
    ($v:vis $name:ident, $ret:ty, $code:expr $(, $($arg:ident: $ty:ty),+)?) => {
        $v fn $name<'src, I: chumsky::input::ValueInput<'src, Token = crate::lexer::Token, Span = chumsky::prelude::SimpleSpan>>($($($arg: $ty),+)?) -> $ret {
            $code
        }
    };
}

macro_rules! parser {
    ($v:vis $name:ident, $ret:ty, $code:expr $(, $($arg:ident: $ty:ty),+)?) => {
        parser_shell!($v $name, impl chumsky::prelude::Parser<'src, I, $ret, chumsky::extra::Err<chumsky::prelude::Rich<'src, crate::lexer::Token>>> + Clone, $code $(, $($arg: $ty),+)?);
    };
}

macro_rules! rec_parser {
    ($name:ident, $ret:ty, $this:ident => $code:expr) => {
        parser_shell!(
            $name,
            chumsky::prelude::Recursive<
                chumsky::recursive::Indirect<
                    'src,
                    'src,
                    I,
                    $ret,
                    chumsky::extra::Err<chumsky::prelude::Rich<'src, crate::lexer::Token>>,
                >,
            >,
            {
                let mut $this = chumsky::prelude::Recursive::declare();
                $this.define($code);

                $this
            }
        );
    };
}

macro_rules! rec_child_parser {
    ($name:ident, $ret:ty, $parent:ident: $parent_ret:ty => $code:expr) => {
        parser!($name, $ret, $code, $parent: chumsky::prelude::Recursive<chumsky::recursive::Indirect<'src, 'src, I, $parent_ret, chumsky::extra::Err<chumsky::prelude::Rich<'src, crate::lexer::Token>>>>);
    };
}

macro_rules! token {
    ($kind:ident) => {
        select! {
            crate::lexer::Token {
                kind: crate::lexer::TokenKind::$kind,
                span: s,
            } => s,
        }
    };
}

parser!(
    pub create,
    Vec<Def>,
    choice((
        generic_definition()
            .map(|def| Def::Generic(def)),
        definition(expr())
            .map(|def| Def::Value(def)),
        type_definition()
            .map(|def| Def::Type(def)),
    ))
    .repeated()
    .collect()
    //TODO: I'm tired
    // .map(|defs: Vec<Def>| {
    //     ast::Root {
    //         text: defs.first().map(|f| f.text().range().start..defs.last().unwrap().text().range().end).unwrap_or(0..0),
    //         defs,
    //     }
    // })
    .labelled("definition")
);

parser!(
    generic_definition,
    ast::GenericDef,
    name()
        .then_ignore(token!(DollarSign))
        .then(generic_arg_def().separated_by(token!(Comma)).collect())
        .then(token!(Semicolon))
        .map(|((name, args), end_span)| ast::GenericDef {
            text: end_span
                .parent()
                .substr(name.text().range().start..end_span.range().end),
            name,
            args
        })
        .labelled("generic definition")
);

parser!(
    generic_arg_def,
    (ast::Name, Vec<TypeRef>),
    name()
        .then(type_ref().separated_by(token!(Ampersand)).collect())
        .labelled("generic type argument")
);

rec_child_parser!(
    definition,
    ast::ValueDef,
    expr: Box<Expr> => name()
        .then_ignore(token!(Equal))
        .then(expr)
        .then(token!(Semicolon))
        .map(|((name, body), end_span)| ast::ValueDef { text: end_span.parent().substr(name.text().range().start..end_span.range().end), name, body: *body })
        .labelled("value definition")
);

parser!(
    type_definition,
    ast::TypeDef,
    name()
        .then_ignore(token!(Pipe))
        .then(field_def().separated_by(token!(Comma)).collect())
        .then(token!(Semicolon))
        .map(|((name, fields), end_span)| ast::TypeDef {
            text: end_span
                .parent()
                .substr(name.text().range().start..end_span.range().end),
            name,
            fields
        })
        .labelled("type definition")
);

parser!(
    field_def,
    (ast::Name, TypeRef),
    name().then(type_ref()).labelled("field definition")
);

rec_parser!(
    expr,
    Box<Expr>,
    this => if_then_else(this.clone())
        .or(let_in(this.clone()))
        .map(|expr| Box::new(expr))
        .or(non_call_expr(this.clone()).pratt((
            postfix(2, non_call_expr(this.clone()), |func: Box<Expr>, arg: Box<Expr>, _| {
                Box::new(Expr::Call(func.text().parent().substr(func.text().range().start..arg.text().range().end), func, arg))
            }),
            infix(left(1), token!(PipeInto), |arg: Box<Expr>, _, func: Box<Expr>, _| {
                Box::new(Expr::Call(arg.text().parent().substr(arg.text().range().start..func.text().range().end), arg, func))
            }),
        )))
        .labelled("expression")
);

rec_child_parser!(
    non_call_expr,
    Box<Expr>,
    expr: Box<Expr> => choice((
        fn_def(expr.clone()),
        constant(),
        literal(),
    )).map(|expr| Box::new(expr))
    .or(choice((
        token!(PipeFrom).ignore_then(expr.clone()),
        expr.clone().delimited_by(token!(OpenParen), token!(CloseParen)),
    )))
);

rec_child_parser!(
    fn_def,
    Expr,
    expr: Box<Expr> => name()
        .then(type_ref())
        .then_ignore(token!(Arrow))
        .then(type_ref().or_not())
        .then(expr)
        .map(|(((arg_name, arg_type), ret_type), body)| {
            Expr::Func(
                arg_name.text().parent().substr(arg_name.text().range().start..body.text().range().end),
                arg_name,
                arg_type,
                ret_type,
                body,
            )
        })
        .labelled("function definition")
);

rec_child_parser!(
    if_then_else,
    Expr,
    expr: Box<Expr> => token!(If)
        .then(expr.clone())
        .then_ignore(token!(Then))
        .then(expr.clone())
        .then_ignore(token!(Else))
        .then(expr)
        .map(|(((start_span, condition_expr), then_expr), else_expr)| {
            Expr::IfThenElse(
                start_span.parent().substr(start_span.range().start..else_expr.text().range().end),
                condition_expr,
                then_expr,
                else_expr,
            )
        })
        .labelled("branching expression")
);

rec_child_parser!(
    let_in,
    Expr,
    expr: Box<Expr> => token!(Let)
        .then(definition(expr.clone()).repeated().collect())
        .then_ignore(token!(In))
        .then(expr)
        .map(|((start_span, defs), body)| {
            Expr::LetIn(
                start_span.parent().substr(start_span.range().start..body.text().range().end),
                defs,
                body,
            )
        })
        .labelled("let expression")
);

//TODO: probably don't need this
parser!(
    constant,
    Expr,
    name()
        .map(|name| Expr::SymbolRef(name.text().clone(), name))
        .labelled("name reference")
);

parser!(
    literal,
    Expr,
    //TODO: add support for other literals
    number().labelled("literal")
);

parser!(
    number,
    Expr,
    choice((
        token!(Float)
            .map(|s| Expr::Float(s.clone(), s.parse().unwrap()))
            .labelled("float literal"),
        token!(Int)
            .map(|s| Expr::Int(s.clone(), s.parse().unwrap()))
            .labelled("int literal"),
    ))
    .labelled("number literal")
);

parser!(
    type_ref,
    TypeRef,
    token!(Colon)
        .ignore_then(
            name()
                .map(|r| TypeRef::Named(r.text().clone(), r, Vec::new()))
                .or(token!(OpenParen)
                    .ignore_then(inner_type_ref())
                    .then_ignore(token!(CloseParen)))
        )
        .labelled("type reference")
);

rec_parser!(
    inner_type_ref,
    TypeRef,
    this => non_fn_inner_type_ref(this.clone()).pratt(infix(
        right(1),
        token!(Arrow),
        |arg: TypeRef, _, ret: TypeRef, _| TypeRef::Function(arg.text().parent().substr(arg.text().range().start..ret.text().range().end), Box::new(arg), Box::new(ret)),
    ))
);

rec_child_parser!(
    non_fn_inner_type_ref,
    TypeRef,
    type_ref: TypeRef => recursive(|this| {
        choice((
            name()
                .then(this.clone().repeated().collect())
                .map(|(name, type_args)| {
                    let type_args: Vec<TypeRef> = type_args;
                    let end = type_args.last()
                        .map(|t| t.text().range().end)
                        .unwrap_or(name.text().range().end);

                    TypeRef::Named(name.text().parent().substr(name.text().range().start..end), name, type_args)
                }),
            type_ref
                .clone()
                .delimited_by(token!(OpenParen), token!(CloseParen)),
        ))
    })
);

parser!(
    name,
    ast::Name,
    token!(Name).map(ast::Name).labelled("name")
);
