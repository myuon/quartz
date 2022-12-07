use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Ident(pub String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Omit(usize),
    Nil,
    Bool,
    I32,
    Func(Vec<Type>, Box<Type>),
    Record(Vec<(Ident, Type)>),
    Ident(Ident),
    Pointer(Box<Type>),
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
            Type::Pointer(t) => format!("pointer<{}>", t.to_string()),
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
    Record(Ident, Vec<(Ident, Expr)>),
    Project(Box<Expr>, Type, Ident),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Type, Expr),
    Return(Expr),
    Expr(Expr),
    Assign(Box<Expr>, Box<Expr>),
    If(Expr, Type, Vec<Statement>, Option<Vec<Statement>>),
    While(Expr, Vec<Statement>),
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
    Type(Ident, Type),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
