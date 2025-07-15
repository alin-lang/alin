use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::Evaluator;

pub fn start_repl() {
    let mut evaluator = Evaluator::new();

    println!("Alin REPL v0.1 — type 'exit()' to quit");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error membaca input");
            continue;
        }

        let trimmed = input.trim();
        if trimmed == "exit()" {
            break;
        }

        let mut lexer = Lexer::new(trimmed);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let exprs = parser.parse();

        evaluator.run(&exprs);
    }
}
