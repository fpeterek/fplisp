mod lexer;
mod interpreter;

use std::env;
use crate::interpreter::Interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => println!("Error: no file to interpret"),
        _ => run_script(args[0].clone()),
    }
}


fn run_script(file: String) {
    let mut int = Interpreter::new();
    int.interpret(file);
}

