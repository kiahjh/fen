#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // primitives
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
    Comma,

    // keywords
    Struct,
    Enum,

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
