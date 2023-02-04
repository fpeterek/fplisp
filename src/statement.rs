use crate::atom::Atom;

pub enum Statement {
    ConsCell { left: Box<Statement>, right: Box<Statement> },
    Atom { atom: Atom },
}
