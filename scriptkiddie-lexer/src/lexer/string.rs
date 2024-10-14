//! Lexer implementations for tokenizing an entire string

use crate::token::{Token, TokenType};

use super::Lexer;

impl Lexer {
    /// Reads a complete string within two quote marks
    pub(crate) fn collect_string(&mut self) -> Token {
        let start = self.column;
        let mut result = String::new();
        self.advance();
        let mut closed = false;

        while let Some(c) = self.current_char() {
            match *c {
                '"' => {
                    self.advance();
                    closed = true;
                    break;
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current_char() {
                        result.push(*escaped)
                    }
                    self.advance();
                }
                _ => {
                    result.push(*c);
                    self.advance();
                }
            }
        }

        if !closed {
            todo!("Return error for unclosed string")
        }

        Token::new(TokenType::String(result), self.line, start)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn lexer_read_string_reads_entire_string() {
        let input = "\"Hello World!\"".to_string();
        let mut lexer = Lexer::new(input);

        let string = lexer.collect_string();
        if let TokenType::String(val) = string.token_type {
            assert_eq!("Hello World!", val)
        } else {
            panic!("Token type was not a string")
        }
    }

    #[test]
    fn lexer_read_string_with_escapes() {
        let input = "\"Hello \'World!\'\"".to_string();
        let mut lexer = Lexer::new(input);

        let string = lexer.collect_string();
        if let TokenType::String(val) = string.token_type {
            assert_eq!("Hello 'World!'", val)
        } else {
            panic!("Token type was not a string")
        }
    }
}
