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
