use phf::phf_map;

pub enum KeyWord {
    Do,
    Then,

    If,
    Else,
    Unless,

    Make,
    Const,
    Set,
    Define,
    
    Include,
    Import,
    
    Return,

    For,
    While,
    Until,

    Catch,

    Loop,
    Break,
    Continue,
    
    Switch,

    Class,
    Inherit,
    Enum,

    Other,

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

    Error,

    Void
}

// Maps for matching strings to keywords and types
pub static KEYWORD_MAP: phf::Map<&'static str, KeyWord> = phf_map! {
    "do" => KeyWord::Do,
    "then" => KeyWord::Then,

    "if" => KeyWord::If,
    "else" => KeyWord::Else,
    "unless" => KeyWord::Unless,

    "make" => KeyWord::Make,
    "const" => KeyWord::Const,
    "set" => KeyWord::Set,
    "define" => KeyWord::Define,

    "include" => KeyWord::Include,
    "import" => KeyWord::Import,

    "return" => KeyWord::Return,

    "for" => KeyWord::For,
    "while" => KeyWord::While,
    "until" => KeyWord::Until,
    
    "catch" => KeyWord::Catch,

    "loop" => KeyWord::Loop,
    "break" => KeyWord::Break,
    "continue" => KeyWord::Continue,

    "switch" => KeyWord::Switch,

    "class" => KeyWord::Class,
    "inherit" => KeyWord::Inherit,
    "enum" => KeyWord::Enum,

    "other" => KeyWord::Other,

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

    "error" => KeyWord::Error,

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
            KeyWord::Else => write!(f, "Else"),
            KeyWord::Unless => write!(f, "Unless"),
            KeyWord::Make => write!(f, "Make"),
            KeyWord::Const => write!(f, "Const"),
            KeyWord::Set => write!(f, "Set"),
            KeyWord::Define => write!(f, "Define"),
            KeyWord::Include => write!(f, "Include"),
            KeyWord::Import => write!(f, "Import"),
            KeyWord::Return => write!(f, "Return"),
            KeyWord::For => write!(f, "For"),
            KeyWord::While => write!(f, "While"),
            KeyWord::Until => write!(f, "Until"),
            KeyWord::Catch => write!(f, "Catch"),
            KeyWord::Loop => write!(f, "Loop"),
            KeyWord::Break => write!(f, "Break"),
            KeyWord::Continue => write!(f, "Continue"),
            KeyWord::Switch => write!(f, "Switch"),
            KeyWord::Class => write!(f, "Class"),
            KeyWord::Inherit => write!(f, "Inherit"),
            KeyWord::Enum => write!(f, "Enum"),
            KeyWord::Other => write!(f, "Other"),
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
            KeyWord::Error => write!(f, "Error"),
            KeyWord::Void => write!(f, "Void"),
        }
    }
}

impl KeyWord {
    pub fn is_master_keyword(&self) -> bool {
        matches!(self,
            KeyWord::Do
            | KeyWord::If
            | KeyWord::Unless
            | KeyWord::Else
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
            | KeyWord::Class
            | KeyWord::Inherit
            | KeyWord::Enum
            | KeyWord::Catch
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
            | KeyWord::Error
            | KeyWord::Void
        )
    }

    pub fn allow_extension_keywords(&self) -> bool {
        matches!(self,
            KeyWord::Do
            | KeyWord::If
            | KeyWord::Set
            | KeyWord::Return
            | KeyWord::For
            | KeyWord::While
            | KeyWord::Until
            | KeyWord::Loop
            | KeyWord::Break
            | KeyWord::Continue
            | KeyWord::Switch
        )
    }

    pub fn check_keyword(s: &str) -> Option<KeyWord> {
        KEYWORD_MAP.get(s).copied()
    }
}
