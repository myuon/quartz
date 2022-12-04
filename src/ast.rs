use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Omit,
    I32,
    Func(Vec<Type>, Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Omit => "_".to_string(),
            Type::I32 => "i32".to_string(),
            Type::Func(args, ret) => format!(
                "({}) -> {}",
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                ret.to_string()
            ),
        }
    }

    pub fn to_func(self) -> Result<(Vec<Type>, Box<Type>)> {
        match self {
            Type::Func(args, ret) => Ok((args, ret)),
            _ => bail!("expected function type, but found {}", self.to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lit {
    I32(i32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Lit(Lit),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Type, Expr),
    Return(Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub result: Type,
    pub body: Vec<Statement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Decl {
    Func(Func),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
