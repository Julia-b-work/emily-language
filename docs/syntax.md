# Emily — Syntax Guide

A quick, friendly tour of Emily's syntax. For the full formal specification,
see [`specs.md`](specs.md).

## How a program works

A program is a list of *forms*. They run top to bottom, and the **last form's
value** becomes the output, printed as JSON.

## Comments

```lisp
;; a comment runs to the end of the line
```

## Literals

```lisp
42          ; a number
3.14        ; a float
"hello"     ; a string
true        ; a boolean
```

## Records

The core data structure — a mapping of keywords to values:

```lisp
{:host "localhost" :port 8080 :debug false}
```

```json
{"host": "localhost", "port": 8080, "debug": false}
```

## Lists

A list of *data* uses square brackets:

```lisp
[1 2 3]
["api" "worker"]
```

Parentheses are for *code* — calling a function or special form:

```lisp
(+ 1 2)      ; call the + function
(def x 10)   ; the def special form
```

## `def` — define a name

Binds a name for the rest of the program:

```lisp
(def server {:host "localhost" :port 8080})
server
```

## `let` — local bindings

Binds names only inside its body:

```lisp
(let [x 10 y 20] (+ x y))
```

## `if` — conditionals

```lisp
(if debug true false)
```

Only the taken branch is evaluated.

## `fn` — functions

```lisp
(def add (fn (a b) (+ a b)))
(add 1 2)    ; => 3
```

Functions are values — you can pass them around and store them.

## Field access — `.`

```lisp
(def api {:host "api.example.com" :port 443})
api.host     ; => "api.example.com"
```

## Built-in functions

| Form | What it does |
|---|---|
| `(+ a b ...)` | sum numbers |
| `(concat a b ...)` | join strings |
| `(show x)` | turn a value into a string |
| `(merge a b ...)` | combine records (later wins) |
| `(append list value)` | add a value to the end of a list |
| `(fold list initial (fn (acc x) ...))` | reduce a list to one value |

## Putting it together

```lisp
(def base {:host "localhost" :port 8080 :debug false})

(def prod (merge base {:host "prod.example.com"}))

(def services ["api" "worker" "scheduler"])
(def configs
  (fold services [] (fn (acc name)
    (append acc {:name name :port 8080}))))

{:prod prod :configs configs}
```

## Running a program

```bash
cargo run -- examples/hello.em
```

The last form's value is printed as JSON.

## Not here yet

Static types, macros, `import`, and YAML/TOML output are planned (see the
roadmap in the [README](../README.md)).
