use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dosato <file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read file");
    let _source_map = dosato_core::source::SourceMap::new(); // Initialize an empty source map

    dosato_core::eval_simple(&source).unwrap_or_else(|e: dosato_core::error::Error| {
        e.display();
    });
}