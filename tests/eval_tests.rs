//! End-to-end tests for the evaluator: lex → parse → eval a whole program.

use emily::env::Env;
use emily::eval::eval;
use emily::lexer::Lexer;
use emily::parser::Parser;
use emily::value::Value;

/// Lex, parse, and evaluate a whole program in a fresh environment,
/// returning the value of the last form.
fn run(src: &str) -> Value {
    let tokens = Lexer::new(src).tokenize().unwrap();
    let forms = Parser::new(tokens).parse().unwrap();
    let env = Env::new();
    let mut result = Value::Bool(false);
    for form in &forms {
        result = eval(form, env.clone()).unwrap();
    }
    result
}

#[test]
fn evals_def_then_symbol_lookup() {
    assert!(matches!(run("(def x 42) x"), Value::Int(42)));
}

#[test]
fn evals_let() {
    assert!(matches!(run("(let [x 10 y 5] x)"), Value::Int(10)));
    assert!(matches!(run("(let [x 10 y 5] y)"), Value::Int(5)));
}

#[test]
fn evals_if() {
    assert!(matches!(run("(if true 1 2)"), Value::Int(1)));
    assert!(matches!(run("(if false 1 2)"), Value::Int(2)));
}

#[test]
fn evals_record_and_vector() {
    assert!(matches!(run("{:a 1 :b 2}"), Value::Record(_)));
    assert!(matches!(run("[1 2 3]"), Value::List(_)));
}
