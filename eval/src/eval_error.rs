use std::fmt;
use lexer::token::Token;

use crate::object::Object;

#[derive(Debug)]
pub enum EvalError {
    GenericError(String),
    InvalidIntOperation(Object,String),
    InvalidOperation(Object,String),
    IncompatibleTypes(Object,Object,String),
    WrongComparisonToken(Token),
    WrongObjectType(Object,String)
    
}


impl std::error::Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::GenericError(str) => write!(f, "{}", str),
            EvalError::InvalidIntOperation(value,t) => write!(f,"{} is an invalid int operation with {}", value,t),
            EvalError::InvalidOperation(lhs, opp) =>  write!(f,"{} does not support the {} opperation",lhs,opp),
            EvalError::IncompatibleTypes(lhs, rhs, opp) => write!(f,"cannot preform {} between {} and {}",opp,lhs,rhs),
            EvalError::WrongComparisonToken(t) => write!(f,"{} is not a valid comparison token",t),
            EvalError::WrongObjectType(obj, expected) => write!(f,"got {} but was expecting {}", obj,expected)
        }
    }
}

