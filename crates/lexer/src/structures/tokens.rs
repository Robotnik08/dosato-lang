use crate::structures::keywords::KeyWord;
use crate::structures::operators::Operator;

pub enum TokenKind {
    Comment(String),

    StringLiteral(String),
    CharLiteral(char),
    NumberLiteral(f64),
    IntegerLiteral(i64), // For 0x, 0o, 0b prefixed numbers
    BooleanLiteral(bool),
    Null,
    Infinity,
    NaN,

    Operator(Operator),

    KeyWord(KeyWord),

    Identifier(String),

    BracketOpen(BracketType),
    BracketClose(BracketType),

    TemplateStart,
    TemplateEnd,
}

pub enum BracketType {
    // (u16) is depth of nested brackets, so they can be paired correctly
    Parenthesis(u16),
    Brace(u16),
    Bracket(u16),
}

pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize), // (start, end) in the source code
    pub position: TokenPosition,
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize), position: TokenPosition) -> Self {
        Self { kind, span, position }
    }
}