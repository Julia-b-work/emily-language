//! Imports: `(import "path.em")` — splice another file's forms into the program.
//!
//! Imports are a *pre-evaluation* pass: `expand_imports` runs after parsing and
//! before `eval`, replacing each top-level `(import "path")` with the imported
//! file's own (recursively expanded) forms. This keeps `eval` unchanged.
//!
//! Paths resolve relative to the importing file's directory, and a stack of
//! in-progress files catches circular imports.

use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::Expr;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Expands every top-level `(import "path")` in `forms`, resolving paths
/// relative to `base_dir` and splicing the imported file's forms in place.
pub fn expand_imports(forms: Vec<Expr>, base_dir: &Path) -> Result<Vec<Expr>, String> {
    let mut stack: Vec<PathBuf> = Vec::new();
    expand(forms, base_dir, &mut stack)
}

/// Walks a list of forms, replacing each import with the file's expanded forms.
fn expand(
    forms: Vec<Expr>,
    base_dir: &Path,
    stack: &mut Vec<PathBuf>,
) -> Result<Vec<Expr>, String> {
    let mut out = Vec::new();
    for form in forms {
        if let Some(path) = as_import(&form)? {
            out.extend(load_import(&path, base_dir, stack)?);
        } else {
            out.push(form);
        }
    }
    Ok(out)
}

/// If `form` is an `(import ...)` list, returns the path string; otherwise `None`.
///
/// A list whose head is the symbol `import` is treated as an import. A
/// malformed import (wrong arity or a non-string path) is an error rather than
/// silently falling through to "unknown function".
fn as_import(form: &Expr) -> Result<Option<String>, String> {
    let Expr::List(items) = form else {
        return Ok(None);
    };
    let Some(Expr::Symbol(head)) = items.first() else {
        return Ok(None);
    };
    if head != "import" {
        return Ok(None);
    }
    if items.len() != 2 {
        return Err("import expects exactly one argument: (import \"path\")".to_string());
    }
    match &items[1] {
        Expr::String(path) => Ok(Some(path.clone())),
        _ => Err("import expects a string path: (import \"path\")".to_string()),
    }
}

/// Loads the file at `path` (relative to `base_dir`), lexes, parses, and
/// recursively expands its own imports, returning the spliced forms.
fn load_import(path: &str, base_dir: &Path, stack: &mut Vec<PathBuf>) -> Result<Vec<Expr>, String> {
    let resolved = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        base_dir.join(path)
    };

    let canonical = fs::canonicalize(&resolved).map_err(|_| format!("import not found: {path}"))?;

    if stack.contains(&canonical) {
        return Err(format!("circular import: {path}"));
    }

    let source = fs::read_to_string(&canonical)
        .map_err(|e| format!("could not read {}: {e}", canonical.display()))?;
    let tokens = Lexer::new(&source).tokenize()?;
    let forms = Parser::new(tokens).parse()?;

    let sub_dir = canonical.parent().unwrap_or(base_dir).to_path_buf();

    stack.push(canonical);
    let expanded = expand(forms, &sub_dir, stack)?;
    stack.pop();
    Ok(expanded)
}
