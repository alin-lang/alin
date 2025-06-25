use std::collections::HashMap;

use crate::ast::Expr;

pub struct Evaluator {
    env: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, exprs: &[Expr]) {
        for expr in exprs {
            self.eval(expr);
        }
    }

    fn eval(&mut self, expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Number(n) => Some(Value::Number(*n)),

            Expr::StringLiteral(s) => Some(Value::String(s.clone())),

            Expr::Variable(name) => self.env.get(name).cloned(),

            Expr::Assignment { name, expr } => {
                if let Some(val) = self.eval(expr) {
                    self.env.insert(name.clone(), val.clone());
                    Some(val)
                } else {
                    None
                }
            }

            Expr::Binary { left, op, right } => {
                let lhs = self.eval(left)?;
                let rhs = self.eval(right)?;

                match (lhs, rhs) {
                    (Value::Number(a), Value::Number(b)) => {
                        match op {
                            crate::token::Token::Plus => Some(Value::Number(a + b)),
                            crate::token::Token::Minus => Some(Value::Number(a - b)),
                            crate::token::Token::Star => Some(Value::Number(a * b)),
                            crate::token::Token::Slash => Some(Value::Number(a / b)),
                            _ => None,
                        }
                    }
                    (Value::String(a), Value::String(b)) if op == &crate::token::Token::Plus => {
                        Some(Value::String(a + &b))
                    }
                    _ => {
                        println!("Type mismatch in binary expression");
                        None
                    }
                }
            }

            Expr::FunctionCall { name, args } => {
                if name == "print" {
                    let mut output = Vec::new();
                    for arg in args {
                        match self.eval(arg) {
                            Some(val) => output.push(format!("{}", val)),
                            None => output.push("nil".to_string()),
                        }
                    }
                    println!("{}", output.join(" "));
                    None
                } else {
                    println!("Unknown function: {}", name);
                    None
                }
            }

            

            _ => None, // skip unimplemented expressions (If, While, etc.)
        }
    }
}
