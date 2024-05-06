use std::rc::Rc;
use crate::value::Value;
use crate::export_redefinition_error::ExportRedefinitionError;


struct ScriptContext {
    file: Rc<String>,
    statements: Vec<Value>,
    export_symbols: Vec<Value>,
}

impl ScriptContext {

    pub fn file(&self) -> Rc<String> {
        self.file.clone()
    }

    pub fn statements(&self) -> &Vec<Value> {
        &self.statements
    }

    pub fn export_symbols(&self) -> &Vec<Value> {
        &self.export_symbols
    }

    pub fn set_export(&mut self, symbols: Vec<Value>) -> Result<(), ExportRedefinitionError> {
        if self.export_symbols.is_empty() {
            self.export_symbols = symbols;
            Ok(())
        } else {
            Err(ExportRedefinitionError::new(self.file()))
        }
    }
    
}
