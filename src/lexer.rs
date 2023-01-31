use std::iter::Peekable;
use std::str::Chars;


pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    lexemes: Vec<String>,

    char: u64,
    line: u64,

    file: String,
}

impl Lexer<'_> {
    fn new<'a>(input: &'a String) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
            lexemes: Vec::new(),
            char: 1,
            line: 1
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

    fn drop_line(&mut self) {
        loop {
            match self.chars.next() {
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
        self.chars.next();

        loop {
            let next = self.chars.next();

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
            let next = self.chars.peek();

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

            self.chars.next();
        }

        if !number.is_empty() {
            self.lexemes.push(number)
        }
    }

    fn load_id(&mut self) {

    }

    fn load_atom(&mut self) {
        match self.chars.peek() {
            Some(x) => if x.is_digit(10) {
                self.load_num()
            } else if Lexer::is_valid_id_start(*x) {
                self.load_id()            
            }

            None => () 
        }
    }

    fn process_next(&mut self) {
        match self.chars.next() {
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
                ()
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

    pub fn lex(str: &String) -> Vec<String> {
        let mut lexer = Lexer::new(&str);
        lexer.lex_analysis();
        lexer.lexemes
    }
}
