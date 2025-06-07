use std::{env, fs};

use arcstr::ArcStr;
use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::Parser;

mod ast;
mod compiler;
mod lexer;
mod parser;

//TODO: this needs some clean-up ~~and also we need to be able to resolve imports!!! that's
//important~~ no imports are pain
fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename).expect("Failed to read file");
    let arcstr = ArcStr::from(src.as_str());

    let (tokens, errs) = lexer::tokenize(&arcstr);

    let parse_errs = if let Some(tokens) = &tokens {
        let (ast, parse_errs) = parser::create().parse(tokens.as_slice()).into_output_errors();

        if let Some(defs) = ast.filter(|_| errs.len() + parse_errs.len() == 0) {
            println!("{:#?}", defs)
        }

        parse_errs
    } else {
        Vec::new()
    };

    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|tok| format!("{:#?}", tok.kind))),
        )
        .for_each(|e| {
            Report::build(ReportKind::Error, (filename.clone(), e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new((filename.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename.clone(), span.into_range()))
                        .with_message(format!("while parsing this {label}"))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(filename.clone(), src.clone())]))
                .unwrap()
        });
}
