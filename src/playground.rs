use std::fs;
use crate::{lexer::Lexer, parser::Parser, eval::Evaluator};

pub fn run_file(path: &str) {
    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("File not found: {}", path);
        return;
    };

    println!("Running file: {}", path);
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut evaluator = Evaluator::new();
    evaluator.run(&ast);
}