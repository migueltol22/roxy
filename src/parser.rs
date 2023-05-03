use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        let expression = self.expression();
        println!("{:?}", expression);
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(token_type) = self.current_token() {
            match *token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.comparison();
                    expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(token_type) = self.current_token() {
            match *token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.term();
                    expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(token_type) = self.current_token() {
            match *token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.factor();
                    expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while let Some(token_type) = self.current_token() {
            match *token_type {
                TokenType::Star | TokenType::Slash => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.unary();
                    expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(token_type) = self.current_token() {
            match *token_type {
                TokenType::Bang | TokenType::Minus => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.unary();
                    return Expr::Unary(operator, Box::new(right));
                }
                _ => (),
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if let Some(token_type) = self.current_token() {
            match token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number(_)
                | TokenType::String(_) => {
                    self.advance();
                    return Expr::Literal(self.previous());
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.expression();
                    self.consume(TokenType::RightParen, "Expect ')' after expression.");
                    return Expr::Grouping(Box::new(expr));
                }
                // Change these to errors and return a Result instead
                _ => panic!("Expect expression."),
            }
        }
        panic!("Expect expression.")
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("{}", message);
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn current_token(&self) -> Option<&TokenType> {
        if self.is_at_end() {
            return None;
        }
        Some(&self.tokens[self.current].token_type)
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
