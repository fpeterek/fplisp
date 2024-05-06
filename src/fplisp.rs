use std::collections::HashSet;
use std::rc::Rc;
use std::fs;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::report::Report;
use crate::value::Value;


pub struct FPLisp {
    files: HashSet<Rc<String>>,
    current_file: Option<Rc<String>>,
}

impl FPLisp {

    fn interpret_ast(&mut self, statements: Vec<Value>) -> Result<(), Vec<Report>> {
        // for s in statements {
        //     println!("{s}");
        // }
        let interpreter = Interpreter::new();
        interpreter.interpret(statements);

        Ok(())
    }

    fn print_errors(errors: Vec<Report>) {
        for err in errors {
            println!("{}", err);
        }
    }

    fn parse_and_run(&mut self, file: Rc<String>, contents: String) -> Result<(), Vec<Report>> {

        let lexemes = Lexer::lex(file, &contents)?;
        let ast = Parser::parse(lexemes)?;
        self.interpret_ast(ast)?;

        Ok(())
    }

    fn interpret_string(&mut self, file: Rc<String>, contents: String) {
        let res = self.parse_and_run(file, contents);

        match res {
            Ok(_) => (),
            Err(errors) => {
                FPLisp::print_errors(errors)
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
            Ok(con) => self.interpret_string(rc, con)
        }
    }

    pub fn new() -> FPLisp {
        FPLisp {
            files: HashSet::new(),
            current_file: None,
        }
    }
}
