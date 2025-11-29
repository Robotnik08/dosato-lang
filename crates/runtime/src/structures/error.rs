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
    pub line: usize,
    pub column: usize,
    pub error_length: usize, // Length of the error in characters starting from the column
    pub exception: bool, // Whether this error is a runtime exception or a compile-time error
    pub kind: ErrorKind,
    pub source: String // Track the source of the error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_text = match self.kind {
            ErrorKind::Error => "Error",
            ErrorKind::SyntaxError => "SyntaxError",
            ErrorKind::RuntimeError => "RuntimeError",
            ErrorKind::TypeError => "TypeError",
        };
        write!(f, "<{}: {}>", kind_text, self.message)
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            line: self.line,    
            column: self.column,
            error_length: self.error_length,
            exception: self.exception,
            kind: self.kind.clone(),
            source: self.source.clone()
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: String, source: String, (line, column, error_length): (usize, usize, usize), exception: bool) -> Self {
        Self {
            message,
            line,
            column,
            error_length,
            exception,
            kind,
            source
        }
    }

    pub fn display(&self) {
        if self.exception {
            let exception_text = "Uncaught Exception:".red();
            println!("{}", exception_text);
        }

        let file_string = format!("File \"{}\"", self.source).yellow();
        println!("{}", file_string);

        let error_kind_text = match self.kind {
            ErrorKind::Error => "Error:".red().bold(),
            ErrorKind::SyntaxError => "SyntaxError:".red().bold(),
            ErrorKind::RuntimeError => "RuntimeError:".red().bold(),
            ErrorKind::TypeError => "TypeError:".red().bold(),
        };

        println!("\n{}: {}", error_kind_text, self.message);
    }

    pub fn display_with_source(&self, source_map: crate::source::SourceMap) {
        if self.exception {
            let exception_text = "Uncaught Exception:".red();
            println!("{}", exception_text);
        }

        let file_string = format!("File \"{}\": line {}, column {} ({}:{})", self.source, self.line, self.column, 
                                                                         self.line, self.column).yellow();
        println!("{}", file_string);
        
        // get source from source map
        if let Some(source) = source_map.get(&self.source) {
            let lines: Vec<&str> = source.lines().collect();
            if self.line > 0 && self.line <= lines.len() {
                let error_line = lines[self.line - 1];
                println!("{}", error_line);

                // Print indicator line
                let mut indicator_line = String::new();
                let mut current_col = 1;
                let mut current_line = self.line - 1;
                let mut current_line_str = lines[current_line];
                for _ in 0..(self.column - 1) {
                    indicator_line.push(' ');
                    current_col += 1;
                }
                for _ in 0..self.error_length.max(1) {
                    indicator_line.push('^');
                    if current_line_str.chars().nth(current_col - 1) == None {
                        current_line += 1;
                        current_line_str = lines.get(current_line).unwrap_or(&"");
                        current_col = 1;
                        println!("{}\n{}", indicator_line.red(), current_line_str);
                        indicator_line = String::new();
                    } else {
                        current_col += 1;
                    }
                }
                println!("{}", indicator_line.red());
            }
        } else {
            println!("<Source not available>");
        }

        let error_kind_text = match self.kind {
            ErrorKind::Error => "Error:".red().bold(),
            ErrorKind::SyntaxError => "SyntaxError:".red().bold(),
            ErrorKind::RuntimeError => "RuntimeError:".red().bold(),
            ErrorKind::TypeError => "TypeError:".red().bold(),
        };

        println!("{} {}", error_kind_text, self.message);
    }
}