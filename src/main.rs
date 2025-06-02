use std::fs;

use ast::Expr;
use nom_recursive::RecursiveInfo;

mod ast;
mod parser;
mod generator;

fn main() {
    let text = fs::read_to_string("./test/file.cae").unwrap();
    let result = parser::parse(&text).unwrap().1;

    println!("Finished parsing, printing results");
    for item in result {
        println!("{}", item.debug_text());
    }
}
