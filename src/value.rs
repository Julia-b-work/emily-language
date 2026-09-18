//! Runtime values: what expressions evaluate to.

use crate::ast::Expr;
use crate::env::Env;
use std::cell::RefCell;
use std::rc::Rc;

/// A runtime value — the result of evaluating an expression.
///
/// Mirrors [`Expr`], but the "syntactic" variants are gone: symbols have been
/// looked up, keywords have become record keys, and lists/vectors are merged.
/// The new variant is [`Closure`], a function value.
///
/// Note: there's no `PartialEq` derive here — `Closure` contains a `RefCell`,
/// which can't be compared for equality.
#[derive(Debug, Clone)]
pub enum Value {
    /// A whole number.
    Int(i64),
    /// A floating-point number.
    Float(f64),
    /// A string.
    String(String),
    /// A boolean.
    Bool(bool),
    /// A list of values (both `(...)` and `[...]` become this).
    List(Vec<Value>),
    /// A record: key/value pairs.
    Record(Vec<(String, Value)>),
    /// A function value.
    Closure(Closure),
}

/// A function value: its parameters, body, and the environment it captured.
#[derive(Debug, Clone)]
pub struct Closure {
    /// The parameter names, e.g. `["a", "b"]` for `(fn (a b) ...)`.
    pub params: Vec<String>,
    /// The function body (an unevaluated expression, evaluated on call).
    pub body: Expr,
    /// The environment the closure was created in — what it "closes over".
    pub env: Rc<RefCell<Env>>,
}
