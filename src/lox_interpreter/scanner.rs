use anyhow::{anyhow, Result};

use super::token::{self, Literal, Token, TokenType};

// TODO: Visit this later and cleanup if necessary.
#[allow(dead_code)]
const VARIABLE_NAME_REGEX: &str = r"[a-zA-Z_][a-zA-Z_0_9]";

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            token::Literal::String(String::new()),
            self.line,
        ));
        return Ok(self.tokens.clone());
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();

        return match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type)
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type)
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            ' ' => Ok(()),
            '\r' => Ok(()),
            '\t' => Ok(()),
            '\n' => Ok(self.line += 1),
            c if self.is_digit(c) => self.read_number(),
            '"' => self.read_string(),
            c if self.is_alphabetic(c) => self.read_identifier(),
            c => Err(anyhow!(format!(
                "{}, Unexpected Character At Line {}.",
                c, self.line
            ))),
        };
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char as char
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<()> {
        self.add_token_with_literal(token_type, Literal::String(String::new()))?;
        Ok(())
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) -> Result<()> {
        let text = String::from_utf8(self.source[self.start..self.current].into())?;
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
        Ok(())
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source[self.current] as char;
        }
    }

    fn read_string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(anyhow!(format!(
                "Unterminated string at line: {}",
                self.line
            )));
        }

        self.advance();

        let value = String::from_utf8(self.source[self.start..self.current].into())?;
        self.add_token_with_literal(TokenType::STRING, Literal::String(value))
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn read_number(&mut self) -> Result<()> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // We just consume the .
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number_bytes = &self.source[self.start..self.current];
        let number_str = String::from_utf8(number_bytes.into()).expect("Invalid UTF-8 sequence");
        let number: f64 = number_str.parse().unwrap();
        self.add_token_with_literal(TokenType::NUMBER, token::Literal::Float(number))
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1] as char
        }
    }

    fn is_alphabetic(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alphabetic(c)
    }

    fn read_identifier(&mut self) -> Result<()> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let value = String::from_utf8(self.source[self.start..self.current].into())?;
        let token_type = match token::lookup_keyword(value) {
            Some(token_type) => token_type,
            None => TokenType::IDENTIFIER,
        };

        self.add_token(token_type)?;
        Ok(())
    }
}
