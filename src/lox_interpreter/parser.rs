// TODO: The error reporting in my version is horrendous, fix it at some point!!
use super::{
    ast_tools::{Expr, Stmt},
    error::{report_parse_error, LoxError},
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    // Tokens scanner provided that we need to parse.
    tokens: Vec<Token>,
    // The current token we are at while parsing.
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
        //println!(
        //"Current Token: {}\n{}",
        //self.current, self.tokens[self.current]
        //);
        let statement = if self.match_tokens(vec![TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        // TODO: Handle synchronize in case of parsing error.
        match statement {
            Err(LoxError::Parse) => {
                println!("Parse Error. Trying to synchronize.");
                self.synchronize();
                Ok(Stmt::NONE)
            }
            other => other,
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected variable name.")?;

        let initializer = if self.match_tokens(vec![TokenType::EQUAL]) {
            //println!("Parser:\nInitializer: {}", init);
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::SEMICOLON, "Expected ';' after variable name.")?;
        Ok(Stmt::Var { name, initializer })
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
            TokenType::IDENTIFIER => Expr::Variable {
                name: self.peek().clone(),
            },
            TokenType::LEFT_PAREN => {
                self.consume(TokenType::LEFT_PAREN, "Expected '('.")?;
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.")?;
                Expr::Grouping {
                    expression: Box::new(expr),
                }
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
            if self.previous().token_type == TokenType::SEMICOLON {
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
        if self.match_tokens(vec![TokenType::IF]) {
            return self.if_statement();
        } else if self.match_tokens(vec![TokenType::PRINT]) {
            return self.print_statement();
        } else if self.match_tokens(vec![TokenType::LEFT_BRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        } else if self.match_tokens(vec![TokenType::WHILE]) {
            return self.while_statement();
        } else if self.match_tokens(vec![TokenType::FOR]) {
            return self.for_statement();
        }

        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expected '(' after if.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expected ')' after if condition.")?;

        let then_branch = self.statement()?;

        // TODO: I think this can be cleaner, but can't think of anything right now.
        if self.match_tokens(vec![TokenType::ELSE]) {
            let else_branch = Some(self.statement()?);
            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            })
        } else {
            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Box::new(None),
            })
        }
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
        //println!("Parser:");
        //println!(
        //"\nCurrent Token: {}\n{}\n",
        //self.current, self.tokens[self.current]
        //);
        //println!("Next Token: {}", self.peek());
        //println!("In Print Value: {}", value);
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.")?;

        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.")?;

        Ok(Stmt::Expression { expression: expr })
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.match_tokens(vec![TokenType::EQUAL]) {
            // NOTE: If you change the ordering it complains that immuatble borrow occurs before
            // mutalbe borrow hence, you cannot do that.
            // Finally got why you can't do it. It's simile, equals holds the reference to self as
            // I'm returning a reference, and  that leads to the clash.
            // If I clone the value then it would not comlain.
            // Clonig is the other solution.
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => return Err(self.error(&equals, "Invalid assignment target.")),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let expr = self.and()?;

        while self.match_tokens(vec![TokenType::OR]) {
            let operator = self.previous().clone();
            let right = Box::new(self.and()?);

            return Ok(Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        while self.match_tokens(vec![TokenType::AND]) {
            let operator = self.previous().clone();
            let right = Box::new(self.equality()?);

            return Ok(Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

        // Check if we got an initializtion, i.e. var x = 0 or something of the like.
        let initializer = if self.match_tokens(vec![TokenType::SEMICOLON]) {
            None
        } else if self.match_tokens(vec![TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        // Check if we got the condition to loop over.
        let condition = if !self.check(TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::SEMICOLON, "Expected ';' after loop condition.")?;

        // Chek if we got the increment condition, x++ and the like.
        let increment = if !self.check(TokenType::RIGHT_PAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RIGHT_PAREN, "Expected ')' after for clauses.")?;

        // For loop body.
        let mut body = self.statement()?;

        // NOTE: Now begins the fun part. We do not use a new Stmt visitor or something for this,
        // we just convert it to the matching while loop and we already got while in place.
        // What we do is:
        //  1. Get the increment statement
        //  2. Make a block with the body and increment statement, essentially attach the increment
        //     statement to the for block.
        //  3. Make a while statement with the condition and the body the we got after
        //     concatenation from step 2.
        //  4. Get the initialization statement.
        //  5. Create a new Block statement that does the initialization once and then executes the
        //     While loop.
        if let Some(incr) = increment {
            let increment_statement = Stmt::Expression { expression: incr };
            body = Stmt::Block {
                statements: vec![body, increment_statement],
            }
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: Literal::Boolean(true),
            }),
            body: Box::new(body),
        };

        if let Some(initializon_statement) = initializer {
            body = Stmt::Block {
                statements: vec![initializon_statement, body],
            }
        }

        Ok(body)
    }
}
