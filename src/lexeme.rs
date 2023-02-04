use crate::position::Position;
use crate::atom::LexAtom;


#[derive(Debug)]
pub enum Lexeme {
    OpenPar  { position: Position, },
    ClosePar { position: Position, },
    Atom     { position: Position, atom: LexAtom, },
    Quote    { position: Position, },
}
