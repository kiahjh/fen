#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // types
    Int,
    Float,
    String,
    Bool,
    Date,
    Uuid,

    // syntax
    QuestionMark,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    At,
    Rule,
    StringLiteral(String),
    BoolLiteral(bool),

    // identifiers
    Identifier(String),

    // other
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub index: usize,
}

impl Token {
    pub const fn new(kind: TokenKind, index: usize) -> Self {
        Self { kind, index }
    }
}
