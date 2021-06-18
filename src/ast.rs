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
    Panic,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Fun(Vec<String>, Vec<Statement>),
    Call(String, Vec<Expr>),
    Ref(Box<Expr>),
    Deref(Box<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Statement>);
