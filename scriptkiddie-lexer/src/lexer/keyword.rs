//! Lexer implementations for reading keywords, identifiers and operators

use crate::token::{Keyword, Token, TokenType};

use super::Lexer;

impl Lexer {
    /// Reads an entire identifier/keyword and registers it as such
    pub(crate) fn collect_identifier_or_keyword(&mut self) -> Token {
        let start = self.column;
        let mut result = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == &'_' || c == &'$' {
                result.push(*c);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if let Some(keyword) = Keyword::to_keyword(&result) {
            TokenType::Keyword(keyword)
        } else {
            TokenType::Identifier(result)
        };

        Token::new(token_type, self.line, start)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Keyword, TokenType},
    };

    #[test]
    fn lexer_reads_an_identifier() {
        let input = "foo = 100".to_string();
        let mut lexer = Lexer::new(input);

        let token = lexer.collect_identifier_or_keyword();
        if let TokenType::Identifier(word) = token.token_type {
            assert_eq!("foo", word)
        } else {
            panic!("Token was not an identifier")
        }
    }

    #[test]
    fn lexer_reads_a_keyword() {
        let input = "let foo = 100".to_string();
        let mut lexer = Lexer::new(input);

        let token = lexer.collect_identifier_or_keyword();
        if let TokenType::Keyword(word) = token.token_type {
            assert_eq!(Keyword::Let, word)
        } else {
            panic!("Token was not a keyword")
        }
    }
}
