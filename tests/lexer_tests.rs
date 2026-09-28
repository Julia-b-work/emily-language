//! Integration tests for the lexer: source text → tokens.

use emily::lexer::Lexer;
use emily::token::{Token, TokenKind};

fn lex(src: &str) -> Vec<Token> {
    Lexer::new(src).tokenize().unwrap()
}

/// Lexes and drops positions, so tests can assert on kinds only.
fn kinds(src: &str) -> Vec<TokenKind> {
    lex(src).into_iter().map(|t| t.kind).collect()
}

#[test]
fn lexes_symbols_and_numbers() {
    assert_eq!(
        kinds("(+ 1 2)"),
        vec![
            TokenKind::LParen,
            TokenKind::Symbol("+".to_string()),
            TokenKind::Int(1),
            TokenKind::Int(2),
            TokenKind::RParen,
        ]
    );
}

#[test]
fn int_test() {
    assert_eq!(kinds("42"), vec![TokenKind::Int(42)]);
}

#[test]
fn float_test() {
    assert_eq!(kinds("3.14"), vec![TokenKind::Float(3.14)]);
}

#[test]
fn neg_or_symb_test1() {
    assert_eq!(kinds("-1"), vec![TokenKind::Int(-1)]);
}

#[test]
fn neg_or_symb_test2() {
    assert_eq!(kinds("-"), vec![TokenKind::Symbol("-".to_string())]);
}

#[test]
fn keyword_vs_colon() {
    assert_eq!(kinds(":port"), vec![TokenKind::Keyword("port".to_string())]);
    assert_eq!(
        kinds(": Int"),
        vec![TokenKind::Colon, TokenKind::Symbol("Int".to_string()),]
    );
}

#[test]
fn dot_is_field_access_in_symbols() {
    assert_eq!(
        kinds("foo.bar"),
        vec![TokenKind::Symbol("foo.bar".to_string())]
    );
}

#[test]
fn booleans() {
    assert_eq!(
        kinds("true false"),
        vec![TokenKind::Bool(true), TokenKind::Bool(false),]
    );
}

#[test]
fn string_with_escape() {
    assert_eq!(
        kinds("\"hi\\n\""),
        vec![TokenKind::String("hi\n".to_string()),]
    );
}

#[test]
fn reader_macros() {
    assert_eq!(
        kinds("'x"),
        vec![TokenKind::Quote, TokenKind::Symbol("x".to_string())]
    );
    assert_eq!(
        kinds("`x"),
        vec![TokenKind::Quasiquote, TokenKind::Symbol("x".to_string())]
    );
    assert_eq!(
        kinds(",x"),
        vec![TokenKind::Unquote, TokenKind::Symbol("x".to_string())]
    );
    assert_eq!(
        kinds(",@x"),
        vec![TokenKind::UnquoteSplice, TokenKind::Symbol("x".to_string())]
    );
}

#[test]
fn record_literal() {
    assert_eq!(
        kinds("{:a 1}"),
        vec![
            TokenKind::LBrace,
            TokenKind::Keyword("a".to_string()),
            TokenKind::Int(1),
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn comments_are_skipped() {
    assert_eq!(kinds("; hello\n42"), vec![TokenKind::Int(42)]);
}

#[test]
fn empty_input() {
    assert_eq!(kinds(""), vec![]);
}

#[test]
fn tracks_line_and_column() {
    let tokens = lex("(def\n  x 42)");
    // `42` is on the second line, after two spaces and `x `.
    let int = tokens
        .iter()
        .find(|t| t.kind == TokenKind::Int(42))
        .unwrap();
    assert_eq!(int.line, 2);
    assert_eq!(int.col, 5);
}

#[test]
fn lexer_errors_include_position() {
    // `@` is an unexpected character at line 1, column 6.
    let err = Lexer::new("(foo @)").tokenize().unwrap_err();
    assert!(err.contains("line 1, column 6"), "got: {err}");
}
