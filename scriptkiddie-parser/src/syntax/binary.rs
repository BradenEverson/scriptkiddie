//! Binary Expression Implementations

use super::SyntaxGrammar;

/// A binary expression pattern matcher
pub struct BinaryExpr;

impl SyntaxGrammar for BinaryExpr {
    fn parse_grammar(&self, tokens: &[scriptkiddie_lexer::token::Token], place: &mut usize) -> crate::parser::Result<crate::ast::ASTNode> {
        todo!()
    }
    fn matches_pattern(&self, tokens: &[scriptkiddie_lexer::token::Token], place: usize) -> bool {
        todo!()
    }
}
