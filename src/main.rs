mod token;
mod lexer;
mod parser;
mod ast;
mod eval;
mod repl;

use lexer::Lexer;
use parser::Parser;
use eval::Evaluator;

fn main() {
    repl::start_repl();
}
