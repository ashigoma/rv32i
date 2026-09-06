use std::fmt;
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("let")]
    LET,

    #[token("rec")]
    REC,

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

    #[token("!=")]
    NEQ,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("&&")]
    AND,

    #[token("||")]
    OR,

    #[token("not")]
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
    
    #[regex("[0-9]+", |lex| lex.slice().parse::<i32>().ok())]
    INT(i32),

    #[regex("true|false", |lex| lex.slice().parse::<bool>().ok())]
    BOOL(bool),

    #[regex("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    ID(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Expr {
  INT(i32),
  BOOL(bool),
  STRING(String),
  UNIT,
  ID(String),
  ADD(Box<Expr>, Box<Expr>),
  SUB(Box<Expr>, Box<Expr>),
  GEQ(Box<Expr>, Box<Expr>),
  LEQ(Box<Expr>, Box<Expr>),
  GT(Box<Expr>, Box<Expr>),
  LT(Box<Expr>, Box<Expr>),
  EQ(Box<Expr>, Box<Expr>),
  AND(Box<Expr>, Box<Expr>),
  OR(Box<Expr>, Box<Expr>),
  NOT(Box<Expr>),
  LET(Box<Expr>, Box<Expr>, Box<Expr>),
  LETREC(Box<Expr>, Box<Expr>, Box<Expr>),
  FUN(Box<Expr>, Box<Expr>),
  IF(Box<Expr>, Box<Expr>, Box<Expr>),
  SEMI(Box<Expr>, Box<Expr>)
}