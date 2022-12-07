use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuntimeVal {
    Number(isize),
    Null,
    Bool(bool),
    Object(HashMap<String, RuntimeVal>),
}

impl Display for RuntimeVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVal::Number(num) => write!(f, "{:#?}", num),
            RuntimeVal::Null => write!(f, "null"),
            RuntimeVal::Bool(b_val) => write!(f, "{:#?}", b_val),
            RuntimeVal::Object(map) => write!(f, "{:#?}", map),
        }
    }
}
