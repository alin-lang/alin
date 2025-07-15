#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Number(f64),
    String(String),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    If,
    Else,
    While,
    Fn,
    Return,
    Eof
}