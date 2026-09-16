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

    /// Reads a number, returning an `Int` or `Float` token.
    fn lex_number(&mut self) -> Result<Token, String> {
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
            let value: f64 = text.parse().map_err(|_| format!("invalid float: {text}"))?;
            return Ok(Token::Float(value));
        }

        let value: i64 = text.parse().map_err(|_| format!("invalid int: {text}"))?;
        Ok(Token::Int(value))
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
    fn lex_symbol(&mut self) -> Token {
        let s = self.read_symbol_chars();
        match s.as_str() {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Symbol(s),
        }
    }

    /// Reads a keyword (`:name`). Called only after the `:` is confirmed.
    fn lex_keyword(&mut self) -> Token {
        self.advance(); // skip the ':'
        let s = self.read_symbol_chars();
        Token::Keyword(s)
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            match self.peek() {
                None => break,
                Some('(') => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                Some(')') => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                Some('{') => {
                    self.advance();
                    tokens.push(Token::LBrace);
                }
                Some('}') => {
                    self.advance();
                    tokens.push(Token::RBrace);
                }
                Some('[') => {
                    self.advance();
                    tokens.push(Token::LBracket);
                }
                Some(']') => {
                    self.advance();
                    tokens.push(Token::RBracket);
                }

                Some('\'') => {
                    self.advance();
                    tokens.push(Token::Quote);
                }
                Some('`') => {
                    self.advance();
                    tokens.push(Token::Quasiquote);
                }
                Some(',') => {
                    self.advance();
                    if self.peek() == Some('@') {
                        self.advance();
                        tokens.push(Token::UnquoteSplice);
                    } else {
                        tokens.push(Token::Unquote);
                    }
                }
                Some('"') => tokens.push(self.lex_string()?),
                Some(c) if c.is_ascii_digit() => tokens.push(self.lex_number()?),
                Some('-') => {
                    let next_is_digit = match self.peek_next() {
                        Some(c) => c.is_ascii_digit(),
                        None => false,
                    };
                    if next_is_digit {
                        tokens.push(self.lex_number()?);
                    } else {
                        tokens.push(self.lex_symbol());
                    }
                }

                Some(':') => {
                    let next_is_symbol = match self.peek_next() {
                        Some(c) => is_symbol_start(c),
                        None => false,
                    };
                    if next_is_symbol {
                        tokens.push(self.lex_keyword());
                    } else {
                        self.advance();
                        tokens.push(Token::Colon);
                    }
                }
                Some(c) if is_symbol_start(c) => tokens.push(self.lex_symbol()),
                Some(c) => return Err(format!("unexpected character: {c}")),
            }
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
