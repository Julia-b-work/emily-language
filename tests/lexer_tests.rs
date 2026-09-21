//! Integration tests for the lexer: source text → tokens.

use emily::lexer::Lexer;
use emily::token::Token;

fn lex(src: &str) -> Vec<Token> {
    Lexer::new(src).tokenize().unwrap()
}

#[test]
fn lexes_symbols_and_numbers() {
    assert_eq!(
        lex("(+ 1 2)"),
        vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Int(1),
            Token::Int(2),
            Token::RParen,
        ]
    );
}

#[test]
fn int_test() {
    assert_eq!(lex("42"), vec![Token::Int(42)]);
}

#[test]
fn float_test() {
    assert_eq!(lex("3.14"), vec![Token::Float(3.14)]);
}

#[test]
fn neg_or_symb_test1() {
    assert_eq!(lex("-1"), vec![Token::Int(-1)]);
}

#[test]
fn neg_or_symb_test2() {
    assert_eq!(lex("-"), vec![Token::Symbol("-".to_string())]);
}

#[test]
fn keyword_vs_colon() {
    assert_eq!(lex(":port"), vec![Token::Keyword("port".to_string())]);
    assert_eq!(
        lex(": Int"),
        vec![Token::Colon, Token::Symbol("Int".to_string()),]
    );
}

#[test]
fn dot_is_field_access_in_symbols() {
    assert_eq!(lex("foo.bar"), vec![Token::Symbol("foo.bar".to_string())]);
}

#[test]
fn booleans() {
    assert_eq!(
        lex("true false"),
        vec![Token::Bool(true), Token::Bool(false),]
    );
}

#[test]
fn string_with_escape() {
    assert_eq!(lex("\"hi\\n\""), vec![Token::String("hi\n".to_string()),]);
}

#[test]
fn reader_macros() {
    assert_eq!(
        lex("'x"),
        vec![Token::Quote, Token::Symbol("x".to_string())]
    );
    assert_eq!(
        lex("`x"),
        vec![Token::Quasiquote, Token::Symbol("x".to_string())]
    );
    assert_eq!(
        lex(",x"),
        vec![Token::Unquote, Token::Symbol("x".to_string())]
    );
    assert_eq!(
        lex(",@x"),
        vec![Token::UnquoteSplice, Token::Symbol("x".to_string())]
    );
}

#[test]
fn record_literal() {
    assert_eq!(
        lex("{:a 1}"),
        vec![
            Token::LBrace,
            Token::Keyword("a".to_string()),
            Token::Int(1),
            Token::RBrace,
        ]
    );
}

#[test]
fn comments_are_skipped() {
    assert_eq!(lex("; hello\n42"), vec![Token::Int(42)]);
}

#[test]
fn empty_input() {
    assert_eq!(lex(""), vec![]);
}
