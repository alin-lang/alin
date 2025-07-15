use std::collections::HashMap;
use crate::ast::Expr;

pub struct Evaluator {
    env: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Nil, // untuk nilai return yang kosong
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
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
                let value = self.eval(expr)?;
                self.env.insert(name.clone(), value.clone());
                Some(value)
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
                            crate::token::Token::EqualEqual => Some(Value::Number((a == b) as i32 as f64)),
                            crate::token::Token::BangEqual => Some(Value::Number((a != b) as i32 as f64)),
                            crate::token::Token::Less => Some(Value::Number((a < b) as i32 as f64)),
                            crate::token::Token::LessEqual => Some(Value::Number((a <= b) as i32 as f64)),
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
                    let output: Vec<String> = args
                        .iter()
                        .map(|arg| match self.eval(arg) {
                            Some(val) => val.to_string(),
                            None => "nil".to_string(),
                        })
                        .collect();
                    println!("{}", output.join(" "));
                    None
                } else {
                    println!("Unknown function: {}", name);
                    None
                }
            }

            Expr::Block(statements) => {
                let mut last = Value::Nil;
                for stmt in statements {
                    if let Some(val) = self.eval(stmt) {
                        last = val;
                    }
                }
                Some(last)
            }

            Expr::If { condition, then_branch, else_branch } => {
                let cond_val = self.eval(condition)?;
                match cond_val {
                    Value::Number(n) if n != 0.0 => self.eval(then_branch),
                    _ => {
                        if let Some(else_expr) = else_branch {
                            self.eval(else_expr)
                        } else {
                            Some(Value::Nil)
                        }
                    }
                }
            }

            Expr::While { condition, body } => {
                loop {
                    let cond_val = self.eval(condition)?;
                    if let Value::Number(n) = cond_val {
                        if n == 0.0 {
                            break;
                        }
                        self.eval(body);
                    } else {
                        break;
                    }
                }
                Some(Value::Nil)
            }

            Expr::Return(expr) => {
                self.eval(expr)
            }

            _ => {
                println!("Unsupported expression");
                None
            }
        }
    }
}
