# Emily

[![Version](https://img.shields.io/github/v/tag/Julia-b-work/emily-language)](https://github.com/Julia-b-work/emily-language/releases)
[![Tests](https://github.com/Julia-b-work/emily-language/actions/workflows/test.yml/badge.svg)](https://github.com/Julia-b-work/emily-language/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A Lisp for configuration and data, written in Rust. It compiles to JSON, YAML,
and TOML, and is designed to replace YAML/JSON where you need reuse and safety.

Static typing is in progress: annotations parse today, but the type checker is
still being built and doesn't enforce anything yet.

## Dedication

This project is dedicated to Emily, a wonderful girl who changed my life.
I hope this language is as safe as she makes me feel.

## What it looks like

```lisp
(def server {:host "localhost" :port 8080 :debug false})

(def prod (merge server {:host "prod.example.com"}))

{:name "my-app" :server prod}
```

```json
{
  "name": "my-app",
  "server": {
    "host": "prod.example.com",
    "port": 8080,
    "debug": false
  }
}
```

Configuration with functions, records, and reuse, instead of
copy-pasted YAML.

## Demo

![Emily compiling a config to JSON and YAML, then catching an error with line/column](emily.gif)

## Status

v1 is complete. The pipeline is:

- [x] **Lexer** -> source text → tokens (tested)
- [x] **Parser** -> tokens → AST (tested)
- [x] **Evaluator** -> AST → values (`def`/`let`/`if`/`fn`, closures, field access, builtins)
- [x] **CLI + JSON/YAML/TOML output** -> `cargo run -- examples/hello.em` (or `--format yaml` / `--format toml`)

Type annotations (`(def x : Int 42)`, `(fn (a : Int) ...)`) parse but are **not
checked yet**: the type checker exists in `src/typechecker.rs` but isn't wired
into the CLI, so today annotations are syntax only.

## Documentation

- [`docs/syntax.md`](docs/syntax.md): a quick tour of the syntax
- [`docs/specs.md`](docs/specs.md): the language specification
- [`docs/grammar.ebnf`](docs/grammar.ebnf): lexical and syntactic grammar
- [`docs/ambiguities.md`](docs/ambiguities.md): resolved syntax ambiguities
- [`docs/dev_diary.md`](docs/dev_diary.md): developer diary

## Building

```bash
cargo build   # compile
cargo test    # run the tests
```

## Project structure

```
.
├── CHANGELOG.md
├── src/
│   ├── token.rs   # token types
│   ├── lexer.rs   # source text → tokens
│   ├── ast.rs     # the expression tree
│   ├── parser.rs  # tokens → AST
│   ├── value.rs   # runtime values
│   ├── env.rs     # scopes
│   ├── eval.rs    # AST → values
│   ├── import.rs  # (import "file.em") expansion
│   ├── types.rs   # the Type enum (annotations)
│   ├── typechecker.rs # static type checker (in development, not enforced)
│   ├── lib.rs     # module declarations
│   └── main.rs    # CLI entry point
├── tests/
│   ├── import_tests.rs
│   ├── lexer_tests.rs
│   ├── parser_tests.rs
│   ├── eval_tests.rs
│   ├── acceptance_tests.rs
│   └── types_tests.rs
├── examples/
│   ├── hello.em
│   ├── env_config.em
│   ├── computed_urls.em
│   ├── database.em        # imported by config.em
│   ├── server.em          # imported by config.em
│   ├── config.em          # imports database.em + server.em
│   ├── typed.em           # type annotations (not checked yet)
│   └── error_example.em   # deliberately broken (shows error messages)
└── docs/
    ├── specs.md
    ├── grammar.ebnf
    ├── ambiguities.md
    ├── syntax.md
    └── dev_diary.md
```

## Roadmap

### v1 (complete)

- [x] Language spec, grammar, and ambiguity resolutions
- [x] Lexer
- [x] Parser
- [x] Evaluator (tree-walking)
- [x] CLI + JSON output

### v0.2.0 (in progress)

- [x] YAML and TOML output
- [x] Imports (path-based, with cycle detection)
- [ ] Static types and inference (annotations parse; checker written, not enforced)
- [ ] Hygienic macros
- [ ] ADTs + pattern matching
