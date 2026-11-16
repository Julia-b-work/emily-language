//! Type expressions: the `Type` enum and parsing type syntax from the AST.

use crate::ast::Expr;

/// A type in Emily's type system (first version: monomorphic).
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Record(Vec<(String, Type)>),
    Fn {
        params: Vec<Type>,
        ret: Box<Type>,
    },
    /// A named type, e.g. `Server`. Resolved structurally by the type checker;
    /// at the syntax layer any non-primitive symbol becomes a `Named`.
    Named(String),
}

impl Type {
    /// Parses a type from an already-parsed expression.
    pub fn from_expr(expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Symbol(name) => match name.as_str() {
                "Int" => Ok(Type::Int),
                "Float" => Ok(Type::Float),
                "String" => Ok(Type::String),
                "Bool" => Ok(Type::Bool),
                other => Ok(Type::Named(other.to_string())),
            },
            Expr::Record(fields) => {
                let mut out = Vec::new();
                for (key, value) in fields {
                    out.push((key.clone(), Type::from_expr(value)?));
                }
                Ok(Type::Record(out))
            }
            Expr::List(items) => {
                if let [Expr::Symbol(name), inner] = items.as_slice() {
                    if name == "List" {
                        return Ok(Type::List(Box::new(Type::from_expr(inner)?)));
                    }
                }
                if items
                    .iter()
                    .any(|it| matches!(it, Expr::Symbol(s) if s == "->"))
                {
                    return parse_fn_type(items);
                }
                Err(format!("not a type: {items:?}"))
            }
            other => Err(format!("not a type: {other:?}")),
        }
    }
}

/// Parses `(A -> B -> R)` into `Type::Fn { params: [A, B], ret: R }`.
///
/// Minimal version for now; the design discussion may change this.
fn parse_fn_type(items: &[Expr]) -> Result<Type, String> {
    let parts: Vec<&[Expr]> = items
        .split(|it| matches!(it, Expr::Symbol(s) if s == "->"))
        .collect();
    if parts.len() < 2 {
        return Err("a function type needs at least one ->".to_string());
    }
    let mut types = Vec::new();
    for part in &parts {
        if part.len() != 1 {
            return Err(format!(
                "each side of -> must be a single type, got {part:?}"
            ));
        }
        types.push(Type::from_expr(&part[0])?);
    }
    let ret = Box::new(types.pop().unwrap());
    Ok(Type::Fn { params: types, ret })
}
