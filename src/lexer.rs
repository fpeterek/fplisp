use std::iter::Peekable;
use std::str::Chars;
use std::rc::Rc;


pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    lexemes: Vec<String>,

    char: u64,
    line: u64,

    file: Rc<String>,
}

impl Lexer<'_> {
    fn new<'a>(file: Rc<String>, input: &'a String) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
            lexemes: Vec::new(),
            char: 1,
            line: 1,
            file,
        }
    }

    fn position(&self) -> Position {
        Position {
            char: self.char,
            line: self.line,
            file: self.file.clone(),
        }
    }

    fn is_valid_id_char(ch: char) -> bool {
        let num = ch as u32;

        ch == '!' || (35 <= num && num <= 38) || ch == '*' || ch == '+' || ch == '-' || ch == '/' || ch == ':' ||
            ch.is_alphanumeric() || ch == '|' || ch == '~'
    }

    fn is_valid_id_start(ch: char) -> bool {
        let num = ch as u32;

        ch == '!' || (35 <= num && num <= 38) || ch == '*' || ch == '+' || ch == '-' || ch == '/' || ch == ':' ||
            ch.is_alphabetic() || ch == '|' || ch == '~'
    }

    fn next(&mut self) -> Option<char> {
        let char = self.chars.next();

        match char {
            Some('\n') => {
                self.line += 1;
                self.char = 0;
            }
            Some(_) => {
                self.char += 1;
            }
            None => ()
        }

        char
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn drop_line(&mut self) {
        loop {
            match self.next() {
                Some('\n') => return,
                Some(_) => (),
                None => return,
            }
        }
    }

    fn parse_str(&mut self) {
        let mut is_escape = false;

        let mut string = "".to_string();

        // Ignore quotes
        self.next();

        loop {
            let next = self.next();

            match next {
                Some('"') => if is_escape {
                    string.push('"');
                } else {
                    break;
                }

                Some('\\') => is_escape = !is_escape,

                Some(x) => string.push(x),

                None => break,
            }
        }

        if !string.is_empty() {
            self.lexemes.push(string);
        }
    }

    fn load_num(&mut self) {

        let mut has_period = false;
        let mut number = "".to_string();

        loop {
            // We peek so as not to consume the character following the number
            let next = self.peek();

            match next {
                None => break,
                Some(x) => if *x == '.' && has_period {
                    break;
                } else if *x == '.' {
                    has_period = true;
                    number.push('.');
                } else if x.is_digit(10) {
                    number.push(*x);
                } else {
                    break;
                }
            }

            self.next();
        }

        if !number.is_empty() {
            self.lexemes.push(number)
        }
    }

    fn load_id(&mut self) {

    }

    fn load_atom(&mut self) {
        match self.peek() {
            Some(x) => if x.is_digit(10) {
                self.load_num()
            } else if Lexer::is_valid_id_start(*x) {
                self.load_id()            
            }

            None => () 
        }
    }

    fn process_next(&mut self) {
        match self.peek() {
            Some('\'') => {
                self.lexemes.push("'".to_string());
            }

            Some('(') => {
                self.lexemes.push("(".to_string());
            }

            Some(')') => {
                self.lexemes.push(")".to_string());
            }

            Some(';') => {
                self.drop_line();
            }

            Some('"') => {
                self.parse_str();
            }

            Some(x) => if x.is_whitespace() {
                drop(x);
            } else {
                self.load_atom();
            }

            None => (),
        }
    }

    fn lex_analysis(&mut self) {
        while self.chars.peek() != None {
            self.process_next();
        }
    }

    pub fn lex<'a>(file: Rc<String>, contents: &'a String) -> Vec<String> {
        let mut lexer = Lexer::new(file, contents);
        lexer.lex_analysis();
        lexer.lexemes
    }
}
