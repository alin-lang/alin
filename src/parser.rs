use crate::ast::Expr;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    fn next(&mut self) -> Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    fn eat(&mut self, expected: &Token) -> bool {
        if self.peek() == *expected {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        while self.peek() != Token::Eof {
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
                self.eat(&Token::Semicolon); // optional
            } else {
                panic!("Syntax error near {:?}", self.peek());
            }
        }
        exprs
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::LBrace => self.parse_block(),
            Token::Return => self.parse_return(),
            _ => self.parse_assignment(),
        }
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        let expr = self.parse_comparison()?;

        if self.peek() == Token::Equal {
            self.next(); // consume '='
            if let Expr::Variable(name) = expr {
                let value = self.parse_expr()?;
                return Some(Expr::Assignment {
                    name,
                    expr: Box::new(value),
                });
            } else {
                panic!("Invalid assignment target");
            }
        }

        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_binary(5)?;

        while matches!(
            self.peek(),
            Token::EqualEqual | Token::BangEqual | Token::Less | Token::LessEqual
        ) {
            let op = self.next();
            let right = self.parse_binary(5)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_binary(&mut self, min_prec: u8) -> Option<Expr> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => self.peek(),
                _ => break,
            };

            let prec = get_precedence(&op);
            if prec < min_prec {
                break;
            }

            let op_token = self.next();
            let rhs = self.parse_binary(prec + 1)?;
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: op_token,
                right: Box::new(rhs),
            };
        }

        Some(lhs)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.next() {
            Token::Number(n) => Some(Expr::Number(n)),
            Token::String(s) => Some(Expr::StringLiteral(s)),
            Token::Identifier(name) => {
                if self.peek() == Token::LParen {
                    self.next(); // consume '('
                    let mut args = Vec::new();
                    while self.peek() != Token::RParen {
                        if let Some(arg) = self.parse_expr() {
                            args.push(arg);
                        }
                        if self.peek() == Token::Comma {
                            self.next(); // consume ','
                        } else {
                            break;
                        }
                    }
                    self.eat(&Token::RParen);
                    Some(Expr::FunctionCall { name, args })
                } else {
                    Some(Expr::Variable(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.eat(&Token::RParen);
                Some(expr)
            }
            Token::Break => Some(Expr::Break),

            Token::Continue => Some(Expr::Continue),

            
            _ => None,
        }
    }

    fn parse_if(&mut self) -> Option<Expr> {
        self.next(); // consume `if`
        let condition = self.parse_expr()?;
        let then_branch = Box::new(self.parse_expr()?);
        let else_branch = if self.peek() == Token::Else {
            self.next(); // consume `else`
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };

        Some(Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Option<Expr> {
        self.next(); // consume `while`
        let condition = self.parse_expr()?;
        let body = Box::new(self.parse_expr()?);
        Some(Expr::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn parse_return(&mut self) -> Option<Expr> {
        self.next(); // consume `return`
        let value = self.parse_expr()?;
        Some(Expr::Return(Box::new(value)))
    }

    fn parse_block(&mut self) -> Option<Expr> {
        self.eat(&Token::LBrace); // consume '{'
        let mut exprs = Vec::new();
        while self.peek() != Token::RBrace && self.peek() != Token::Eof {
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
                self.eat(&Token::Semicolon); // optional
            }
        }
        self.eat(&Token::RBrace); // consume '}'
        Some(Expr::Block(exprs))
    }
}

fn get_precedence(tok: &Token) -> u8 {
    match tok {
        Token::Star | Token::Slash => 10,
        Token::Plus | Token::Minus => 5,
        _ => 0,
    }
}
