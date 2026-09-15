# Emily — Ambiguities & Resolutions

Every syntax has ambiguities — places where the same text could be read two
ways. These are the ones Emily hits, and the decisions made. Each is a rule the
lexer/parser must follow exactly.

---

## 1. `:` — keyword prefix vs. type-annotation marker

**The problem.** `:` does two jobs:
- `:port` is a *keyword* (a record key).
- `(def x : Int 42)` uses `:` to introduce a *type annotation*.

**Resolution.** The presence or absence of a following space decides:

| Text | Token |
|---|---|
| `:port` (`:` immediately followed by a symbol character) | `Keyword` |
| `: Int` (`:` followed by whitespace) | `Colon` (annotation marker) |

So the lexer checks the character *after* `:`. If it's a symbol-start character,
it lexes a keyword; otherwise it lexes a standalone `Colon` token.

---

## 2. `-` — symbol vs. negative number

**The problem.** `(- 1 2)` uses `-` as the subtraction *symbol*, but `-1` is a
*number*.

**Resolution.** Lex `number` before `symbol`. When the lexer sees `-`:
- if the very next character is a digit, the `-` is the number's sign → `Int(-1)`;
- otherwise, `-` is the symbol → `Symbol("-")`.

This is safe because a bare `-` (followed by space, `)`, `}`, etc.) is never a
valid number anyway.

---

## 3. `.` — symbol character vs. float decimal point

**The problem.** `foo.bar` is a single *symbol* (field access), but `1.5` is a
*float*.

**Resolution.** Lex `float` before `int` and `symbol`. Digits form numbers
greedily: the lexer sees `1`, keeps consuming `1.5`, and only stops at a
non-numeric character. A `.` is treated as part of a number only when it follows
a digit; anywhere else it's a symbol character.

`number = float | int` (float first) is what enforces this.

---

## 4. Comments — `;` vs `#`

**Resolution.** `;` to end of line (Lisp convention). `#` is not special.

```
; this is a comment
(def x 42)  ; so is this
```

---

## 5. List separator — whitespace vs. comma

**Resolution.** Whitespace-separated. `[1 2 3]`, not `[1, 2, 3]`. The comma `,`
is reserved for `unquote` (`'`, `,`, `,@` reader macros).

---

## 6. Records — dedicated AST node vs. list expansion

**Resolution.** Dedicated AST nodes for `record` and `vector`. They are
first-class data, not function calls. This gives better error messages and
faster evaluation than expanding `{:a 1}` into `(record :a 1)`.

---

## Summary table

| # | Input | Reads as | Because |
|---|---|---|---|
| 1 | `:port` | `Keyword` | `:` + no space |
| 1 | `: Int` | `Colon` | `:` + space |
| 2 | `-1` | `Int(-1)` | `-` + digit, `number` lexed first |
| 2 | `-` | `Symbol` | `-` not followed by digit |
| 3 | `1.5` | `Float` | `float` lexed before `int` |
| 3 | `foo.bar` | `Symbol` | `.` after a letter, not a digit |
| 4 | `; ...` | comment | `;` starts a comment |
| 5 | `[1 2 3]` | vector | whitespace-separated |
| 6 | `{:a 1}` | `Record` node | dedicated AST node |
