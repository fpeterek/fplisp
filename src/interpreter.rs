use crate::value::Value;


pub struct Interpreter {

}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter { }
    }

    fn interpret_statement(&self, stmt: Value) {
        match stmt {
            _ => {},
        }

    }

    pub fn interpret(&self, statements: Vec<Value>) {
        for stmt in statements {
            self.interpret_statement(stmt)
        }
    }
}
