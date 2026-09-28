use std::fs;
use std::path::PathBuf;

use emily::ast::Expr;
use emily::import::expand_imports;
use emily::lexer::Lexer;
use emily::parser::Parser;

fn parse(src: &str) -> Vec<Expr> {
    let tokens = Lexer::new(src).tokenize().unwrap();
    Parser::new(tokens).parse().unwrap()
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("emily_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn imports_splice_file_forms_in_place() {
    let dir = temp_dir("import_basic");
    fs::write(
        dir.join("try_me.em"),
        "(def test {:host \"localhost\" :port 1337})",
    )
    .unwrap();

    let main = parse("(import \"try_me.em\") {:name \"an-app\" :test test}");
    let expanded = expand_imports(main, &dir).unwrap();

    assert_eq!(expanded.len(), 2);
    assert_eq!(
        expanded[0],
        parse("(def test {:host \"localhost\" :port 1337})")[0]
    );
    assert_eq!(expanded[1], parse("{:name \"an-app\" :test test}")[0]);
}

#[test]
fn imports_resolve_relative_to_the_importing_file() {
    let dir = temp_dir("import_relative");
    fs::create_dir_all(dir.join("config")).unwrap();
    fs::write(
        dir.join("config").join("base.em"),
        "(def host \"localhost\")",
    )
    .unwrap();

    let main = parse("(import \"base.em\") {:host host}");
    let expanded = expand_imports(main, &dir.join("config")).unwrap();

    assert_eq!(expanded[0], parse("(def host \"localhost\")")[0]);
}

#[test]
fn errors_on_circular_import() {
    let dir = temp_dir("import_cycle");
    fs::write(dir.join("a.em"), "(import \"b.em\")").unwrap();
    fs::write(dir.join("b.em"), "(import \"a.em\")").unwrap();

    let forms = parse("(import \"a.em\")");
    assert!(expand_imports(forms, &dir).is_err());
}

#[test]
fn errors_on_missing_file() {
    let dir = temp_dir("import_missing");
    let forms = parse("(import \"nope.em\")");
    assert!(expand_imports(forms, &dir).is_err());
}

#[test]
fn errors_on_malformed_import() {
    let dir = temp_dir("import_malformed");
    let forms = parse("(import)");
    assert!(expand_imports(forms, &dir).is_err());
    let forms = parse("(import 42)");
    assert!(expand_imports(forms, &dir).is_err());
}
