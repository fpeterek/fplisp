use std::iter::Peekable;
use std::str::Chars;
use std::rc::Rc;

use crate::lexeme::Lexeme;
use crate::position::Position;
use crate::report::Report;
use crate::atom::Atom;


pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    lexemes: Vec<Lexeme>,
    reports: Vec<Report>,

    char: u64,
    line: u64,

    file: Rc<String>,
}

impl Lexer<'_> {
    fn new<'a>(file: Rc<String>, input: &'a String) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
            lexemes: Vec::new(),
            reports: Vec::new(),
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

    fn special_chars() -> &'static str {
        "!%&*+-/:<>|~=^?"
    }

    fn is_valid_id_char(ch: char) -> bool {
        let special_chars = Lexer::special_chars();

        special_chars.contains(ch) || ch.is_alphanumeric()
    }

    fn is_valid_id_start(ch: char) -> bool {
        let special_chars = Lexer::special_chars();

        special_chars.contains(ch) || ch.is_alphabetic()
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

    fn parse_str(&mut self, position: Position) {
        let mut is_escape = false;
        let mut string = "".to_string();

        // Ignore quotes
        self.next();

        let terminated = loop {
            let next = self.next();

            match next {
                Some('"') => if is_escape {
                    string.push('"');
                } else {
                    break true;
                }

                Some('\\') => is_escape = !is_escape,

                Some('\n') | None => break false,

                Some(x) => string.push(x),
            }
        };

        if !terminated {
            self.reports.push(Report { msg: "Unterminated string".to_string(), position: position.clone() });
        }

        self.lexemes.push(
            Lexeme::Atom { 
                position,
                atom: Atom::Str { value: string } 
            });
    }

    fn parse_flt(&mut self, string: &String, position: Position) -> Lexeme {
        let float = string.parse();

        let value = match float {
            Ok(val) => val,
            Err(_) => {
                self.reports.push(
                    Report {
                        msg: format!("Invalid float literal {}", string),
                        position: position.clone(),
                    });
                0.0
            }
        };

        Lexeme::Atom { 
            position,
            atom: Atom::Float { value }
        }
    }

    fn parse_int(&mut self, string: &String, position: Position) -> Lexeme {
        let lower = string.to_lowercase();

        let (base, num) = if lower.starts_with("0b") {
            (2, lower[2..].to_string())
        } else if lower.starts_with("0d") {
            (10, lower[2..].to_string())
        } else if lower.starts_with("0x") {
            (16, lower[2..].to_string())
        } else if lower.starts_with("0o") {
            (8, lower[2..].to_string())
        } else {
            (10, lower)
        };

        let value = i64::from_str_radix(&num, base);

        let value = match value {
            Ok(val) => val,
            Err(_) => {
                self.reports.push(Report { msg: format!("Invalid int literal {}", string), position: position.clone() });
                0
            }
        };

        Lexeme::Atom {
            position,
            atom: Atom::Int { value }
        }
    } 

    fn parse_num(&mut self, string: &String, position: Position) -> Lexeme {
        if string.contains('.') {
            self.parse_flt(string, position)
        } else {
            self.parse_int(string, position)
        }
    }

    fn load_num(&mut self, position: Position) {

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
                } else if x.is_digit(10) || x.is_ascii_alphabetic() {
                    number.push(*x);
                } else {
                    break;
                }
            }

            self.next();
        }

        let lexeme = self.parse_num(&number, position);
        self.lexemes.push(lexeme);
    }

    fn load_id(&mut self, position: Position) {
        let mut id = "".to_string();

        loop {
            let next = self.peek();

            match next {
                Some(x) => if Lexer::is_valid_id_char(*x) {
                    id.push(*x);
                } else {
                    break;
                }
                None => break,
            }

            self.next();
        }

        self.lexemes.push(
            Lexeme::Atom {
                position,
                atom: Atom::Symbol { value: id }
            });
    }

    fn load_atom(&mut self, position: Position) {
        // I cannot borrow self as mutable, thus I must borrow self.chars to satisfy the borrow
        // checker 
        match self.chars.peek() {
            Some(x) => if x.is_digit(10) {
                self.load_num(position);
            } else if Lexer::is_valid_id_start(*x) {
                self.load_id(position);
            } else {
                self.reports.push(Report { msg: format!("Invalid character '{}'", *x), position });
                self.next();
            }

            None => () 
        }
    }

    fn load_quote(&mut self) {
        self.lexemes.push(
            Lexeme::Quote { 
                position: self.position(),
            });

        self.next();
    }

    fn load_open_paren(&mut self) {
        self.lexemes.push(
            Lexeme::OpenPar { 
                position: self.position(),
            });

        self.next();
    }

    fn load_closed_paren(&mut self) {
        self.lexemes.push(
            Lexeme::ClosePar { 
                position: self.position(),
            });

        self.next();
    }

    fn process_next(&mut self) {
        match self.peek() {
            Some('\'') => self.load_quote(),
            Some('(')  => self.load_open_paren(),
            Some(')')  => self.load_closed_paren(),
            Some(';')  => self.drop_line(),
            Some('"')  => self.parse_str(self.position()),

            Some(x) => if x.is_whitespace() {
                self.next();
            } else {
                self.load_atom(self.position());
            }

            None => (),
        }
    }

    fn lex_analysis(&mut self) {
        while self.peek() != None {
            self.process_next();
        }
    }

    pub fn lex<'a>(file: Rc<String>, contents: &'a String) -> Result<Vec<Lexeme>, Vec<Report>> {
        let mut lexer = Lexer::new(file, contents);
        lexer.lex_analysis();

        match lexer.reports.is_empty() {
            true  => Ok(lexer.lexemes),
            false => Err(lexer.reports)
        }
    }
}
