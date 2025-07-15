
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),

    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    If,
    Else,
    While,
    Fn,
    Return,
    Break,
    Continue,

    Eof,
}
