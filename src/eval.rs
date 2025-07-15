use std::collections::HashMap;
use crate::ast::Expr;
use crate::token::Token;

pub struct Evaluator {
    env: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Nil,
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

enum EvalResult {
    Value(Option<Value>),
    Break,
    Continue,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, exprs: &[Expr]) {
        for expr in exprs {
            match self.eval(expr) {
                EvalResult::Value(_) => {},
                EvalResult::Break | EvalResult::Continue => {
                    println!("'break' or 'continue' used outside of loop");
                }
            }
        }
    }

    fn eval(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Number(n) => EvalResult::Value(Some(Value::Number(*n))),
            Expr::StringLiteral(s) => EvalResult::Value(Some(Value::String(s.clone()))),
            Expr::Variable(name) => {
                EvalResult::Value(self.env.get(name).cloned())
            }

            Expr::Assignment { name, expr } => {
                match self.eval(expr) {
                    EvalResult::Value(Some(val)) => {
                        self.env.insert(name.clone(), val.clone());
                        EvalResult::Value(Some(val))
                    },
                    _ => EvalResult::Value(None),
                }
            }

            Expr::Binary { left, op, right } => {
                let lhs = match self.eval(left) {
                    EvalResult::Value(Some(val)) => val,
                    _ => return EvalResult::Value(None),
                };
                let rhs = match self.eval(right) {
                    EvalResult::Value(Some(val)) => val,
                    _ => return EvalResult::Value(None),
                };

                let result = match (lhs, rhs) {
                    (Value::Number(a), Value::Number(b)) => {
                        match op {
                            Token::Plus => Value::Number(a + b),
                            Token::Minus => Value::Number(a - b),
                            Token::Star => Value::Number(a * b),
                            Token::Slash => Value::Number(a / b),
                            Token::EqualEqual => Value::Number((a == b) as i32 as f64),
                            Token::BangEqual => Value::Number((a != b) as i32 as f64),
                            Token::Less => Value::Number((a < b) as i32 as f64),
                            Token::LessEqual => Value::Number((a <= b) as i32 as f64),
                            _ => return EvalResult::Value(None),
                        }
                    }
                    (Value::String(a), Value::String(b)) if op == &Token::Plus => {
                        Value::String(a + &b)
                    }
                    _ => {
                        println!("Type mismatch in binary expression");
                        return EvalResult::Value(None);
                    }
                };

                EvalResult::Value(Some(result))
            }

            Expr::FunctionCall { name, args } => {
                if name == "print" {
                    let output: Vec<String> = args
                        .iter()
                        .map(|arg| match self.eval(arg) {
                            EvalResult::Value(Some(val)) => val.to_string(),
                            _ => "nil".to_string(),
                        })
                        .collect();
                    println!("{}", output.join(" "));
                    EvalResult::Value(Some(Value::Nil))
                } else {
                    println!("Unknown function: {}", name);
                    EvalResult::Value(Some(Value::Nil))
                }
            }

            Expr::Block(statements) => {
                for stmt in statements {
                    match self.eval(stmt) {
                        EvalResult::Break => return EvalResult::Break,
                        EvalResult::Continue => return EvalResult::Continue,
                        val => {} // ignore other values
                    }
                }
                EvalResult::Value(Some(Value::Nil))
            }

            Expr::If { condition, then_branch, else_branch } => {
                match self.eval(condition) {
                    EvalResult::Value(Some(Value::Number(n))) if n != 0.0 => {
                        self.eval(then_branch)
                    },
                    _ => {
                        if let Some(else_expr) = else_branch {
                            self.eval(else_expr)
                        } else {
                            EvalResult::Value(Some(Value::Nil))
                        }
                    }
                }
            }

            Expr::While { condition, body } => {
                loop {
                    match self.eval(condition) {
                        EvalResult::Value(Some(Value::Number(n))) if n != 0.0 => {
                            match self.eval(body) {
                                EvalResult::Break => break,
                                EvalResult::Continue => continue,
                                _ => {}
                            }
                        }
                        _ => break,
                    }
                }
                EvalResult::Value(Some(Value::Nil))
            }

            Expr::Break => EvalResult::Break,
            Expr::Continue => EvalResult::Continue,

            Expr::Return(expr) => self.eval(expr),

            _ => {
                println!("Unsupported expression");
                EvalResult::Value(None)
            }
        }
    }
}
