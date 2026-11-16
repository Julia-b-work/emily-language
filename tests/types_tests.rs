//! Integration tests for the type syntax layer: turning annotation ASTs into
//! [`Type`] values. No checking happens here; this is parse-only.

use emily::ast::Expr;
use emily::lexer::Lexer;
use emily::parser::Parser;
use emily::types::Type;

/// Parses a single annotated form and returns the parsed type (or the error).
fn ann_type_result(src: &str) -> Result<Type, String> {
    let tokens = Lexer::new(src).tokenize().unwrap();
    let forms = Parser::new(tokens).parse().unwrap();
    // `(x : Int)` parses as a list holding one annotation.
    let Expr::List(items) = &forms[0] else {
        panic!("expected a list, got {:?}", forms[0]);
    };
    match items.as_slice() {
        [Expr::Ann { ty, .. }] => Type::from_expr(ty),
        other => panic!("expected a single annotation, got {other:?}"),
    }
}

/// Like [`ann_type_result`], panicking on a parse error.
fn ann_type(src: &str) -> Type {
    ann_type_result(src).unwrap()
}

#[test]
fn parses_primitive_types() {
    assert_eq!(ann_type("(x : Int)"), Type::Int);
    assert_eq!(ann_type("(x : Float)"), Type::Float);
    assert_eq!(ann_type("(x : String)"), Type::String);
    assert_eq!(ann_type("(x : Bool)"), Type::Bool);
}

#[test]
fn parses_named_types() {
    assert_eq!(ann_type("(x : Server)"), Type::Named("Server".to_string()));
}

#[test]
fn parses_list_types() {
    assert_eq!(
        ann_type("(x : (List Int))"),
        Type::List(Box::new(Type::Int))
    );
    assert_eq!(
        ann_type("(x : (List (List Bool)))"),
        Type::List(Box::new(Type::List(Box::new(Type::Bool))))
    );
}

#[test]
fn parses_fn_types() {
    assert_eq!(
        ann_type("(f : (Int -> Bool))"),
        Type::Fn {
            params: vec![Type::Int],
            ret: Box::new(Type::Bool),
        }
    );
    assert_eq!(
        ann_type("(f : (String -> Int -> Config))"),
        Type::Fn {
            params: vec![Type::String, Type::Int],
            ret: Box::new(Type::Named("Config".to_string())),
        }
    );
}

#[test]
fn parses_record_types() {
    assert_eq!(
        ann_type("(s : {:host String :port Int})"),
        Type::Record(vec![
            ("host".to_string(), Type::String),
            ("port".to_string(), Type::Int),
        ])
    );
}

#[test]
fn rejects_malformed_types() {
    // `(List Int Bool)` is neither a one-element List type nor a fn type.
    assert!(ann_type_result("(x : (List Int Bool))").is_err());
}
