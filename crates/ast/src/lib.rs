mod structures;

// Re-exporting for easier access
pub use structures::ast::*;

pub struct Parser {
    tokens: Vec<dosato_lexer::Token>
}

impl Parser {
    pub fn new(tokens: Vec<dosato_lexer::Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Node {
        // Placeholder implementation
        Node::Program(vec![])
    }
}