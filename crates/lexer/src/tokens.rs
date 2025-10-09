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

    // MasterKeyWord(MasterKeyword),
    // ExtensionKeyWord(ExtensionKeyWord),
    // ReservedKeyWord(ReservedKeyword),

    Identifier(String),

    BracketOpen(BracketType),
    BracketClose(BracketType),

    // TypeKeyword(TypeKeyword),

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
    FalseyCoalesce,
    FalseyCoalesceAssign,
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
            OrOr | NullCoalesce | FalseyCoalesce | Pipe => 12,
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
            | Operator::FalseyCoalesceAssign
            | Operator::RootAssign
            | Operator::PipeAssign
        )
    }

    /// Is this a unary operator?
    pub fn is_unary(&self) -> bool {
        matches!(self,
            Operator::Not | Operator::Increment | Operator::Decrement | Operator::Subtract | Operator::BitNot
        )
    }
}

pub enum BracketType {
    Parenthesis,
    Brace,
    Bracket,
}