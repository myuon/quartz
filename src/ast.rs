#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(
        bool, // static or not
        String,
        Expr,
    ),
    Expr(Expr),
    Return(Expr),
    ReturnIf(Expr, Expr),
    If(Box<Expr>, Vec<Statement>, Vec<Statement>),
    Continue,
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
