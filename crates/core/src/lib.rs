pub use dosato_runtime::*;
pub use dosato_vm::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn eval_simple(source: &str) -> Result<(), error::Error> {
    let mut table = names::NameTable::new();
    let mut lexer = dosato_lexer::Lexer::new(source, &mut table);
    
    let tokens = lexer.tokenise()?;
    let mut parser = dosato_ast::Parser::new(tokens.clone(), Some("main".to_string()));
    
    println!("Token amount: {}", tokens.len());

    let mut index = 0;
    for token in tokens {
        println!("{}. {:?}", index, token);
        index += 1;
    }

    let ast = parser.parse(table)?;

    println!("{:#?}", ast);

    Ok(())
}