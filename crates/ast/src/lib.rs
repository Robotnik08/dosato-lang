mod structures;

// Re-exporting for easier access
pub use structures::ast::*;
use dosato_runtime::*;

pub struct Parser {
    tokens: Vec<dosato_lexer::Token>
}

impl Parser {
    pub fn new(tokens: Vec<dosato_lexer::Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Node, error::Error> {
        Ok(Node::Program(vec![]))
    }

    pub fn parse_node(&self, node: Node, span: (usize, usize)) -> Result<Node, error::Error> {
        
    }
}