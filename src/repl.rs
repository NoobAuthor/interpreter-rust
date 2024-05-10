use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // Print the prompt
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        // Read user input
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                // EOF reached, e.g., Ctrl-D pressed
                break;
            }
            Ok(_) => {
                let mut lexer = Lexer::new(&line);
                loop {
                    let token = lexer.next_token();
                    if token.typ == TokenType::EOF {
                        break;
                    }
                    println!("{:?}", token);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        }
    }
}
