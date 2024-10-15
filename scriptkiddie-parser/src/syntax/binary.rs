//! Binary Expression Implementations

use super::SyntaxGrammar;

/// A binary expression pattern matcher
pub struct BinaryExpr;

impl SyntaxGrammar for BinaryExpr {
    fn parse_grammar(
        &self,
        lexer: &mut std::iter::Peekable<&mut scriptkiddie_lexer::lexer::Lexer>,
    ) -> crate::parser::Result<crate::ast::ASTNode> {
        todo!()
    }

    fn matches_pattern(
        &self,
        lexer: &std::iter::Peekable<&mut scriptkiddie_lexer::lexer::Lexer>,
    ) -> bool {
        // A binary expression should have the form: 
        // E op E 
        // where E: Expression and op: Operator
        todo!()
    }
}
