mod lexer;
use std::fs;
use std::process;

fn main() {
    let code = match fs::read_to_string("tests/add.ml") {
        Ok(s) => s,
        Err(_) => {eprintln!("failed to open code"); process::exit(1)}
    };

    println!("{}", code);

    let tokens = match lexer::tokenize(&code) {
        Ok(t) => t,
        Err(e) => {eprintln!("failed to tokenize: {}", e); process::exit(1)}
    };

    println!("{:?}", tokens);
}
