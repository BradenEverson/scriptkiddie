//! All Token primatives and utilities wrapped around these primatives

/// A contextual token with location and typing
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The token's type
    pub token_type: TokenType,
    /// The token's line
    pub line: usize,
    /// The token's column
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

/// A single lexical token's type
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Number(f64),
    String(String),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
}

/// A keyword token's variants
#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Const,
    Var,
    For,
}

impl Keyword {
    /// Checks if a string is a valid keyword, if so, returns the keyword that it is
    pub fn to_keyword(check: &str) -> Option<Keyword> {
        match check {
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "var" => Some(Keyword::Var),
            "for" => Some(Keyword::For),

            _ => None,
        }
    }
}

/// All operation types
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Assignment,
    Not,
    Add,
    Sub,
    Div,
    Mod,
    Mult,
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Dot,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    Inc,
    Dec,
}

impl Operator {
    /// Checks if a string is a valid keyword, if so, returns the keyword that it is
    pub fn to_operator(check: &str) -> Option<Operator> {
        match check {
            "=" => Some(Operator::Assignment),
            "!" => Some(Operator::Not),
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "/" => Some(Operator::Div),
            "%" => Some(Operator::Mod),
            "*" => Some(Operator::Mult),
            "==" => Some(Operator::Eq),
            "!=" => Some(Operator::Ne),
            ">" => Some(Operator::Gt),
            ">=" => Some(Operator::Gte),
            "<" => Some(Operator::Lt),
            "<=" => Some(Operator::Lte),
            "." => Some(Operator::Dot),
            "+=" => Some(Operator::AddAssign),
            "-=" => Some(Operator::SubAssign),
            "*=" => Some(Operator::MulAssign),
            "/=" => Some(Operator::DivAssign),
            "%=" => Some(Operator::ModAssign),
            "++" => Some(Operator::Inc),
            "--" => Some(Operator::Dec),
            _ => None,
        }
    }
}

/// All punctuation types
#[derive(Clone, Debug, PartialEq)]
pub enum Punctuation {
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenSquiggle,
    CloseSquiggle,
    OpenBracket,
    CloseBracket,
}

impl Punctuation {
    /// Checks if a string is a valid punctuation mark, if so, returns the punctuation that it is
    pub fn to_puncutation(check: &str) -> Option<Punctuation> {
        match check {
            ";" => Some(Punctuation::Semicolon),
            "," => Some(Punctuation::Comma),
            "(" => Some(Punctuation::OpenParen),
            ")" => Some(Punctuation::CloseParen),
            "{" => Some(Punctuation::OpenSquiggle),
            "}" => Some(Punctuation::CloseSquiggle),
            "[" => Some(Punctuation::OpenBracket),
            "]" => Some(Punctuation::CloseBracket),
            _ => None,
        }
    }
}
