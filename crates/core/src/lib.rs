pub use dosato_runtime::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn eval_simple(source: &str) -> Result<(), error::Error> {
    let mut lexer = dosato_lexer::Lexer::new(source);
    let tokens = lexer.tokenise()?;
    let parser = dosato_ast::Parser::new(tokens.clone(), source);

    let ast = parser.parse()?;

    println!("Token amount: {}", tokens.len());

    for token in tokens {
        println!("{:?}", token);
    }

    println!("{:#?}", ast);

    Ok(())
}