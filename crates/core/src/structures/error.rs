
pub enum Error {
    Error(ErrorData),
    SyntaxError(ErrorData),
    RuntimeError(ErrorData),
    TypeError(ErrorData),
}

pub struct ErrorData {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}