#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(String, Expr),
    Expr(Expr),
    Return(Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Fun(Vec<String>, Vec<Expr>),
    Call(String, Vec<Expr>),
    Statement(Box<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Expr>);
