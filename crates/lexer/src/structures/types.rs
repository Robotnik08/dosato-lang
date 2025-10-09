use phf::phf_map;

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