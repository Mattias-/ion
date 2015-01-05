
#[derive(Show, Clone)]
pub struct Type(pub String);

#[derive(Show, Clone)]
pub enum Stm {
    Vardef(Expr, Type),
    Assign(Expr, Expr),
}

#[derive(Show, Clone)]
pub enum Expr {
    Id(String),
    LitInt(int),
    Neg(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>)
}
