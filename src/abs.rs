
#[deriving(Show, Clone)]
pub enum Expr {
    Id(String),
    Litint(int),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>)
}

#[deriving(Show, Clone)]
pub struct Type(pub String);

#[deriving(Show, Clone)]
pub enum Stm {
    Vardef(Expr, Type),
    Assign(Expr, Expr),
}
