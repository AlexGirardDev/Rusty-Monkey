use lexer::token::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    TypeMismatch(String, String),
    InvalidPrefix(Token),

    InvalidOperator(String, String, String),
    IdentifierNotFount(String),
}

impl std::error::Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::TypeMismatch(lhs, rhs) => write!(f, "{lhs} and {rhs} are different types"),
            EvalError::InvalidPrefix(prefx) => write!(f,"{prefx} is an invalid prefix"),
            EvalError::InvalidOperator(l, opp, r) => write!(f,"{l} {opp} {r} is an invalid operation"),
            EvalError::IdentifierNotFount(i) => write!(f,"could not find {i}")
        }
    }
}
