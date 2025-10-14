pub use dosato_lexer::*;

mod structures;

pub use structures::source;
pub use structures::error;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn eval_simple(source: &str) -> Result<(), error::Error> {
    let mut lexer = dosato_lexer::Lexer::new(source);
    let tokens = lexer.tokenise();

    println!("Token amount: {}", tokens.len());

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}