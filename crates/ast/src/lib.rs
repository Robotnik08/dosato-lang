mod structures;

use std::mem::Discriminant;

// Re-exporting for easier access
pub use structures::ast::*;
use dosato_runtime::*;

pub struct Parser<'a> {
    tokens: Vec<dosato_lexer::Token>,
    source: &'a str,
    source_name: Option<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<dosato_lexer::Token>, source: &'a str, source_name: Option<String>) -> Self {
        Self { tokens, source, source_name }
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
                    if let dosato_lexer::TokenKind::KeyWord(keyword_type) = token.kind {
                        if !keyword_type.is_master_keyword() {
                            continue;
                        }
                        // if not the first statement, parse the previous statement
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
                // Just a placeholder implementation
                Ok(Node::Statement(Box::new(Node::Program(Vec::new()))))
            }
            _ => {
                Err(
                    error::Error::new(error::ErrorKind::SyntaxError, "Unknown node type".to_string(), self.source_name.clone().unwrap_or("".to_string()), 0, 0, 0, false)
                )
            }
        }
    }
}