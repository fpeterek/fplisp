mod lexer;
mod interpreter;

use std::env;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => println!("Error: no file to interpret"),
        _ => run_script(args[0].clone()),
    }
}


fn run_script(file: String) {
    let int = Interpreter::new();
    int.interpret(file);
}

fn interpret(file: &String, str: &String) {
    let lexemes = lex(file, &str);
}

fn lex(file: &String, str: &String) {
    Lexer::lex(file, str);
}

