# Changelog

All notable changes to this project are recorded here. Versions follow
[Semantic Versioning](https://semver.org/): MAJOR.MINOR.PATCH.

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
