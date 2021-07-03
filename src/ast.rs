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
    ReturnIf(Expr, Expr),
    Panic,
    If(Box<Expr>, Vec<Statement>, Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Fun(
        usize, // position of fn
        Vec<String>,
        Vec<Statement>,
    ),
    Call(String, Vec<Expr>),
    Ref(Box<Expr>),
    Deref(Box<Expr>),
    Loop(Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Statement>);
