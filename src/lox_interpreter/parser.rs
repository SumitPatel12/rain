// TODO: The error reporting in my version is horrendous, fix it at some point!!
use super::{
    ast_tools::Expr,
    error::{parse_error, LoxError},
    token::{Literal, Token, TokenType},
};
use anyhow::{anyhow, bail, Ok, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_tokens(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            // NOTE: You gotta clone here otherwise mutable and immutable references clash.
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Trying to read token after reaching the end.")
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Trying to read the token at index -1")
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_tokens(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_tokens(vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_tokens(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.peek();
        let expr = match &token.token_type {
            TokenType::FALSE => Expr::Literal {
                value: Literal::Boolean(false),
            },
            TokenType::TRUE => Expr::Literal {
                value: Literal::Boolean(true),
            },
            TokenType::NIL => Expr::Literal {
                value: Literal::None,
            },
            TokenType::STRING => Expr::Literal {
                value: Literal::String(token.lexeme.clone()),
            },
            TokenType::NUMBER => Expr::Literal {
                // TODO: Check the performance implication of this. Maybe it'd be better to store
                // a copy of the float in the enum itself.
                value: Literal::Float(token.lexeme.parse::<f64>().unwrap()),
            },
            TokenType::LEFT_PAREN => {
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.")?;
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => return bail!(self.error(self.peek(), "Expected expression.")),
        };
        self.advance();
        Ok(expr)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            bail!(self.error(self.peek(), message))
        }
    }

    pub fn error(&self, token: &Token, message: &str) -> LoxError {
        parse_error(&token, message);
        LoxError::Parse
    }
}
