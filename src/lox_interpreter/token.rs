use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

const KEYWORDS: [(&str, TokenType); 16] = [
    ("and", TokenType::AND),
    ("class", TokenType::CLASS),
    ("else", TokenType::ELSE),
    ("false", TokenType::FALSE),
    ("for", TokenType::FOR),
    ("fun", TokenType::FUN),
    ("if", TokenType::IF),
    ("nil", TokenType::NIL),
    ("or", TokenType::OR),
    ("print", TokenType::PRINT),
    ("return", TokenType::RETURN),
    ("super", TokenType::SUPER),
    ("this", TokenType::THIS),
    ("true", TokenType::TRUE),
    ("var", TokenType::VAR),
    ("while", TokenType::WHILE),
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
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Literal,
    line: usize,
    column: usize,
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
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
