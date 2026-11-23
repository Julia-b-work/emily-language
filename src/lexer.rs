
use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

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

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn error(&self, msg: &str) -> String {
        format!("{msg} (line {}, column {})", self.line, self.col)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                    self.advance();
                }
                Some(';') => {
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

    fn lex_string(&mut self) -> Result<TokenKind, String> {
        self.advance();
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

    fn lex_symbol(&mut self) -> TokenKind {
        let s = self.read_symbol_chars();
        match s.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            _ => TokenKind::Symbol(s),
        }
    }

    fn lex_keyword(&mut self) -> TokenKind {
        self.advance();
        let s = self.read_symbol_chars();
        TokenKind::Keyword(s)
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            let (line, col) = (self.line, self.col);
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

fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || matches!(c, '_' | '+' | '-' | '*' | '/' | '!' | '?' | '<' | '>' | '=')
}
