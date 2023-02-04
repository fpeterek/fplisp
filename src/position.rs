use std::rc::Rc;


#[derive(Debug)]
pub struct Position {
    pub char: u64,
    pub line: u64,
    pub file: Rc<String>,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position { 
            char: self.char,
            line: self.line,
            file: self.file.clone(),
        }
    }
}
