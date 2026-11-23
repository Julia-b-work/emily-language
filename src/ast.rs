
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Symbol(String),
    Keyword(String),
    Ann { value: Box<Expr>, ty: Box<Expr> },
    List(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    Vector(Vec<Expr>),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
    UnquoteSplice(Box<Expr>),
}
