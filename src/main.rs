use std::{env, fs};

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{input::Input, Parser};

mod util;
mod ast;
mod lexer;
mod parser;
mod generator;

fn main() {
    let file = env::args().nth(1).expect("Expected file argument");
    let text = fs::read_to_string(file.clone()).unwrap();
    let input = text.clone();
    let (output, errors) = lexer::create().parse(&input).into_output_errors();

    for error in errors {
        Report::build(ReportKind::Error, (file.clone(), error.span().into_range()))
            .with_message(format!("Failed to tokenize input file {}", file.clone()))
            .with_label(
                Label::new((file.clone(), error.span().into_range())).with_message(format!(
                    "Found '{}' but expected one of {}",
                    error
                        .found()
                        .map(|c| format!("{}", c).escape_debug().to_string())
                        .unwrap_or(String::from("EOF")),
                    error
                        .expected()
                        .map(|pattern| pattern
                            .clone()
                            .map_token(|c| format!("{}", c).escape_debug().to_string())
                            .to_string()
                            .replace("''", "'"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )),
            )
            .finish()
            .eprint((file.clone(), Source::from(text.clone())))
            .unwrap();
    }

    output.map(|result| {
        println!("{:#?}", result);

        let (output, errors) = parser::create()
            .map_with(|ast, e| (ast, e.span()))
            .parse(
                result
                    .as_slice()
                    .map((text.len()..text.len()).into(), |(t, s)| (t, s))
            ).into_output_errors();

        for error in errors {
            Report::build(ReportKind::Error, (file.clone(), error.span().into_range()))
                .with_message(format!("Failed to parse input file {}", file.clone()))
                .with_label(
                    Label::new((file.clone(), error.span().into_range())).with_message(format!(
                        "Found '{}' but expected one of {}",
                        error
                            .found()
                            .map(|c| format!("{}", c).escape_debug().to_string())
                            .unwrap_or(String::from("EOF")),
                        error
                            .expected()
                            .map(|pattern| pattern
                                .clone()
                                .map_token(|c| format!("{}", c).escape_debug().to_string())
                                .to_string()
                                .replace("''", "'"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )),
                )
                .finish()
                .eprint((file.clone(), Source::from(text.clone())))
                .unwrap();
        }

        output.map(|result| {
            println!("{:#?}", result);
        });
    });
}
