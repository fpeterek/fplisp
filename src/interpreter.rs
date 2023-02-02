use std::collections::HashSet;
use std::rc::Rc;
use std::fs;

use crate::lexer::Lexer;


pub struct Interpreter {
    files: HashSet<Rc<String>>,
    current_file: Option<Rc<String>>,
}

impl Interpreter {

    fn run(&mut self, file: Rc<String>, contents: String) {
        let lexemes = Lexer::lex(file, &contents);
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
