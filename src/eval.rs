//! The evaluator: walks the AST and produces runtime values.
//!
//! `eval` turns an [`Expr`] tree into a [`Value`], using [`Env`] to look up
//! and bind names. Special forms (`def`, `let`, `if`, `fn`) are dispatched in
//! `eval_list`, along with the built-in functions.

use crate::ast::Expr;
use crate::env::Env;
use crate::value::{Closure, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Evaluates an expression in the given environment, producing a value.
///
/// Atoms evaluate to themselves, symbols are looked up (or treated as field
/// access), and compound forms are evaluated element-wise or dispatched.
pub fn eval(expr: &Expr, env: Rc<RefCell<Env>>) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Float(f) => Ok(Value::Float(*f)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Bool(b) => Ok(Value::Bool(*b)),

        Expr::Symbol(name) => {
            // `base.field` is field access; otherwise it's a plain lookup.
            if let Some((base, field)) = name.split_once('.') {
                let base_value = env
                    .borrow()
                    .get(base)
                    .ok_or_else(|| format!("unknown name: {base}"))?;
                match base_value {
                    Value::Record(fields) => {
                        for (key, value) in fields {
                            if key == field {
                                return Ok(value);
                            }
                        }
                        Err(format!("no field '{field}' in record"))
                    }
                    _ => Err(format!("'{base}' is not a record")),
                }
            } else {
                env.borrow()
                    .get(name)
                    .ok_or_else(|| format!("unknown name: {name}"))
            }
        }

        Expr::Vector(items) => {
            let mut out = Vec::new();
            for item in items {
                out.push(eval(item, env.clone())?);
            }
            Ok(Value::List(out))
        }

        Expr::Record(fields) => {
            let mut out = Vec::new();
            for (key, value) in fields {
                out.push((key.clone(), eval(value, env.clone())?));
            }
            Ok(Value::Record(out))
        }

        Expr::List(items) => eval_list(items, env),

        other => Err(format!("not implemented yet: {other:?}")),
    }
}

/// Evaluates a list by dispatching on its head symbol.
fn eval_list(items: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let (head, rest) = items.split_first().ok_or("empty list")?;
    match head {
        Expr::Symbol(name) => match name.as_str() {
            "def" => eval_def(rest, env),
            "let" => eval_let(rest, env),
            "if" => eval_if(rest, env),
            "fn" => eval_fn(rest, env),
            "+" => builtin_add(rest, env),
            "concat" => builtin_concat(rest, env),
            "show" => builtin_show(rest, env),
            "fold" => builtin_fold(rest, env),
            "append" => builtin_append(rest, env),
            "merge" => builtin_merge(rest, env),
            _ => apply_function(name, rest, env),
        },
        _ => Err("list must start with a symbol".to_string()),
    }
}

/// Evaluates `(def name expr)`, binding `name` in the current scope.
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

/// Evaluates `(fn (args) body)`, producing a closure that captures `env`.
fn eval_fn(rest: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let params = match rest.first() {
        Some(Expr::List(p)) => {
            let mut names = Vec::new();
            for param in p {
                match param {
                    Expr::Symbol(n) => names.push(n.clone()),
                    _ => return Err("fn parameters must be symbols".to_string()),
                }
            }
            names
        }
        _ => return Err("fn needs a parameter list".to_string()),
    };
    let body = rest.get(1).ok_or("fn needs a body")?;
    Ok(Value::Closure(Closure {
        params,
        body: body.clone(),
        env: env.clone(),
    }))
}

/// Calls a closure with already-evaluated argument values.
///
/// Creates a child scope of the closure's *captured* environment, binds the
/// parameters to the arguments, and evaluates the body.
fn call_closure(closure: &Closure, arg_values: Vec<Value>) -> Result<Value, String> {
    if arg_values.len() != closure.params.len() {
        return Err(format!(
            "expected {} arguments, got {}",
            closure.params.len(),
            arg_values.len()
        ));
    }
    let call_env = Env::child(closure.env.clone());
    for (param, value) in closure.params.iter().zip(arg_values) {
        call_env.borrow_mut().define(param.clone(), value);
    }
    eval(&closure.body, call_env)
}

/// Calls a user-defined function by name.
fn apply_function(name: &str, args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let func = env
        .borrow()
        .get(name)
        .ok_or_else(|| format!("unknown function: {name}"))?;
    match func {
        Value::Closure(closure) => {
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(eval(arg, env.clone())?);
            }
            call_closure(&closure, arg_values)
        }
        _ => Err(format!("{name} is not a function")),
    }
}

/// `(+ a b ...)` — sums integer arguments.
fn builtin_add(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let mut total = 0i64;
    for arg in args {
        match eval(arg, env.clone())? {
            Value::Int(n) => total += n,
            other => return Err(format!("+ expects numbers, got {other:?}")),
        }
    }
    Ok(Value::Int(total))
}

/// `(concat a b ...)` — concatenates string arguments.
fn builtin_concat(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let mut out = String::new();
    for arg in args {
        match eval(arg, env.clone())? {
            Value::String(s) => out.push_str(&s),
            other => return Err(format!("concat expects strings, got {other:?}")),
        }
    }
    Ok(Value::String(out))
}

/// `(show x)` — turns a value into a string.
fn builtin_show(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("show expects one argument".to_string());
    }
    let value = eval(&args[0], env.clone())?;
    let text = match value {
        Value::Int(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s,
        Value::Bool(b) => b.to_string(),
        other => return Err(format!("can't show {other:?}")),
    };
    Ok(Value::String(text))
}

/// `(append list value)` — returns the list with `value` added at the end.
fn builtin_append(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("append expects a list and a value".to_string());
    }
    let list = eval(&args[0], env.clone())?;
    let value = eval(&args[1], env.clone())?;
    match list {
        Value::List(mut items) => {
            items.push(value);
            Ok(Value::List(items))
        }
        _ => Err("append expects a list as its first argument".to_string()),
    }
}

/// `(fold list initial (fn (acc x) body))` — reduces a list with a function.
fn builtin_fold(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("fold expects a list, an initial value, and a function".to_string());
    }
    let list = eval(&args[0], env.clone())?;
    let mut acc = eval(&args[1], env.clone())?;
    let func = eval(&args[2], env.clone())?;
    match (list, func) {
        (Value::List(items), Value::Closure(closure)) => {
            for item in items {
                acc = call_closure(&closure, vec![acc, item])?;
            }
            Ok(acc)
        }
        _ => Err("fold expects a list and a function".to_string()),
    }
}

/// `(merge a b ...)` — combines records; later records override earlier keys.
fn builtin_merge(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Value, String> {
    let mut merged: Vec<(String, Value)> = Vec::new();
    for arg in args {
        match eval(arg, env.clone())? {
            Value::Record(fields) => {
                for (key, value) in fields {
                    // Update in place if the key already exists (preserving
                    // order), otherwise append it.
                    match merged.iter_mut().find(|(k, _)| k == &key) {
                        Some(entry) => entry.1 = value,
                        None => merged.push((key, value)),
                    }
                }
            }
            other => return Err(format!("merge expects records, got {other:?}")),
        }
    }
    Ok(Value::Record(merged))
}
