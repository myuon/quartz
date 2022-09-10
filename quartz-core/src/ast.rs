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
    Array,
}

#[derive(PartialEq, Debug, Clone)]
pub enum OptionalMode {
    Nil,
    Some,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(Vec<String>), // qualifier in vector
    Method(Type, String),
    Make(Type, Vec<Source<Expr>>),
    Lit(Literal, Type),
    Call(CallMode, Box<Source<Expr>>, Vec<Source<Expr>>),
    Struct(String, Vec<(String, Source<Expr>, Type)>),
    Project(
        bool, // is_method (be decided in typecheck phase)
        Type,
        Box<Source<Expr>>,
        String,
    ),
    Deref(Box<Source<Expr>>, Type),
    As(Box<Source<Expr>>, Type, Type),
    Ref(Box<Source<Expr>>, Type),
    Address(Box<Source<Expr>>, Type), // [compiler only] take the address of expr (same as ref, but no heap allocation)
    Optional(OptionalMode, Type, Box<Source<Expr>>),
    Unwrap(Box<Source<Expr>>, Type),
}

impl Expr {
    pub fn function_call(callee: Source<Expr>, args: Vec<Source<Expr>>) -> Expr {
        Expr::Call(CallMode::Function, Box::new(callee), args)
    }

    pub fn member(proj: Source<Expr>, field: impl Into<String>) -> Expr {
        Expr::Project(false, Type::Infer(0), Box::new(proj), field.into())
    }

    pub fn unwrap(expr: Source<Expr>) -> Expr {
        Expr::Unwrap(Box::new(expr), Type::Infer(0))
    }

    pub fn require_same_structure(&self, other: &Expr) -> Result<()> {
        use Expr::*;

        match (self, other) {
            (Var(x), Var(y)) => {
                if x != y {
                    bail!("[var] {:?} vs {:?}", x, y);
                }
            }
            (Method(t, x), Method(s, y)) => {
                if t != s {
                    bail!("[method] {:?} vs {:?}", t, s);
                }
                if x != y {
                    bail!("[method] {:?} vs {:?}", x, y);
                }
            }
            (Make(t, x), Make(s, y)) => {
                if t != s {
                    bail!("[make] {:?} vs {:?}", t, s);
                }
                if x.len() != y.len() {
                    bail!("[make] {:?} vs {:?}", x, y);
                }
                for (a, b) in x.iter().zip(y.iter()) {
                    a.data.require_same_structure(&b.data)?;
                }
            }
            (Lit(t, _x), Lit(s, _y)) => {
                if t != s {
                    bail!("[lit] {:?} vs {:?}", t, s);
                }
            }
            (Call(t, x, y), Call(s, a, b)) => {
                if t != s {
                    bail!("[call] {:?} vs {:?}", t, s);
                }
                x.data.require_same_structure(&a.data)?;
                if y.len() != b.len() {
                    bail!("[call] {:?} vs {:?}", a, b);
                }
                for (a, b) in y.iter().zip(b.iter()) {
                    a.data.require_same_structure(&b.data)?;
                }
            }
            (Struct(t, x), Struct(s, y)) => {
                if t != s {
                    bail!("[struct] {:?} vs {:?}", t, s);
                }
                if x.len() != y.len() {
                    bail!("[struct] {:?} vs {:?}", x, y);
                }
                for (a, b) in x.iter().zip(y.iter()) {
                    if a.0 != b.0 {
                        bail!("[struct] {:?} vs {:?}", a.0, b.0);
                    }
                    a.1.data.require_same_structure(&b.1.data)?;
                }
            }
            (Project(t, _, x, y), Project(s, _, a, b)) => {
                if t != s {
                    bail!("[project] {:?} vs {:?}", t, s);
                }
                x.data.require_same_structure(&a.data)?;
                if y != b {
                    bail!("[project] {:?} vs {:?}", y, b);
                }
            }
            (Deref(x, _), Deref(a, _)) => {
                x.data.require_same_structure(&a.data)?;
            }
            (As(x, _, t), As(a, _, s)) => {
                x.data.require_same_structure(&a.data)?;
                if t != s {
                    bail!("[as] {:?} vs {:?}", t, s);
                }
            }
            (Ref(x, _), Ref(a, _)) => {
                x.data.require_same_structure(&a.data)?;
            }
            (Address(x, _), Address(a, _)) => {
                x.data.require_same_structure(&a.data)?;
            }
            (Optional(t, _, x), Optional(s, _, a)) => {
                if t != s {
                    bail!("[optional] {:?} vs {:?}", t, s);
                }
                x.data.require_same_structure(&a.data)?;
            }
            (Unwrap(x, _), Unwrap(a, _)) => {
                x.data.require_same_structure(&a.data)?;
            }
            (x, y) => {
                bail!("[expr] {:?} vs {:?}", x, y);
            }
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<(String, Type)>,
    pub dead_code: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: Source<String>,
    pub type_params: Vec<String>,
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
    Import(Source<String>),
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
                func.name.data
            )),
            Declaration::Function(func) => Some(func.name.data.clone()),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module {
    pub module_path: String,
    pub imports: Vec<String>,
    pub decls: Vec<Declaration>,
}

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
    TypeApp(Vec<String>, Box<Type>),
    TypeVar(String),
}

