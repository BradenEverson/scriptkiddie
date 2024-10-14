//! Whitespace skipping and comment ignoring implementations for the lexer

use super::Lexer;

impl Lexer {
    /// Skips whitespace until we are no longer at a whitespace char
    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }

    /// Skips a single line comment
    pub(crate) fn skip_single_line_comment(&mut self) {
        while let Some(c) = self.current_char() {
            if c == &'\n' {
                break;
            }
            self.advance()
        }
    }

    /// Skips a multiline comment
    pub(crate) fn skip_multi_line_comment(&mut self) {
        // Skip the initial "/*"
        self.advance();
        self.advance();

        while let Some(c) = self.current_char() {
            if c == &'*' && self.peek_char() == Some(&'/') {
                self.advance();
                self.advance();
                break;
            }

            self.advance()
        }
    }
}
