

use crate::builtin::get_builtin_fns;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, PartialEq, Default)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get(&self, key: impl Into<String>) -> Option<Rc<Object>> {
        let key = key.into();
        match self.store.get(&key) {
            Some(s) => Some(s.clone()),
            None => self.outer.clone()?.borrow().get(key),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: Rc<Object>) {
        self.store.insert(key.into(), value);
    }

    pub fn new_with_builtin() -> Self {
        Environment {
            outer: None,
            store: get_builtin_fns(),
        }
    }
    pub fn new() -> Self {
        Environment {
            outer: None,
            store: Default::default(),
        }
    }

    pub fn new_closed(env: Rc<RefCell<Environment>>) -> Self {
        Environment {
            outer: Some(env),
            store: Default::default(),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.store.iter() {
            writeln!(f, "[{k} : {v}]")?;
        }
        match self.outer.clone() {
            Some(outer) => {
                writeln!(f, "<><><>OUTER<><><>")?;
                writeln!(f, "[{:?}]", outer)
            }
            None => Ok(()),
        }
    }
}
