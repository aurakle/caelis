use std::fmt::Display;

use chumsky::{prelude::*, text::*};

use crate::util::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Let,
    If,
    Then,
    Else,
    Arrow,
    PipeInto,
    PipeFrom,
    DollarSign,
    Ampersand,
    Equal,
    Colon,
    Semicolon,
    Period,
    Comma,
    OpenParen,
    CloseParen,
    Name(&'src str),
    FloatLiteral(f64),
    IntLiteral(i64),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::Arrow => write!(f, "->"),
            Token::PipeInto => write!(f, "|>"),
            Token::PipeFrom => write!(f, "<|"),
            Token::DollarSign => write!(f, "$"),
            Token::Ampersand => write!(f, "&"),
            Token::Equal => write!(f, "="),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Name(s) => write!(f, "{s}"),
            Token::FloatLiteral(float) => write!(f, "{float}"),
            Token::IntLiteral(int) => write!(f, "{int}"),
        }
    }
}

pub(crate) fn create<'src>() -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char>>> {
    choice((
        just("let").to(Token::Let),
        just("if").to(Token::If),
        just("then").to(Token::Then),
        just("else").to(Token::Else),
        just("->").to(Token::Arrow),
        just("|>").to(Token::PipeInto),
        just("<|").to(Token::PipeFrom),
        just('$').to(Token::DollarSign),
        just('&').to(Token::Ampersand),
        just('=').to(Token::Equal),
        just(':').to(Token::Colon),
        just(';').to(Token::Semicolon),
        just('.').to(Token::Period),
        just(',').to(Token::Comma),
        just('(').to(Token::OpenParen),
        just(')').to(Token::CloseParen),
        ident().map(Token::Name),
        int(10)
            .then(just('.')
                .then(text::digits(10)).or_not())
            .to_slice()
            .from_str()
            .unwrapped()
            .map(Token::FloatLiteral),
        int(10)
            .to_slice()
            .from_str()
            .unwrapped()
            .map(Token::IntLiteral),
    )).map_with(|token, info| (token, info.span())).padded().repeated().collect()
}
