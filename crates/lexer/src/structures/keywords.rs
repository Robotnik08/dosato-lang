use phf::phf_map;

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
    In,

    // Types
    Any,

    Float,
    Int,
    Number,

    Bool,
    
    String,
    Char,
    
    Array,
    Object,
    Function,

    Void
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

    "any" => KeyWord::Any,

    "float" => KeyWord::Float,
    "int" => KeyWord::Int,
    "number" => KeyWord::Number,

    "bool" => KeyWord::Bool,

    "string" => KeyWord::String,
    "char" => KeyWord::Char,

    "array" => KeyWord::Array,
    "object" => KeyWord::Object,
    "function" => KeyWord::Function,

    "void" => KeyWord::Void,
};

impl Copy for KeyWord {}
impl Clone for KeyWord {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::fmt::Debug for KeyWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyWord::Do => write!(f, "Do"),
            KeyWord::Then => write!(f, "Then"),
            KeyWord::If => write!(f, "If"),
            KeyWord::IfNot => write!(f, "IfNot"),
            KeyWord::Else => write!(f, "Else"),
            KeyWord::ElseIf => write!(f, "ElseIf"),
            KeyWord::ElseIfNot => write!(f, "ElseIfNot"),
            KeyWord::When => write!(f, "When"),
            KeyWord::Unless => write!(f, "Unless"),
            KeyWord::Make => write!(f, "Make"),
            KeyWord::Const => write!(f, "Const"),
            KeyWord::Set => write!(f, "Set"),
            KeyWord::Define => write!(f, "Define"),
            KeyWord::Include => write!(f, "Include"),
            KeyWord::Import => write!(f, "Import"),
            KeyWord::Return => write!(f, "Return"),
            KeyWord::For => write!(f, "For"),
            KeyWord::With => write!(f, "With"),
            KeyWord::While => write!(f, "While"),
            KeyWord::During => write!(f, "During"),
            KeyWord::Until => write!(f, "Until"),
            KeyWord::Til => write!(f, "Til"),
            KeyWord::Loop => write!(f, "Loop"),
            KeyWord::Break => write!(f, "Break"),
            KeyWord::Continue => write!(f, "Continue"),
            KeyWord::Switch => write!(f, "Switch"),
            KeyWord::Match => write!(f, "Match"),
            KeyWord::Class => write!(f, "Class"),
            KeyWord::Implement => write!(f, "Implement"),
            KeyWord::Inherit => write!(f, "Inherit"),
            KeyWord::Enum => write!(f, "Enum"),
            KeyWord::Other => write!(f, "Other"),
            KeyWord::In => write!(f, "In"),
            KeyWord::Any => write!(f, "Any"),
            KeyWord::Float => write!(f, "Float"),
            KeyWord::Int => write!(f, "Int"),
            KeyWord::Number => write!(f, "Number"),
            KeyWord::Bool => write!(f, "Bool"),
            KeyWord::String => write!(f, "String"),
            KeyWord::Char => write!(f, "Char"),
            KeyWord::Array => write!(f, "Array"),
            KeyWord::Object => write!(f, "Object"),
            KeyWord::Function => write!(f, "Function"),
            KeyWord::Void => write!(f, "Void"),
        }
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

    pub fn is_type_keyword(&self) -> bool {
        matches!(self,
            KeyWord::Any
            | KeyWord::Float
            | KeyWord::Int
            | KeyWord::Number
            | KeyWord::Bool
            | KeyWord::String
            | KeyWord::Char
            | KeyWord::Array
            | KeyWord::Object
            | KeyWord::Function
            | KeyWord::Void
        )
    }

    pub fn allow_extension_keywords(&self) -> bool {
        matches!(self,
            KeyWord::Do
            | KeyWord::If
            | KeyWord::IfNot
            | KeyWord::Set
            | KeyWord::Return
            | KeyWord::For
            | KeyWord::While
            | KeyWord::Until
            | KeyWord::Loop
            | KeyWord::Break
            | KeyWord::Continue
            | KeyWord::Switch
            | KeyWord::Match
        )
    }

    pub fn check_keyword(s: &str) -> Option<KeyWord> {
        KEYWORD_MAP.get(s).copied()
    }
}
