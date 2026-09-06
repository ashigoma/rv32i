use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("let rec")]
    LETREC,

    #[token("let")]
    LET,

    #[token("fun")]
    FUN,

    #[token("in")]
    IN,

    #[token("if")]
    IF,

    #[token("then")]
    THEN,

    #[token("else")]
    ELSE,

    #[token("->")]
    ARROW,

    #[token(">=")]
    GEQ,

    #[token("<=")]
    LEQ,

    #[token(">")]
    GT,

    #[token("<")]
    LT,

    #[token("=")]
    EQ,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("&")]
    AND,

    #[token("|")]
    OR,

    #[token("!")]
    NOT,

    #[token("()")]
    UNIT,

    #[token("(")]
    OPEN,
    
    #[token(")")]
    CLOSE,

    #[token(";")]
    SEMI,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
            let s = lex.slice();
            s[1..s.len() - 1].to_string()
        })]
    STRING(String),
    
    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    INT(i64),

    #[regex("true|false", |lex| lex.slice().parse::<bool>().ok())]
    BOOL(bool),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    ID(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(e) => return Err(format!("invalid token: {:?}", lexer.slice()))
        }
    }

    Ok(tokens)
}