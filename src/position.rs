use std::rc::Rc;


pub struct Position {
    char: u64,
    line: u64,
    file: Rc<String>,
}
