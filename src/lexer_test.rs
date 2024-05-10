#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected_tokens = [
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_type, expected_literal) in expected_tokens.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok.typ, *expected_type);
            assert_eq!(tok.literal, *expected_literal);
        }
    }
}
