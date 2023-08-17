use std::{fmt, io};

#[derive(Debug)]
pub enum EvalError {
    GenericError(String),
}


impl std::error::Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::GenericError(str) => write!(f, "{}", str),
        }
    }
}

