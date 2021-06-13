#[derive(PartialEq, Debug)]
pub enum Literal {
    IntLiteral(i32),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Return(Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Decl {
    Func(String, Vec<Expr>),
}
