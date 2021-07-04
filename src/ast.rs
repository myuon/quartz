use crate::vm::DataType;

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
}

impl Literal {
    pub fn into_datatype(self) -> DataType {
        match self {
            Literal::Nil => DataType::Nil,
            Literal::Bool(b) => DataType::Bool(b),
            Literal::Int(i) => DataType::Int(i),
            Literal::String(s) => DataType::String(s),
        }
    }
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
