use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    /// Angka literal, seperti `42`, `3.14`
    Number(f64),

    /// String literal, seperti `"hello"` atau `'world'`
    StringLiteral(String),

    /// Variabel seperti `x`, `nama_user`
    Variable(String),

    /// Operasi biner seperti `1 + 2`, `x * 3`
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },

    /// Assignment seperti `x = 10`
    Assignment {
        name: String,
        expr: Box<Expr>,
    },

    /// Blok `{ expr1; expr2; }`
    Block(Vec<Expr>),

    /// `if` expression
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,              // biasanya Block
        else_branch: Option<Box<Expr>>,      // optional else
    },

    /// `while` loop
    While {
        condition: Box<Expr>,
        body: Box<Expr>,                     // biasanya Block
    },

    /// Definisi fungsi: `fn name(params) { body }`
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },

    /// Pemanggilan fungsi: `print(x)`
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },

    /// `return` statement
    Return(Box<Expr>),
}
