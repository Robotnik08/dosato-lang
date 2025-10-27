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

macro_rules! skip_block_backwards {
    ($self:ident, $index:ident, $span:ident) => {
        if let TokenKind::BracketClose(bracket_type) = &$self.tokens[$index].kind {
            while $index > $span.0 {
                $index -= 1;
                if let TokenKind::BracketOpen(open_bracket_type) = &$self.tokens[$index].kind {
                    if open_bracket_type == bracket_type {
                        break;
                    }
                }
            }

            if $index <= $span.0 {
                return Err(
                    error::Error::new(
                        error::ErrorKind::SyntaxError,
                        "Unclosed bracket".to_string(),
                        $self.source_name.clone().unwrap_or("".to_string()),
                        get_line_column_len!($self.tokens[$index + 1], $self.tokens[$index + 1]),
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

    pub fn encased_in_parentheses(&self, span: (usize, usize)) -> bool {
        if span.1 - span.0 < 2 {
            return false;
        }

        if let TokenKind::BracketOpen(BracketType::Parenthesis(_)) = &self.tokens[span.0].kind {
            if let TokenKind::BracketClose(BracketType::Parenthesis(_)) = &self.tokens[span.1 - 1].kind {
                return true;
            }
        }

        false
    }
    
    pub fn parse_node(&self, node_type: NodeType, span: (usize, usize)) -> Result<Node, error::Error> {
        match node_type {
            NodeType::Expression => {
                if self.encased_in_parentheses(span) {
                    return self.parse_node(NodeType::Expression, (span.0 + 1, span.1 - 1));
                }

                if span.1 - span.0 == 1 {
                    let token = &self.tokens[span.0];
                    match &token.kind {
                        TokenKind::Identifier(id) => {
                            return Ok(Node::Identifier(0));
                        }
                        TokenKind::IntegerLiteral(_) | 
                        TokenKind::NumberLiteral(_) | 
                        TokenKind::StringLiteral(_) | 
                        TokenKind::BooleanLiteral(_) | 
                        TokenKind::CharLiteral(_) |
                        TokenKind::Infinity |
                        TokenKind::NaN |
                        TokenKind::Null => {
                            return Ok(Node::Literal(token.clone()));
                        }

                        _ => {
                            return Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Expected an expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.0]), false)
                            );
                        }
                    }
                } else if span.1 - span.0 == 0 {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Empty expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.0]), false)
                    );
                }

                Ok(Node::Blank)
            }
            
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
                    match key_word_type {
                        dosato_lexer::KeyWord::Do => {
                            let body = self.parse_node(NodeType::DoBody, (span.0 + 1, span.1))?;
                            Ok(Node::Do { body: vec![body] })
                        }
                        _ => Ok(Node::Blank)
                    }
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected a master keyword at the start of statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.0], self.tokens[span.0]), false)
                    )
                }
            }

            NodeType::DoBody => {
                let body = self.parse_node(NodeType::CallExpression, span)?;
                Ok(Node::DoBody { body: Box::new(body) })
            }

            NodeType::CallExpression => {
                let end_token = &self.tokens[span.1 - 1];
                // must end with a closing parenthesis, everything inside until the first opening parenthesis are the arguments
                // the rest up front is the callee AKA expression
                if let TokenKind::BracketClose(BracketType::Parenthesis(_)) = &end_token.kind {
                    let mut index = span.1 - 1;
                    skip_block_backwards!(self, index, span);
                    if let TokenKind::BracketOpen(BracketType::Parenthesis(_)) = &self.tokens[index].kind {
                        let arguments = Node::Blank;
                        let callee = self.parse_node(NodeType::Expression, (span.0, index))?;
                        Ok(Node::CallExpression {
                            callee: Box::new(callee),
                            arguments: vec![arguments],
                        })
                    } else {
                        Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected opening parenthesis for call expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[index], self.tokens[index]), false)
                        )
                    }
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected closing parenthesis for call expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), get_line_column_len!(self.tokens[span.1 - 1], self.tokens[span.1 - 1]), false)
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