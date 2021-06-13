#[derive(PartialEq, Debug)]
pub enum Literal {
    IntLiteral(i32),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Let(String, Box<Expr>),
    Const(String, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Return(Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Decl {
    Func(String, Vec<Expr>),
}
