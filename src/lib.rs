//! Emily — a total, statically typed Lisp for configuration and data.
//!
//! This crate is a library (not just a binary) so that integration tests in
//! `tests/` can import the lexer and parser; `main.rs` is a thin binary that
//! calls into it.

pub mod ast;
pub mod env;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod value;
