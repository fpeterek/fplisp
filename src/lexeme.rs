use crate::position::Position;
use crate::atom::Atom;


#[derive(Debug)]
pub enum Lexeme {
    OpenPar  { position: Position, },
    ClosePar { position: Position, },
    Atom     { position: Position, atom: Atom, },
    Quote    { position: Position, },
}
