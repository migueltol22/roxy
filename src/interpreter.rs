use std::ops::{Div, Mul, Sub};

use crate::parser::Expr;
use crate::token::TokenType;

#[derive(Debug)]
enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl RuntimeValue {
    fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Nil => false,
            RuntimeValue::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, rhs: RuntimeValue) -> bool {
        match (self, rhs) {
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => *n1 == n2,
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => *s1 == s2,
            (RuntimeValue::Boolean(b1), RuntimeValue::Boolean(b2)) => *b1 == b2,
            _ => false,
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<(), anyhow::Error> {
        let val = self.evaluate(expr)?;
        println!("{:?}", val);
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<RuntimeValue, anyhow::Error> {
        match expr {
            Expr::Literal(token) => match token.token_type {
                TokenType::Number(n) => Ok(RuntimeValue::Number(n)),
                TokenType::String(s) => Ok(RuntimeValue::String(s)),
                TokenType::Bool(b) => Ok(RuntimeValue::Boolean(b)),
                TokenType::Nil => Ok(RuntimeValue::Nil),
                _ => Err(anyhow::anyhow!("Invalid literal")),
            },
            Expr::Grouping(g) => self.evaluate(*g),
            Expr::Unary(token, right) => {
                let right = self.evaluate(*right)?;
                match token.token_type {
                    TokenType::Bang => Ok(RuntimeValue::Boolean(!right.is_truthy())),
                    TokenType::Minus => match right {
                        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(-n)),
                        _ => Err(anyhow::anyhow!("Operand must be a number")),
                    },
                    _ => Err(anyhow::anyhow!("Invalid unary operator")),
                }
            }
            Expr::Binary(left, token, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                match token.token_type {
                    TokenType::Minus => self.eval_arithmetic_op(left, right, f64::sub),
                    TokenType::Slash => self.eval_arithmetic_op(left, right, f64::mul),
                    TokenType::Star => self.eval_arithmetic_op(left, right, f64::div),
                    TokenType::Plus => match (left, right) {
                        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                            Ok(RuntimeValue::Number(l + r))
                        }
                        (RuntimeValue::String(l), RuntimeValue::String(r)) => {
                            Ok(RuntimeValue::String(l + &r))
                        }
                        _ => Err(anyhow::anyhow!("Unsupported type for plus operator")),
                    },
                    TokenType::Greater => self.eval_boolean_op(left, right, |l, r| l > r),
                    TokenType::GreaterEqual => self.eval_boolean_op(left, right, |l, r| l >= r),
                    TokenType::Less => self.eval_boolean_op(left, right, |l, r| l < r),
                    TokenType::LessEqual => self.eval_boolean_op(left, right, |l, r| l <= r),
                    TokenType::BangEqual => Ok(RuntimeValue::Boolean(!left.is_equal(right))),
                    TokenType::EqualEqual => Ok(RuntimeValue::Boolean(left.is_equal(right))),
                    _ => Err(anyhow::anyhow!("Invalid binary expression")),
                }
            }
        }
    }

    fn eval_arithmetic_op<F>(
        &self,
        l: RuntimeValue,
        r: RuntimeValue,
        f: F,
    ) -> Result<RuntimeValue, anyhow::Error>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        match (l, r) {
            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(f(l, r))),
            _ => Err(anyhow::anyhow!("Operands must be numbers")),
        }
    }

    fn eval_boolean_op<F>(
        &self,
        l: RuntimeValue,
        r: RuntimeValue,
        f: F,
    ) -> Result<RuntimeValue, anyhow::Error>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        match (l, r) {
            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                Ok(RuntimeValue::Boolean(f(l, r)))
            }
            _ => Err(anyhow::anyhow!("Operands must be numbers")),
        }
    }
}
