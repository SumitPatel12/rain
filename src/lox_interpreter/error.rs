use core::fmt;

use super::token::{Token, TokenType};

pub fn report(column: usize, line: usize, position: &str, message: &str) {
    eprintln!(
        "[column: {}, line {}] Error {}: {}",
        column, line, position, message
    );
}

pub fn report_parse_error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.column, token.line, " at end of file", message);
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
