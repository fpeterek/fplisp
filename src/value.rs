use std::fmt::Display;
use std::rc::Rc;

use crate::atom::Atom;

#[derive(PartialEq)]
pub enum Value {
    ConsCell { left: Rc<Value>, right: Rc<Value> },
    Atom { atom: Atom },
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Atom { atom } => 
                Value::Atom { atom: atom.clone() },

            Value::ConsCell { left, right } =>
                Value::ConsCell { left: left.clone(), right: right.clone() },
        }
    }
}

impl Value {

    fn is_nil(&self) -> bool {
        if let Value::Atom { atom } = self {
            *atom == Atom::Nil
        } else {
            false
        }
    }

    fn not_nil(&self) -> bool {
        !self.is_nil()
    }

    fn is_empty_list(&self) -> bool {
        if let Value::Atom { atom } = self {
            *atom == Atom::EmptyList
        } else {
            false
        }
    }

    fn flatten(l: Rc<Value>, r: Rc<Value>) -> Vec<Rc<Value>> {
        let mut res = Vec::new();
        res.push(l);

        let mut rest = r;

        while rest.not_nil() && !rest.is_empty_list() {
            match &*rest {
                Value::Atom { atom: _ } => {
                    res.push(rest.clone())
                }
                Value::ConsCell { left, right } => {
                    res.push(left.clone());
                    rest = right.clone()
                }
            }
        }

        res
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Atom { atom } => write!(f, "{atom}"),
            Value::ConsCell { left, right } => {
                let flat = Value::flatten(left.clone(), right.clone());

                write!(f, "(")?;
                for (idx, s) in flat.iter().enumerate() {
                    write!(f, "{s}")?;
                    if idx < flat.len()-1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}
