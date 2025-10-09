use phf::phf_map;

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

    TypeKeyword(Types),

    TemplateStart,
    TemplateEnd,
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Assign,
    GreaterThen,
    LessThen,
    Not,
    And,
    Xor,
    Or,
    BitNot,
    Question,
    Colon,
    Arrow,
    Comma,
    Hash,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    Increment,
    Decrement,
    Equals,
    NotEquals,
    GreaterThanEqual,
    LessThanEqual,
    AndAnd,
    OrOr,
    XorXor,
    ShiftLeft,
    ShiftRight,
    AndAssign,
    OrAssign,
    XorAssign,
    Power,
    Root,
    Max,
    Min,
    Absolute,
    FatArrow,
    ShiftRightAssign,
    ShiftLeftAssign,
    PowerAssign,
    MaxAssign,
    MinAssign,
    Semicolon,
    RangeUp,
    RangeDown,
    RangeUpInclusive,
    RangeDownInclusive,
    NullCoalesce,
    NullCoalesceAssign,
    NullCoalesceAccess,
    RootAssign,
    Pipe,
    StrictEquals,
    TripleNotEquals,
    PipeAssign,
    Spaceship,
    ArrayUnwrap,
    AndAndAssign,
    OrOrAssign,
    XorXorAssign,
}

impl Operator {
    /// Return precedence: lower numbers bind more tightly (you mentioned lower => higher precedence).
    /// I kept your numeric mapping idea: smaller number = higher priority.
    pub fn precedence(&self) -> u8 {
        use Operator::*;
        match self {
            Arrow | Hash | NullCoalesceAccess => 0,
            Not | BitNot | Increment | Decrement | Absolute => 1,
            Power | Root | Min | Max => 2,
            Multiply | Divide | Modulo => 3,
            Add | Subtract => 4,
            ShiftLeft | ShiftRight => 5,
            GreaterThen | LessThen | GreaterThanEqual | LessThanEqual | Spaceship => 6,
            Equals | NotEquals | StrictEquals | TripleNotEquals => 7,
            And => 8,
            Xor => 9,
            Or => 10,
            AndAnd | XorXor => 11,
            OrOr | NullCoalesce | Pipe => 12,
            Question | Colon | Semicolon | RangeUp | RangeDown | RangeUpInclusive | RangeDownInclusive => 13,
            Comma | FatArrow => 15,

            _ => 14, // Assignment and others
        }
    }

    /// Is this operator considered an assignment-like operator?
    pub fn is_assignment(&self) -> bool {
        matches!(self,
            Operator::Assign
            | Operator::AddAssign
            | Operator::SubtractAssign
            | Operator::MultiplyAssign
            | Operator::DivideAssign
            | Operator::ModuloAssign
            | Operator::AndAssign
            | Operator::OrAssign
            | Operator::XorAssign
            | Operator::PowerAssign
            | Operator::ShiftRightAssign
            | Operator::ShiftLeftAssign
            | Operator::MaxAssign
            | Operator::MinAssign
            | Operator::NullCoalesceAssign
            | Operator::RootAssign
            | Operator::PipeAssign
            | Operator::AndAndAssign
            | Operator::OrOrAssign
            | Operator::XorXorAssign
            | Operator::ArrayUnwrap
        )
    }

    /// Is this a unary operator?
    pub fn is_unary(&self) -> bool {
        matches!(self,
            Operator::Not 
            | Operator::Increment 
            | Operator::Decrement 
            | Operator::Subtract 
            | Operator::BitNot
        )
    }
}

pub enum BracketType {
    // (u16) is depth of nested brackets, so they can be paired correctly
    Parenthesis(u16),
    Brace(u16),
    Bracket(u16),
}

pub enum Types {
    Any,

    Float,
    Int,
    Number, // Is same as Float, just different name
    
    Bool,
    
    String,
    Char,
    
    Array, // List of values
    Object, // Dictionary, Class instance
    Function, // Function pointer / lambda
    Class, // Class definition

    Void
}

