use chumsky::{prelude::*, pratt::*};

use crate::ast::{self, Def, DynExpr, TypeRef};

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
        parser_shell!($name, chumsky::prelude::Recursive<chumsky::recursive::Indirect<'src, 'src, I, $ret, chumsky::extra::Err<chumsky::prelude::Rich<'src, crate::lexer::Token>>>>, {
            let mut $this = chumsky::prelude::Recursive::declare();
            $this.define($code);

            $this
        });
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
        generic_definition(),
        definition(expr())
            .map(|def| Def::Value(def)),
        type_definition()
    ))
    .repeated()
    .collect()
    .labelled("definition")
);

parser!(
    generic_definition,
    Def,
    name()
        .then_ignore(token!(DollarSign))
        .then(generic_arg_def().separated_by(token!(Comma)).collect())
        .then_ignore(token!(Semicolon))
        .map(|(name, args)| Def::Generic(ast::GenericDef { name, args }))
        .labelled("generic definition")
);

parser!(
    generic_arg_def,
    (String, Vec<TypeRef>),
    name()
        .then(type_ref().separated_by(token!(Ampersand)).collect())
        .labelled("generic type argument")
);

rec_child_parser!(
    definition,
    ast::ValueDef,
    expr: DynExpr => name()
        .then_ignore(token!(Equal))
        .then(expr)
        .then_ignore(token!(Semicolon))
        .map(|(name, body)| ast::ValueDef { name, body })
        .labelled("value definition")
);

parser!(
    type_definition,
    Def,
    name()
        .then_ignore(token!(Pipe))
        .then(field_def().separated_by(token!(Comma)).collect())
        .then_ignore(token!(Semicolon))
        .map(|(name, fields)| Def::Type(ast::TypeDef { name, fields }))
        .labelled("type definition")
);

parser!(
    field_def,
    (String, TypeRef),
    name().then(type_ref()).labelled("field definition")
);

rec_parser!(
    expr,
    DynExpr,
    this => if_then_else(this.clone())
        .or(let_in(this.clone()))
        .or(non_call_expr(this.clone()).pratt((
            postfix(2, non_call_expr(this.clone()), |func, arg, _| {
                Box::new(ast::Call { func, arg }) as DynExpr
            }),
            infix(left(1), token!(PipeInto), |arg, _, func, _| {
                Box::new(ast::Call { func, arg }) as DynExpr
            }),
        )))
        .labelled("expression")
);

rec_child_parser!(
    non_call_expr,
    DynExpr,
    expr: DynExpr => choice((
        fn_def(expr.clone()),
        constant(),
        literal(),
        token!(PipeFrom).ignore_then(expr.clone()),
        expr.clone().delimited_by(token!(OpenParen), token!(CloseParen)),
    ))
);

rec_child_parser!(
    fn_def,
    DynExpr,
    expr: DynExpr => name()
        .then(type_ref())
        .then_ignore(token!(Arrow))
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
);

rec_child_parser!(
    if_then_else,
    DynExpr,
    expr: DynExpr => token!(If)
        .ignore_then(expr.clone())
        .then_ignore(token!(Then))
        .then(expr.clone())
        .then_ignore(token!(Else))
        .then(expr)
        .map(|((condition_expr, then_expr), else_expr)| {
            Box::new(ast::IfThenElse {
                condition_expr,
                then_expr,
                else_expr,
            }) as DynExpr
        })
        .labelled("branching expression")
);

rec_child_parser!(
    let_in,
    DynExpr,
    expr: DynExpr => token!(Let)
        .ignore_then(definition(expr.clone()).repeated().collect())
        .then_ignore(token!(In))
        .then(expr)
        .map(|(defs, body)| {
            Box::new(ast::LetIn {
                defs,
                body,
            }) as DynExpr
        })
        .labelled("let expression")
);

parser!(
    constant,
    DynExpr,
    name()
        .map(|name| Box::new(ast::Constant { name }) as DynExpr)
        .labelled("name reference")
);

parser!(
    literal,
    DynExpr,
    //TODO: add support for other literals
    number().labelled("literal")
);

parser!(
    number,
    DynExpr,
    choice((
        token!(Float).map(|s| Box::new(s.parse::<f64>().unwrap()) as DynExpr).labelled("float literal"),
        token!(Int).map(|s| Box::new(s.parse::<i64>().unwrap()) as DynExpr).labelled("int literal"),
    )).labelled("number literal")
);

parser!(
    type_ref,
    TypeRef,
    token!(Colon)
        .ignore_then(
            name()
                .map(|r| TypeRef::Named(r, Vec::new()))
                .or(inner_type_ref().delimited_by(token!(OpenParen), token!(CloseParen))),
        )
        .labelled("type reference")
);

rec_parser!(
    inner_type_ref,
    TypeRef,
    this => non_fn_inner_type_ref(this.clone()).pratt(infix(
        right(1),
        token!(Arrow),
        |arg, _, ret, _| TypeRef::Function(Box::new(arg), Box::new(ret)),
    ))
);

rec_child_parser!(
    non_fn_inner_type_ref,
    TypeRef,
    type_ref: TypeRef => recursive(|this| {
        choice((
            name()
                .then(this.clone().repeated().collect())
                .map(|(name, type_args)| TypeRef::Named(name, type_args)),
            type_ref
                .clone()
                .delimited_by(token!(OpenParen), token!(CloseParen)),
        ))
    })
);

parser!(
    name,
    String,
    token!(Name)
        .map(|s| s.to_string())
        .labelled("name")
);
