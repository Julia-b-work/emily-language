use emily::ast::Expr;
use emily::lexer::Lexer;
use emily::parser::Parser;

fn parse(src: &str) -> Vec<Expr> {
    let tokens = Lexer::new(src).tokenize().unwrap();
    Parser::new(tokens).parse().unwrap()
}

fn parse_result(src: &str) -> Result<Vec<Expr>, String> {
    let tokens = Lexer::new(src).tokenize().unwrap();
    Parser::new(tokens).parse()
}

#[test]
fn parses_int() {
    assert_eq!(parse("42"), vec![Expr::Int(42)]);
}

#[test]
fn parses_float() {
    assert_eq!(parse("3.14"), vec![Expr::Float(3.14)]);
}

#[test]
fn parses_string() {
    assert_eq!(parse("\"hi\""), vec![Expr::String("hi".to_string())]);
}

#[test]
fn parses_bool() {
    assert_eq!(parse("true"), vec![Expr::Bool(true)]);
    assert_eq!(parse("false"), vec![Expr::Bool(false)]);
}

#[test]
fn parses_symbol() {
    assert_eq!(parse("foo"), vec![Expr::Symbol("foo".to_string())]);
}

#[test]
fn parses_keyword() {
    assert_eq!(parse(":port"), vec![Expr::Keyword("port".to_string())]);
}

#[test]
fn parses_list() {
    assert_eq!(
        parse("(+ 1 2)"),
        vec![Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Int(1),
            Expr::Int(2),
        ])]
    );
}

#[test]
fn parses_vector() {
    assert_eq!(
        parse("[1 2 3]"),
        vec![Expr::Vector(
            vec![Expr::Int(1), Expr::Int(2), Expr::Int(3),]
        )]
    );
}

#[test]
fn parses_record() {
    assert_eq!(
        parse("{:a 1 :b 2}"),
        vec![Expr::Record(vec![
            ("a".to_string(), Expr::Int(1)),
            ("b".to_string(), Expr::Int(2)),
        ])]
    );
}

#[test]
fn parses_quote() {
    assert_eq!(
        parse("'x"),
        vec![Expr::Quote(Box::new(Expr::Symbol("x".to_string())))]
    );
}

#[test]
fn parses_nested_forms() {
    assert_eq!(
        parse("(+ 1 (* 2 3))"),
        vec![Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Int(1),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Int(2),
                Expr::Int(3),
            ]),
        ])]
    );
}

#[test]
fn parses_multiple_top_level_forms() {
    assert_eq!(
        parse("1 2 3"),
        vec![Expr::Int(1), Expr::Int(2), Expr::Int(3)]
    );
}

#[test]
fn errors_on_unterminated_list() {
    assert!(parse_result("(1").is_err());
}

#[test]
fn errors_on_unterminated_record() {
    assert!(parse_result("{:a 1").is_err());
}

#[test]
fn errors_on_unexpected_token() {
    assert!(parse_result(")").is_err());
}
