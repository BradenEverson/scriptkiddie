//! Trait definitions for all expression grammar. An instance of every struct that implements this
//! is held in a static array to be used at parse-time

use std::cell::LazyCell;

use binary::BinaryExpr;
use scriptkiddie_lexer::token::Token;

use crate::{ast::ASTNode, parser::Result};

pub mod binary;

/// Dynamic array of all syntax patterns we want to check on parse
pub const SYNTAX_PATTERNS: LazyCell<[Box<dyn SyntaxGrammar>; 1]> =
    LazyCell::new(|| [Box::new(BinaryExpr)]);

/// The trait that defines what a syntax pattern looks like and what expression it may map to.
/// Defines methods both for identifying if the pattern is valid for a current spot in a Token
/// Iterator, and methods for creating an ASTNode based on this pattern
pub trait SyntaxGrammar {
    /// Identifies if the current point of the lexer matches the desired pattern, constructing an
    /// ASTNode from it if so
    fn parse_grammar(&self, tokens: &[Token], place: &mut usize) -> Result<ASTNode>;
    /// Immutably checks if the lexer's current position (plus peeks) matches the syntax pattern
    fn matches_pattern(&self, tokens: &[Token], place: usize) -> bool;
}
