use std::rc::Rc;
use crate::{value::Value, atom::Atom};
use crate::export_redefinition_error::ExportRedefinitionError;


struct ScriptContext {
    file: Rc<String>,
    statements: Vec<Value>,
    export_symbols: Vec<Atom>,
}

impl ScriptContext {

    pub fn file(&self) -> Rc<String> {
        self.file.clone()
    }

    pub fn statements(&self) -> &Vec<Value> {
        &self.statements
    }

    pub fn export_symbols(&self) -> &Vec<Atom> {
        &self.export_symbols
    }

    pub fn set_export(&mut self, symbols: Vec<Atom>) -> Result<(), ExportRedefinitionError> {
        if !self.export_symbols.is_empty() {
            self.export_symbols = symbols;
            Ok(())
        } else {
            Err(ExportRedefinitionError::new(self.file()))
        }
    }
    
}
