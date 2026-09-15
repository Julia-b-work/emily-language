//! The token types produced by the lexer.

/// A single lexical token — the smallest meaningful unit the lexer produces.
///
/// Variants with payloads (`Int(i64)`, `Symbol(String)`, …) carry a value;
/// unit variants (`LParen`, `Quote`, …) are just markers.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A whole number.
    Int(i64),
    /// A floating-point number.
    Float(f64),
    /// A string literal, with escapes already translated.
    String(String),
    /// A boolean literal (`true` / `false`).
    Bool(bool),
    /// An identifier or operator symbol.
    Symbol(String),
    /// A record key, e.g. `:port`.
    Keyword(String),
    /// A standalone `:` used to introduce a type annotation.
    Colon,
    // Delimiters.
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // Reader macros (`'`, `` ` ``, `,`, `,@`).
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplice,
}
