//! Acceptance tests: the example programs from `docs/specs.md`, asserting they
//! produce the exact JSON output the spec promises.
//!
//! Examples 2 and 4 are adapted slightly: the spec includes type annotations
//! (`: Server`, `: (String -> Int -> Config)`), which are v2. These tests run
//! the same behavior without the annotation.

use emily::env::Env;
use emily::eval::eval;
use emily::lexer::Lexer;
use emily::parser::Parser;
use emily::value::Value;

/// Runs a whole program through lex → parse → eval, returning its JSON.
fn run_json(src: &str) -> String {
    let tokens = Lexer::new(src).tokenize().unwrap();
    let forms = Parser::new(tokens).parse().unwrap();
    let env = Env::new();
    let mut result = Value::Bool(false);
    for form in &forms {
        result = eval(form, env.clone()).unwrap();
    }
    result.to_json().unwrap()
}

#[test]
fn example_1_record() {
    assert_eq!(
        run_json("{:name \"my-app\" :port 8080 :debug false}"),
        "{\"name\": \"my-app\", \"port\": 8080, \"debug\": false}"
    );
}

#[test]
fn example_2_define_and_return_record() {
    assert_eq!(
        run_json("(def server {:host \"localhost\" :port 8080 :debug false}) server"),
        "{\"host\": \"localhost\", \"port\": 8080, \"debug\": false}"
    );
}

#[test]
fn example_3_field_access() {
    assert_eq!(
        run_json(
            "(def base {:host \"localhost\" :port 8080 :path \"/api\"}) {:url (concat \"http://\" base.host \":\" (show base.port) base.path) :timeout 30}"
        ),
        "{\"url\": \"http://localhost:8080/api\", \"timeout\": 30}"
    );
}

#[test]
fn example_4_function() {
    assert_eq!(
        run_json(
            "(def make-config (fn (name port) {:name name :port port :url (concat \"http://\" name \":\" (show port))})) (make-config \"api\" 8080)"
        ),
        "{\"name\": \"api\", \"port\": 8080, \"url\": \"http://api:8080\"}"
    );
}

#[test]
fn example_5_fold() {
    assert_eq!(
        run_json(
            "(def services [\"api\" \"worker\" \"scheduler\"]) (def instances (fold services [] (fn (acc name) (append acc {:name name :port 8080 :replicas 3})))) instances"
        ),
        "[{\"name\": \"api\", \"port\": 8080, \"replicas\": 3}, {\"name\": \"worker\", \"port\": 8080, \"replicas\": 3}, {\"name\": \"scheduler\", \"port\": 8080, \"replicas\": 3}]"
    );
}
