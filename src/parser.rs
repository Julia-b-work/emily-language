//! The parser: turns a flat token stream into a tree of [`Expr`]s.
//!
//! A recursive-descent parser. This is trivial for a Lisp because the
//! delimiters (`( )`, `[ ]`, `{ }`) define the structure directly.
//!
//! Parse errors report the offending token's line and column.

use crate::ast::Expr;
use crate::token::{Token, TokenKind};

/// A cursor over the token stream — the same idea as the lexer, but over
/// [`Token`]s instead of characters.
pub struct Parser {
    /// The tokens produced by the lexer.
    tokens: Vec<Token>,
    /// The current cursor position (an index into `tokens`).
    pos: usize,
}

impl Parser {
    /// Builds a parser over the given token stream.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Returns the token at the cursor without moving, or `None` at end of input.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Returns the token at the cursor and advances by one.
    fn advance(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    /// Parses a whole program: a sequence of top-level forms.
    pub fn parse(mut self) -> Result<Vec<Expr>, String> {
        let mut forms = Vec::new();
        while self.peek().is_some() {
            forms.push(self.parse_form()?);
        }
        Ok(forms)
    }

    /// Parses a single form, recursing for compound forms and quotes.
    fn parse_form(&mut self) -> Result<Expr, String> {
        let token = self
            .advance()
            .ok_or_else(|| "unexpected end of input".to_string())?;
        let Token { kind, line, col } = token;
        match kind {
            TokenKind::Int(n) => Ok(Expr::Int(n)),
            TokenKind::Float(f) => Ok(Expr::Float(f)),
            TokenKind::String(s) => Ok(Expr::String(s)),
            TokenKind::Bool(b) => Ok(Expr::Bool(b)),
            TokenKind::Symbol(s) => Ok(Expr::Symbol(s)),
            TokenKind::Keyword(k) => Ok(Expr::Keyword(k)),
            TokenKind::LParen => self.parse_list(),
            TokenKind::LBracket => self.parse_vector(),
            TokenKind::LBrace => self.parse_record(),
            TokenKind::Quote => Ok(Expr::Quote(Box::new(self.parse_form()?))),
            TokenKind::Quasiquote => Ok(Expr::Quasiquote(Box::new(self.parse_form()?))),
            TokenKind::Unquote => Ok(Expr::Unquote(Box::new(self.parse_form()?))),
            TokenKind::UnquoteSplice => Ok(Expr::UnquoteSplice(Box::new(self.parse_form()?))),
            other => Err(format!(
                "unexpected token: {other:?} (line {line}, column {col})"
            )),
        }
    }

    /// Parses the elements of a `( ... )` list, up to the closing `)`.
    fn parse_list(&mut self) -> Result<Expr, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated list (missing ')')".to_string()),
                Some(tok) if tok.kind == TokenKind::RParen => {
                    self.advance();
                    break;
                }
                Some(_) => items.push(self.parse_form()?),
            }
        }
        Ok(Expr::List(items))
    }

    /// Parses the elements of a `[ ... ]` vector, up to the closing `]`.
    fn parse_vector(&mut self) -> Result<Expr, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated vector (missing ']')".to_string()),
                Some(tok) if tok.kind == TokenKind::RBracket => {
                    self.advance();
                    break;
                }
                Some(_) => items.push(self.parse_form()?),
            }
        }
        Ok(Expr::Vector(items))
    }

    /// Parses the key/value pairs of a `{ ... }` record, up to the closing `}`.
    fn parse_record(&mut self) -> Result<Expr, String> {
        let mut fields = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated record (missing '}')".to_string()),
                Some(tok) => match &tok.kind {
                    TokenKind::RBrace => {
                        self.advance();
                        break;
                    }
                    TokenKind::Keyword(name) => {
                        let name = name.clone();
                        self.advance();
                        let value = self.parse_form()?;
                        fields.push((name, value));
                    }
                    _ => {
                        return Err(format!(
                            "expected a keyword in record, got {:?} (line {}, column {})",
                            tok.kind, tok.line, tok.col
                        ));
                    }
                },
            }
        }
        Ok(Expr::Record(fields))
    }
}
