use crate::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Environment {
    store: RefCell<HashMap<String, Rc<Object>>>,
}

impl Environment {
    pub fn get(&self, key: impl Into<String>) -> Option<Rc<Object>> {
        match self.store.borrow().get(&key.into()) {
            Some(o) => Some(o.clone()),
            None => None,
        }
    }

    pub fn set(&self, key: impl Into<String>, value: Rc<Object>) {
        self.store.borrow_mut().insert(key.into(), value);
    }
}
