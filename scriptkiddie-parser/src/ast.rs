//! AST Implementations

use scriptkiddie_lexer::token::Operator;

/// An ASTNode built from tokens
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// An entire program's span
    Program(Vec<ASTNode>),
    /// A function wrt it's name, params and body
    FunctionDeclaration {
        /// Function name
        name: String,
        /// Function parameters
        params: Vec<String>,
        /// Function body
        body: Vec<ASTNode>,
    },
    /// Declare a variable
    VariableDeclaration {
        /// Whether the variable is let, const or var
        kind: VariableKind,
        /// Variable name
        name: String,
        /// What defines the variable
        initializer: Option<Box<ASTNode>>,
    },
    /// An arbitrary expression
    ExpressionStatement(Box<ASTNode>),
    /// A binary expression
    BinaryExpression {
        /// The operator acting on two operands
        operator: Operator,
        /// The left operand
        left: Box<ASTNode>,
        /// The right operand
        right: Box<ASTNode>,
    },
    /// An identifier
    Identifier(String),
    /// A number
    NumberLiteral(f64),
    /// A string
    StringLiteral(String),
}

/// The scopes a variable can have
#[derive(Debug, Clone)]
pub enum VariableKind {
    /// Let scope
    Let,
    /// Const scope
    Const,
    /// Var scope
    Var,
}
