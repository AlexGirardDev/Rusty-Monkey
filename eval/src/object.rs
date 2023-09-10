use itertools::Itertools;
use parser::ast::{BlockStatement, Identifier};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

use crate::environment::Environment;
use crate::eval::EvalResponse;
use crate::eval_error::EvalError;
pub type BuiltinFn = fn(&[Rc<Object>]) -> EvalResponse;

#[derive(Debug, PartialEq, Default)]
pub enum Object {
    #[default]
    Null,
    String(String),
    Int(i64),
    Bool(bool),
    Return(Rc<Object>),
    Function(Vec<Identifier>, BlockStatement, Rc<RefCell<Environment>>),
    Builtin(BuiltinFn),
    Array(Vec<Rc<Object>>),
    Hash(HashMap<HashKey, HashPair>),
}

#[derive(Debug, Default)]
pub struct HashKey {
    key: u64,
}

impl Hash for HashKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl Eq for HashKey {}

impl PartialEq for HashKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct HashPair {
    pub key: Rc<Object>,
    pub value: Rc<Object>,
}
impl From<u64> for HashKey {
    fn from(value: u64) -> Self {
        HashKey { key: value }
    }
}

impl Object {
    pub fn hash_key(&self) -> HashKey {
        let mut hasher = DefaultHasher::new();
        match self {
            Object::Null => hasher.write_u32(0),
            Object::String(s) => hasher.write(s.as_bytes()),
            Object::Int(i) => hasher.write_i64(*i),
            Object::Bool(b) => {
                if *b {
                    hasher.write_u8(1)
                } else {
                    hasher.write_u8(0)
                }
            }
            _ => panic!("wrong type reeeeeee"),
        }

        HashKey {
            key: hasher.finish(),
        }
    }
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

impl Add for &Object {
    type Output = Result<Rc<Object>, EvalError>;

    fn add(self, rhs: Self) -> Result<Rc<Object>, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => Ok(Object::Int(*l + *r).into()),
            (Object::String(l), Object::String(r)) => Ok(Object::String(format!("{l}{r}")).into()),
            (Object::Int(l), rhs) => Err(EvalError::TypeMismatch(l.to_string(), rhs.to_string())),
            (lhs, rhs) => Err(EvalError::InvalidOperator(
                lhs.to_string(),
                "+".to_string(),
                rhs.to_string(),
            )),
        }
    }
}

impl Sub for &Object {
    type Output = Result<Rc<Object>, EvalError>;

    fn sub(self, rhs: Self) -> Result<Rc<Object>, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => Ok(Object::Int(*l - *r).into()),
            (Object::Int(l), rhs) => Err(EvalError::TypeMismatch(l.to_string(), rhs.to_string())),
            (lhs, rhs) => Err(EvalError::InvalidOperator(
                lhs.to_string(),
                "-".to_string(),
                rhs.to_string(),
            )),
        }
    }
}

impl Div for &Object {
    type Output = Result<Rc<Object>, EvalError>;

    fn div(self, rhs: Self) -> Result<Rc<Object>, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => Ok(Object::Int(*l / *r).into()),
            (Object::Int(lhs), rhs) => {
                Err(EvalError::TypeMismatch(lhs.to_string(), rhs.to_string()))
            }
            (lhs, rhs) => Err(EvalError::InvalidOperator(
                lhs.to_string(),
                "/".to_string(),
                rhs.to_string(),
            )),
        }
    }
}

impl Mul for &Object {
    type Output = Result<Rc<Object>, EvalError>;

    fn mul(self, rhs: Self) -> Result<Rc<Object>, EvalError> {
        match (self, rhs) {
            (Object::Int(l), Object::Int(r)) => Ok(Object::Int(*l * *r).into()),
            (Object::Int(lhs), rhs) => {
                Err(EvalError::TypeMismatch(lhs.to_string(), rhs.to_string()))
            }
            (lhs, rhs) => Err(EvalError::InvalidOperator(
                lhs.to_string(),
                "*".to_string(),
                rhs.to_string(),
            )),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Int(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Return(r) => write!(f, "return {}", r),
            Object::Null => write!(f, "null"),
            Object::Function(idents, blk, _) => write!(f, "fn({}) {}", idents.join(" ,"), blk),
            Object::Builtin(func) => write!(f, "fn({:?}) ", func),
            Object::Array(array) => write!(f, "[{}] ", array.iter().format(", ")),
            Object::Hash(hash) => write!(f, "{:?}", hash),
        }
    }
}
