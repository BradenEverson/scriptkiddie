//! Lexical parsing logic

use crate::token::Token;

pub mod keyword;
pub mod number;
pub mod operator;
pub mod string;
pub mod whitespace_comments;

/// The lexer struct responsible for reading a stream of text and converting it into tokens. Can be
/// treated as a token iterator
pub struct Lexer {
    /// The input context
    input: Vec<char>,
    /// The placement position of the input
    pos: usize,
    /// The current line
    line: usize,
    /// The current column
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut token = None;
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.skip_whitespace()
            } else if c.is_alphabetic() || c == &'_' {
                token = Some(self.collect_identifier_or_keyword());
                break;
            } else if c.is_ascii_digit() {
                token = Some(self.collect_number());
                break;
            } else if c == &'"' || c == &'\'' {
                token = Some(self.collect_string());
                break;
            } else if c == &'/' && self.peek_char() == Some(&'/') {
                self.skip_single_line_comment()
            } else if c == &'/' && self.peek_char() == Some(&'*') {
                self.skip_multi_line_comment()
            } else {
                token = Some(self.collect_operator_or_punctuation());
                break;
            }
        }

        token
    }

    /// Advances the lexer by 1 position, advancing line if need be
    fn advance(&mut self) {
        if self.current_char() == Some(&'\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.pos += 1;
    }

    /// Gets the current character at the current position
    fn current_char(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    /// Peeks at the next char
    fn peek_char(&self) -> Option<&char> {
        self.input.get(self.pos + 1)
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::token::{Keyword, Operator, Punctuation, TokenType};

    use super::Lexer;

    #[test]
    fn lexer_iterator_full_lexing() {
        let mut input_file = std::fs::File::open("../test/simple.js").expect("Failed to read file");
        let mut text = String::new();
        input_file
            .read_to_string(&mut text)
            .expect("Failed to read file");

        let lexer = Lexer::new(text);
        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        let should_be = vec![
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("a".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Number(100.0),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("b".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Number(20.0),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("c".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Identifier("a".to_string()),
            TokenType::Operator(Operator::Add),
            TokenType::Identifier("b".to_string()),
            TokenType::Punctuation(Punctuation::Semicolon),
        ];

        assert_eq!(tokens, should_be)
    }

    #[test]
    fn lexer_tokenizes_string_types() {
        let input = "let foo = \"Hello!\";".to_string();
        let lexer = Lexer::new(input);

        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        let should_be = vec![
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("foo".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::String("Hello!".to_string()),
            TokenType::Punctuation(Punctuation::Semicolon),
        ];

        assert_eq!(tokens, should_be)
    }

    #[test]
    fn lexer_tokenizes_punctuation() {
        let input = "for (let i = 0; i < 10; i++) {}".to_string();
        let lexer = Lexer::new(input);

        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        let should_be = vec![
            TokenType::Keyword(Keyword::For),
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("i".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Number(0.0),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Identifier("i".to_string()),
            TokenType::Operator(Operator::Lt),
            TokenType::Number(10.0),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Identifier("i".to_string()),
            TokenType::Operator(Operator::Inc),
            TokenType::Punctuation(Punctuation::CloseParen),
            TokenType::Punctuation(Punctuation::OpenSquiggle),
            TokenType::Punctuation(Punctuation::CloseSquiggle),
        ];

        assert_eq!(tokens, should_be)
    }

    #[test]
    fn lexer_tokenizes_functions() {
        let mut input_file =
            std::fs::File::open("../test/functions.js").expect("Failed to read file");
        let mut text = String::new();
        input_file
            .read_to_string(&mut text)
            .expect("Failed to read file");

        let lexer = Lexer::new(text);
        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        let should_be = vec![
            TokenType::Keyword(Keyword::Function),
            TokenType::Identifier("foo".to_string()),
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::Punctuation(Punctuation::CloseParen),
            TokenType::Punctuation(Punctuation::OpenSquiggle),
            TokenType::Identifier("Console".to_string()),
            TokenType::Operator(Operator::Dot),
            TokenType::Identifier("log".to_string()),
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::String("Bar".to_string()),
            TokenType::Punctuation(Punctuation::CloseParen),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Punctuation(Punctuation::CloseSquiggle),
            TokenType::Keyword(Keyword::Function),
            TokenType::Identifier("add".to_string()),
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::Identifier("a".to_string()),
            TokenType::Punctuation(Punctuation::Comma),
            TokenType::Identifier("b".to_string()),
            TokenType::Punctuation(Punctuation::CloseParen),
            TokenType::Punctuation(Punctuation::OpenSquiggle),
            TokenType::Keyword(Keyword::Return),
            TokenType::Identifier("a".to_string()),
            TokenType::Operator(Operator::Add),
            TokenType::Identifier("b".to_string()),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Punctuation(Punctuation::CloseSquiggle),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("a".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Number(100.0),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("b".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Number(5.6),
            TokenType::Punctuation(Punctuation::Semicolon),
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("c".to_string()),
            TokenType::Operator(Operator::Assignment),
            TokenType::Identifier("add".to_string()),
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::Identifier("a".to_string()),
            TokenType::Punctuation(Punctuation::Comma),
            TokenType::Identifier("b".to_string()),
            TokenType::Punctuation(Punctuation::CloseParen),
            TokenType::Punctuation(Punctuation::Semicolon),
        ];

        assert_eq!(tokens, should_be)
    }
}
