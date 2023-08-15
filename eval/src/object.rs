use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Object {
    String(String),
    Int(i64),
    Bool(bool),
    Null,

}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Int(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
        }
    }
}
