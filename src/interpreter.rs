use core::panic;

use crate::{atom::Atom, value::Value};


pub struct Interpreter {

}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter { }
    }

    fn eval_fn(&self, fn_call: &Value) -> Value {
        Value::Atom { atom: Atom::Nil }
    }

    fn eval_symbol(&self, symbol: &Atom) -> Value {
        Value::Atom { atom: Atom::Nil }
    }

    fn eval_atom(&self, atom: Value) -> Value {
        match atom {
            Value::ConsCell { left: _, right: _ } => {
                panic!("Invalid interpreter state")
            },
            Value::Atom { atom: ref value } => match value {
                Atom::Symbol { value: _ } => self.eval_symbol(value),
                _ => atom.clone()
            }
        }
    }

    fn interpret_statement(&self, stmt: Value) -> Value {
        match stmt {
            Value::ConsCell { left: _, right: _ } => self.eval_fn(&stmt),
            Value::Atom { atom: _ } => self.eval_atom(stmt)
        }

    }

    pub fn interpret(&self, statements: Vec<Value>) {
        for stmt in statements {
            self.interpret_statement(stmt);
        }
    }
}
