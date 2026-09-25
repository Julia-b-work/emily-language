# Emily — Developer Diary

My thoughts about building Emily. Written after each dev session, just for fun.

---

## 2026-09-15 — Starting the lexer

- I consider this the real start of the project, although I have already worked on other important aspects of the project (i.e grammar, tokens and general design decisions) the lexer is the first actual hard part.

- In this session I started to build the lexer, implementing important functions such as `peek`, `peek_next`, `advance`. They seem, and are, very basic, but important nonetheless.

- I am not very familiar with Rust, but I am getting it very quickly, at first glance it does seem complicated and unusual, but after using it for a while I did get the logic of it. My favorite features so far are the `match` expression and `enum`s, both very useful.

- The real challenge so far is the Rust syntax, it is very different from the languages I am familiar with, (not needing `return` is really something!), but again its just a question of practice, soon I will be familiar with it and question why I even struggled with it.

- Next session I plan on maybe finishing the lexer, it all depends on how long I will be able to work.

---

## 2026-09-16 — Finishing the lexer

- Finished the lexer today, writing `lex_number`, `lex_symbol`, `lex_keyword` and the `tokenize()` function that ties everything together.

- The most interesting part was the ambiguities — `:` can be a keyword or a type annotation, `-` can be a symbol or a negative number. The lexer decides by looking one character ahead. It's cool that a single `peek` call resolves a whole design problem we documented weeks ago.

- 13 tests now, all passing. Writing one test for each ambiguity felt like locking the behavior in place.

- Rust keeps catching me on case sensitivity: `Option`, `Some`, `String` are all capitalized. I wrote them lowercase a lot. The compiler is patient, at least.

- I feel that I'm slowly starting to love Rust! I definitely want to explore the language more in other projects.

- Next: the parser, hope I can finish it tomorrow.

---

## 2026-09-17 — The parser

- Built the parser, which turns the flat token list into a tree (the AST). It's a recursive-descent parser, but for a Lisp it's almost trivial — the parentheses do all the structuring for you.

- The Expr enum is like Token but recursive: a List contains other Exprs. This is where I learned about Box — a recursive type needs a pointer, otherwise it'd be infinitely large.

- 15 parser tests, 28 total. The pipeline — source text → tokens → tree — works end to end.

- I feel like I'm spending most of my time in this project, it's mostly fine but I don't want to get lost in something as big as a programming language. 

- Next: the evaluator, where the language finally starts doing something!!!

---

## 2026-09-19 — The evaluator: values and scopes

- Started the evaluator today, building the two data structures it needs: `Value` (what expressions become) and `Env` (scopes).

- This is where I finally met `Rc` and `RefCell`. `Rc` lets many closures share one environment; `RefCell` lets them mutate it through the sharing. Together, `Rc<RefCell<Env>>` is "a shared, mutable bag of variables" — exactly what a scope is.

- The `Env` is a `HashMap` of name → value plus a `parent` link. Looking up a name walks up that chain — that's lexical scoping. It's what makes `let` not leak its variables.

- Honestly the hardest concept in the whole project so far. The borrow checker is strict, not impossible.

- I saw Emily and was reminded what everything is about.

- Next: the `eval` function itself, where the language actually runs.

---

## 2026-09-20 — The evaluator: it runs!

- Wrote `eval.rs` — the tree-walker. Atoms evaluate to themselves, symbols get looked up, and lists dispatch to special forms.

- Implemented `def`, `let`, and `if`. `let` makes a child scope; `if` runs exactly one branch.

- I'm starting to get familiar with the Rust syntax.

- 32 tests now, including the first end-to-end eval tests. Emily *runs* programs — `(def x 42) x` gives `42`. That's the milestone where it stops reading code and starts executing it.

- Next: functions and closures, then the builtins. I'm excited!

## 2026-09-21 — v1 is done!!!

- Finished the evaluator today: `fn` and closures, plus the builtins (`+`, `concat`, `show`, `fold`, `append`, `merge`) and field access.

- Closures are complicated but very interesting.

- Tests are very important, caught bugs like: `fn` parameters use `()` but I'd written `Vector` in the code. The test failed, I fixed it.

- Emily compiles to JSON now. The acceptance tests run the spec's own examples and check the exact JSON output. 40 tests, all green.

- Wrote a syntax guide (`docs/syntax.md`) — a friendly tour of the language, separate from the formal spec.

- Released v0.1.0. Emily is a real, working language now — lexer, parser, evaluator, JSON output, all done and tested. I built a programming language.

- Emily would be proud!!!

- Next: v2 — static types, macros, ADTs. But not right now :).

---

## 2026-09-25 — YAML and TOML output

- Emily now compiles to three formats: JSON, YAML, and TOML!

- YAML was the interesting one. Block style needs indentation, and getting nested records and lists right took some thinking. The trick I liked: the serializer returns a list of *lines* with no parent-relative indent, and the parent indents a nested block by prefixing its lines with two spaces. No messy indent counter to keep in sync.

- TOML is stricter than I thought — a document has to be a table at the top level, so a top-level list or number is just an error. Nested records turn into inline tables like `{ host = "prod.example.com" }`, which seems fine for config.

- Added a `--format` flag (`-f` short). Now it's `emily --format yaml file.em`. It feels like a real command-line tool (yay).

- Typos keep getting me! Rust's case sensitivity does not forgive, but the compiler is nice and patient.

- Up next: imports! Spreading config across files is what would make Emily actually usable in the real world.
