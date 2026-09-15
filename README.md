# Emily

A total, statically typed Lisp for configuration and data. It compiles to JSON,
YAML, and TOML, and is designed to replace YAML/JSON where you need types,
reuse, and safety.

**Status: work in progress.** The lexer is partially implemented; the parser,
evaluator, and CLI are not built yet.

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
- [ ] Lexer (in progress)
- [ ] Parser
- [ ] Evaluator (tree-walking)
- [ ] CLI + JSON output
