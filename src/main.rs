use std::fs;

use ast::Expr;
use chumsky::Parser;

mod ast;
mod parser;
mod generator;

fn main() {
    let text = fs::read_to_string("./test/file.cae").unwrap();
    let result = parser::parser().parse(&text).into_result().unwrap();

    println!("Finished parsing, printing results");
    for item in result {
        println!("{}", item.debug_text());
    }
}
