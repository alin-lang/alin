use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    StringLiteral(String),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assignment {
        name: String,
        expr: Box<Expr>,
    },
    Block(Vec<Expr>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Return(Box<Expr>),
}
