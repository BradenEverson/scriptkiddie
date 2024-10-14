//! Parser struct definitions

use scriptkiddie_lexer::lexer::Lexer;

/// A parser that holds onto a mutable context of a Lexer
pub struct Parser<'lex> {
    lexer: &'lex mut Lexer
}
