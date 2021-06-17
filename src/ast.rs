#[derive(PartialEq, Debug)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(String, Expr),
    Expr(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Fun(Vec<String>, Vec<Expr>),
    Call(String, Vec<Expr>),
    Statement(Box<Statement>),
}

#[derive(PartialEq, Debug)]
pub struct Module(pub Vec<Expr>);
