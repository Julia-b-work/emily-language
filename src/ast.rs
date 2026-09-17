#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Symbol(String),
    Keyword(String),
    List(Vec<Expr>),             // (a b c)
    Record(Vec<(String, Expr)>), // {:key val ...}
    Vector(Vec<Expr>),           // [a b c]
    Quote(Box<Expr>),            // 'x
    Quasiquote(Box<Expr>),       // `x
    Unquote(Box<Expr>),          // ,x
    UnquoteSplice(Box<Expr>),    // ,@x
}
