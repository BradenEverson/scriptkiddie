//! Parser rules for variable declarations

use scriptkiddie_lexer::token::{Keyword, Operator, Punctuation, Token, TokenType};

use crate::ast::{ASTNode, VariableKind};

use super::{AstParseError, Parser, Result};

impl<'lex> Parser<'lex> {
    /// Parses a variable declaration with a scope
    pub(crate) fn parse_declaration(&mut self) -> Result<ASTNode> {
        let kind = match self.lexer.next() {
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Let),
                ..
            }) => VariableKind::Let,
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Const),
                ..
            }) => VariableKind::Const,
            Some(Token {
                token_type: TokenType::Keyword(Keyword::Var),
                ..
            }) => VariableKind::Var,
            Some(token) => return Err(AstParseError::UnexpectedToken(token)),
            None => return Err(AstParseError::UnexpectedEof),
        };

        let name = match self.lexer.next() {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name,
            Some(token) => return Err(AstParseError::UnexpectedToken(token)),
            None => return Err(AstParseError::UnexpectedEof),
        };

        let initializer = if let Some(Token {
            token_type: TokenType::Operator(Operator::Assignment),
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume_punctuation(Punctuation::Semicolon)?;

        Ok(ASTNode::VariableDeclaration {
            kind,
            name,
            initializer,
        })
    }

    fn consume_punctuation(&mut self, expected: Punctuation) -> Result<()> {
        match self.lexer.next() {
            Some(Token {
                token_type: TokenType::Punctuation(punc),
                ..
            }) if punc == expected => Ok(()),
            Some(token) => Err(AstParseError::UnexpectedToken(token)),
            None => Err(AstParseError::UnexpectedEof),
        }
    }
}

#[cfg(test)]
mod tests {
    use scriptkiddie_lexer::lexer::Lexer;

    use crate::{
        ast::{ASTNode, VariableKind},
        parser::Parser,
    };

    #[test]
    fn parse_declarations() {
        let input = "let a;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::from_lexer(&mut lexer);

        let parsed = parser.parse_program().expect("Failed to parse expression");
        assert_eq!(
            ASTNode::Program(vec![ASTNode::VariableDeclaration {
                kind: VariableKind::Let,
                name: "a".into(),
                initializer: None
            }]),
            parsed
        )
    }
}
