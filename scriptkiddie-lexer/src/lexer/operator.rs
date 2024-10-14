//! Lexer implementation for reading operator and punctuation syntax

use crate::token::{Operator, Punctuation, Token, TokenType};

use super::Lexer;

impl Lexer {
    /// Collects an operator or punctuation token
    pub(crate) fn collect_operator_or_punctuation(&mut self) -> Token {
        let start = self.column;
        let mut c = self.current_char().unwrap().to_string();

        match self.peek_char() {
            Some(&'=') => {
                c += "=";
                self.advance();
            }
            Some(&'+') => {
                c += "+";
                self.advance();
            }
            Some(&'-') => {
                c += "-";
                self.advance();
            }
            _ => {}
        }

        let token_type = if let Some(op) = Operator::to_operator(&c) {
            TokenType::Operator(op)
        } else {
            let punc = Punctuation::to_puncutation(&c).unwrap();
            TokenType::Punctuation(punc)
        };

        self.advance();

        Token::new(token_type, self.line, start)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Operator, Punctuation, TokenType},
    };

    #[test]
    fn lexer_collects_operators() {
        let input = "*2".to_string();
        let mut lexer = Lexer::new(input);

        let operator = lexer.collect_operator_or_punctuation();
        if let TokenType::Operator(op) = operator.token_type {
            assert_eq!(op, Operator::Mult)
        } else {
            panic!("Token was not an operator")
        }
    }

    #[test]
    fn lexer_collects_eq_operators() {
        let input = ">=2".to_string();
        let mut lexer = Lexer::new(input);

        let operator = lexer.collect_operator_or_punctuation();
        if let TokenType::Operator(op) = operator.token_type {
            assert_eq!(op, Operator::Gte)
        } else {
            panic!("Token was not an operator")
        }
    }

    #[test]
    fn lexer_collects_punctuation() {
        let input = ";".to_string();
        let mut lexer = Lexer::new(input);

        let operator = lexer.collect_operator_or_punctuation();
        if let TokenType::Punctuation(punc) = operator.token_type {
            assert_eq!(Punctuation::Semicolon, punc)
        } else {
            panic!("Token was not an operator")
        }
    }

    #[test]
    fn lexer_collects_inc_dec() {
        let input = "++".to_string();
        let mut lexer = Lexer::new(input);

        let operator = lexer.collect_operator_or_punctuation();
        if let TokenType::Operator(op) = operator.token_type {
            assert_eq!(op, Operator::Inc)
        } else {
            panic!("Token was not an operator")
        }
    }
}
