use nom::{
    branch::alt,
    bytes::{tag, take_while},
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_opt, map_res, opt},
    multi::{many0, many1},
    number::double,
    sequence::delimited,
    IResult, ParseTo, Parser,
};
use nom_recursive::{recursive_parser, RecursiveInfo};
use util::Span;

use crate::ast::{self, DynExpr, Expr};

mod util;

pub(crate) fn parse(input: &str) -> IResult<Span, Vec<ast::Assignment>> {
    parse_internal(Span(input, RecursiveInfo::new()))
}

fn parse_internal(input: Span) -> IResult<Span, Vec<ast::Assignment>> {
    many0(delimited(opt(whitespace), assignment, opt(whitespace))).parse(input)
}

fn assignment(input: Span) -> IResult<Span, ast::Assignment> {
    map(
        (name, whitespace, tag("="), whitespace, expr, tag(";")),
        |r| ast::Assignment {
            name: r.0,
            body: r.4,
        },
    )
    .parse(input)
}

fn expr(input: Span) -> IResult<Span, DynExpr> {
    alt((fn_def, fn_call, constant)).parse(input)
}

fn constant(input: Span) -> IResult<Span, DynExpr> {
    alt((
        int,
        float,
        // char,
        // string
    )).parse(input)
}

// fn type_def(input: Span) -> IResult<Span, DynExpr> {
//     todo!()
// }

fn fn_def(input: Span) -> IResult<Span, DynExpr> {
    map(
        (
            name,
            whitespace,
            type_ref,
            whitespace,
            tag("->"),
            whitespace,
            opt((type_ref, whitespace)),
            expr,
        ),
        |r| {
            Box::new(ast::FnDef {
                arg_name: r.0,
                arg_type: r.2,
                ret_type: r.6.map(|v| v.0),
                body: r.7,
            }) as DynExpr
        },
    )
    .parse(input)
}

#[recursive_parser]
fn fn_call(input: Span) -> IResult<Span, DynExpr> {
    let (input, left) = expr(input)?;
    let (input, _) = whitespace(input)?;
    let (input, right) = expr(input)?;

    Ok((
        input,
        Box::new(ast::FnCall {
            func: left,
            arg: right,
        }) as DynExpr,
    ))
}

fn int(input: Span) -> IResult<Span, DynExpr> {
    map(
        (map_opt(digit1, |span| Span::parse_to(&span)), opt(tag("i"))),
        |r| Box::<i64>::new(r.0) as DynExpr,
    )
    .parse(input)
}

fn float(input: Span) -> IResult<Span, DynExpr> {
    map((double(), opt(tag("f"))), |r| Box::new(r.0) as DynExpr).parse(input)
}

fn char(input: Span) -> IResult<Span, DynExpr> {
    todo!()
}

fn string(input: Span) -> IResult<Span, DynExpr> {
    todo!()
}

fn type_ref(input: Span) -> IResult<Span, String> {
    delimited(tag(":"), name, whitespace).parse(input)
}

fn name(input: Span) -> IResult<Span, String> {
    map(
        (
            take_while(|c| c == '_' || char::is_alphabetic(c)),
            take_while(|c| c == '_' || char::is_alphanumeric(c)),
        ),
        |r| format!("{}{}", r.0, r.1),
    )
    .parse(input)
}

fn whitespace(input: Span) -> IResult<Span, ()> {
    map(many1(alt((line_ending, space1))), |_r| ()).parse(input)
}
