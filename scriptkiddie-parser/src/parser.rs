//! Parser struct definitions

use scriptkiddie_lexer::lexer::Lexer;

/// A parser that holds onto a mutable context of a Lexer
pub struct Parser<'lex> {
    /// The internal lexer session
    lexer: &'lex mut Lexer,
}

impl<'lex> Parser<'lex> {
    /// Creates a new parser session from a lexer
    pub fn from_lexer(lexer: &'lex mut Lexer) -> Self {
        Self { lexer  }
    }
}
