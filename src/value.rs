
use crate::ast::Expr;
use crate::env::Env;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Record(Vec<(String, Value)>),
    Closure(Closure),
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Rc<RefCell<Env>>,
}

impl Value {
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

    pub fn to_yaml(&self) -> Result<String, String> {
        Ok(yaml_lines(self)?.join("\n"))
    }

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

fn is_container(v: &Value) -> bool {
    matches!(v, Value::List(_) | Value::Record(_))
}

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
