use std::fs;

use ast::Expr;
use chumsky::Parser;

mod ast;
mod parser;
mod generator;

fn main() {
    let text = fs::read_to_string("./test/file.cae").unwrap();
    //TODO: this gives bad error messages
    let (output, errors) = parser::create().parse(&text).into_output_errors();

    for item in errors {
        println!("Error {} in span {}", item, item.span());
    }

    output.map(|result| {
        println!("Finished parsing, printing results");
        for item in result {
            println!("{}", item.debug_text());
        }
    });
}
