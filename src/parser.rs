
use crate::ast::Expr;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    pub fn parse(mut self) -> Result<Vec<Expr>, String> {
        let mut forms = Vec::new();
        while self.peek().is_some() {
            forms.push(self.parse_form()?);
        }
        Ok(forms)
    }

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

    fn parse_list(&mut self) -> Result<Expr, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated list (missing ')')".to_string()),
                Some(tok) if tok.kind == TokenKind::RParen => {
                    self.advance();
                    break;
                }
                Some(tok) if tok.kind == TokenKind::Colon => {
                    let (line, col) = (tok.line, tok.col);
                    self.advance();
                    self.parse_annotation(&mut items, line, col)?;
                }
                Some(_) => items.push(self.parse_form()?),
            }
        }
        Ok(Expr::List(items))
    }

    fn parse_vector(&mut self) -> Result<Expr, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated vector (missing ']')".to_string()),
                Some(tok) if tok.kind == TokenKind::RBracket => {
                    self.advance();
                    break;
                }
                Some(tok) if tok.kind == TokenKind::Colon => {
                    let (line, col) = (tok.line, tok.col);
                    self.advance();
                    self.parse_annotation(&mut items, line, col)?;
                }
                Some(_) => items.push(self.parse_form()?),
            }
        }
        Ok(Expr::Vector(items))
    }

    fn parse_annotation(
        &mut self,
        items: &mut Vec<Expr>,
        line: usize,
        col: usize,
    ) -> Result<(), String> {
        let value = items.pop().ok_or_else(|| {
            format!("type annotation with nothing before ':' (line {line}, column {col})")
        })?;
        let ty = match self.peek() {
            None
            | Some(Token {
                kind: TokenKind::RParen | TokenKind::RBracket,
                ..
            }) => {
                return Err(format!(
                    "type annotation with nothing after ':' (line {line}, column {col})"
                ));
            }
            _ => self.parse_form()?,
        };
        items.push(Expr::Ann {
            value: Box::new(value),
            ty: Box::new(ty),
        });
        Ok(())
    }

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
