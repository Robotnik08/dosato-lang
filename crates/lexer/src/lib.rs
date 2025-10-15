use crate::structures::{keywords, operators, tokens};
use dosato_runtime::*;

mod structures;

pub use tokens::*;
pub use keywords::*;
pub use operators::*;

pub struct Lexer {
    source: String,
    position: usize,
    line: usize,
    column: usize,
}

const MAX_TEMPLATE_DEPTH: usize = 256;

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<tokens::Token>, error::Error> {
        let mut tokens = Vec::new();
        self.remove_windows_carriage_returns(); // Normalize line endings
        let chars: Vec<char> = self.source.chars().collect();
        let length = chars.len();

        let mut template_n = 0; // 0 means not in template, >0 means in template
        let mut current_template_id: u16 = 0;
        let mut template_bracket_depths: [u16; MAX_TEMPLATE_DEPTH] = [0; MAX_TEMPLATE_DEPTH];
        let mut template_ids: [u16; MAX_TEMPLATE_DEPTH] = [0; MAX_TEMPLATE_DEPTH];

        let mut bracket_depth = 0;

        while self.position < length {
            let current_char = chars[self.position];

            match current_char {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance_line();
                }
                '0'..='9' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };

                    if chars[self.position] == '0' {
                        if self.position + 1 < length {
                            match chars[self.position + 1] {
                                'x' | 'X' => {
                                    // Hexadecimal
                                    self.advance_by(2); // Skip '0x'
                                    let hex_start = self.position;
                                    while self.position < length && chars[self.position].is_digit(16) {
                                        self.advance();
                                    }
                                    let hex_str: String = chars[hex_start..self.position].iter().collect();
                                    if let Ok(number) = i64::from_str_radix(&hex_str, 16) {
                                        tokens.push(tokens::Token::new(
                                            tokens::TokenKind::IntegerLiteral(number),
                                            (start_char_position, self.position),
                                            start_position,
                                        ));
                                    }
                                    continue;
                                }
                                'o' | 'O' => {
                                    // Octal
                                    self.advance_by(2); // Skip '0o'
                                    let oct_start = self.position;
                                    while self.position < length && (chars[self.position] >= '0' && chars[self.position] <= '7') {
                                        self.advance();
                                    }
                                    let oct_str: String = chars[oct_start..self.position].iter().collect();
                                    if let Ok(number) = i64::from_str_radix(&oct_str, 8) {
                                        tokens.push(tokens::Token::new(
                                            tokens::TokenKind::IntegerLiteral(number),
                                            (start_char_position, self.position),
                                            start_position,
                                        ));
                                    }
                                    continue;
                                }
                                'b' | 'B' => {
                                    // Binary
                                    self.advance_by(2); // Skip '0b'
                                    let bin_start = self.position;
                                    while self.position < length && (chars[self.position] == '0' || chars[self.position] == '1') {
                                        self.advance();
                                    }
                                    let bin_str: String = chars[bin_start..self.position].iter().collect();
                                    if let Ok(number) = i64::from_str_radix(&bin_str, 2) {
                                        tokens.push(tokens::Token::new(
                                            tokens::TokenKind::IntegerLiteral(number),
                                            (start_char_position, self.position),
                                            start_position,
                                        ));
                                    }
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }

                    let mut has_dot = false;
                    while self.position < length && (chars[self.position].is_digit(10) || chars[self.position] == '.') {
                        if chars[self.position] == '.' {
                            if has_dot {
                                return Err(error::Error::new(
                                    error::ErrorKind::SyntaxError, "Invalid number format".to_string(), "main".to_string(), self.line, self.column, 1, false
                                ));
                            }
                            has_dot = true;
                        }
                        self.advance();
                    }

                    // Check for scientific notation (e/E)
                    let mut has_exponent = false;
                    if self.position < length && (chars[self.position] == 'e' || chars[self.position] == 'E') {
                        has_exponent = true;
                        self.advance(); // Skip 'e' or 'E'
                        if self.position < length && (chars[self.position] == '+' || chars[self.position] == '-') {
                            self.advance(); // Skip sign
                        }
                        // Exponent digits
                        while self.position < length && chars[self.position].is_digit(10) {
                            self.advance();
                        }
                    }

                    let number_str = chars[start_char_position..self.position].iter().collect::<String>();

                    if has_dot || has_exponent {
                        if let Ok(number) = number_str.parse::<f64>() {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::NumberLiteral(number),
                                (start_char_position, self.position),
                                start_position,
                            ));
                        }
                    } else if let Ok(number) = number_str.parse::<i64>() {
                        tokens.push(tokens::Token::new(
                            tokens::TokenKind::IntegerLiteral(number),
                            (start_char_position, self.position),
                            start_position,
                        ));
                    }
                }

                '(' | '{' | '[' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };

                    let bracket_type = match current_char {
                        '(' => tokens::BracketType::Parenthesis(bracket_depth),
                        '{' => tokens::BracketType::Brace(bracket_depth),
                        '[' => tokens::BracketType::Bracket(bracket_depth),
                        _ => unreachable!(),
                    };

                    bracket_depth += 1;

                    tokens.push(tokens::Token::new(
                        tokens::TokenKind::BracketOpen(bracket_type),
                        (start_char_position, self.position + 1),
                        start_position,
                    ));
                    self.advance();
                }
                ')' | '}' | ']' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };
                    
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                    } else {
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, "Unmatched closing bracket".to_string(), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }

                    let bracket_type = match current_char {
                        ')' => tokens::BracketType::Parenthesis(bracket_depth),
                        '}' => tokens::BracketType::Brace(bracket_depth),
                        ']' => tokens::BracketType::Bracket(bracket_depth),
                        _ => unreachable!(),
                    };


                    tokens.push(tokens::Token::new(
                        tokens::TokenKind::BracketClose(bracket_type),
                        (start_char_position, self.position + 1),
                        start_position,
                    ));
                    self.advance();

                    // If we are in a template, and the bracket is } and the bracket depth matches the template's starting depth - 1, continue the template
                    if current_char == '}' && template_n > 0 && bracket_depth == template_bracket_depths[template_n - 1] {
                        let start_char_position = self.position;
                        let start_position = tokens::TokenPosition {
                            line: self.line,
                            column: self.column,
                        };

                        while self.position < length {
                            let current_char = chars[self.position];
                            if current_char == '{' {
                                // put the template part token
                                let part_content = chars[start_char_position..self.position].iter().collect::<String>();
                                let part_content_unescaped = unescape_string(part_content.as_str());
                                tokens.push(tokens::Token::new(
                                    tokens::TokenKind::StringTemplate(tokens::StringTemplatePart {
                                        id: template_ids[template_n - 1],
                                        value: part_content_unescaped,
                                    }),
                                    (start_char_position, self.position),
                                    start_position,
                                ));
                                break;
                            }

                            if current_char == '`' {
                                // put the template end token
                                let part_content = chars[start_char_position..self.position].iter().collect::<String>();
                                let part_content_unescaped = unescape_string(part_content.as_str());
                                tokens.push(tokens::Token::new(
                                    tokens::TokenKind::StringTemplateEnd(tokens::StringTemplatePart {
                                        id: template_ids[template_n - 1],
                                        value: part_content_unescaped,
                                    }),
                                    (start_char_position, self.position + 1),
                                    start_position,
                                ));

                                self.advance(); // Skip closing backtick

                                template_n -= 1;
                                break;
                            }

                            if current_char == '\\' && self.position + 1 < length && (chars[self.position + 1] == '`' || chars[self.position + 1] == '{' || chars[self.position + 1] == '\\') {
                                self.advance(); // Skip escape character or escaped brace
                            }

                            if current_char == '\n' {
                                self.advance_line();
                            } else {
                                self.advance();
                            }
                        }
                    }
                }

                '"' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };
                    self.advance(); // Skip opening quote
                    let string_start = self.position;

                    while self.position < length && chars[self.position] != '"' {
                        if chars[self.position] == '\\' && self.position + 1 < length && (chars[self.position + 1] == '"' || chars[self.position + 1] == '\\') {
                            self.advance(); // Skip escape character
                        }

                        if chars[self.position] == '\n' {
                            self.advance_line();
                        } else {
                            self.advance();
                        }
                    }

                    let string_content = unescape_string(chars[string_start..self.position].iter().collect::<String>().as_str());


                    if self.position < length && chars[self.position] == '"' {
                        self.advance(); // Skip closing quote
                        tokens.push(tokens::Token::new(
                            tokens::TokenKind::StringLiteral(string_content),
                            (start_char_position, self.position),
                            start_position,
                        ));
                    } else {
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, "Unterminated string literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }
                }

                '\'' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };
                    self.advance(); // Skip opening quote

                    if self.position < length {
                        let char_content = chars[self.position];
                        self.advance();

                        if char_content == '\\' && self.position < length {
                            // Handle escape sequences
                            let escape_char = chars[self.position];
                            let escaped_char = match escape_char {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                '\\' => '\\',
                                '\'' => '\'',
                                '"' => '"',
                                '0' => '\0',
                                _ => {
                                    return Err(error::Error::new(
                                        error::ErrorKind::SyntaxError, "Invalid escape sequence in char literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                                    ));
                                }
                            };
                            self.advance();
                            if self.position < length && chars[self.position] == '\'' {
                                self.advance(); // Skip closing quote
                                tokens.push(tokens::Token::new(
                                    tokens::TokenKind::CharLiteral(escaped_char),
                                    (start_char_position, self.position),
                                    start_position,
                                ));
                            } else {
                                return Err(error::Error::new(
                                    error::ErrorKind::SyntaxError, "Unterminated char literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                                ));
                            }
                        } else if char_content == '\'' || char_content == '\n' || char_content == '\r' || char_content == '\t'{
                            return Err(error::Error::new(
                                error::ErrorKind::SyntaxError, "Invalid character in char literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                            ));
                        } else if self.position < length && chars[self.position] == '\'' {
                            self.advance(); // Skip closing quote
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::CharLiteral(char_content),
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else {
                            return Err(error::Error::new(
                                error::ErrorKind::SyntaxError, "Unterminated char literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                            ));
                        }
                    } else {
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, "Unterminated char literal".to_string(), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }
                }

                // start of template string
                '`' => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };
                    self.advance(); // Skip opening backtick

                    if template_n >= MAX_TEMPLATE_DEPTH {
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, "Maximum template string depth exceeded".to_string(), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }
                    
                    template_bracket_depths[template_n] = bracket_depth;
                    template_ids[template_n] = current_template_id;
                    template_n += 1;
                    current_template_id += 1;

                    while self.position < length {
                        let current_char = chars[self.position];
                        if current_char == '{' {
                            // put the template part token
                            let part_content = chars[start_char_position + 1..self.position].iter().collect::<String>();
                            let part_content_unescaped = unescape_string(part_content.as_str());
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::StringTemplate(tokens::StringTemplatePart {
                                    id: template_ids[template_n - 1],
                                    value: part_content_unescaped,
                                }),
                                (start_char_position, self.position),
                                start_position,
                            ));
                            break;
                        }

                        if current_char == '`' {
                            // put the template end token
                            let part_content = chars[start_char_position + 1..self.position].iter().collect::<String>();
                            let part_content_unescaped = unescape_string(part_content.as_str());
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::StringTemplateEnd(tokens::StringTemplatePart {
                                    id: template_ids[template_n - 1],
                                    value: part_content_unescaped,
                                }),
                                (start_char_position, self.position + 1),
                                start_position,
                            ));

                            self.advance(); // Skip closing backtick

                            template_n -= 1;
                            break;
                        }

                        if current_char == '\\' && self.position + 1 < length && (chars[self.position + 1] == '`' || chars[self.position + 1] == '{' || chars[self.position + 1] == '\\') {
                            self.advance(); // Skip escape character or escaped brace
                        }

                        if current_char == '\n' {
                            self.advance_line();
                        } else {
                            self.advance();
                        }
                    }

                    if self.position >= length && template_n > 0 {
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, "Unterminated template string".to_string(), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }

                }

                // operators
                op if operators::OPERATOR_CHARS.contains(current_char.to_string().as_str()) => {
                    let start_char_position = self.position;
                    let start_position = tokens::TokenPosition {
                        line: self.line,
                        column: self.column,
                    };

                    // first check if operator is 3 chars
                    if self.position + 2 < length {
                        let three_char_op: String = chars[self.position..self.position + 3].iter().collect();
                        if let Some(op_kind) = operators::OPERATOR_MAP.get(three_char_op.as_str()) {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Operator(*op_kind),
                                (start_char_position, self.position + 3),
                                start_position,
                            ));
                            self.advance_by(3);
                            continue;
                        }
                    }

                    // then check if operator is 2 chars
                    if self.position + 1 < length {
                        let two_char_op: String = chars[self.position..self.position + 2].iter().collect();
                        if let Some(op_kind) = operators::OPERATOR_MAP.get(two_char_op.as_str()) {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Operator(*op_kind),
                                (start_char_position, self.position + 2),
                                start_position,
                            ));
                            self.advance_by(2);
                            continue;
                        }

                        // check if // or /* comment
                        if two_char_op == "//" {
                            // single line comment
                            let comment_start = self.position;
                            self.advance_to_end_of_line(&chars, length);
                            let comment_content: String = chars[comment_start..self.position].iter().collect();
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Comment(comment_content),
                                (start_char_position, self.position),
                                start_position,
                            ));
                            continue;
                        } else if two_char_op == "/*" {
                            // multi line comment
                            let comment_start = self.position;
                            while self.position + 1 < length && !(chars[self.position] == '*' && chars[self.position + 1] == '/') {
                                if chars[self.position] == '\n' {
                                    self.advance_line();
                                } else {
                                    self.advance();
                                }
                            }
                            if self.position + 1 < length {
                                self.advance_by(2); // Skip closing */
                            }
                            let comment_content: String = chars[comment_start..self.position].iter().collect();
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Comment(comment_content),
                                (start_char_position, self.position),
                                start_position,
                            ));
                            continue;
                        }
                    }

                    // simple 1 char operator
                    if let Some(op_kind) = operators::OPERATOR_MAP.get(op.to_string().as_str()) {
                        tokens.push(tokens::Token::new(
                            tokens::TokenKind::Operator(*op_kind),
                            (start_char_position, self.position + 1),
                            start_position,
                        ));
                        self.advance();
                    }
                }

                _ => {
                    // check if token is alpha or underscore (identifier or keyword)
                    if current_char.is_alphabetic() || current_char == '_' {
                        let start_char_position = self.position;
                        let start_position = tokens::TokenPosition {
                            line: self.line,
                            column: self.column,
                        };

                        let word = self.advance_get_word(&chars);

                        if let Some(keyword) = keywords::KeyWord::check_keyword(&word) {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::KeyWord(keyword),
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else if word == "true" || word == "false" {
                            let boolean_value = word == "true";
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::BooleanLiteral(boolean_value),
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else if word == "null" {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Null,
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else if word == "Infinity" {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Infinity,
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else if word == "NaN" {
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::NaN,
                                (start_char_position, self.position),
                                start_position,
                            ));
                        } else {
                            // identifier
                            tokens.push(tokens::Token::new(
                                tokens::TokenKind::Identifier(word),
                                (start_char_position, self.position),
                                start_position,
                            ));
                        }
                    } else {
                        // Unknown character
                        return Err(error::Error::new(
                            error::ErrorKind::SyntaxError, format!("Unexpected character: '{}'", current_char), "main".to_string(), self.line, self.column, 1, false
                        ));
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    fn advance_by(&mut self, n: usize) {
        self.position += n;
        self.column += n;
    }

    fn advance_line(&mut self) {
        self.position += 1;
        self.line += 1;
        self.column = 1;
    }

    fn advance_to_end_of_line(&mut self, chars: &[char], length: usize) {
        while self.position < length && chars[self.position] != '\n' {
            self.advance();
        }
    }

    /// Advance the lexer and return the word (identifier or keyword) at the current position.
    /// Stops at the first non-alphanumeric character or underscore.
    fn advance_get_word(&mut self, chars: &[char]) -> String {
        let start_position = self.position;
        let length = chars.len();

        while self.position < length && (chars[self.position].is_alphanumeric() || chars[self.position] == '_') {
            self.advance();
        }

        chars[start_position..self.position].iter().collect()
    }

    fn remove_windows_carriage_returns(&mut self) {
        self.source = self.source.replace("\r\n", "\n");
    }
}

fn unescape_string(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('\\') => result.push('\\'),
                Some('`') => result.push('`'),
                Some('{') => result.push('{'),
                Some('0') => result.push('\0'),
                Some(other) => {
                    // Unknown escape, keep both characters
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
}