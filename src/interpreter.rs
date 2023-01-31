use std::collections::HashSet;
use std::fs;


pub struct Interpreter<'a> {
    files: HashSet<String>,
    current_file: Option<&'a String>,
}

impl Interpreter<'_> {

    fn run(&mut self, file: String, contents: String) {

    }

    pub fn interpret(&mut self, file: String) {
        self.files.insert(file.clone());
        self.current_file = Some(&self.files.get(&file).unwrap());

        self.files.get(&file);
        let contents = fs::read_to_string(file);

        match contents {
            Err(x) => println!("Contents of file {} could not be loaded", file),
            Ok(con) => self.run(file, con)
        }
    }

    pub fn new() -> Interpreter {
        Interpreter {
            files: HashSet::new(),
            current_file: None,
        }
    }
}
