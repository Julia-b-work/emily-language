use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, Default)]
pub struct Env {
    bindings: HashMap<String, Value>,
    parent: Option<Weak<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn child(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            bindings: HashMap::new(),
            parent: Some(Rc::downgrade(&parent)),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.bindings.get(name) {
            return Some(v.clone());
        }
        match &self.parent {
            Some(p) => p.upgrade()?.borrow().get(name),
            None => None,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
}
