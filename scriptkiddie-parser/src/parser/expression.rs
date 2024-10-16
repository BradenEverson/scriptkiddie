//! Parser rules for expression statements

use crate::{ast::ASTNode, syntax::SYNTAX_PATTERNS};

use super::{AstParseError, Parser, Result};

impl<'lex> Parser<'lex> {
    /// Parses an expression as an AST Node
    pub(crate) fn parse_expression(&mut self) -> Result<ASTNode> {
        for expression in SYNTAX_PATTERNS.iter() {
            if expression.matches_pattern(self.tokens, self.place) {
                return expression.parse_grammar(self.tokens, &mut self.place);
            }
        }
        Err(AstParseError::UnknownTokenPattern)
    }
}
