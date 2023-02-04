use std::iter::Peekable;
use std::slice::Iter;

use crate::atom::Atom;
use crate::lexeme::Lexeme;
use crate::report::Report;
use crate::statement::Statement;
use crate::position::Position;


pub struct Parser<'a> {
    statements: Vec<Statement>,
    reports: Vec<Report>,
    lexemes: Peekable<Iter<'a, Lexeme>>
}

impl Parser<'_> {

    fn parse_quote(&mut self) -> Statement {
        let quote_lexeme = self.lexemes.next().unwrap();

        let quote = Statement::Atom { atom: Atom::Symbol { value: "quote".to_string() } };

        let quoted = self.lexemes.peek();

        match quoted {
            None | Some(Lexeme::ClosePar { position: _ }) => {
                self.reports.push(
                    Report {
                        msg: "Unmatched quote".to_string(),
                        position: Parser::position_of(quote_lexeme),
                    });
            }

            _ => {
                let quoted = self.parse_next();
                return Statement::ConsCell {
                    left: Box::new(quote),
                    right: Box::new(quoted),
                }
            }

        }

        quote
    }

    fn load_list_contents(&mut self) -> Vec<Statement> {
        let mut vec = Vec::new();

        loop {

            let next = self.lexemes.peek();

            match next {
                None => break,
                Some(Lexeme::ClosePar { position: _ }) => break,
                _ => {
                    vec.push(self.parse_next());
                },
            }
        }


        vec
    }

    fn to_list(mut vec: Vec<Statement>) -> Statement {
        let mut res = Statement::Atom { atom: Atom::Nil };

        while !vec.is_empty() {
            let last = vec.pop().unwrap();
            res = Statement::ConsCell { 
                left: Box::new(last),
                right: Box::new(res),
            }
        }

        res
    }

    fn position_of(lexeme: &Lexeme) -> Position {
        match lexeme {
            Lexeme::OpenPar  { position          } => position.clone(),
            Lexeme::ClosePar { position          } => position.clone(),
            Lexeme::Atom     { position, atom: _ } => position.clone(),
            Lexeme::Quote    { position          } => position.clone(),
        }
    }

    fn parse_list(&mut self) -> Statement {
        let begin = self.lexemes.next().expect("Interpreter error");
        let next = self.lexemes.next();

        let res = match next {
            Some(Lexeme::ClosePar { position: _ }) => Statement::Atom { atom: Atom::EmptyList },
            Some(_) => {
                let contents = self.load_list_contents();
                Parser::to_list(contents)
            }
            None => {
                self.reports.push(
                    Report {
                        msg: "Unterminated list".to_string(),
                        position: Parser::position_of(&begin),
                    });

                Statement::Atom { atom: Atom::EmptyList }
            }
        };

        let paren = self.lexemes.next();

        match paren {
            None => {
                self.reports.push(
                    Report {
                        msg: "Unterminated list".to_string(),
                        position: Parser::position_of(&begin),
                    });
            }
            Some(Lexeme::ClosePar { position: _ }) => (),
            _ => panic!("Interpreter error!"),
        }

        res
    }

    fn parse_atom(&mut self) -> Statement {
        let lexeme = self.lexemes.next().expect("Interpreter error");

        match lexeme {
            Lexeme::Atom { position: _, atom } => Statement::Atom { atom: atom.clone() },
            _ => panic!("Interpreter error")
        }
    }

    fn parse_next(&mut self) -> Statement {
        let next = self.lexemes.peek();

        match next {
            Some(Lexeme::OpenPar { position: _ }) => self.parse_list(),
            Some(Lexeme::ClosePar { position }) => {
                self.lexemes.next();
                self.reports.push(
                    Report {
                        msg: "Unmatched closed parenthesis.".to_string(),
                        position: position.clone(),
                    });
                Statement::Atom { atom: Atom::Nil }
            }
            Some(Lexeme::Atom { position: _, atom: _ }) => self.parse_atom(),
            Some(Lexeme::Quote { position: _ }) => self.parse_quote(),
            None => panic!("Interpreter error"),
        }
    }

    fn run(&mut self) {
        while self.lexemes.peek().is_some() {
            let next = self.parse_next();
            self.statements.push(next);
        }
    }

    pub fn parse(lexemes: Vec<Lexeme>) -> Result<Vec<Statement>, Vec<Report>> {
        let mut parser = Parser {
            statements: Vec::new(),
            reports: Vec::new(),
            lexemes: lexemes.iter().peekable(),
        };

        parser.run();

        Err(parser.reports)
    }

}
