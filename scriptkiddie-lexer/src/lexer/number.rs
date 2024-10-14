//! Lexer implementation for reading a number

use crate::token::{Token, TokenType};

use super::Lexer;

impl Lexer {
    /// Collects an entire number as a token
    fn collect_number(&mut self) -> Token {
        let start = self.column;
        let mut number_str = String::new();
        let mut has_decimal = false;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number_str.push(*c);
                self.advance();
            } else if c == &'.' && !has_decimal {
                has_decimal = true;
                number_str.push(*c);
                self.advance();
            } else {
                break;
            }
        }

        let number_value = number_str.parse::<f64>().unwrap_or(0.0);

        Token::new(TokenType::Number(number_value), self.line, start)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn lexer_collect_number() {
        let input = "150".to_string();
        let mut lexer = Lexer::new(input);

        let number_token = lexer.collect_number();
        if let TokenType::Number(num) = number_token.token_type {
            assert_eq!(150.0, num)
        } else {
            panic!("Token was not a number")
        }
    }

    #[test]
    fn lexer_collect_number_with_decimal() {
        let input = "3.14".to_string();
        let mut lexer = Lexer::new(input);

        let number_token = lexer.collect_number();
        if let TokenType::Number(num) = number_token.token_type {
            assert_eq!(3.14, num)
        } else {
            panic!("Token was not a number")
        }
    }
}
