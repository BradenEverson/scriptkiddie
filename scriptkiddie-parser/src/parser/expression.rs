//! Parser rules for expression statements

use crate::ast::ASTNode;

use super::{Parser, Result};

impl<'lex> Parser<'lex> {
    /// Parses an expression as an AST Node
    pub(crate) fn parse_expression(&mut self) -> Result<ASTNode> {
        todo!()
    }
}
