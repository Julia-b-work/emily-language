//! The type checker: walks the AST and computes the type of each expression.
//!
//! `type_check` is the static twin of [`crate::eval::eval`]: it descends the
//! same tree, but instead of producing runtime values it produces
//! [`Type`]s. Special forms (`def`, `let`, `if`, `fn`) are dispatched in
//! `type_list`, along with the built-in functions and user-function calls.

use crate::ast::Expr;
use crate::types::Type;
use std::collections::HashMap;

/// A type-checking scope: names bound to their types, plus an optional parent
/// scope. Looking up a name walks up the chain, so inner scopes see (and
/// shadow) outer bindings.
///
/// The parent is a plain `&` reference: type checking is a straight-line walk
/// with strictly nested scopes, so a child never outlives its parent and no
/// `Rc`/`RefCell` machinery is needed (unlike [`crate::env::Env`], which
/// closures capture at runtime).
pub struct Ctx<'a> {
    bindings: HashMap<String, Type>,
    parent: Option<&'a Ctx<'a>>,
}

impl<'a> Ctx<'a> {
    /// Creates a new, empty top-level scope.
    pub fn new() -> Self {
        Ctx { bindings: HashMap::new(), parent: None }
    }

    /// Creates a new scope nested inside `parent`.
    pub fn child(parent: &'a Ctx<'a>) -> Self {
        Ctx { bindings: HashMap::new(), parent: Some(parent) }
    }

    /// Binds `name` to `ty` in this scope (shadowing any existing binding).
    pub fn define(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    /// Looks up `name`, checking this scope then walking up the parent chain.
    pub fn get(&self, name: &str) -> Option<Type> {
        if let Some(ty) = self.bindings.get(name) {
            return Some(ty.clone());
        }
        self.parent?.get(name)
    }
}



pub fn type_check(expr: &Expr, ctx: &mut Ctx) -> Result<Type, String> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::String(_) => Ok(Type::String),
        Expr::Bool(_) => Ok(Type::Bool),

        Expr::Ann { value, ty } => {
            let declared = Type::from_expr(ty)?;
            let actual = type_check(value, ctx)?;
            if actual != declared {
                return Err(format!("type mismatch: expected {declared:?}, got {actual:?}"));
            }
            Ok(declared)
        }

        Expr::Symbol(emily) => {
            if let Some((base, field)) = emily.split_once('.') {
                let base_type = ctx
                    .get(base)
                    .ok_or_else(|| format!("unknown name: {base}"))?;
                match base_type {
                    Type::Record(fields) => {
                        for (key, ty) in fields {
                            if key == field {
                                return Ok(ty.clone());
                            }
                        }
                        Err(format!("no field '{field}' in record"))
                    }
                    _ => Err(format!("'{base}' is not a record")),
                }
            }
            else {
                ctx.get(emily).ok_or_else(|| format!("unknown name: {emily}"))
                }
    }

    Expr::Vector(items) => {
        let (first, others) = items.split_first().ok_or("placeholder")?;
        let first_type = type_check(first, ctx)?;
        for item in others {
            let ty = type_check(item, ctx)?;
            if ty != first_type {
                return Err(format!("list elements must all have the same type: {first_type:?} vs {ty:?}"));
            }
        } 
        Ok(Type::List(Box::new(first_type)))
    }


    Expr::Record(fields) => {
        let mut out = Vec::new();
        for (key, value) in fields {
            out.push((key.clone(), type_check(value, ctx)?));
        }
        Ok(Type::Record(out))
    }


    Expr::List(items) => type_list(items, ctx),

        other => Err(format!("typecheck: not implemented yet: {other:?}")),
    }
}

/// Type-checks a whole program: every top-level form in order, in one shared
/// root scope (later `def`s see earlier ones).
pub fn typecheck(forms: &[Expr]) -> Result<(), String> {
    let mut ctx = Ctx::new();
    for form in forms {
        type_check(form, &mut ctx)?;
    }
    Ok(())
}

