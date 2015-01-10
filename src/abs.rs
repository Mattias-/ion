
#[derive(Show, Clone)]
pub struct Type<'a>(pub &'a str);

#[derive(Show, Clone)]
pub enum Stm<'a> {
    Vardef(Expr<'a>, Type<'a>),
    Assign(Expr<'a>, Expr<'a>),
}

#[derive(Show, Clone)]
pub enum Expr<'a> {
    Id(&'a str),
    LitInt(i32),
    Neg(Box<Expr<'a>>),
    Plus(Box<Expr<'a>>, Box<Expr<'a>>),
    Minus(Box<Expr<'a>>, Box<Expr<'a>>)
}
