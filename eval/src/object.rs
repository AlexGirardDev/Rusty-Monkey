use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use crate::eval_error::EvalError;

#[derive(PartialEq, Debug)]
pub enum Object {
    String(String),
    Int(i64),
    Bool(bool),
    Null,

}

impl Add for Object {
    type Output = Result<Self, EvalError>;

    fn add(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l + r)),
            (lhs,_) => Err(EvalError::InvalidOperation(lhs,"add".to_string()))
        }
    }
}

impl Sub for Object {
    type Output = Result<Self, EvalError>;

    fn sub(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l - r)),
            (lhs,_) => Err(EvalError::InvalidOperation(lhs,"subtract".to_string()))
        }
    }
}

impl Div for Object {
    type Output = Result<Self, EvalError>;

    fn div(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l / r)),
            (lhs,_) => Err(EvalError::InvalidOperation(lhs,"divide".to_string()))
        }
    }
}

impl Mul for Object {
    type Output = Result<Self, EvalError>;

    fn mul(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l * r)),
            (lhs,_) => Err(EvalError::InvalidOperation(lhs,"multiply".to_string()))
        }
    }
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
