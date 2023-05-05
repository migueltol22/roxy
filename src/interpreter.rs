use std::ops::{Add, Sub, Mul, Div};

use crate::parser::{Expr, Parser};
use crate::scanner::Scanner;
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
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), anyhow::Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser
            .parse()
            .ok_or(anyhow::anyhow!("Failed to parse expression"))?;
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
            },
            Expr::Binary(left, token, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                match token.token_type {
                    TokenType::Minus => self.evaluate_op(left, right, f64::sub),
                    TokenType::Plus => self.evaluate_op(left, right, f64::add),
                    TokenType::Slash => self.evaluate_op(left, right, f64::mul),
                    TokenType::Star => self.evaluate_op(left, right, f64::div),
                    _ => Err(anyhow::anyhow!("Invalid binary operator"))
                }
            }
        }
    }

    fn evaluate_op<F>(&self, l: RuntimeValue, r: RuntimeValue, f: F) -> Result<RuntimeValue, anyhow::Error> 
    where 
        F: FnOnce(f64, f64) -> f64{
        match (l, r) {
            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(f(l, r))),
            _ => Err(anyhow::anyhow!("Operands must be numbers"))
        }
    }
}
