#[derive(Debug)]
pub enum LexAtom {
    Float  { value: f64    },
    Int    { value: i64    },
    Str    { value: String },
    Symbol { value: String },
}
