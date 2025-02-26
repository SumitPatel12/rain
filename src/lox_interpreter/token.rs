use core::fmt;
use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NONE,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
    BREAK,
    CONTINUE,
}

// TODO: Check if this is the correct way to do this.
const KEYWORDS: [(&str, TokenType); 18] = [
    ("and", TokenType::AND),
    ("class", TokenType::CLASS),
    ("else", TokenType::ELSE),
    ("false", TokenType::FALSE),
    ("for", TokenType::FOR),
    ("fun", TokenType::FUN),
    ("if", TokenType::IF),
    ("nil", TokenType::NONE),
    ("or", TokenType::OR),
    ("print", TokenType::PRINT),
    ("return", TokenType::RETURN),
    ("super", TokenType::SUPER),
    ("this", TokenType::THIS),
    ("true", TokenType::TRUE),
    ("var", TokenType::VAR),
    ("while", TokenType::WHILE),
    ("break", TokenType::BREAK),
    ("continue", TokenType::CONTINUE),
];

pub fn lookup_keyword(keyword: String) -> Option<TokenType> {
    for (key, token) in KEYWORDS {
        if key == keyword {
            return Some(token);
        }
    }
    None
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    None,
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Boolean(bool) => write!(f, "{}", bool),
            Literal::None => write!(f, "none"),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::String(string) => write!(f, "{}", string),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Literal,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        // TODO: Replace with someting better. The book doesn't define the object right now so
        // going with a generic.
        literal: Literal,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token Strat: Token Type: {:?} Lexeme: {} Literal: {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
