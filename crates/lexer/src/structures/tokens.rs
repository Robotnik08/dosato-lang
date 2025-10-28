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

    StringTemplate(StringTemplatePart),
    StringTemplateEnd(StringTemplatePart),
}

pub enum BracketType {
    // (u16) is depth of nested brackets, so they can be paired correctly
    Parenthesis(u16),
    Brace(u16),
    Bracket(u16),
}

impl PartialEq for BracketType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BracketType::Parenthesis(a), BracketType::Parenthesis(b)) => a == b,
            (BracketType::Brace(a), BracketType::Brace(b)) => a == b,
            (BracketType::Bracket(a), BracketType::Bracket(b)) => a == b,
            _ => false,
        }
    }
}

pub struct StringTemplatePart {
    pub id: u16,            // id for identifying which template this part belongs to
    pub value: String,      // The string content of the template part
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

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            kind: match &self.kind {
                TokenKind::Comment(content) => TokenKind::Comment(content.clone()),
                TokenKind::StringLiteral(content) => TokenKind::StringLiteral(content.clone()),
                TokenKind::CharLiteral(content) => TokenKind::CharLiteral(*content),
                TokenKind::NumberLiteral(content) => TokenKind::NumberLiteral(*content),
                TokenKind::IntegerLiteral(content) => TokenKind::IntegerLiteral(*content),
                TokenKind::BooleanLiteral(content) => TokenKind::BooleanLiteral(*content),
                TokenKind::Null => TokenKind::Null,
                TokenKind::Infinity => TokenKind::Infinity,
                TokenKind::NaN => TokenKind::NaN,
                TokenKind::Operator(op) => TokenKind::Operator(*op),
                TokenKind::KeyWord(kw) => TokenKind::KeyWord(*kw),
                TokenKind::Identifier(name) => TokenKind::Identifier(name.clone()),
                TokenKind::BracketOpen(bracket) => match bracket {
                    BracketType::Parenthesis(depth) => TokenKind::BracketOpen(BracketType::Parenthesis(*depth)),
                    BracketType::Brace(depth) => TokenKind::BracketOpen(BracketType::Brace(*depth)),
                    BracketType::Bracket(depth) => TokenKind::BracketOpen(BracketType::Bracket(*depth)),
                },
                TokenKind::BracketClose(bracket) => match bracket {
                    BracketType::Parenthesis(depth) => TokenKind::BracketClose(BracketType::Parenthesis(*depth)),
                    BracketType::Brace(depth) => TokenKind::BracketClose(BracketType::Brace(*depth)),
                    BracketType::Bracket(depth) => TokenKind::BracketClose(BracketType::Bracket(*depth)),
                },
                TokenKind::StringTemplate(part) => TokenKind::StringTemplate(StringTemplatePart { id: part.id, value: part.value.clone() }),
                TokenKind::StringTemplateEnd(part) => TokenKind::StringTemplateEnd(StringTemplatePart { id: part.id, value: part.value.clone() }),
            },
            span: self.span,
            position: self.position.clone(),
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
            TokenKind::StringTemplate(part) => write!(f, "StringTemplate(id={}, value=\"{}\")", part.id, part.value.escape_default()),
            TokenKind::StringTemplateEnd(part) => write!(f, "StringTemplateEnd(id={}, value=\"{}\")", part.id, part.value.escape_default()),
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