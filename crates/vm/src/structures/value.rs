pub enum Value {
    Null,

    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Copy for Value {}
impl Clone for Value {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Integer(i) => write!(f, "Integer({})", i),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
        }
    }
}