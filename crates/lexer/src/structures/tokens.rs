use crate::structures::keywords::KeyWord;
use crate::structures::operators::Operator;

pub enum TokenKind {
    Comment(String),

    StringLiteral(String),
    CharLiteral(char),
    NumberLiteral(f64),
    IntegerLiteral(i64), // For whole numbers (without a dot), or 0x, 0o, 0b prefixed numbers.
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

impl Clone for TokenPosition {
    fn clone(&self) -> Self {
        Self {
            line: self.line,
            column: self.column,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize), position: TokenPosition) -> Self {
        Self { kind, span, position }
    }
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Comment(content) => write!(f, "Comment({})", content.escape_default()),
            TokenKind::StringLiteral(content) => write!(f, "StringLiteral(\"{}\")", content.escape_default()),
            TokenKind::CharLiteral(content) => write!(f, "CharLiteral('{}')", content.escape_default()),
            TokenKind::NumberLiteral(content) => write!(f, "NumberLiteral({})", content),
            TokenKind::IntegerLiteral(content) => write!(f, "IntegerLiteral({})", content),
            TokenKind::BooleanLiteral(content) => write!(f, "BooleanLiteral({})", content),
            TokenKind::Null => write!(f, "Null"),
            TokenKind::Infinity => write!(f, "Infinity"),
            TokenKind::NaN => write!(f, "NaN"),
            TokenKind::Operator(op) => write!(f, "Operator({:?})", op),
            TokenKind::KeyWord(kw) => write!(f, "KeyWord({:?})", kw),
            TokenKind::Identifier(name) => write!(f, "Identifier({})", name),
            TokenKind::BracketOpen(bracket) => match bracket {
                BracketType::Parenthesis(depth) => write!(f, "BracketOpen(Parenthesis, depth={})", depth),
                BracketType::Brace(depth) => write!(f, "BracketOpen(Brace, depth={})", depth),
                BracketType::Bracket(depth) => write!(f, "BracketOpen(Bracket, depth={})", depth),
            },
            TokenKind::BracketClose(bracket) => match bracket {
                BracketType::Parenthesis(depth) => write!(f, "BracketClose(Parenthesis, depth={})", depth),
                BracketType::Brace(depth) => write!(f, "BracketClose(Brace, depth={})", depth),
                BracketType::Bracket(depth) => write!(f, "BracketClose(Bracket, depth={})", depth),
            },
            TokenKind::TemplateStart => write!(f, "TemplateStart"),
            TokenKind::TemplateEnd => write!(f, "TemplateEnd"),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ kind: {:?}, span: ({}, {}), position: {:?} }}",
            self.kind, self.span.0, self.span.1, self.position
        )
    }
}

impl std::fmt::Debug for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.column)
    }
}