use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};


#[derive(Debug, Clone)]
pub struct ExportRedefinitionError {
    filename: Rc<String>,
}

impl ExportRedefinitionError {
    pub fn new(filename: Rc<String>) -> ExportRedefinitionError {
        ExportRedefinitionError { filename }
    }
}

impl Display for ExportRedefinitionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f, 
           "Redefinition of export symbols for file {}",
           self.filename,
       )
    }
}
