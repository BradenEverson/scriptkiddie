//! Lexical parsing logic

use crate::token::Token;

pub mod keyword;
pub mod number;
pub mod string;
pub mod operator;
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
                self.skip_single_line_comment()
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
