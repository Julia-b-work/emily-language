//! The parser: turns a flat token stream into a tree of [`Expr`]s.
//!
//! A recursive-descent parser. This is trivial for a Lisp because the
//! delimiters (`( )`, `[ ]`, `{ }`) define the structure directly.

use crate::ast::Expr;
use crate::token::Token;

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
        while let Some(_) = self.peek() {
            forms.push(self.parse_form()?);
        }
        Ok(forms)
    }

    /// Parses a single form, recursing for compound forms and quotes.
    fn parse_form(&mut self) -> Result<Expr, String> {
        match self.advance() {
            None => Err("unexpected end of input".to_string()),
            Some(Token::Int(n)) => Ok(Expr::Int(n)),
            Some(Token::Float(f)) => Ok(Expr::Float(f)),
            Some(Token::String(s)) => Ok(Expr::String(s)),
            Some(Token::Bool(b)) => Ok(Expr::Bool(b)),
            Some(Token::Symbol(s)) => Ok(Expr::Symbol(s)),
            Some(Token::Keyword(k)) => Ok(Expr::Keyword(k)),
            Some(Token::LParen) => self.parse_list(),
            Some(Token::LBracket) => self.parse_vector(),
            Some(Token::LBrace) => self.parse_record(),
            Some(Token::Quote) => Ok(Expr::Quote(Box::new(self.parse_form()?))),
            Some(Token::Quasiquote) => Ok(Expr::Quasiquote(Box::new(self.parse_form()?))),
            Some(Token::Unquote) => Ok(Expr::Unquote(Box::new(self.parse_form()?))),
            Some(Token::UnquoteSplice) => Ok(Expr::UnquoteSplice(Box::new(self.parse_form()?))),
            Some(other) => Err(format!("unexpected token: {other:?}")),
        }
    }

    /// Parses the elements of a `( ... )` list, up to the closing `)`.
    fn parse_list(&mut self) -> Result<Expr, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated list (missing ')')".to_string()),
                Some(Token::RParen) => {
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
                Some(Token::RBracket) => {
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
                Some(Token::RBrace) => {
                    self.advance();
                    break;
                }
                Some(Token::Keyword(name)) => {
                    let name = name.clone();
                    self.advance();
                    let value = self.parse_form()?;
                    fields.push((name, value));
                }
                Some(_) => return Err("expected a keyword in record".to_string()),
            }
        }
        Ok(Expr::Record(fields))
    }
}
