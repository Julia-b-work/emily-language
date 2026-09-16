# Emily — Developer Diary

My thoughts about building Emily. Written after each dev session, just for fun.

---

## 2026-09-15 — Starting the lexer

- I consider this the real start of the project, although I have already worked on other important aspects of the project (i.e grammar, tokens and general design decisions) the lexer is the first actual hard part.

- In this session I started to build the lexer, implementing important functions such as `peek`, `peek_next`, `advance`. They seem, and are, very basic, but important nonetheless.

- I am not very familiar with Rust, but I am getting it very quickly, at first glance it does seem complicated and unusual, but after using it for a while I did get the logic of it. My favorite features so far are the `match` expression and `enum`s, both very useful.

- The real challenge so far is the Rust syntax, it is very different from the languages I am familiar with, (not needing `return` is really something!), but again its just a question of practice, soon I will be familiar with it and question why I even struggled with it.

- Next session I plan on maybe finishing the lexer, it all depends on how long I will be able to work.
