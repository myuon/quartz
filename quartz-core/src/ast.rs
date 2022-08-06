use std::collections::HashMap;

use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone)]
pub struct Source<T> {
    pub data: T,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl<T> Source<T> {
    pub fn new(data: T, start: usize, end: usize) -> Source<T> {
        Source {
            data,
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn unknown(data: T) -> Source<T> {
        Source {
            data,
            start: None,
            end: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Array(Vec<Source<Expr>>, Type),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(String, Source<Expr>, Type),
    Expr(Source<Expr>, Type),
    Return(Source<Expr>, Type),
    If(
        Box<Source<Expr>>,
        Vec<Source<Statement>>,
        Vec<Source<Statement>>,
    ),
    Continue,
    Assignment(Type, Source<Expr>, Source<Expr>),
    While(Source<Expr>, Vec<Source<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum CallMode {
    Function,
    SizedArray,
    Array(Type), // return type
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(Vec<String>, Type), // qualifier in vector
    Method(Type, String, Type),
    Make(Type, Vec<Source<Expr>>),
    Lit(Literal, Type),
    Call(CallMode, Box<Source<Expr>>, Vec<Source<Expr>>),
    Struct(String, Vec<(String, Source<Expr>, Type)>),
    Project(
        bool, // is_method
        Type,
        Box<Source<Expr>>,
        String,
    ),
    Deref(Box<Source<Expr>>, Type),
    As(Box<Source<Expr>>, Type, Type),
    Ref(Box<Source<Expr>>, Type),
    Address(Box<Source<Expr>>, Type), // [compiler only] take the address of expr (same as ref, but no heap allocation)
    Unwrap(Box<Source<Expr>>, Type),
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
    pub body: Vec<Source<Statement>>,
    pub dead_code: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Method(Type, Function),
    Function(Function),
    Variable(String, Source<Expr>, Type),
    Struct(Struct),
}

impl Declaration {
    pub fn into_function(self) -> Result<Function> {
        match self {
            Declaration::Function(f) => Ok(f),
            _ => bail!("Expected function declaration, but found {:?}", self),
        }
    }

    // if the function is a method, [STRUCT_NAME]::[METHOD_NAME]
    pub fn function_path(&self) -> Option<String> {
        match self {
            Declaration::Method(typ, func) => Some(format!(
                "{}::{}",
                typ.method_selector_name().ok()?,
                func.name
            )),
            Declaration::Function(func) => Some(func.name.clone()),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Declaration>);

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Infer(usize), // for typecheck
    Any,
    Nil, // literal "nil"
    Bool,
    Int,
    Byte,
    Fn(Vec<Type>, Box<Type>),
    Method(Box<Type>, Vec<Type>, Box<Type>),
    Struct(String),
    Ref(Box<Type>),
    Array(Box<Type>),
    SizedArray(Box<Type>, usize),
    Optional(Box<Type>),
    Self_,
}

impl Type {
    pub fn as_fn_type(&self) -> Option<(&Vec<Type>, &Box<Type>)> {
        match self {
            Type::Fn(args, ret) => Some((args, ret)),
            Type::Method(_, args, ret) => Some((args, ret)),
            _ => None,
        }
    }

    pub fn is_method_type(&self) -> bool {
        match self {
            Type::Method(_, _, _) => true,
            _ => false,
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

    pub fn as_sized_array(&self) -> Option<(&Box<Type>, &usize)> {
        match self {
            Type::SizedArray(t, size) => Some((t, size)),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Box<Type>> {
        match self {
            Type::Array(t) => Some(t),
            _ => None,
        }
    }

    pub fn has_infer(&self, index: usize) -> bool {
        match self {
            Type::Infer(t) => *t == index,
            Type::Any => false,
            Type::Bool => false,
            Type::Int => false,
            Type::Fn(args, ret) => {
                args.iter().find(move |t| t.has_infer(index)).is_some() || ret.has_infer(index)
            }
            Type::Method(self_, args, ret) => {
                self_.has_infer(index)
                    || args.iter().find(move |t| t.has_infer(index)).is_some()
                    || ret.has_infer(index)
            }
            Type::Struct(_) => false,
            Type::Ref(_) => todo!(),
            Type::Byte => false,
            Type::Array(t) => t.has_infer(index),
            Type::SizedArray(t, _n) => t.has_infer(index),
            Type::Optional(t) => t.has_infer(index),
            Type::Nil => false,
            Type::Self_ => false,
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
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(args, ret) => {
                for arg in args {
                    arg.subst(index, typ);
                }

                ret.subst(index, typ);
            }
            Type::Method(self_, args, ret) => {
                self_.subst(index, typ);
                for arg in args {
                    arg.subst(index, typ);
                }

                ret.subst(index, typ);
            }
            Type::Struct(_) => {}
            Type::Ref(_) => todo!(),
            Type::Byte => {}
            Type::Array(t) => t.subst(index, typ),
            Type::SizedArray(t, _n) => t.subst(index, typ),
            Type::Optional(t) => t.subst(index, typ),
            Type::Nil => {}
            Type::Self_ => {}
        }
    }

    // Whether the representation is an adress or not
    pub fn is_struct(&self) -> bool {
        match self {
            Type::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
    }

    pub fn method_selector_name(&self) -> Result<String> {
        Ok(match self {
            Type::Any => "any".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Int => "int".to_string(),
            Type::Byte => "byte".to_string(),
            Type::Struct(s) => s.to_string(),
            Type::Ref(r) => r.method_selector_name()?,
            Type::Array(_) => "array".to_string(),
            Type::SizedArray(_, _) => "sized_array".to_string(),
            Type::Optional(n) => n.method_selector_name()?,
            s => bail!("{:?} is not a method selector", s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Structs(pub HashMap<String, Vec<(String, Type)>>);

impl Structs {
    pub fn get_projection_type(&self, val: &str, label: &str) -> Result<Type> {
        let struct_fields = self.0.get(val).ok_or(anyhow::anyhow!(
            "project type: {} not found in {}",
            label,
            val
        ))?;

        let (_, t) =
            struct_fields
                .iter()
                .find(|(name, _)| name == label)
                .ok_or(anyhow::anyhow!(
                    "project type: {} not found in {}",
                    label,
                    val
                ))?;

        Ok(t.clone())
    }

    pub fn get_projection_offset(&self, val: &str, label: &str) -> Result<usize> {
        let struct_fields =
            self.0
                .get(val)
                .ok_or(anyhow::anyhow!("project: {} not found in {}", label, val))?;

        let mut index = 1; // pointer to info table
        for (l, _) in struct_fields {
            if l == label {
                break;
            }

            index += 1;
        }

        Ok(index)
    }
}

#[derive(Debug, Clone)]
pub struct Functions(
    pub  HashMap<
        String,
        (
            Vec<(String, Type)>,    // argument types
            Box<Type>,              // return type
            Vec<Source<Statement>>, // body
        ),
    >,
);
