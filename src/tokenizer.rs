use std::iter::Peekable;
use std::str::Chars;


pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    toks: Vec<String>,
}

impl Tokenizer<'_> {
    fn new<'a>(input: &'a String) -> Tokenizer<'a> {
        Tokenizer {
            chars: input.chars().peekable(),
            toks: Vec::new(),
        }
    }

    fn is_valid_id_char(ch: char) -> bool {
        let num = ch as u32;

        ch == '!' || (35 <= num && num <= 38) || ch == '*' || ch == '+' || ch == '-' || ch == '/' || ch == ':' ||
            (60 <= num && num <= 90) || (94 <= num && num <= 122) || ch == '|' || ch == '~'
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
            self.toks.push(string);
        }
    }

    fn load_num(&mut self) {

        let mut has_period = false;
        let mut number = "".to_string();

        loop {
            // We peek so as not to consume the character following the number
            let next = self.chars.peek();

            // TODO: Ignore period if number ends with a period
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
            self.toks.push(number)
        }
    }

    fn load_id(&mut self) {

    }

    fn load_atom(&mut self) {
        match self.chars.peek() {
            Some(x) => if x.is_digit(10) {
                self.load_num()
            } else {
                self.load_id()            
            }

            None => () 
        }
    }

    fn process_next(&mut self) {
        match self.chars.next() {
            Some('\'') => {
                self.toks.push("'".to_string());
            }

            Some('(') => {
                self.toks.push("(".to_string());
            }

            Some(')') => {
                self.toks.push(")".to_string());
            }

            Some(';') => {
                self.drop_line();
            }

            Some('"') => {
                self.parse_str();
            }

            Some(_) => {
                self.load_atom();
            }

            None => (),
        }
    }

    fn tok(&mut self) {
        while self.chars.peek() != None {
            self.process_next();
        }
    }

    pub fn tokenize(str: &String) -> Vec<String> {
        let mut tokenizer = Tokenizer::new(&str);
        tokenizer.tok();
        tokenizer.toks
    }
}
