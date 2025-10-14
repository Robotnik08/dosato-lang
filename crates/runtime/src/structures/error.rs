use colored_text::Colorize;

pub enum ErrorKind {
    Error,
    SyntaxError,
    RuntimeError,
    TypeError,
}

impl Copy for ErrorKind {}
impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Error {
    pub message: String,
    pub token_index: usize,
    pub exception: bool, // Whether this error is a runtime exception or a compile-time error
    pub kind: ErrorKind,
    pub source: String // Track the source of the error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at token index {} in source:\n{}", self.message.red().bold(), self.token_index, self.source)
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            token_index: self.token_index,
            exception: self.exception,
            kind: self.kind.clone(),
            source: self.source.clone()
        }
    }
}

impl Error {
    pub fn new(message: String, token_index: usize, exception: bool, kind: ErrorKind, source: String) -> Self {
        Self {
            message,
            token_index,
            exception,
            kind,
            source
        }
    }

    pub fn display(&self) {
        if self.exception {
            let exception_text = "Uncaught Exception".red();
            println!("{}", exception_text);
        } else {
            let error_text = "Compiler Error".red();
            println!("{}", error_text);
        }
    }

    pub fn display_with_source(&self, _source_map: crate::source::SourceMap) {
        // temp
    }
}