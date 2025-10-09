use phf::phf_map;

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
    Dot,
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
    ArrayUnwrapAssign,
    AndAndAssign,
    OrOrAssign,
    XorXorAssign,
}

pub static OPERATOR_MAP: phf::Map<&'static str, Operator> = phf_map! {
    "+" => Operator::Add,
    "-" => Operator::Subtract,
    "*" => Operator::Multiply,
    "/" => Operator::Divide,
    "%" => Operator::Modulo,
    "=" => Operator::Assign,
    ">" => Operator::GreaterThen,
    "<" => Operator::LessThen,
    "!" => Operator::Not,
    "&" => Operator::And,
    "^" => Operator::Xor,
    "|" => Operator::Or,
    "~" => Operator::BitNot,
    "?" => Operator::Question,
    ":" => Operator::Colon,
    "->" => Operator::Arrow,
    "." => Operator::Dot,
    "," => Operator::Comma,
    "+=" => Operator::AddAssign,
    "-=" => Operator::SubtractAssign,
    "*=" => Operator::MultiplyAssign,
    "/=" => Operator::DivideAssign,
    "%=" => Operator::ModuloAssign,
    "++" => Operator::Increment,
    "--" => Operator::Decrement,
    "==" => Operator::Equals,
    "!=" => Operator::NotEquals,
    ">=" => Operator::GreaterThanEqual,
    "<=" => Operator::LessThanEqual,
    "&&" => Operator::AndAnd,
    "||" => Operator::OrOr,
    "^^" => Operator::XorXor,
    "<<" => Operator::ShiftLeft,
    ">>" => Operator::ShiftRight,
    "&=" => Operator::AndAssign,
    "|=" => Operator::OrAssign,
    "^=" => Operator::XorAssign,
    "**" => Operator::Power,
    "^/" => Operator::Root,
    ">|" => Operator::Max,
    "<|" => Operator::Min,
    "!-" => Operator::Absolute,
    "=>" => Operator::FatArrow,
    ">>=" => Operator::ShiftRightAssign,
    "<<=" => Operator::ShiftLeftAssign,
    "**=" => Operator::PowerAssign,
    ">|=" => Operator::MaxAssign,
    "<|=" => Operator::MinAssign,
    ";" => Operator::Semicolon,
    ".." => Operator::RangeUp, // Range up (exclusive)
    "..<" => Operator::RangeDown, // Range down (exclusive)
    "..=" => Operator::RangeUpInclusive, // Range up (inclusive)
    "..<=" => Operator::RangeDownInclusive, // Range down (inclusive)
    "??" => Operator::NullCoalesce,
    "??=" => Operator::NullCoalesceAssign,
    "?." => Operator::NullCoalesceAccess,
    "^/=" => Operator::RootAssign,
    "|>" => Operator::Pipe,
    "===" => Operator::StrictEquals,
    "!==" => Operator::TripleNotEquals,
    "<=>" => Operator::Spaceship,
    "#=" => Operator::ArrayUnwrapAssign,
    "&&=" => Operator::AndAndAssign,
    "||=" => Operator::OrOrAssign,
    "^^=" => Operator::XorXorAssign,
    "#" => Operator::Hash,
};

impl Copy for Operator {}
impl Clone for Operator {
    fn clone(&self) -> Self {
        *self
    }
}

impl Operator {
    /// Return precedence: lower numbers bind more tightly (you mentioned lower => higher precedence).
    /// I kept your numeric mapping idea: smaller number = higher priority.
    pub fn precedence(&self) -> u8 {
        use Operator::*;
        match self {
            Arrow | Hash | Dot | NullCoalesceAccess => 0,
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
            | Operator::ArrayUnwrapAssign
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
            | Operator::Absolute
            | Operator::Root
            | Operator::Add
            | Operator::Multiply
        )
    }

    pub fn is_binary(&self) -> bool {
        matches!(self,
            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Modulo
            | Operator::Assign
            | Operator::GreaterThen
            | Operator::LessThen
            | Operator::And
            | Operator::Xor
            | Operator::Or
            | Operator::Power
            | Operator::Root
            | Operator::ShiftLeft
            | Operator::ShiftRight
            | Operator::Equals
            | Operator::NotEquals
            | Operator::GreaterThanEqual
            | Operator::LessThanEqual
            | Operator::AndAnd
            | Operator::OrOr
            | Operator::XorXor
            | Operator::NullCoalesce
            | Operator::Pipe
            | Operator::StrictEquals
            | Operator::TripleNotEquals
            | Operator::Spaceship
            | Operator::FatArrow
            | Operator::RangeUp
            | Operator::RangeDown
            | Operator::RangeUpInclusive
            | Operator::RangeDownInclusive
        )
    }

    pub fn check_operator(s: &str) -> Option<Operator> {
        OPERATOR_MAP.get(s).copied()
    }
}