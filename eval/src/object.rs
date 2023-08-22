use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use crate::eval_error::EvalError;

#[derive(Debug,PartialEq)]
pub enum Object {
    String(String),
    Int(i64),
    Bool(bool),
    Null,
    Return(Box<Object>)

}

impl From<i64> for Object {
    fn from(v: i64) -> Self {
        Object::Int(v)
    }
}
impl From<bool> for Object {
    fn from(v: bool) -> Self {
        Object::Bool(v)
    }
}

impl<'a> From<&'a str> for Object {
    fn from(s: &'a str) -> Self {
        Object::String(String::from(s))
    }
}

impl Add for Object {
    type Output = Result<Self, EvalError>;

    fn add(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l + r)),
            (Object::Int(lhs), rhs) => return Err(EvalError::TypeMismatch(lhs.into(),rhs)),
            (lhs,rhs) => Err(EvalError::InvalidOperator(lhs,"+".to_string(),rhs))
        }
    }
}

impl Sub for Object {
    type Output = Result<Self, EvalError>;

    fn sub(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l - r)),
            (Object::Int(lhs), rhs) => return Err(EvalError::TypeMismatch(lhs.into(),rhs)),

            (lhs,rhs) => Err(EvalError::InvalidOperator(lhs,"-".to_string(),rhs))
        }
    }
}

impl Div for Object {
    type Output = Result<Self, EvalError>;

    fn div(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l / r)),
            (Object::Int(lhs), rhs) => return Err(EvalError::TypeMismatch(lhs.into(),rhs)),
            (lhs,rhs) => Err(EvalError::InvalidOperator(lhs,"/".to_string(),rhs))
        }
    }
}

impl Mul for Object {
    type Output = Result<Self, EvalError>;

    fn mul(self, rhs: Self) -> Result<Self, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => return Ok(Object::Int(l * r)),
            (Object::Int(lhs), rhs) => return Err(EvalError::TypeMismatch(lhs.into(),rhs)),
            (lhs,rhs) => Err(EvalError::InvalidOperator(lhs,"*".to_string(),rhs))
        }
    }
}


impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Int(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Return(r) => write!(f,"return {}",r),
            Object::Null => write!(f, "null"),
        }
    }
}
