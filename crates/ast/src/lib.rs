mod structures;

// Re-exporting for easier access
pub use structures::ast::*;
use dosato_runtime::*;
use dosato_lexer::*;

macro_rules! get_line_column_len {
    ($start:expr, $end:expr) => {
        (
            $start.position.line,
            $start.position.column,
            $end.span.1 - $start.span.0
        )
    };
}

macro_rules! skip_block {
    ($self:ident, $index:ident, $span:ident) => {
        if let TokenKind::BracketOpen(bracket_type) = &$self.tokens[$index].kind {
            while $index < $span.1 {
                $index += 1;
                if let TokenKind::BracketClose(close_bracket_type) = &$self.tokens[$index].kind {
                    if close_bracket_type == bracket_type {
                        break;
                    }
                }
            }

            if $index >= $span.1 {
                return Err(
                    error::Error::new(
                        error::ErrorKind::SyntaxError,
                        "Unclosed bracket".to_string(),
                        $self.source_name.clone().unwrap_or("".to_string()),
                        get_line_column_len!($self.tokens[$index - 1], $self.tokens[$index - 1]),
                        false
                    )
                )
            }
        }
    };
}

pub struct Parser {
    tokens: Vec<dosato_lexer::Token>,
    source_name: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<dosato_lexer::Token>, source_name: Option<String>) -> Self {
        Self { tokens, source_name }
    }

    pub fn parse(&self) -> Result<Node, error::Error> {
        self.parse_node(NodeType::Program, (0, self.tokens.len()))
    }
    
    pub fn parse_node(&self, node_type: NodeType, span: (usize, usize)) -> Result<Node, error::Error> {
        match node_type {
            NodeType::Program => {
                let mut statements: Vec<Node> = Vec::new();

                let mut index = span.0;
                // if master keyword is found, it's a statement, until the next master keyword or end of span
                let mut current_statement_start = index;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::KeyWord(keyword_type) = token.kind {
                        if !keyword_type.is_master_keyword() {
                            index += 1;
                            continue;
                        }

                        if index > current_statement_start {
                            let statement = self.parse_node(NodeType::Statement, (current_statement_start, index))?;
                            statements.push(statement);
                        }
                        current_statement_start = index;
                    }
                    index += 1;
                }

                // parse the last statement
                if index > current_statement_start {
                    let statement = self.parse_node(NodeType::Statement, (current_statement_start, index))?;
                    statements.push(statement);
                }

                Ok(Node::Program(statements))
            }
            NodeType::Statement => {
                // Get first keyword
                let first_token = &self.tokens[span.0];
                let key_word_type = match &first_token.kind {
                    TokenKind::KeyWord(kw) => kw,
                    _ => {
                        return Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected a master keyword at the start of statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.0]), false)
                        );
                    }
                };

                if key_word_type.is_master_keyword() {
                    // For now, just return a Blank node for master keywords
                    Ok(Node::Blank)
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected a master keyword at the start of statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.0]), false)
                    )
                }
            }
            _ => {
                Err(
                    error::Error::new(error::ErrorKind::SyntaxError, "Unknown node type".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.1 - 1]), false)
                )
            }
        }
    }
}