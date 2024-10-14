//! AST Implementations

use scriptkiddie_lexer::token::Token;

/// An AST Node
pub enum AstNode {
    /// A value node in the tree
    Value(Token),
    /// An operation on a single node
    UnaryOp(Box<AstNode>),
    /// An operation on two nodes
    BinaryOp(Box<AstNode>, Box<AstNode>),
}
