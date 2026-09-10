mod enums;
mod grammer;
mod lexer;
mod parser;
mod typing;

use std::fs;
use std::process;

fn main() {
    let code = match fs::read_to_string("tests/arith.ml") {
        Ok(r) => r,
        Err(_) => {
            eprintln!("failed to open code");
            process::exit(1)
        }
    };

    println!("{}", code);

    let tokens = match lexer::tokenize(&code) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to tokenize: {}", e);
            process::exit(1)
        }
    };

    println!("{:?}", tokens);

    let ast = match parser::parse(tokens) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to parse: {}", e);
            process::exit(1)
        }
    };

    println!("{:?}", ast);

    let (ast_type, constr) = typing::type_check(ast, Vec::new());

    println!("{:?}", ast_type);
    println!("{:?}", constr);
}
