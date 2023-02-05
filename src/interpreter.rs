use std::collections::HashSet;
use std::rc::Rc;
use std::fs;

use crate::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::statement::Statement;


pub struct Interpreter {
    files: HashSet<Rc<String>>,
    current_file: Option<Rc<String>>,
}

impl Interpreter {

    fn int(&mut self, statements: Vec<Statement>) {
        for s in statements {
            println!("{s}");
        }
    }

    fn parse(&mut self, lexemes: Vec<Lexeme>) {
        let result = Parser::parse(lexemes);

        match result {
            Ok(s) => {
                self.int(s)
            }
            Err(errors) => {
                for err in errors {
                    println!("{}", err);
                }
            }
        }
    }

    fn run(&mut self, file: Rc<String>, contents: String) {
        let result = Lexer::lex(file, &contents);

        match result {
            Ok(lexemes) => self.parse(lexemes),
            Err(errors) => {
                for err in errors {
                    println!("{}", err);
                }
            }
        }
    }

    pub fn interpret(&mut self, file: String) {
        let rc = Rc::new(file.clone());
        self.files.insert(rc.clone());
        self.current_file = Some(rc.clone());

        let contents = fs::read_to_string(&file);

        match contents {
            Err(_) => println!("Contents of file {} could not be loaded", file),
            Ok(con) => self.run(rc, con)
        }
    }

    pub fn new() -> Interpreter {
        Interpreter {
            files: HashSet::new(),
            current_file: None,
        }
    }
}
