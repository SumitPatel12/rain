use core::fmt;

use super::token::{Token, TokenType};

pub fn error(column: usize, line: usize, message: &str) {
    report(column, line, "", message);
}

pub fn report(column: usize, line: usize, position: &str, message: &str) {
    eprintln!(
        "[column: {}, line {}] Error {}: {}",
        column, line, position, message
    );
}

pub fn parse_error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.column, token.line, " end of file", message);
    } else {
        report(
            token.column,
            token.line,
            &format!(" at '{}'", token.lexeme),
            message,
        );
    }
}

#[derive(Debug, Clone)]
pub enum LoxError {
    Parse,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Parse => write!(f, "ParseError"),
        }
    }
}
