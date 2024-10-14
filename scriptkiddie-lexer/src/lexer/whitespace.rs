//! Whitespace skipping implementations for the lexer

use super::Lexer;

impl Lexer {
    /// Skips whitespace until we are no longer at a whitespace char
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }
}
