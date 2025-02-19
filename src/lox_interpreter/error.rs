use std::{io, string::FromUtf8Error};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("Error Converting to sting from UTF8")]
    FromUTF8(#[from] FromUtf8Error),
    #[error("Error: {0}")]
    Error(String),
    #[error("Io Error")]
    IoError(#[from] io::Error),
    #[error("Error Parsing File")]
    Parse,
    #[error("Runtime enrorn: Message: {message:?}")]
    Runtime { token: Token, message: String },
}
