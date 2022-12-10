use anyhow::{bail, Result};

use crate::util::{ident::Ident, path::Path, source::Source};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Omit(usize),
    Nil,
    Bool,
    I32,
    Byte,
    Func(Vec<Type>, Box<Type>),
    Record(Vec<(Ident, Type)>),
    Ident(Ident),
    Ptr(Box<Type>),
    Array(Box<Type>, usize),
    Vec(Box<Type>),
    Range(Box<Type>),
    Optional(Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Omit(i) => format!("?{}", i),
            Type::Nil => "nil".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::Func(args, ret) => format!(
                "({}) -> {}",
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                ret.to_string()
            ),
            Type::Record(fields) => format!(
                "{{{}}}",
                fields
                    .iter()
                    .map(|(name, t)| format!("{}: {}", name.as_str(), t.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Ident(name) => format!("{}", name.as_str()),
            Type::Ptr(t) => format!("pointer[{}]", t.to_string()),
            Type::Array(type_, size) => format!("array[{}, {}]", type_.to_string(), size),
            Type::Byte => "byte".to_string(),
            Type::Range(type_) => format!("range[{}]", type_.to_string()),
            Type::Vec(type_) => format!("vec[{}]", type_.to_string()),
            Type::Optional(type_) => format!("optional[{}]", type_.to_string()),
        }
    }

    pub fn to_func(self) -> Result<(Vec<Type>, Box<Type>)> {
        match self {
            Type::Func(args, ret) => Ok((args, ret)),
            _ => bail!("expected function type, but found {}", self.to_string()),
        }
    }

    pub fn to_record(self) -> Result<Vec<(Ident, Type)>> {
        match self {
            Type::Record(fields) => Ok(fields),
            _ => bail!("expected record type, but found {}", self.to_string()),
        }
    }

    pub fn to_ident(self) -> Result<Ident> {
        match self {
            Type::Ident(name) => Ok(name),
            Type::Array(_, _) => Ok(Ident("array".to_string())),
            Type::Vec(_) => Ok(Ident("vec".to_string())),
            _ => bail!("expected identifier type, but found {}", self.to_string()),
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

    pub fn as_range_type(&self) -> Result<&Type> {
        match self {
            Type::Range(t) => Ok(t),
            _ => bail!("expected range type, but found {}", self.to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lit {
    Nil,
    I32(i32),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Self_,
    Lit(Lit),
    Call(Box<Source<Expr>>, Vec<Source<Expr>>),
    Record(Ident, Vec<(Ident, Source<Expr>)>),
    Project(Box<Source<Expr>>, Type, Ident),
    Make(Type, Vec<Source<Expr>>),
    SizeOf(Type),
    Range(Box<Source<Expr>>, Box<Source<Expr>>),
    As(Box<Source<Expr>>, Type),
    Path(Path),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Type, Source<Expr>),
    Return(Source<Expr>),
    Expr(Source<Expr>),
    Assign(Box<Source<Expr>>, Box<Source<Expr>>),
    If(Source<Expr>, Type, Vec<Statement>, Option<Vec<Statement>>),
    While(Source<Expr>, Vec<Statement>),
    For(Ident, Source<Expr>, Vec<Statement>),
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
    Let(Ident, Type, Source<Expr>),
    Type(Ident, Type),
    Module(Ident, Module),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
