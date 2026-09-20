//! The evaluator: walks the AST and produces runtime values.
//!
//! `eval` turns an [`Expr`] tree into a [`Value`], using [`Env`] to look up
//! and bind names. Special forms (`def`, `let`, `if`) are dispatched in
//! `eval_list`; function calls and builtins come in a later step.

use crate::ast::Expr;
use crate::env::Env;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

/// Evaluates an expression in the given environment, producing a value.
///
/// Atoms evaluate to themselves, symbols are looked up, and compound forms
/// (vectors, records, lists) are evaluated element-wise or dispatched.
pub fn eval(expr: &Expr, env: Rc<RefCell<Env>>) -> Result<Value, String> {
    match expr {
        // Atoms evaluate to themselves.
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Float(f) => Ok(Value::Float(*f)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Bool(b) => Ok(Value::Bool(*b)),

        // A symbol is looked up in the environment.
        Expr::Symbol(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| format!("unknown name: {name}")),

        // A vector evaluates each element, then becomes a plain list.
        Expr::Vector(items) => {
            let mut out = Vec::new();
            for item in items {
                out.push(eval(item, env.clone())?);
            }
            Ok(Value::List(out))
        }

        // A record evaluates each value but keeps its keys.
        Expr::Record(fields) => {
            let mut out = Vec::new();
            for (key, value) in fields {
                out.push((key.clone(), eval(value, env.clone())?));
            }
            Ok(Value::Record(out))
        }

        // A list is a special form or a function call — dispatch on its head.
        Expr::List(items) => eval_list(items, env),

        // Quote forms (reader macros) are deferred to v2.
        other => Err(format!("not implemented yet: {other:?}")),
    }
}

/// Evaluates a list by dispatching on its head symbol.
///
/// If the head is a special form (`def`/`let`/`if`), routes to its handler;
/// otherwise it's a function call (handled in a later step).
fn eval_list(items: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let (head, rest) = items.split_first().ok_or("empty list")?;
    match head {
        Expr::Symbol(name) => match name.as_str() {
            "def" => eval_def(rest, env),
            "let" => eval_let(rest, env),
            "if" => eval_if(rest, env),
            _ => Err(format!("unknown function: {name}")),
        },
        _ => Err("list must start with a symbol".to_string()),
    }
}

/// Evaluates `(def name expr)`, binding `name` in the current scope.
///
/// The value is the last element, so an optional `: Type` annotation between
/// the name and value is ignored (type checking comes in v2).
fn eval_def(rest: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let name = match rest.first() {
        Some(Expr::Symbol(n)) => n.clone(),
        _ => return Err("def needs a name".to_string()),
    };
    let value_expr = rest.last().ok_or("def needs a value")?;
    let value = eval(value_expr, env.clone())?;
    env.borrow_mut().define(name, value.clone());
    Ok(value)
}

/// Evaluates `(let [name value ...] body)` in a fresh child scope.
///
/// Bindings are sequential (later ones see earlier ones), and the body runs
/// in the child so the names don't leak into the outer scope.
fn eval_let(rest: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let bindings = match rest.first() {
        Some(Expr::Vector(b)) => b,
        _ => return Err("let needs a bindings vector".to_string()),
    };
    let body = rest.get(1).ok_or("let needs a body")?;

    if bindings.len() % 2 != 0 {
        return Err("let bindings must be name/value pairs".to_string());
    }

    let child = Env::child(env);
    let mut i = 0;
    while i < bindings.len() {
        let name = match &bindings[i] {
            Expr::Symbol(n) => n.clone(),
            _ => return Err("let binding name must be a symbol".to_string()),
        };
        let value = eval(&bindings[i + 1], child.clone())?;
        child.borrow_mut().define(name, value);
        i += 2;
    }
    eval(body, child)
}

/// Evaluates `(if cond then else)`, running exactly one branch.
fn eval_if(rest: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    if rest.len() != 3 {
        return Err("if needs a condition, then-branch, and else-branch".to_string());
    }
    let cond = eval(&rest[0], env.clone())?;
    match cond {
        Value::Bool(true) => eval(&rest[1], env),
        Value::Bool(false) => eval(&rest[2], env),
        _ => Err("if condition must be a Bool".to_string()),
    }
}
