use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
}

impl Literal {
    pub fn into_datatype(self) -> DataValue {
        match self {
            Literal::Nil => DataValue::Nil,
            Literal::Bool(b) => DataValue::Bool(b),
            Literal::Int(i) => DataValue::Int(i),
            Literal::String(s) => DataValue::String(s),
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
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Function(Function),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Declaration>);

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Infer(usize), // for typecheck
    Any,
    Unit,
    Bool,
    Int,
    String,
    Ref(Box<Type>),
    Fn(Vec<Type>, Box<Type>),
}

impl Type {
    pub fn as_fn_type(&self) -> Option<(&Vec<Type>, &Box<Type>)> {
        match self {
            Type::Fn(args, ret) => Some((args, ret)),
            _ => None,
        }
    }

    pub fn as_ref_type(&self) -> Option<&Box<Type>> {
        match self {
            Type::Ref(t) => Some(t),
            _ => None,
        }
    }

    pub fn has_infer(&self, index: usize) -> bool {
        match self {
            Type::Infer(t) => *t == index,
            Type::Any => false,
            Type::Unit => false,
            Type::Bool => false,
            Type::Int => false,
            Type::String => false,
            Type::Ref(t) => t.has_infer(index),
            Type::Fn(args, ret) => {
                args.iter().find(move |t| t.has_infer(index)).is_some() || ret.has_infer(index)
            }
        }
    }

    pub fn subst(&mut self, index: usize, typ: &Type) {
        match self {
            Type::Infer(t) => {
                if *t == index {
                    *self = typ.clone();
                }
            }
            Type::Any => {}
            Type::Unit => {}
            Type::Bool => {}
            Type::Int => {}
            Type::String => {}
            Type::Ref(t) => {
                t.subst(index, typ);
            }
            Type::Fn(args, ret) => {
                for arg in args {
                    arg.subst(index, typ);
                }

                ret.subst(index, typ);
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum DataValue {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Closure(Vec<String>, Vec<Statement>),
}

impl DataValue {
    pub fn as_bool(self) -> Result<bool> {
        match self {
            DataValue::Bool(i) => Ok(i),
            d => bail!("expected a bool, but found {:?}", d),
        }
    }

    pub fn as_int(self) -> Result<i32> {
        match self {
            DataValue::Int(i) => Ok(i),
            d => bail!("expected a int, but found {:?}", d),
        }
    }

    pub fn as_closure(self) -> Result<(Vec<String>, Vec<Statement>)> {
        match self {
            DataValue::Closure(params, body) => Ok((params, body)),
            d => bail!("Expected a closure, but found {:?}", d),
        }
    }
}
