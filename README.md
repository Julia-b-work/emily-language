# Emily

A total, statically typed Lisp for configuration and data. It compiles to JSON,
YAML, and TOML, and is designed to replace YAML/JSON where you need types,
reuse, and safety.

## Dedication

This project is dedicated to Emily — a wonderful girl who changed my life.
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

Configuration with types, reuse, and no surprises — functions and records
instead of copy-pasted YAML.

## Status

Work in progress. The pipeline is:

- [x] **Lexer** — source text → tokens (tested)
- [x] **Parser** — tokens → AST (tested)
- [ ] **Evaluator** — AST → values *(in progress: `Value` and `Env` are built)*
- [ ] **CLI + JSON output**

## Documentation

- [`docs/specs.md`](docs/specs.md) — the language specification
- [`docs/grammar.ebnf`](docs/grammar.ebnf) — lexical and syntactic grammar
- [`docs/ambiguities.md`](docs/ambiguities.md) — resolved syntax ambiguities
- [`docs/dev_diary.md`](docs/dev_diary.md) — developer diary

## Building

```bash
cargo build   # compile
cargo test    # run the tests
```

## Project structure

```
.
├── src/
│   ├── token.rs   # token types
│   ├── lexer.rs   # source text → tokens
│   ├── ast.rs     # the expression tree
│   ├── parser.rs  # tokens → AST
│   ├── value.rs   # runtime values
│   ├── env.rs     # scopes
│   ├── lib.rs     # module declarations
│   └── main.rs    # CLI entry point (coming)
├── tests/
│   ├── lexer_tests.rs
│   └── parser_tests.rs
└── docs/
    ├── specs.md
    ├── grammar.ebnf
    ├── ambiguities.md
    └── dev_diary.md
```

## Roadmap

- [x] Language spec, grammar, and ambiguity resolutions
- [x] Lexer
- [x] Parser
- [ ] Evaluator (tree-walking)
- [ ] CLI + JSON output

Beyond v1: static types and inference, hygienic macros, ADTs + pattern
matching, imports with integrity checking, and YAML/TOML output.
