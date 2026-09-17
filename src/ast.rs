//! The abstract syntax tree (AST).
//!
//! The parser produces a tree of [`Expr`] values; the evaluator later walks
//! that tree to run a program.

/// A parsed expression — one node in the abstract syntax tree.
///
/// This mirrors the [`crate::token::Token`] enum, but the compound variants
/// hold *other* expressions, which makes the type recursive. The quote-family
/// variants use `Box<Expr>` because a recursive type that contains itself
/// directly would have infinite size.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A whole-number literal, e.g. `42`.
    Int(i64),
    /// A floating-point literal, e.g. `3.14`.
    Float(f64),
    /// A string literal, e.g. `"hello"`.
    String(String),
    /// A boolean literal (`true` or `false`).
    Bool(bool),
    /// An identifier or operator, e.g. `x` or `+`.
    Symbol(String),
    /// A record key, e.g. `:port`.
    Keyword(String),
    /// A list of forms: `(a b c)`.
    List(Vec<Expr>),
    /// A record: `{:key value ...}`, stored as key/value pairs.
    Record(Vec<(String, Expr)>),
    /// A vector: `[a b c]`.
    Vector(Vec<Expr>),
    /// A quoted form, from the `'` reader macro.
    Quote(Box<Expr>),
    /// A quasiquoted form, from the backtick reader macro.
    Quasiquote(Box<Expr>),
    /// An unquoted form, from the `,` reader macro.
    Unquote(Box<Expr>),
    /// An unquote-splicing form, from the `,@` reader macro.
    UnquoteSplice(Box<Expr>),
}