/// Extracts a binding name from an expression, unwrapping type annotations
/// (`x : Int` names `x`). Returns `None` for anything that is not a symbol.
fn binding_name(expr: &Expr) -> Option<&str> {
    match expr {
        Expr::Symbol(n) => Some(n),
        Expr::Ann { value, .. } => binding_name(value),
        _ => None,
    }
}

/// Type-checks a list by dispatching on its head symbol: special forms
/// (`def`, `let`, `if`, `fn`), the built-in functions, or a user-function call.
fn type_list(items: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    let (head, rest) = items.split_first().ok_or("empty list")?;
    match head {
        Expr::Symbol(name) => match name.as_str() {
            "def" => type_def(rest, ctx),
            "let" => type_let(rest, ctx),
            "if" => type_if(rest, ctx),
            "fn" => type_fn(rest, ctx),
            "+" | "-" | "*" | "/" => numeric_builtin(name, rest, ctx),
            "=" | ">" | "<" | ">=" | "<=" => compare_builtin(name, rest, ctx),
            "and" | "or" => bool_builtin(name, rest, ctx),
            "not" => not_builtin(name, rest, ctx),
            "concat" => concat_builtin(name, rest, ctx),
            "show" => show_builtin(name, rest, ctx),
            "fold" | "append" | "merge" => {
                Err(format!("{name}: not implemented in the type checker yet"))
            }
            _ => apply_function(name, rest, ctx),
        },
        _ => Err("list must start with a symbol".to_string()),
    }
}

/// `(def name expr)` or `(def name : Type expr)` — checks the value against
/// the annotation (if any) and binds the name to its type in the current scope.
fn type_def(rest: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    let [name_expr, value_expr] = rest else {
        return Err("def needs a name and a value".to_string());
    };
    let name = binding_name(name_expr)
        .ok_or("def needs a name")?
        .to_string();
    let value_ty = type_check(value_expr, ctx)?;
    if let Expr::Ann { ty, .. } = name_expr {
        let declared = Type::from_expr(ty)?;
        if value_ty != declared {
            return Err(format!("def {name}: expected {declared:?}, got {value_ty:?}"));
        }
    }
    ctx.define(name, value_ty.clone());
    Ok(value_ty)
}

/// `(let [name value ...] body)` — checks the bindings sequentially in a fresh
/// child scope (later bindings see earlier ones), then the body in that scope.
fn type_let(rest: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    let bindings = match rest.first() {
        Some(Expr::Vector(b)) => b,
        _ => return Err("let needs a bindings vector".to_string()),
    };
    let body = rest.get(1).ok_or("let needs a body")?;

    if bindings.len() % 2 != 0 {
        return Err("let bindings must be name/value pairs".to_string());
    }

    let mut child = Ctx::child(ctx);
    let mut i = 0;
    while i < bindings.len() {
        let name = binding_name(&bindings[i])
            .ok_or("let binding name must be a symbol")?
            .to_string();
        let value_ty = type_check(&bindings[i + 1], &mut child)?;
        if let Expr::Ann { ty, .. } = &bindings[i] {
            let declared = Type::from_expr(ty)?;
            if value_ty != declared {
                return Err(format!("let {name}: expected {declared:?}, got {value_ty:?}"));
            }
        }
        child.define(name, value_ty);
        i += 2;
    }
    type_check(body, &mut child)
}

/// `(if cond then else)` — the condition must be a `Bool`, and both branches
/// must have the same type, which is the type of the whole form.
fn type_if(rest: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    if rest.len() != 3 {
        return Err("if needs a condition, then-branch, and else-branch".to_string());
    }
    let cond_ty = type_check(&rest[0], ctx)?;
    if cond_ty != Type::Bool {
        return Err(format!("if condition must be a Bool, got {cond_ty:?}"));
    }
    let then_ty = type_check(&rest[1], ctx)?;
    let else_ty = type_check(&rest[2], ctx)?;
    if then_ty != else_ty {
        return Err(format!(
            "if branches must have the same type: {then_ty:?} vs {else_ty:?}"
        ));
    }
    Ok(then_ty)
}

