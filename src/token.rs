//! The token types produced by the lexer.

/// A single lexical token — the smallest meaningful unit the lexer produces —
/// plus the line and column where it starts, so later passes can report
/// precise error locations.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The token's kind and payload.
    pub kind: TokenKind,
    /// The 1-based line the token starts on.
    pub line: usize,
    /// The 1-based column the token starts at.
    pub col: usize,
}

/// The kind of a token, with its payload (if any).
///
/// Variants with payloads (`Int(i64)`, `Symbol(String)`, …) carry a value;
/// unit variants (`LParen`, `Quote`, …) are just markers.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
