# Emily — Language Specification

> **Emily** is a total, statically typed Lisp for configuration and data. It
> compiles to JSON, YAML, and TOML. It has hygienic macros, full type
> inference, and first-class imports with integrity checking. It is designed to
> replace YAML and JSON where you need types, reuse, and safety.

## Goals

- **Total** — every program terminates; no exceptions, no partial functions.
- **Statically typed** — types checked before evaluation; no null; no implicit
  conversions.
- **Lisp** — s-expressions, code-as-data, hygienic macros, a small core.
- **For configuration and data** — declarative, pure, deterministic.
- **Interoperable** — emits JSON, YAML, and TOML.
- **Safe** — memory safe, type safe, error values instead of surprises.

## Non-goals

- Not a general-purpose language (no I/O, concurrency, or system calls).
- Not a build system.
- Not object-oriented (no classes or inheritance).
- Not a research language (use proven techniques only).
- Not backward-compatible with any existing Lisp.
- Not a large standard library.
- Niche by design: config and data only.

---

## Syntax

Emily is a Lisp: everything is a form, and code is data.

```lisp
;; comment
42                              ; number
"hello"                         ; string
true false                      ; booleans
name                            ; symbol
:port                           ; keyword (a record key)
(+ 1 2)                         ; list (application or special form)
{:host "localhost" :port 8080}  ; record literal
["api" "worker"]                ; list literal
```

Reader sugar (expands at parse time):

| Source | Expands to |
|---|---|
| `'x` | `(quote x)` |
| `` `x `` | `(quasiquote x)` |
| `,x` | `(unquote x)` |
| `,@x` | `(unquote-splicing x)` |

---

## Core forms

Special forms are the constructs the compiler understands directly. Everything
else is a function or a macro. Emily's core is **five** forms.

### `def` — top-level binding

```lisp
(def name expr)            ; bind a value
(def name : Type expr)     ; bind with an explicit type check
```

```lisp
(def port 8080)
(def host : String "localhost")
```

Binds a name at the top level of a module. The `: Type` annotation is optional;
when present the value is checked against it.

### `fn` — anonymous function

```lisp
(fn (args) body)
```

```lisp
(def add (fn (a b) (+ a b)))
(add 1 2)                          ; => 3
```

Creates a closure. Parameters bind names; the body evaluates with them in scope.

### `let` — local binding

```lisp
(let [name value ...] body)
```

```lisp
(let [x 10
      y (+ x 5)]
  (+ x y))                         ; => 25
```

Binds names in a local scope. Bindings are **sequential** — later bindings can
see earlier ones.

### `if` — conditional

```lisp
(if cond then else)
```

```lisp
(if debug true false)
```

`cond` must be a `Bool`. Both branches must have the same type. Only one branch
is evaluated.

### `import` — load a file or environment variable

```lisp
(import "path")
(import "path" :sha256 hash)
(import "path" :type Type)
(import "env:VAR" :type Type)
```

Loads and evaluates another file (or reads an env var), returning its value.
`:sha256` pins the import to a content hash; if the file changes the import
fails. `:type` checks the imported value against a type.

---

## Types

- **Records** — `{:host String :port Int}`
- **Unions** — `(Union :dev :staging :prod)`
- **Functions** — `(String -> Int -> Config)`
- **Algebraic data types** — data with several parts, e.g.
  `point = x and y`, `bool = true or false`
- **Pattern matching** — inspect an ADT and pull its parts apart, with
  exhaustiveness checking.
- **No null** — absence is `Optional` or a union.
- **Inference** — annotations are optional; types are inferred where possible.

The output boundary is strict: functions and types never appear in output. Only
pure data — numbers, strings, booleans, lists, records — is serialized.

---

## Example programs

These are the acceptance tests: when all of them run and produce the right
output, the language works.

### 1. Hello, record

```lisp
{:name "my-app" :port 8080 :debug false}
```

```json
{"name": "my-app", "port": 8080, "debug": false}
```

### 2. Typed record

```lisp
(type Server {:host String :port Int :debug Bool})

(def server : Server
  {:host "localhost" :port 8080 :debug false})

server
```

### 3. Field access

```lisp
(def base {:host "localhost" :port 8080 :path "/api"})

{:url (concat "http://" base.host ":" (show base.port) base.path)
 :timeout 30}
```

```json
{"url": "http://localhost:8080/api", "timeout": 30}
```

### 4. Functions

```lisp
(def make-config : (String -> Int -> Config)
  (fn (name port)
    {:name name :port port :url (concat "http://" name ":" (show port))}))

(make-config "api" 8080)
```

### 5. Lists and folds

```lisp
(def services ["api" "worker" "scheduler"])

(def instances
  (fold services [] (fn (acc name)
    (append acc {:name name :port 8080 :replicas 3}))))

instances
```

```json
[
  {"name": "api", "port": 8080, "replicas": 3},
  {"name": "worker", "port": 8080, "replicas": 3},
  {"name": "scheduler", "port": 8080, "replicas": 3}
]
```

### 6. Multi-file config

**`defaults.em`**

```lisp
(def defaults
  {:host "localhost" :port 8080 :debug false})
```

**`prod.em`**

```lisp
(import "./defaults.em")

(def prod
  (merge defaults
    {:host "prod.example.com" :debug false}))

prod
```

---

## Output formats

Emily evaluates to a value and serializes it to one of:

- **JSON** — `emily eval config.em --to json`
- **YAML** — `emily eval config.em --to yaml`
- **TOML** — `emily eval config.em --to toml`

Output is deterministic: same input, same bytes.

---

## Open decisions

To be resolved in `docs/grammar.ebnf` and `docs/ambiguities.md`:

- `:` as keyword prefix vs. type-annotation marker
- `-` as symbol vs. negative number
- `.` as symbol character vs. float decimal point

---

*Emily must be a safe language.*
