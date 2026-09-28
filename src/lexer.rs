//! The lexer: turns source text into a stream of tokens.
//!
//! The lexer walks the input one character at a time and produces a flat list
//! of [`Token`]s, resolving the ambiguities documented in `docs/ambiguities.md`
//! (e.g. `:` as keyword vs annotation, `-` as symbol vs negative number).
//!
//! Each token records the line/column where it starts, and lexing errors carry
//! a position too.

use crate::token::{Token, TokenKind};

/// A cursor over the source text.
///
/// Holds the input as a list of characters plus a position, and exposes
/// `peek`/`advance` primitives that the scanning methods build on. It also
/// tracks the current line and column (both 1-based) for error reporting.
pub struct Lexer {
    /// The source, split into individual characters.
    chars: Vec<char>,
    /// The current cursor position (an index into `chars`).
    pos: usize,
    /// The current 1-based line number.
    line: usize,
    /// The current 1-based column number.
    col: usize,
}

impl Lexer {
    /// Builds a lexer over the given source text.
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Returns the character at the cursor without moving, or `None` at end of input.
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// Returns the character at the cursor, advancing the cursor and updating
    /// the line/column tracking.
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    /// Returns the character just after the cursor without moving (used for lookahead).
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    /// Formats an error message with the current line and column.
    fn error(&self, msg: &str) -> String {
        format!("{msg} (line {}, column {})", self.line, self.col)
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
    fn lex_string(&mut self) -> Result<TokenKind, String> {
        self.advance(); // consume the opening quote
        let mut out = String::new();
        while let Some(c) = self.advance() {
            match c {
                '"' => return Ok(TokenKind::String(out)),
                '\\' => {
                    let esc = self
                        .advance()
                        .ok_or_else(|| self.error("unterminated string"))?;
                    match esc {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        other => return Err(self.error(&format!("unknown escape: \\{other}"))),
                    }
                }
                other => out.push(other),
            }
        }
        Err(self.error("unterminated string"))
    }

    /// Reads a number, returning an `Int` or `Float` token.
    fn lex_number(&mut self) -> Result<TokenKind, String> {
        let mut text = String::new();

        if self.peek() == Some('-') {
            text.push('-');
            self.advance();
        }

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let next_is_digit = match self.peek_next() {
            Some(c) => c.is_ascii_digit(),
            None => false,
        };
        if self.peek() == Some('.') && next_is_digit {
            text.push('.');
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    text.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            let value: f64 = text
                .parse()
                .map_err(|_| self.error(&format!("invalid float: {text}")))?;
            return Ok(TokenKind::Float(value));
        }

        let value: i64 = text
            .parse()
            .map_err(|_| self.error(&format!("invalid int: {text}")))?;
        Ok(TokenKind::Int(value))
    }

    /// Reads a run of symbol characters, returning them as a `String`.
    ///
    /// A symbol character is a symbol-start, a digit, or a `.` (field access).
    fn read_symbol_chars(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if is_symbol_start(c) || c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    /// Reads a symbol, mapping the spellings `true`/`false` to `Bool` tokens.
    fn lex_symbol(&mut self) -> TokenKind {
        let s = self.read_symbol_chars();
        match s.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            _ => TokenKind::Symbol(s),
        }
    }

    /// Reads a keyword (`:name`). Called only after the `:` is confirmed.
    fn lex_keyword(&mut self) -> TokenKind {
        self.advance(); // skip the ':'
        let s = self.read_symbol_chars();
        TokenKind::Keyword(s)
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            let (line, col) = (self.line, self.col); // start of the next token
            let kind = match self.peek() {
                None => break,
                Some('(') => {
                    self.advance();
                    TokenKind::LParen
                }
                Some(')') => {
                    self.advance();
                    TokenKind::RParen
                }
                Some('{') => {
                    self.advance();
                    TokenKind::LBrace
                }
                Some('}') => {
                    self.advance();
                    TokenKind::RBrace
                }
                Some('[') => {
                    self.advance();
                    TokenKind::LBracket
                }
                Some(']') => {
                    self.advance();
                    TokenKind::RBracket
                }

                Some('\'') => {
                    self.advance();
                    TokenKind::Quote
                }
                Some('`') => {
                    self.advance();
                    TokenKind::Quasiquote
                }
                Some(',') => {
                    self.advance();
                    if self.peek() == Some('@') {
                        self.advance();
                        TokenKind::UnquoteSplice
                    } else {
                        TokenKind::Unquote
                    }
                }
                Some('"') => self.lex_string()?,
                Some(c) if c.is_ascii_digit() => self.lex_number()?,
                Some('-') => {
                    let next_is_digit = match self.peek_next() {
                        Some(c) => c.is_ascii_digit(),
                        None => false,
                    };
                    if next_is_digit {
                        self.lex_number()?
                    } else {
                        self.lex_symbol()
                    }
                }

                Some(':') => {
                    let next_is_symbol = match self.peek_next() {
                        Some(c) => is_symbol_start(c),
                        None => false,
                    };
                    if next_is_symbol {
                        self.lex_keyword()
                    } else {
                        self.advance();
                        TokenKind::Colon
                    }
                }
                Some(c) if is_symbol_start(c) => self.lex_symbol(),
                Some(c) => return Err(self.error(&format!("unexpected character: {c}"))),
            };
            tokens.push(Token { kind, line, col });
        }
        Ok(tokens)
    }
}

/// Whether `c` is a valid first character of a symbol.
///
/// True for letters and the punctuation characters listed in the lexical
/// grammar (`_ + - * / ! ? < > =`).
fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || matches!(c, '_' | '+' | '-' | '*' | '/' | '!' | '?' | '<' | '>' | '=')
}
