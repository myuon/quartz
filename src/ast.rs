use anyhow::{bail, Result};

use crate::util::{ident::Ident, path::Path, source::Source};

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Type {
    Omit(usize),
    Nil,
    Bool,
    I32,
    I64,
    Byte,
    Func(Vec<Type>, Box<Type>),
    VariadicFunc(Vec<Type>, Box<Type>, Box<Type>),
    Record(Vec<(Ident, Type)>),
    Ident(Ident),
    Ptr(Box<Type>),
    Array(Box<Type>, usize),
    Vec(Box<Type>),
    Range(Box<Type>),
    Optional(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Or(Box<Type>, Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Omit(i) => format!("?{}", i),
            Type::Nil => "[nil]".to_string(),
            Type::Bool => "[bool]".to_string(),
            Type::I32 => "[i32]".to_string(),
            Type::I64 => "[i64]".to_string(),
            Type::Func(args, ret) => format!(
                "({}) -> {}",
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                ret.to_string()
            ),
            Type::VariadicFunc(args, ret, variadic) => format!(
                "({}, ..{}) -> {}",
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                variadic.to_string(),
                ret.to_string(),
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
            Type::Map(key, value) => format!("map[{}, {}]", key.to_string(), value.to_string()),
            Type::Or(left, right) => format!("{} or {}", left.to_string(), right.to_string()),
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
            // FIXME: how to handle methods for primitive types naturally?
            Type::Array(_, _) => Ok(Ident("array".to_string())),
            Type::Vec(_) => Ok(Ident("vec".to_string())),
            Type::I32 => Ok(Ident("i32".to_string())),
            Type::Bool => Ok(Ident("bool".to_string())),
            _ => bail!("expected identifier type, but found {}", self.to_string()),
        }
    }

    pub fn to_optional(self) -> Result<Box<Type>> {
        match self {
            Type::Optional(t) => Ok(t),
            _ => bail!("expected optional type, but found {}", self.to_string()),
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

    pub fn is_integer_type(&self) -> bool {
        match self {
            Type::I32 | Type::I64 => true,
            _ => false,
        }
    }

    pub fn is_bool_type(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => false,
        }
    }

    pub fn as_record_type(&self) -> Result<&Vec<(Ident, Type)>> {
        match self {
            Type::Record(fields) => Ok(fields),
            _ => bail!("expected record type, but found {}", self.to_string()),
        }
    }

    pub fn as_vec_type_element(&self) -> Result<&Type> {
        match self {
            Type::Vec(t) => Ok(t),
            _ => bail!("expected vec type, but found {}", self.to_string()),
        }
    }

    pub fn as_or_type(&self) -> Result<(&Type, &Type)> {
        match self {
            Type::Or(left, right) => Ok((left, right)),
            _ => bail!("expected or type, but found {}", self.to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lit {
    Nil,
    Bool(bool),
    I32(i32),
    I64(i64),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariadicCall {
    pub element_type: Type,
    pub index: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnwrapMode {
    Optional,
    Or,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident {
        ident: Ident,
        resolved_path: Option<Path>,
    },
    Self_,
    Lit(Lit),
    Call(
        Box<Source<Expr>>,
        Vec<Source<Expr>>,
        Option<VariadicCall>,
        Option<Box<Source<Expr>>>,
    ),
    BinOp(BinOp, Type, Box<Source<Expr>>, Box<Source<Expr>>),
    Record(
        Source<Ident>,
        Vec<(Ident, Source<Expr>)>,
        Option<Box<Source<Expr>>>,
    ),
    AnonymousRecord(Vec<(Ident, Source<Expr>)>, Type),
    Project(Box<Source<Expr>>, Type, Path),
    Make(Type, Vec<Source<Expr>>),
    SizeOf(Type),
    Range(Box<Source<Expr>>, Box<Source<Expr>>),
    As(Box<Source<Expr>>, Type),
    Path {
        path: Source<Path>,
        resolved_path: Option<Path>,
    },
    Equal(Box<Source<Expr>>, Box<Source<Expr>>),
    NotEqual(Box<Source<Expr>>, Box<Source<Expr>>),
    Wrap(Type, Box<Source<Expr>>),
    Unwrap(Type, Option<UnwrapMode>, Box<Source<Expr>>),
    Omit(Type),
    EnumOr(
        Type,
        Type,
        Option<Box<Source<Expr>>>,
        Option<Box<Source<Expr>>>,
    ),
}

impl Expr {
    pub fn ident(ident: Ident) -> Self {
        Expr::Ident {
            ident,
            resolved_path: None,
        }
    }

    pub fn path(path: Path) -> Self {
        Expr::Path {
            path: Source::unknown(path),
            resolved_path: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Pattern, Type, Source<Expr>),
    Return(Source<Expr>),
    Expr(Source<Expr>, Type),
    Assign(Box<Source<Expr>>, Box<Source<Expr>>),
    If(
        Source<Expr>,
        Type,
        Vec<Source<Statement>>,
        Option<Vec<Source<Statement>>>,
    ),
    While(Source<Expr>, Vec<Source<Statement>>),
    For(Ident, Source<Expr>, Vec<Source<Statement>>),
    Continue,
    Break,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    Ident(Ident),
    Or(Box<Pattern>, Box<Pattern>),
    Omit,
}

impl Pattern {
    pub fn as_ident(&self) -> Result<&Ident> {
        match self {
            Pattern::Ident(ident) => Ok(ident),
            _ => bail!("expected identifier pattern, but found {:?}", self),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub variadic: Option<(Ident, Type)>,
    pub result: Type,
    pub body: Vec<Source<Statement>>,
}

impl Func {
    pub fn to_type(&self) -> Type {
        if let Some((_, t)) = &self.variadic {
            Type::VariadicFunc(
                self.params.iter().map(|(_, t)| t.clone()).collect(),
                Box::new(t.clone()),
                Box::new(self.result.clone()),
            )
        } else {
            Type::Func(
                self.params.iter().map(|(_, t)| t.clone()).collect(),
                Box::new(self.result.clone()),
            )
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Decl {
    Func(Func),
    Let(Ident, Type, Source<Expr>),
    Type(Ident, Type),
    Module(Path, Module),
    Import(Path),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
