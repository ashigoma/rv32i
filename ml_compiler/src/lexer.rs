use crate::enums::Token;
use logos::Logos;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(_) => return Err(format!("invalid token: {:?}", lexer.slice())),
        }
    }

    Ok(tokens)
}

pub fn unescape(s: &str) -> String {
    let mut chars = s.chars();
    let mut result = String::with_capacity(s.len());

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}