pub enum KeyWord {
    Do,
    Then,

    If,
    IfNot,
    Else,
    ElseIf,
    ElseIfNot,
    When,
    Unless,

    Make,
    Const,
    Set,
    Define,
    
    Include,
    Import,
    
    Return,

    For,
    With,
    While,
    During,
    Until,
    Til,

    Loop,
    Break,
    Continue,
    
    Switch,
    Match,

    Class,
    Implement,
    Inherit,
    Enum,

    Other,
    In
}

// Maps for matching strings to keywords and types
pub static KEYWORD_MAP: phf::Map<&'static str, KeyWord> = phf_map! {
    "do" => KeyWord::Do,
    "then" => KeyWord::Then,

    "if" => KeyWord::If,
    "ifnot" => KeyWord::IfNot,
    "else" => KeyWord::Else,
    "elseif" => KeyWord::ElseIf,
    "elseifnot" => KeyWord::ElseIfNot,
    "when" => KeyWord::When,
    "unless" => KeyWord::Unless,

    "make" => KeyWord::Make,
    "const" => KeyWord::Const,
    "set" => KeyWord::Set,
    "define" => KeyWord::Define,

    "include" => KeyWord::Include,
    "import" => KeyWord::Import,

    "return" => KeyWord::Return,

    "for" => KeyWord::For,
    "with" => KeyWord::With,
    "while" => KeyWord::While,
    "during" => KeyWord::During,
    "until" => KeyWord::Until,
    "til" => KeyWord::Til,

    "loop" => KeyWord::Loop,
    "break" => KeyWord::Break,
    "continue" => KeyWord::Continue,

    "switch" => KeyWord::Switch,
    "match" => KeyWord::Match,

    "class" => KeyWord::Class,
    "implement" => KeyWord::Implement,
    "inherit" => KeyWord::Inherit,
    "enum" => KeyWord::Enum,

    "other" => KeyWord::Other,
    "in" => KeyWord::In,
};

impl Copy for KeyWord {}
impl Clone for KeyWord {
    fn clone(&self) -> Self {
        *self
    }
}

impl KeyWord {
    pub fn is_master_keyword(&self) -> bool {
        matches!(self,
            KeyWord::Do
            | KeyWord::If
            | KeyWord::IfNot
            | KeyWord::Make
            | KeyWord::Const
            | KeyWord::Set
            | KeyWord::Define
            | KeyWord::Include
            | KeyWord::Import
            | KeyWord::Return
            | KeyWord::For
            | KeyWord::While
            | KeyWord::Until
            | KeyWord::Loop
            | KeyWord::Break
            | KeyWord::Continue
            | KeyWord::Switch
            | KeyWord::Match
            | KeyWord::Class
            | KeyWord::Implement
            | KeyWord::Inherit
            | KeyWord::Enum
        )
    }

    pub fn is_extension_keyword(&self) -> bool {
        matches!(self,
            KeyWord::Then
            | KeyWord::Else
            | KeyWord::ElseIf
            | KeyWord::ElseIfNot
            | KeyWord::When
            | KeyWord::Unless
            | KeyWord::With
            | KeyWord::During
            | KeyWord::Til
        )
    }

    pub fn check_keyword(s: &str) -> Option<KeyWord> {
        KEYWORD_MAP.get(s).copied()
    }
}

pub static TYPE_MAP: phf::Map<&'static str, Types> = phf_map! {
    "any" => Types::Any,

    "float" => Types::Float,
    "int" => Types::Int,
    "number" => Types::Number,

    "bool" => Types::Bool,

    "string" => Types::String,
    "char" => Types::Char,

    "array" => Types::Array,
    "object" => Types::Object,
    "function" => Types::Function,
    "class" => Types::Class,

    "void" => Types::Void,
};

impl Copy for Types {}
impl Clone for Types {
    fn clone(&self) -> Self {
        *self
    }
}

impl Types {
    pub fn check_type(s: &str) -> Option<Types> {
        TYPE_MAP.get(s).copied()
    }
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