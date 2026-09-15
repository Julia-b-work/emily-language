//! The lexer: turns source text into a stream of tokens.
//!
//! The lexer walks the input one character at a time and produces a flat list
//! of [`Token`]s, resolving the ambiguities documented in `docs/ambiguities.md`
//! (e.g. `:` as keyword vs annotation, `-` as symbol vs negative number).

use crate::token::Token;

/// A cursor over the source text.
///
/// Holds the input as a list of characters plus a position, and exposes
/// `peek`/`advance` primitives that the scanning methods build on.
pub struct Lexer {
    /// The source, split into individual characters.
    chars: Vec<char>,
    /// The current cursor position (an index into `chars`).
    pos: usize,
}

impl Lexer {
    /// Builds a lexer over the given source text.
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    /// Returns the character at the cursor without moving, or `None` at end of input.
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// Returns the character at the cursor and advances by one.
    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    /// Returns the character just after the cursor without moving (used for lookahead).
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    /// Skips whitespace and `;` line comments, leaving the cursor on the next real token.
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                    self.advance();
                }
                Some(';') => {
                    // A comment runs to end of line: consume up to (not including) the newline.
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    /// Reads a string literal, returning a `String` token with escapes translated.
    fn lex_string(&mut self) -> Result<Token, String> {
        self.advance(); // consume the opening quote
        let mut out = String::new();
        while let Some(c) = self.advance() {
            match c {
                '"' => return Ok(Token::String(out)),
                '\\' => {
                    let esc = self.advance().ok_or("unterminated string")?;
                    match esc {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        other => return Err(format!("unknown escape: \\{other}")),
                    }
                }
                other => out.push(other),
            }
        }
        Err("unterminated string".to_string())
    }
}

/// Whether `c` is a valid first character of a symbol.
///
/// True for letters and the punctuation characters listed in the lexical
/// grammar (`_ + - * / ! ? < > =`).
fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || matches!(c, '_' | '+' | '-' | '*' | '/' | '!' | '?' | '<' | '>' | '=')
}
