use std::fmt::Display;

use arcstr::{ArcStr, Substr};
use chumsky::{
    input::{StrInput, ValueInput},
    prelude::*,
    text::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Substr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    In,
    If,
    Then,
    Else,
    Arrow,
    PipeInto,
    PipeFrom,
    DollarSign,
    Ampersand,
    Pipe,
    Equal,
    Colon,
    Semicolon,
    Period,
    Comma,
    OpenParen,
    CloseParen,
    Name,
    Float,
    Int,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Let => write!(f, "let"),
            TokenKind::In => write!(f, "in"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Then => write!(f, "then"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::PipeInto => write!(f, "|>"),
            TokenKind::PipeFrom => write!(f, "<|"),
            TokenKind::DollarSign => write!(f, "$"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Period => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),
            TokenKind::Name => write!(f, "name"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Int => write!(f, "int"),
        }
    }
}

pub fn tokenize<'src>(text: &'src ArcStr) -> (Option<Vec<Token>>, Vec<Rich<'src, char>>) {
    create(text).parse(text.as_str()).into_output_errors()
}

// this only exists for coercion and should only ever be used by `tokenize`
fn create<
    'src,
    I: ValueInput<'src, Token = char, Span = SimpleSpan>
        + StrInput<'src, Slice = &'src str, Span = SimpleSpan>,
>(
    text: &'src ArcStr,
) -> impl Parser<'src, I, Vec<Token>, extra::Err<Rich<'src, char>>> {
    choice((
        just("let").to(TokenKind::Let),
        just("in").to(TokenKind::In),
        just("if").to(TokenKind::If),
        just("then").to(TokenKind::Then),
        just("else").to(TokenKind::Else),
        just("->").to(TokenKind::Arrow),
        just("|>").to(TokenKind::PipeInto),
        just("<|").to(TokenKind::PipeFrom),
        just('$').to(TokenKind::DollarSign),
        just('&').to(TokenKind::Ampersand),
        just('|').to(TokenKind::Pipe),
        just('=').to(TokenKind::Equal),
        just(':').to(TokenKind::Colon),
        just(';').to(TokenKind::Semicolon),
        just('.').to(TokenKind::Period),
        just(',').to(TokenKind::Comma),
        just('(').to(TokenKind::OpenParen),
        just(')').to(TokenKind::CloseParen),
        ident().to(TokenKind::Name),
        int(10)
            .then(just('.').then(text::digits(10)).or_not())
            .to(TokenKind::Float),
        int(10).to(TokenKind::Int),
    ))
    .map_with(|kind, info| {
        let SimpleSpan {
            start,
            end,
            context: _,
        } = info.span();

        Token {
            kind,
            span: text.substr(start..end),
        }
    })
    .padded_by(
        just('#')
            .then(any().and_is(just('\n').not()).repeated())
            .padded()
            .repeated(),
    )
    .padded()
    .repeated()
    .collect()
}
