use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct Environment<'a> {
    store: RefCell<HashMap<String, Rc<Object>>>,
    outer: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn get(&self, key: impl Into<String>) -> Option<Rc<Object>> {
        let key = key.into();
        self.store
            .borrow()
            .get(&key)
            .cloned()
            .or_else(|| self.outer?.get(&key))
    }

    pub fn set(&self, key: impl Into<String>, value: Rc<Object>) {
        self.store.borrow_mut().insert(key.into(), value);
    }

    pub fn new(env: &'a Environment) -> Self {
        Environment {
            outer: Some(env),
            ..Default::default()
        }
    }
}
