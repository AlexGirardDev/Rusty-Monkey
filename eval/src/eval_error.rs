use lexer::token::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    TypeMismatch(String, String),
    InvalidOperation {
        operation: String,
        object_type: String,
    },
    InvalidPrefix(Token),
    InvalidOperator(String, String, String),
    IdentifierNotFount(String),
    ImpossibleState(String),
    InvalidParamTypes {
        expected: String,
        actual: String,
    },
    InvalidParamCount {
        expected: usize,
        actual: usize,
    },
    InvalidObjectType(String, String),
    IndexOperatorNotSupported(String),
    IndexOutOfBounds {
        index: i64,
        max: i64,
    },
    InvalidHashKeyType (String),
    HashKeyNotFound(String),
}

impl std::error::Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::TypeMismatch(lhs, rhs) => write!(f, "{lhs} and {rhs} are different types"),
            EvalError::InvalidOperation {
                operation,
                object_type,
            } => write!(f, "{operation} is not a valid operation on {object_type}"),
            EvalError::InvalidPrefix(prefx) => write!(f, "{prefx} is an invalid prefix"),
            EvalError::InvalidOperator(l, opp, r) => {
                write!(f, "{l} {opp} {r} is an invalid operation")
            }
            EvalError::IdentifierNotFount(i) => write!(f, "could not find {i}"),
            EvalError::ImpossibleState(i) => write!(
                f,
                "Reached what is supposed to be impossible state lol - {i}"
            ),
            EvalError::InvalidParamTypes { expected, actual } => {
                write!(f, "expected params: ({}) but got ({})", expected, actual)
            }
            EvalError::InvalidParamCount { expected, actual } => {
                write!(f, "got {} params but was expecting {}", actual, expected)
            }
            EvalError::InvalidObjectType(expected, acutal) => {
                write!(f, "{expected} was expected, but got {acutal} ")
            }
            EvalError::IndexOperatorNotSupported(s) => {
                write!(f, "could not use index accessor on {s}")
            }
            EvalError::IndexOutOfBounds { index, max } => write!(
                f,
                "attemped to access:{} when array is only {} big",
                max, index
            ),
            EvalError::InvalidHashKeyType(k) => write!(f,"{} is not a valid hash key type",k),
            EvalError::HashKeyNotFound(k) => write!(f,"key: {k} does not exsist")
        }
    }
}
