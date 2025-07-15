mod token;
mod lexer;
mod parser;
mod ast;
mod eval;
mod repl;
mod playground;


use std::env;
use repl::start_repl;
use playground::run_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = &args[1];
        run_file(path);
    } else {
        start_repl();
    }
}
