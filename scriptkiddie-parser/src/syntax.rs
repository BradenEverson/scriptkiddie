//! Trait definitions for all expression grammar. An instance of every struct that implements this
//! is held in a static array to be used at parse-time

use std::iter::Peekable;

use scriptkiddie_lexer::lexer::Lexer;

use crate::{ast::ASTNode, parser::Result};

/// Dynamic array of all syntax patterns we want to check on parse
pub const SYNTAX_PATTERNS: &[Box<dyn SyntaxGrammar>] = &[];

/// The trait that defines what a syntax pattern looks like and what expression it may map to.
/// Defines methods both for identifying if the pattern is valid for a current spot in a Token
/// Iterator, and methods for creating an ASTNode based on this pattern
pub trait SyntaxGrammar {
    /// Identifies if the current point of the lexer matches the desired pattern, constructing an
    /// ASTNode from it if so
    fn parse_grammar(&self, lexer: &mut Peekable<&mut Lexer>) -> Result<ASTNode>;
    /// Immutably checks if the lexer's current position (plus peeks) matches the syntax pattern
    fn matches_pattern(&self, lexer: &Peekable<&mut Lexer>) -> bool;
}
