use crate::structures::tokens;

mod structures {
    pub mod tokens;
    pub mod keywords;
    pub mod operators;
}

pub fn tokenise(source: &str) -> Vec<tokens::Token> {
    Vec::new() // temp
}