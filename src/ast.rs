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
    Let(String, Expr),
    Expr(Expr),
    Return(Expr),
    If(Box<Expr>, Vec<Statement>, Vec<Statement>),
    Continue,
    Assignment(Box<Expr>, Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    Call(Box<Expr>, Vec<Expr>),
    Loop(Vec<Statement>),
    Struct(String, Vec<(String, Expr)>),
    Project(
        bool,   // is_method
        String, // name of the struct (will be filled in typecheck phase)
        Box<Expr>,
        String,
    ),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub method_of: Option<(String, String)>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Function(Function),
    Variable(String, Expr),
    Struct(Struct),
}

impl Declaration {
    pub fn into_function(self) -> Result<Function> {
        match self {
            Declaration::Function(f) => Ok(f),
            _ => bail!("Expected function declaration, but found {:?}", self),
        }
    }
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
    Struct(String),
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

    pub fn as_struct_type(&self) -> Option<&String> {
        match self {
            Type::Struct(s) => Some(s),
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
            Type::Struct(_) => false,
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
            Type::Struct(_) => {}
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
    Tuple(Vec<DataValue>),
    NativeFunction(String),
    Function(String),
    Method(String, String, Box<DataValue>),
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

    pub fn as_string(self) -> Result<String> {
        match self {
            DataValue::String(i) => Ok(i),
            d => bail!("expected a string, but found {:?}", d),
        }
    }

    pub fn as_tuple(self) -> Result<Vec<DataValue>> {
        match self {
            DataValue::Tuple(t) => Ok(t),
            d => bail!("Expected a tuple, but found {:?}", d),
        }
    }
}
