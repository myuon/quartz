use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Ident(pub String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Omit(usize),
    I32,
    Func(Vec<Type>, Box<Type>),
    Nil,
    Bool,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Omit(i) => format!("?{}", i),
            Type::I32 => "i32".to_string(),
            Type::Func(args, ret) => format!(
                "({}) -> {}",
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                ret.to_string()
            ),
            Type::Nil => "nil".to_string(),
            Type::Bool => "bool".to_string(),
        }
    }

    pub fn to_func(self) -> Result<(Vec<Type>, Box<Type>)> {
        match self {
            Type::Func(args, ret) => Ok((args, ret)),
            _ => bail!("expected function type, but found {}", self.to_string()),
        }
    }

    pub fn is_omit(&self) -> bool {
        match self {
            Type::Omit(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Type::Nil => true,
            _ => false,
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
    Call(Ident, Vec<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Type, Expr),
    Return(Expr),
    Expr(Expr),
    Assign(Option<VarType>, Ident, Box<Expr>),
    If(Expr, Type, Vec<Statement>, Option<Vec<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum VarType {
    Local,
    Global,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub result: Type,
    pub body: Vec<Statement>,
}

impl Func {
    pub fn to_type(&self) -> Type {
        Type::Func(
            self.params.iter().map(|(_, t)| t.clone()).collect(),
            Box::new(self.result.clone()),
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Decl {
    Func(Func),
    Let(Ident, Type, Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
