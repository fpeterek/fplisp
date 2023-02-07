mod atom;
mod interpreter;
mod lexeme;
mod lexer;
mod position;
mod report;
mod parser;
mod value;
mod fplisp;
mod script_context;
mod export_redefinition_error;

use std::env;
use crate::fplisp::FPLisp;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => println!("Error: no file to interpret"),
        _ => run_script(args[1].clone()),
    }
}


fn run_script(file: String) {
    let mut int = FPLisp::new();
    int.interpret(file);
}

