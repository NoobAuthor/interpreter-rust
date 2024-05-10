// Define the TokenType as an enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers and literals
    Ident(String), // Identifier
    Int(String),   // Integer literals

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    // Comparison
    LT,    // Less than
    GT,    // Greater than
    Eq,    // Equal
    NotEq, // Not equal
    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

// Define the Token struct
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

// Implement a constructor for Token
impl Token {
    pub fn new(typ: TokenType, literal: &str) -> Self {
        Token {
            typ,
            literal: literal.to_string(),
        }
    }
}

// Keywords mapping
use std::collections::HashMap;

pub fn lookup_ident(ident: &str) -> TokenType {
    let keywords: HashMap<&str, TokenType> = [
        ("fn", TokenType::Function),
        ("let", TokenType::Let),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
    ]
    .iter()
    .cloned()
    .collect();

    keywords
        .get(ident)
        .cloned()
        .unwrap_or(TokenType::Ident(ident.to_string()))
}
