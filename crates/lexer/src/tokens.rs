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
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    Greater,
    Less,
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
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    Inc,
    Dec,
    Eq,
    Neq,
    Gte,
    Lte,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    AndAssign,
    OrAssign,
    XorAssign,
    Power,
    Root,
    Max,
    Min,
    Absolute,
    FatArrow,
    ShrAssign,
    ShlAssign,
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
    TripleEq,
    TripleNotEq,
    PipeAssign,
    FalseyCoalesce,
    FalseyCoalesceAssign,
    Spaceship,
    ArrayUnwrap,
    XorXor,
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
            Not | BitNot | Inc | Dec | Absolute => 1,
            Power | Root | Min | Max => 2,
            Mul | Div | Mod => 3,
            Add | Sub => 4,
            Shl | Shr => 5,
            Greater | Less | Gte | Lte | Spaceship => 6,
            Eq | Neq | TripleEq | TripleNotEq => 7,
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
            | Operator::SubAssign
            | Operator::MulAssign
            | Operator::DivAssign
            | Operator::ModAssign
            | Operator::AndAssign
            | Operator::OrAssign
            | Operator::XorAssign
            | Operator::PowerAssign
            | Operator::ShrAssign
            | Operator::ShlAssign
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
            Operator::Not | Operator::Inc | Operator::Dec | Operator::Sub | Operator::BitNot
        )
    }
}

pub enum BracketType {
    Parenthesis,
    Brace,
    Bracket,
}