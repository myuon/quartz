#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(Ident),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Expr),
    Return(Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Function(Ident, Vec<Ident>, Vec<Statement>),
}
