use crate::token::{lookup_ident, Token, TokenType}; // Adjust this path according to your project structure

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,      // Current position in input (points to current char)
    read_position: usize, // Current reading position in input (after current char)
    ch: char,             // Current char under examination
}

impl Lexer {
    // Constructor to initialize the Lexer with input
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0', // Representing end of file as null character initially
        };
        l.read_char(); // Initialize the first character
        l
    }

    // Function to read the next character
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // EOF
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Function to peek at the next character without consuming it
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0' // EOF
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    // Function to skip whitespace
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    // Function to read an identifier
    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    // Function to read a number
    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    // The main method to get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); // Move to next '=' to consume it
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            ',' => Token::new(TokenType::Comma, ","),
            '+' => Token::new(TokenType::Plus, "+"),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            '-' => Token::new(TokenType::Minus, "-"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); // Move to next '=' to consume it
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            '*' => Token::new(TokenType::Asterisk, "*"),
            '/' => Token::new(TokenType::Slash, "/"),
            '<' => Token::new(TokenType::LT, "<"),
            '>' => Token::new(TokenType::GT, ">"),
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let ident = self.read_identifier();
                    return Token::new(lookup_ident(&ident), &ident);
                } else if self.ch.is_digit(10) {
                    return Token::new(TokenType::Int(self.read_number()), &self.read_number());
                } else if self.ch == '\0' {
                    Token::new(TokenType::EOF, "")
                } else {
                    Token::new(TokenType::Illegal, &self.ch.to_string())
                }
            }
        };
        self.read_char();
        token
    }
}
