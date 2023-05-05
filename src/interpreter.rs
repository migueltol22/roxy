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
            Expr::Literal(l) => match l.token_type {
                TokenType::Number(n) => Ok(RuntimeValue::Number(n)),
                TokenType::String(s) => Ok(RuntimeValue::String(s)),
                TokenType::Bool(b) => Ok(RuntimeValue::Boolean(b)),
                TokenType::Nil => Ok(RuntimeValue::Nil),
                _ => Err(anyhow::anyhow!("Invalid literal")),
            },
            Expr::Grouping(g) => self.evaluate(*g),
            Expr::Unary(u, right) => {
                let right = self.evaluate(*right)?;
                match u.token_type {
                    TokenType::Bang => Ok(RuntimeValue::Boolean(!right.is_truthy())),
                    TokenType::Minus => match right {
                        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(-n)),
                        _ => Err(anyhow::anyhow!("Operand must be a number")),
                    },
                    _ => Err(anyhow::anyhow!("Invalid unary operator")),
                }
            }
            _ => Err(anyhow::anyhow!("Invalid expression")),
        }
    }
}
