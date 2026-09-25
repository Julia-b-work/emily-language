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

impl Value {
    /// Serializes this value to a JSON string.
    ///
    /// Only "data" values (numbers, strings, booleans, lists, records) can be
    /// serialized. A closure is an error — matching the spec's rule that
    /// functions never appear in output.
    pub fn to_json(&self) -> Result<String, String> {
        match self {
            Value::Int(n) => Ok(n.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::String(s) => Ok(format!("\"{}\"", escape(s))),
            Value::List(items) => {
                let parts: Result<Vec<String>, String> =
                    items.iter().map(|v| v.to_json()).collect();
                Ok(format!("[{}]", parts?.join(", ")))
            }
            Value::Record(fields) => {
                let parts: Result<Vec<String>, String> = fields
                    .iter()
                    .map(|(k, v)| Ok(format!("\"{}\": {}", escape(k), v.to_json()?)))
                    .collect();
                Ok(format!("{{{}}}", parts?.join(", ")))
            }
            Value::Closure(_) => Err("cannot serialize a function to JSON".to_string()),
        }
    }

    /// Serializes this value to a YAML string (block style).
    ///
    /// Like [`to_json`], a closure is an error — functions never appear in
    /// output.
    pub fn to_yaml(&self) -> Result<String, String> {
        Ok(yaml_lines(self)?.join("\n"))
    }

    /// Serializes this value to a TOML string.
    ///
    /// TOML documents must be a table at the top level, so this errors unless
    /// the value is a record.
    pub fn to_toml(&self) -> Result<String, String> {
        match self {
            Value::Record(fields) => {
                let mut lines = Vec::new();
                for (k, v) in fields {
                    lines.push(format!("{} = {}", k, toml_value(v)?));
                }
                Ok(lines.join("\n"))
            }
            Value::Closure(_) => Err("cannot serialize a function to TOML".to_string()),
            _ => Err("TOML output requires a top-level record (mapping)".to_string()),
        }
    }
}

/// Escapes special characters in a string for JSON output.
fn escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

/// Whether a value is a collection (list or record), i.e. not a scalar.
fn is_container(v: &Value) -> bool {
    matches!(v, Value::List(_) | Value::Record(_))
}

/// Serializes a value into YAML block-style lines.
///
/// Each line carries no parent-relative indentation; the caller indents a
/// nested block's lines by prefixing them with two spaces. Scalars produce a
/// single line, so a list item or record field is "inline" iff its value is a
/// scalar.
fn yaml_lines(v: &Value) -> Result<Vec<String>, String> {
    match v {
        Value::Int(n) => Ok(vec![n.to_string()]),
        Value::Float(f) => Ok(vec![f.to_string()]),
        Value::Bool(b) => Ok(vec![b.to_string()]),
        Value::String(s) => Ok(vec![format!("\"{}\"", escape(s))]),
        Value::List(items) => {
            if items.is_empty() {
                return Ok(vec!["[]".to_string()]);
            }
            let mut lines = Vec::new();
            for item in items {
                let sub = yaml_lines(item)?;
                lines.push(format!("- {}", sub[0]));
                for l in &sub[1..] {
                    lines.push(format!("  {}", l));
                }
            }
            Ok(lines)
        }
        Value::Record(fields) => {
            if fields.is_empty() {
                return Ok(vec!["{}".to_string()]);
            }
            let mut lines = Vec::new();
            for (k, v) in fields {
                let sub = yaml_lines(v)?;
                if is_container(v) {
                    lines.push(format!("{}:", k));
                    for l in &sub {
                        lines.push(format!("  {}", l));
                    }
                } else {
                    lines.push(format!("{}: {}", k, sub[0]));
                }
            }
            Ok(lines)
        }
        Value::Closure(_) => Err("cannot serialize a function to YAML".to_string()),
    }
}

/// Serializes a value as TOML inline (used inside tables and arrays).
fn toml_value(v: &Value) -> Result<String, String> {
    match v {
        Value::Int(n) => Ok(n.to_string()),
        Value::Float(f) => Ok(f.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::String(s) => Ok(format!("\"{}\"", escape(s))),
        Value::List(items) => {
            let parts: Result<Vec<String>, String> = items.iter().map(toml_value).collect();
            Ok(format!("[{}]", parts?.join(", ")))
        }
        Value::Record(fields) => {
            let parts: Result<Vec<String>, String> = fields
                .iter()
                .map(|(k, v)| Ok(format!("{} = {}", k, toml_value(v)?)))
                .collect();
            Ok(format!("{{ {} }}", parts?.join(", ")))
        }
        Value::Closure(_) => Err("cannot serialize a function to TOML".to_string()),
    }
}
