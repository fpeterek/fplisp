use std::fmt::Display;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Atom {
    Float  { value: f64    },
    Int    { value: i64    },
    Str    { value: String },
    Symbol { value: String },
    EmptyList,
    Nil,
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        match self {
            Atom::Float { value } => Atom::Float { value: *value },
            Atom::Int { value } => Atom::Int { value: *value },
            Atom::Str { value } => Atom::Str { value: value.clone() },
            Atom::Symbol { value } => Atom::Symbol { value: value.clone() },
            Atom::EmptyList => Atom::EmptyList,
            Atom::Nil => Atom::Nil,
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Float { value }  => write!(f, "{value}"),
            Atom::Int { value }    => write!(f, "{value}"),
            Atom::Str { value }    => write!(f, "\"{value}\""),
            Atom::Symbol { value } => write!(f, "{value}"),
            Atom::EmptyList => write!(f, "()"),
            Atom::Nil => write!(f, "nil"),
        }
    }
}
