use std::fmt::Display;
use std::rc::Rc;

use crate::atom::Atom;

#[derive(PartialEq)]
pub enum Statement {
    ConsCell { left: Rc<Statement>, right: Rc<Statement> },
    Atom { atom: Atom },
}

impl Clone for Statement {
    fn clone(&self) -> Self {
        match self {
            Statement::Atom { atom } => 
                Statement::Atom { atom: atom.clone() },

            Statement::ConsCell { left, right } =>
                Statement::ConsCell { left: left.clone(), right: right.clone() },
        }
    }
}

impl Statement {

    fn is_nil(&self) -> bool {
        if let Statement::Atom { atom } = self {
            *atom == Atom::Nil
        } else {
            false
        }
    }

    fn not_nil(&self) -> bool {
        !self.is_nil()
    }

    fn is_empty_list(&self) -> bool {
        if let Statement::Atom { atom } = self {
            *atom == Atom::EmptyList
        } else {
            false
        }
    }

    fn flatten(l: Rc<Statement>, r: Rc<Statement>) -> Vec<Rc<Statement>> {
        let mut res = Vec::new();
        res.push(l);

        let mut rest = r;

        while rest.not_nil() && !rest.is_empty_list() {
            match &*rest {
                Statement::Atom { atom: _ } => {
                    res.push(rest.clone())
                }
                Statement::ConsCell { left, right } => {
                    res.push(left.clone());
                    rest = right.clone()
                }
            }
        }

        res
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Atom { atom } => write!(f, "{atom}"),
            Statement::ConsCell { left, right } => {
                let flat = Statement::flatten(left.clone(), right.clone());

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
