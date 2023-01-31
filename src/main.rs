mod lexer;

use std::env;
use std::fs;
use crate::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => println!("Error: no file to interpret"),
        1 => run_script(&args[0]),
        _ => println!("Too many arguments")
    }
}


fn run_script(file: &String) {

    let contents = fs::read_to_string(file);

    match contents {
        Err(_) => println!("File '{file}' could not be read"),
        Ok(con) => interpret(&con)
    }

}

fn interpret(str: &String) {
    let lexemes = lex(&str);
}

fn lex(str: &String) {
    Lexer::lex(str);
}

