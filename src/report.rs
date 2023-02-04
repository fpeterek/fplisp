use crate::position::Position;

use std::fmt::Display;


pub struct Report {
    pub msg: String,
    pub position: Position,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}): {}", self.position.line, self.position.char, self.msg)
    }
}
