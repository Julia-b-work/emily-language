//! Environments: scopes that map names to values.

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A scope: a map of names to values, plus an optional parent scope.
///
/// Scopes form a chain via `parent` — looking up a name walks up the chain, so
/// inner scopes see (and shadow) outer bindings. `Rc` makes an environment
/// shareable by many closures; `RefCell` lets it be mutated through that
/// sharing.
#[derive(Debug, Default)]
pub struct Env {
    /// The variables bound directly in this scope.
    bindings: HashMap<String, Value>,
    /// The enclosing scope, if any (`None` for the top level).
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    /// Creates a new, empty top-level environment.
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    /// Creates a new scope nested inside `parent`.
    pub fn child(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            bindings: HashMap::new(),
            parent: Some(parent),
        }))
    }

    /// Looks up `name`, checking this scope then walking up the parent chain.
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.bindings.get(name) {
            return Some(v.clone());
        }
        match &self.parent {
            Some(p) => p.borrow().get(name),
            None => None,
        }
    }

    /// Binds `name` to `value` in this scope (shadowing any existing binding).
    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
}
