use crate::enums::Token;
use logos::Logos;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(_) => return Err(format!("invalid token: {:?}", lexer.slice()))
        }
    }

    Ok(tokens)
}