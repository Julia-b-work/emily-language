//! Emily's entry point: run a `.em` file through the whole pipeline.
//!
//! Usage:
//!   cargo run -- <file.em>                     # JSON (default)
//!   cargo run -- --format yaml <file.em>       # YAML
//!   cargo run -- -f toml <file.em>             # TOML

use std::env;
use std::fs;
use std::process::exit;

use emily::env::Env;
use emily::eval::eval;
use emily::lexer::Lexer;
use emily::parser::Parser;
use emily::value::Value;

fn main() {
    // Parse the command line: an optional `--format`/`-f` flag plus the
    // `.em` file to run.
    let mut args = env::args().skip(1);
    let mut format = "json".to_string();
    let mut path: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" | "-f" => match args.next() {
                Some(f) => format = f,
                None => {
                    eprintln!("usage: emily [--format json|yaml|toml] <file.em>");
                    exit(1);
                }
            },
            _ => path = Some(arg),
        }
    }

    let path = match path {
        Some(p) => p,
        None => {
            eprintln!("usage: emily [--format json|yaml|toml] <file.em>");
            exit(1);
        }
    };

    // Read the source file.
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not read {path}: {e}");
            exit(1);
        }
    };

    // Lex: source text -> tokens.
    let tokens = match Lexer::new(&source).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("lex error: {e}");
            exit(1);
        }
    };

    // Parse: tokens -> AST.
    let forms = match Parser::new(tokens).parse() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("parse error: {e}");
            exit(1);
        }
    };

    // Evaluate every top-level form in order; the last one's value is the
    // program's result.
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

    // Serialize the result in the requested format and print it.
    let output = match format.as_str() {
        "json" => result.to_json(),
        "yaml" => result.to_yaml(),
        "toml" => result.to_toml(),
        other => {
            eprintln!("unknown format: {other} (expected json, yaml, or toml)");
            exit(1);
        }
    };

    match output {
        Ok(s) => println!("{s}"),
        Err(e) => {
            eprintln!("serialization error: {e}");
            exit(1);
        }
    }
}
