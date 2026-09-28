# Changelog

All notable changes to this project are recorded here. Versions follow
[Semantic Versioning](https://semver.org/): MAJOR.MINOR.PATCH.

## [0.1.3] - 2026-09-28

### Added
- **Imports.** `(import "path.em")` splices another file's forms into the
  program, resolving paths relative to the importing file and catching circular
  imports. Runs as a pre-evaluation pass, so `eval` is untouched.
- **Error messages with positions.** Lexer and parser errors now report the
  offending line and column; tokens carry their source position.
- **Example programs.** `config.em` (imports `database.em` and `server.em`) and
  `error_example.em` (deliberately broken, to show error messages).

### Changed
- `Token` is now a struct (`kind`, `line`, `col`) instead of a bare enum.
- README: imports checked off the roadmap; project structure updated.

## [0.2.1] - 2026-09-25

### Added
- **YAML output.** `--format yaml` emits block-style YAML with correct nesting
  for nested records and lists.
- **TOML output.** `--format toml` emits TOML; the top level must be a record
  (TOML documents are tables), and nested records become inline tables.
- **`--format` flag.** `emily [--format json|yaml|toml] <file.em>` selects the
  output format (`json` is the default; `-f` is a short form).

### Changed
- README roadmap restructured into a v1 (complete) and v2 section.

## [0.1.1] - 2026-09-22

### Fixed
- **Environment reference cycle.** `Env::parent` is now a `Weak` reference
  instead of `Rc`, so a closure that captures its own enclosing scope no longer
  leaks the environment for the life of the process.

### Added
- Example programs: `env_config.em` (environments via `merge`) and
  `computed_urls.em` (endpoint URLs via functions and field access).
- A richer `hello.em`.

## [0.1.0] - 2026-09-21

Initial release: a working pipeline from source text to JSON.

### Added
- **Lexer** — source text → tokens, resolving the documented ambiguities
  (`:` keyword vs annotation, `-` symbol vs negative number, `.` decimal vs
  field access).
- **Parser** — tokens → AST (recursive descent).
- **Evaluator** — tree-walking, with `def`, `let`, `if`, `fn`, closures,
  field access, and builtins (`+`, `concat`, `show`, `fold`, `append`).
- **JSON output** — `cargo run -- <file.em>` evaluates a program and prints
  JSON.
- Test suite: lexer, parser, evaluator, and acceptance tests asserting the
  spec's example programs produce the exact JSON.

### Deferred to v2
- Static types and inference, hygienic macros, ADTs + pattern matching,
  imports with integrity checking, and YAML/TOML output.
