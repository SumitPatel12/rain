// TODO: The error reporting in my version is horrendous, fix it at some point!!
use super::{
    ast_tools::{Expr, Stmt},
    error::{report_parse_error, LoxError},
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let statement = if self.match_tokens(vec![TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        // TODO: Handle synchronize in case of parsing error.
        match statement {
            Err(LoxError::Parse) => {
                self.synchronize();
                Ok(Stmt::NONE)
            }
            other => other,
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected variable name.")?;

        if self.match_tokens(vec![TokenType::EQUAL]) {
            let initializer = self.expression()?;
            self.consume(TokenType::SEMICOLON, "Expected ';' after variable name.")?;
            return Ok(Stmt::Var {
                name,
                initializer: Some(initializer),
            });
        }

        Ok(Stmt::Var {
            name,
            initializer: None,
        })
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
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

    fn comparison(&mut self) -> Result<Expr, LoxError> {
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

    fn term(&mut self) -> Result<Expr, LoxError> {
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

    fn factor(&mut self) -> Result<Expr, LoxError> {
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

    fn unary(&mut self) -> Result<Expr, LoxError> {
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

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let token = self.peek();
        let expr = match &token.token_type {
            TokenType::FALSE => Expr::Literal {
                value: Literal::Boolean(false),
            },
            TokenType::TRUE => Expr::Literal {
                value: Literal::Boolean(true),
            },
            TokenType::NONE => Expr::Literal {
                // TODO: Streamline use of None NIL and NULL.
                value: Literal::None,
            },
            TokenType::STRING => Expr::Literal {
                value: Literal::String(token.lexeme.clone()),
            },
            TokenType::NUMBER => {
                Expr::Literal {
                    // TODO: Check the performance implication of this. Maybe it'd be better to store
                    // a copy of the float in the enum itself.
                    value: Literal::Float(token.lexeme.parse::<f64>().unwrap()),
                }
            }
            TokenType::LEFT_PAREN => {
                self.consume(TokenType::LEFT_PAREN, "Expected '('.")?;
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.")?;
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            TokenType::IDENTIFIER => {
                return Ok(Expr::Variable {
                    name: self.previous().clone(),
                })
            }
            _ => return Err(self.error(token, "Expected Expression.")),
        };
        self.advance();
        Ok(expr)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    pub fn error(&self, token: &Token, message: &str) -> LoxError {
        report_parse_error(&token, message);
        LoxError::Parse
    }

    // Discards tokens until we think it's found a statement boundary.
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().clone().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => self.advance(),
            };
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_tokens(vec![TokenType::PRINT]) {
            return self.print_statement();
        }

        if self.match_tokens(vec![TokenType::LEFT_BRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RIGHT_BRACE, "Expected '}' after blcok.")?;
        return Ok(statements);
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.")?;

        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.")?;

        Ok(Stmt::Expression { expression: expr })
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.match_tokens(vec![TokenType::EQUAL]) {
            // NOTE: If you change the ordering it complains that immuatble borrow occurs before
            // mutalbe borrow hence, you cannot do that.
            let value = self.expression()?;
            let equals = self.previous();

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => return Err(self.error(equals, "Invalid assignment target.")),
            }
        }

        return Ok(expr);
    }
}

#[test]
pub fn test_parser() -> Result<(), LoxError> {
    // TODO: I've got no Idea what to expect here now.

    //let mut scanner = Scanner::new("-123 * 45.67".into());
    //let tokens = scanner.scan_tokens()?;
    //let mut parser = Parser::new(tokens);
    //let expression = parser.parse().expect("Could not parse sample code.");
    //let mut printer = ASTPrinter::new();
    //assert_eq!(printer.print(expression).unwrap(), "(* (- 123) 45.67)");
    return Ok(());
}
