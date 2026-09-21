//! Emily's entry point: run a `.em` file through the whole pipeline.
//!
//! Usage: `cargo run -- examples/hello.em`

use std::env;
use std::fs;
use std::process::exit;

use emily::env::Env;
use emily::eval::eval;
use emily::lexer::Lexer;
use emily::parser::Parser;
use emily::value::Value;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run -- <file.em>");
        exit(1);
    });

    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not read {path}: {e}");
            exit(1);
        }
    };

    let tokens = match Lexer::new(&source).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("lex error: {e}");
            exit(1);
        }
    };

    let forms = match Parser::new(tokens).parse() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("parse error: {e}");
            exit(1);
        }
    };

    let env = Env::new();
    let mut result = Value::Bool(false);
    for form in &forms {
        match eval(form, env.clone()) {
            Ok(v) => result = v,
            Err(e) => {
                eprintln!("eval error: {e}");
                exit(1);
            }
        }
    }

    match result.to_json() {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("serialization error: {e}");
            exit(1);
        }
    }
}
