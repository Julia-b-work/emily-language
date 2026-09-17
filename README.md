# Emily

A total, statically typed Lisp for configuration and data. It compiles to JSON,
YAML, and TOML, and is designed to replace YAML/JSON where you need types,
reuse, and safety.

## Dedication

This project is dedicated to Emily — a wonderful girl who changed my life.
I hope this language is as safe as she makes me feel.

**Status: work in progress.** The lexer and parser are complete (with tests);
the evaluator and CLI are not built yet.

## Documentation

- [`docs/specs.md`](docs/specs.md) — the language specification
- [`docs/grammar.ebnf`](docs/grammar.ebnf) — lexical and syntactic grammar
- [`docs/ambiguities.md`](docs/ambiguities.md) — resolved syntax ambiguities

## Building

```bash
cargo build
```

## Roadmap

- [x] Language spec, grammar, and ambiguity resolutions
- [x] Lexer
- [x] Parser
- [ ] Evaluator (tree-walking)
- [ ] CLI + JSON output
