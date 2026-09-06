mod enums;
mod lexer;
mod grammer;
mod parser;

use std::fs;
use std::process;

fn main() {
    let code = match fs::read_to_string("tests/simple.ml") {
        Ok(r) => r,
        Err(_) => {eprintln!("failed to open code"); process::exit(1)}
    };

    println!("{}", code);

    let tokens = match lexer::tokenize(&code) {
        Ok(r) => r,
        Err(e) => {eprintln!("failed to tokenize: {}", e); process::exit(1)}
    };

    println!("{:?}", tokens);

    let ast = match parser::parse(tokens) {
        Ok(r) => r,
        Err(e) => {eprintln!("failed to parse: {}", e); process::exit(1)}
    };

    println!("{:?}", ast);
}