impl Type {
    pub fn unwrap_type(&self) -> Result<&Type> {
        match self {
            Type::Optional(t) => Ok(t.as_ref()),
            Type::Ref(t) => Ok(t.as_ref()),
            _ => bail!("[type] Expected optional type, but found {:?}", self),
        }
    }

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

    pub fn as_optional(&self) -> Option<&Box<Type>> {
        match self {
            Type::Optional(t) => Some(t),
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
            Type::TypeApp(_, _) => todo!(),
            Type::TypeVar(_) => todo!(),
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
            Type::TypeApp(_, _) => todo!(),
            Type::TypeVar(_) => todo!(),
        }
    }

    pub fn subst_typevar(&mut self, var_name: &String, typ: &Type) {
        match self {
            Type::Infer(_) => {}
            Type::Any => {}
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(args, ret) => {
                for arg in args {
                    arg.subst_typevar(var_name, typ);
                }

                ret.subst_typevar(var_name, typ);
            }
            Type::Method(self_, args, ret) => {
                self_.subst_typevar(var_name, typ);
                for arg in args {
                    arg.subst_typevar(var_name, typ);
                }

                ret.subst_typevar(var_name, typ);
            }
            Type::Struct(_) => {}
            Type::Ref(_) => todo!(),
            Type::Byte => {}
            Type::Array(t) => t.subst_typevar(var_name, typ),
            Type::SizedArray(t, _n) => t.subst_typevar(var_name, typ),
            Type::Optional(t) => t.subst_typevar(var_name, typ),
            Type::Nil => {}
            Type::Self_ => {}
            Type::TypeApp(_, _) => todo!(),
            Type::TypeVar(t) => {
                if t == var_name {
                    *self = typ.clone();
                }
            }
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

    pub fn is_optional(&self) -> bool {
        match self {
            Type::Optional(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Type::Nil => true,
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

#[derive(Debug, Clone, PartialEq)]
pub struct StructTypeInfo {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<(String, Type)>,
}

impl StructTypeInfo {
    pub fn replace_params_in_fields(&mut self, type_params: &Vec<(String, Type)>) {
        for (p, pt) in type_params {
            for (_, t) in &mut self.fields {
                t.subst_typevar(p, pt);
            }
        }
    }

    pub fn field_labels(&self) -> Vec<&String> {
        self.fields.iter().map(|(l, _)| l).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Structs(pub HashMap<String, StructTypeInfo>);

impl Structs {
    pub fn get_projection_type(&self, val: &str, label: &str) -> Result<Type> {
        let struct_fields = self.0.get(val).ok_or(anyhow::anyhow!(
            "project type: {} not found in {}",
            label,
            val
        ))?;

        let (_, t) = struct_fields
            .fields
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

        let mut index = 0;
        for (l, _) in &struct_fields.fields {
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
