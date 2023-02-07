use crate::value::Value;

pub enum FnImpl {
    Builtin,
    Expr { expr: Value },
}

pub enum ArgType {
    Value { name: String },
    Vararg { name: String }
}

pub struct FnArg {
    name: String,
    arg_type: ArgType,
}

pub struct Function {
    name: String,
    args: Vec<FnArg>,
    fn_impl: FnImpl,
}