/// `(fn (args) body)` — every parameter must be annotated (`(fn (a : Int) ...)`,
/// the monomorphic rule); the body is checked in a child scope holding the
/// parameters, and its type is the return type.
fn type_fn(rest: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    let params = match rest.first() {
        Some(Expr::List(p)) => p,
        _ => return Err("fn needs a parameter list".to_string()),
    };
    let body = rest.get(1).ok_or("fn needs a body")?;

    let mut child = Ctx::child(ctx);
    let mut param_tys = Vec::new();
    for param in params {
        match param {
            Expr::Ann { value, ty } => {
                let name = binding_name(value)
                    .ok_or("fn parameter name must be a symbol")?;
                let ty = Type::from_expr(ty)?;
                param_tys.push(ty.clone());
                child.define(name.to_string(), ty);
            }
            Expr::Symbol(name) => {
                return Err(format!("fn parameter '{name}' needs a type annotation"));
            }
            _ => return Err("fn parameters must be symbols".to_string()),
        }
    }
    let ret = type_check(body, &mut child)?;
    Ok(Type::Fn { params: param_tys, ret: Box::new(ret) })
}

/// `(+ a b ...)`, `(- a b)`, `(* a b ...)`, `(/ a b)` — integer arithmetic.
fn numeric_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    for arg in args {
        let ty = type_check(arg, ctx)?;
        if ty != Type::Int {
            return Err(format!("{op} expects numbers, got {ty:?}"));
        }
    }
    Ok(Type::Int)
}

/// `(> a b)`, `(< a b)`, `(>= a b)`, `(<= a b)` — numeric ordering comparisons.
fn compare_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    if args.len() != 2 {
        return Err(format!("{op} expects two numbers"));
    }
    for arg in args {
        let ty = type_check(arg, ctx)?;
        if ty != Type::Int {
            return Err(format!("{op} expects numbers, got {ty:?}"));
        }
    }
    Ok(Type::Bool)
}

/// `(and a b ...)` and `(or a b ...)` — boolean logic.
fn bool_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    for arg in args {
        let ty = type_check(arg, ctx)?;
        if ty != Type::Bool {
            return Err(format!("{op} expects booleans, got {ty:?}"));
        }
    }
    Ok(Type::Bool)
}

/// `(not x)` — logical negation.
fn not_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    if args.len() != 1 {
        return Err(format!("{op} expects one argument"));
    }
    let ty = type_check(&args[0], ctx)?;
    if ty != Type::Bool {
        return Err(format!("{op} expects a boolean, got {ty:?}"));
    }
    Ok(Type::Bool)
}

/// `(concat a b ...)` — string concatenation.
fn concat_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    for arg in args {
        let ty = type_check(arg, ctx)?;
        if ty != Type::String {
            return Err(format!("{op} expects strings, got {ty:?}"));
        }
    }
    Ok(Type::String)
}

/// `(show x)` — turns any value into a string.
fn show_builtin(op: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    if args.len() != 1 {
        return Err(format!("{op} expects one argument"));
    }
    type_check(&args[0], ctx)?; // any type is allowed
    Ok(Type::String)
}

/// `(f a b ...)` — calls a user-defined function: `f` must be a `Fn` type and
/// the arguments must match the parameter types.
fn apply_function(name: &str, args: &[Expr], ctx: &mut Ctx) -> Result<Type, String> {
    let f_ty = ctx
        .get(name)
        .ok_or_else(|| format!("unknown function: {name}"))?;
    match f_ty {
        Type::Fn { params, ret } => {
            if args.len() != params.len() {
                return Err(format!(
                    "{name} expects {} arguments, got {}",
                    params.len(),
                    args.len()
                ));
            }
            for (arg, param_ty) in args.iter().zip(params.iter()) {
                let arg_ty = type_check(arg, ctx)?;
                if arg_ty != *param_ty {
                    return Err(format!("{name}: expected {param_ty:?}, got {arg_ty:?}"));
                }
            }
            Ok(*ret)
        }
        _ => Err(format!("{name} is not a function")),
    }
}














