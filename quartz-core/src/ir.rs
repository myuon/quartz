use std::collections::HashMap;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::ast::{Structs, Type};

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    Bool(bool),
    Int(i32),
    Ident(String),
    Argument(usize),
    Info(usize),
    String(String),
}

impl IrTerm {
    pub fn into_ident(self) -> Result<String> {
        match self {
            IrTerm::Ident(s) => Ok(s),
            _ => bail!("expected ident"),
        }
    }

    pub fn into_int(self) -> Result<i32> {
        match self {
            IrTerm::Int(i) => Ok(i),
            _ => bail!("expected int"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IrBlock {
    pub name: String,
    pub elements: Vec<IrElement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrElement {
    Term(IrTerm),
    Block(IrBlock),
}

impl IrElement {
    pub fn ident(name: impl Into<String>) -> IrElement {
        IrElement::Term(IrTerm::Ident(name.into()))
    }

    pub fn block(name: &str, elements: Vec<IrElement>) -> IrElement {
        IrElement::Block(IrBlock {
            name: name.to_string(),
            elements,
        })
    }

    pub fn instruction(name: &str, elements: Vec<IrTerm>) -> IrElement {
        IrElement::Block(IrBlock {
            name: name.to_string(),
            elements: elements.into_iter().map(|e| IrElement::Term(e)).collect(),
        })
    }

    pub fn into_term(self) -> Result<IrTerm> {
        match self {
            IrElement::Term(t) => Ok(t),
            _ => bail!("Expected a term, but found {}", self.show()),
        }
    }

    pub fn into_block(self) -> Result<IrBlock> {
        match self {
            IrElement::Block(b) => Ok(b),
            _ => bail!("Expected a block, but found {}", self.show()),
        }
    }

    fn show_recur(&self, depth: i32, compact: bool) -> String {
        match self {
            IrElement::Term(t) => match t {
                IrTerm::Nil => "nil".to_string(),
                IrTerm::Bool(b) => format!("{}", b),
                IrTerm::Int(n) => format!("{}", n),
                IrTerm::Ident(i) => format!("${}", i),
                IrTerm::Argument(a) => format!("${}", a),
                IrTerm::Info(i) => format!("{}", i),
                IrTerm::String(s) => format!("\"{}\"", s),
            },
            IrElement::Block(b) => {
                let mut buffer = String::new();
                let indent = if compact {
                    if depth > 0 {
                        " ".to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "  ".repeat(depth as usize)
                };

                buffer.push_str(&format!("{}({}", indent, b.name));
                for e in &b.elements {
                    match e {
                        IrElement::Term(_) => {
                            buffer.push_str(&format!(" {}", e.show_recur(depth, compact)));
                        }
                        IrElement::Block(_) => {
                            buffer.push_str(&format!(
                                "{}{}",
                                if compact { "" } else { "\n" },
                                e.show_recur(depth + 1, compact)
                            ));
                        }
                    }
                }
                buffer.push_str(")");

                buffer
            }
        }
    }

    pub fn show(&self) -> String {
        self.show_recur(0, false)
    }

    pub fn show_compact(&self) -> String {
        self.show_recur(0, true)
    }

    pub fn walk_ident(&mut self, walker: &impl Fn(&mut String) -> IrElement) {
        match self {
            IrElement::Term(t) => match t {
                IrTerm::Ident(i) => {
                    *self = walker(i);
                }
                _ => {}
            },
            IrElement::Block(b) => {
                for e in &mut b.elements {
                    e.walk_ident(walker);
                }
            }
        }
    }

    // = IR instructions

    pub fn nil() -> IrElement {
        IrElement::Term(IrTerm::Nil)
    }

    pub fn bool(b: bool) -> IrElement {
        IrElement::Term(IrTerm::Bool(b))
    }

    pub fn int(num: i32) -> IrElement {
        IrElement::Term(IrTerm::Int(num))
    }

    pub fn string(s: impl Into<String>) -> IrElement {
        IrElement::Term(IrTerm::String(s.into()))
    }

    pub fn i_let(ident: String, element: IrElement) -> IrElement {
        IrElement::block("let", vec![IrElement::Term(IrTerm::Ident(ident)), element])
    }

    pub fn i_assign(lhs: IrElement, rhs: IrElement) -> IrElement {
        IrElement::block("assign", vec![lhs, rhs])
    }

    pub fn i_copy(size: usize, source: IrElement) -> IrElement {
        IrElement::block("copy", vec![IrElement::int(size as i32), source])
    }

    pub fn i_call(name: impl Into<String>, mut args: Vec<IrElement>) -> IrElement {
        args.insert(0, IrElement::Term(IrTerm::Ident(name.into())));

        IrElement::i_call_raw(args)
    }

    pub fn i_call_raw(args: Vec<IrElement>) -> IrElement {
        IrElement::block("call", args)
    }

    pub fn i_deref(element: IrElement) -> IrElement {
        IrElement::block("deref", vec![element])
    }

    pub fn i_coerce(element: IrElement, expected_typ: IrElement) -> IrElement {
        IrElement::block("coerce", vec![element, expected_typ])
    }

    pub fn i_address(element: IrElement) -> IrElement {
        IrElement::block("address", vec![element])
    }

    pub fn i_index(element: IrElement, offset: IrElement) -> IrElement {
        IrElement::block("index", vec![element, offset])
    }

    pub fn i_addr_index(element: IrElement, offset: IrElement) -> IrElement {
        IrElement::block("addr_index", vec![element, offset])
    }

    pub fn i_offset(element: IrElement, offset: usize) -> IrElement {
        IrElement::block("offset", vec![element, IrElement::int(offset as i32)])
    }

    pub fn i_addr_offset(element: IrElement, offset: usize) -> IrElement {
        IrElement::block("addr_offset", vec![element, IrElement::int(offset as i32)])
    }

    pub fn i_tuple(typ: IrElement, mut element: Vec<IrElement>) -> IrElement {
        element.insert(0, typ);

        IrElement::block("tuple", element)
    }

    pub fn i_slice(len: usize, typ: IrElement, element: IrElement) -> IrElement {
        IrElement::block("slice", vec![IrElement::int(len as i32), typ, element])
    }

    pub fn i_slice_raw(len: IrElement, typ: IrType, element: IrElement) -> IrElement {
        IrElement::block("slice", vec![len, typ.to_element(), element])
    }

    pub fn i_size_of(typ: IrType) -> IrElement {
        IrElement::block("size_of", vec![typ.to_element()])
    }

    pub fn i_pop(typ: IrElement) -> IrElement {
        IrElement::block("pop", vec![typ])
    }

    pub fn i_return(element: IrElement) -> IrElement {
        IrElement::block("return", vec![element])
    }

    pub fn i_alloc(typ: IrElement, len: IrElement) -> IrElement {
        IrElement::block("alloc", vec![typ, len])
    }

    pub fn i_while(cond: IrElement, body: Vec<IrElement>) -> IrElement {
        IrElement::block("while", vec![cond, IrElement::block("seq", body)])
    }

    pub fn i_typetag(type_: IrElement) -> IrElement {
        IrElement::block("typetag", vec![type_])
    }

    pub fn d_var(name: impl Into<String>, typ: IrType, expr: IrElement) -> IrElement {
        IrElement::block(
            "var",
            vec![
                IrElement::Term(IrTerm::Ident(name.into())),
                typ.to_element(),
                expr,
            ],
        )
    }

    pub fn d_func(
        name: impl Into<String>,
        args: Vec<IrType>,
        ret: Box<IrType>,
        body: Vec<IrElement>,
    ) -> IrElement {
        let mut elements = vec![
            IrElement::Term(IrTerm::Ident(name.into())),
            IrElement::block(
                "args",
                args.into_iter().rev().map(|t| t.to_element()).collect(),
            ),
            IrElement::block("return", vec![ret.to_element()]),
        ];
        elements.extend(body);

        IrElement::block("func", elements)
    }

    pub fn d_type(name: impl Into<String>, typ: IrType) -> IrElement {
        IrElement::block(
            "type",
            vec![IrElement::ident(name.into()), typ.to_element()],
        )
    }

    // returns if the expr can be passed to element_addr
    pub fn is_address_expr(&self) -> bool {
        match self {
            IrElement::Term(IrTerm::Ident(_)) => true,
            IrElement::Term(IrTerm::Argument(_)) => true,
            IrElement::Block(block) => {
                block.name == "deref"
                    || block.name == "offset"
                    || block.name == "addr_offset"
                    || block.name == "index"
                    || block.name == "addr_index"
                    || block.name == "address"
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrSingleType {
    Nil,
    Bool,
    Int,
    Address(Box<IrType>),
    Fn(Vec<IrType>, Box<IrType>),
    Byte,
    BoxedArray(Box<IrType>), // can be treated as (address T)
}

impl IrSingleType {
    pub fn to_element(&self) -> IrElement {
        match self {
            IrSingleType::Nil => IrElement::ident("nil"),
            IrSingleType::Bool => IrElement::ident("bool"),
            IrSingleType::Int => IrElement::ident("int"),
            IrSingleType::Address(t) => IrElement::block("address", vec![t.to_element()]),
            IrSingleType::Fn(args, ret) => IrElement::block(
                "fn",
                vec![
                    IrElement::block("args", args.iter().map(|t| t.to_element()).collect()),
                    ret.to_element(),
                ],
            ),
            IrSingleType::Byte => IrElement::ident("byte"),
            IrSingleType::BoxedArray(t) => IrElement::block("array", vec![t.to_element()]),
        }
    }

    pub fn unify(self, want: IrSingleType) -> Result<IrSingleType> {
        match (self, want) {
            (IrSingleType::Nil, IrSingleType::Nil) => Ok(IrSingleType::Nil),
            (IrSingleType::Bool, IrSingleType::Bool) => Ok(IrSingleType::Bool),
            (IrSingleType::Int, IrSingleType::Int) => Ok(IrSingleType::Int),
            (IrSingleType::Byte, IrSingleType::Byte) => Ok(IrSingleType::Byte),
            (IrSingleType::Address(t), IrSingleType::Address(u)) => {
                let unified = t.unify(u.as_ref().clone())?;

                Ok(IrSingleType::Address(Box::new(unified)))
            }
            (IrSingleType::Fn(args1, ret1), IrSingleType::Fn(args2, ret2)) => {
                // FIXME: Currenty we don't support generic functions
                if args1
                    .iter()
                    .filter(|t| t.is_typetag())
                    .collect::<Vec<_>>()
                    .len()
                    != args2
                        .iter()
                        .filter(|t| t.is_typetag())
                        .collect::<Vec<_>>()
                        .len()
                {
                    bail!(
                        "function arity mismatch, {} vs {}",
                        args1.len(),
                        args2.len()
                    );
                }

                let mut args = Vec::new();
                for (t, u) in args1.iter().zip(args2.iter()) {
                    let unified = t.clone().unify(u.clone())?;

                    args.push(unified);
                }

                let unified = ret1.unify(ret2.as_ref().clone())?;

                Ok(IrSingleType::Fn(args, Box::new(unified)))
            }
            (IrSingleType::BoxedArray(t), IrSingleType::BoxedArray(u)) => {
                let unified = t.unify(u.as_ref().clone())?;

                Ok(IrSingleType::BoxedArray(Box::new(unified)))
            }
            // nil can be an address
            (IrSingleType::Nil, IrSingleType::Address(t)) => Ok(IrSingleType::Address(t)),
            (IrSingleType::Address(t), IrSingleType::Nil) => Ok(IrSingleType::Address(t)),
            // nil can be a byte
            (IrSingleType::Nil, IrSingleType::Byte) => Ok(IrSingleType::Byte),
            (IrSingleType::Byte, IrSingleType::Nil) => Ok(IrSingleType::Byte),
            // byte can be an address
            (IrSingleType::Byte, IrSingleType::Address(t)) => Ok(IrSingleType::Address(t)),
            (IrSingleType::Address(t), IrSingleType::Byte) => Ok(IrSingleType::Address(t)),
            (s, t) => {
                bail!(
                    "Type want {} but got {}",
                    t.to_element().show_compact(),
                    s.to_element().show_compact(),
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Unknown,
    Single(IrSingleType),
    Tuple(Vec<IrType>),
    Slice(usize, Box<IrType>),
    Ident(String),
    Generic(Vec<String>, Box<IrType>),
    TypeApp(Box<IrType>, Vec<IrType>),
    TypeArgument(usize),
    TypeTag,
}

impl IrType {
    pub fn unknown() -> IrType {
        IrType::Unknown
    }

    pub fn nil() -> IrType {
        IrType::Single(IrSingleType::Nil)
    }

    pub fn bool() -> IrType {
        IrType::Single(IrSingleType::Bool)
    }

    pub fn int() -> IrType {
        IrType::Single(IrSingleType::Int)
    }

    pub fn byte() -> IrType {
        IrType::Single(IrSingleType::Byte)
    }

    pub fn addr_of(t: IrType) -> IrType {
        IrType::Single(IrSingleType::Address(Box::new(t)))
    }

    pub fn addr_unknown() -> IrType {
        IrType::Single(IrSingleType::Address(Box::new(IrType::unknown())))
    }

    pub fn func(args: Vec<IrType>, ret: IrType) -> IrType {
        IrType::Single(IrSingleType::Fn(args, Box::new(ret)))
    }

    pub fn tuple(args: Vec<IrType>) -> IrType {
        IrType::Tuple(args)
    }

    pub fn slice(size: usize, typ: Box<IrType>) -> IrType {
        IrType::Slice(size, typ)
    }

    pub fn boxed_array(t: IrType) -> IrType {
        IrType::Single(IrSingleType::BoxedArray(Box::new(t)))
    }

    pub fn generic(typ: IrType, params: Vec<String>) -> IrType {
        IrType::Generic(params, Box::new(typ))
    }

    pub fn typeapp(typ: IrType, args: Vec<IrType>) -> IrType {
        IrType::TypeApp(Box::new(typ), args)
    }

    pub fn typetag() -> IrType {
        IrType::TypeTag
    }

    pub fn from_element(element: &IrElement) -> Result<IrType> {
        Ok(match element {
            IrElement::Term(t) => match t {
                IrTerm::Ident(ident) => match ident.as_str() {
                    "unknown" => IrType::unknown(),
                    "nil" => IrType::nil(),
                    "bool" => IrType::bool(),
                    "int" => IrType::int(),
                    "byte" => IrType::byte(),
                    t => IrType::Ident(t.to_string()),
                },
                IrTerm::Argument(u) => IrType::TypeArgument(*u),
                t => unreachable!("{:?}", t),
            },
            IrElement::Block(block) => match block.name.as_str() {
                "tuple" => {
                    let mut types = Vec::new();
                    for element in block.elements.iter() {
                        types.push(IrType::from_element(element)?);
                    }
                    IrType::tuple(types)
                }
                "slice" => IrType::slice(
                    block.elements[0].clone().into_term()?.into_int()? as usize,
                    Box::new(IrType::from_element(&block.elements[1])?),
                ),
                "address" => IrType::addr_of(IrType::from_element(&block.elements[0])?),
                "array" => IrType::boxed_array(IrType::from_element(&block.elements[0])?),
                "generic" => {
                    let mut params = Vec::new();
                    for element in block.elements.iter().skip(1) {
                        params.push(element.clone().into_term()?.into_ident()?);
                    }
                    IrType::generic(IrType::from_element(&block.elements[0])?, params)
                }
                "typeapp" => {
                    let mut args = Vec::new();
                    for element in block.elements.iter().skip(1) {
                        args.push(IrType::from_element(element)?);
                    }
                    IrType::typeapp(IrType::from_element(&block.elements[0])?, args)
                }
                t => unreachable!("{:?}", t),
            },
        })
    }

    pub fn from_type_ast(
        typ: &Type,
        structs: &Structs,
        typevar_walker: &impl Fn(&mut String) -> IrType,
    ) -> Result<IrType> {
        Ok(match typ {
            Type::Nil => IrType::nil(),
            Type::Bool => IrType::bool(),
            Type::Int => IrType::int(),
            Type::Byte => IrType::byte(),
            Type::Fn(_, _, _) => todo!(),
            Type::Method(_, _, _) => todo!(),
            Type::Struct(s) if s == "string" => {
                // string = array[byte]
                IrType::from_type_ast(&Type::Array(Box::new(Type::Byte)), structs, typevar_walker)?
            }
            Type::Struct(t) => IrType::Ident(t.clone()),
            Type::Ref(t) => IrType::addr_of(IrType::from_type_ast(t, structs, typevar_walker)?),
            Type::Array(t) => IrType::addr_of(IrType::boxed_array(IrType::from_type_ast(
                t,
                structs,
                typevar_walker,
            )?)),
            Type::SizedArray(t, u) => IrType::slice(
                *u,
                Box::new(IrType::from_type_ast(t.as_ref(), structs, typevar_walker)?),
            ),
            Type::Optional(t) => {
                IrType::addr_of(IrType::from_type_ast(t, structs, typevar_walker)?)
            }
            Type::Self_ => todo!(),
            Type::Any => IrType::byte(),
            Type::TypeVar(t) => IrType::Ident(t.clone()),
            Type::TypeApp(t, ps) => IrType::typeapp(
                IrType::from_type_ast(t, structs, typevar_walker)?,
                ps.into_iter()
                    .map(|t| IrType::from_type_ast(t, structs, typevar_walker))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            t => bail!("Unsupported type: {:?}", t),
        })
    }

    pub fn to_element(&self) -> IrElement {
        match self {
            IrType::Unknown => IrElement::ident("unknown"),
            IrType::Single(s) => s.to_element(),
            IrType::Tuple(ts) => {
                let mut elements = vec![];
                for t in ts {
                    elements.push(t.to_element());
                }

                IrElement::block("tuple", elements)
            }
            IrType::Slice(u, t) => {
                let mut elements = vec![];
                elements.push(IrElement::int(*u as i32));
                elements.push(t.to_element());
                IrElement::block("slice", elements)
            }
            IrType::Ident(t) => IrElement::ident(t),
            IrType::Generic(ps, t) => {
                let mut params = vec![];
                params.push(t.to_element());
                params.extend(
                    ps.into_iter()
                        .map(|u| IrElement::ident(u))
                        .collect::<Vec<_>>(),
                );

                IrElement::block("generic", params)
            }
            IrType::TypeApp(t, ps) => {
                let mut params = vec![];
                params.push(t.to_element());
                params.extend(ps.into_iter().map(|u| u.to_element()).collect::<Vec<_>>());

                IrElement::block("typeapp", params)
            }
            IrType::TypeTag => IrElement::ident("typetag"),
            IrType::TypeArgument(t) => IrElement::Term(IrTerm::Argument(*t)),
        }
    }

    fn subst_typevar(&mut self, var: String, typ: IrType) {
        match self {
            IrType::Unknown => {}
            IrType::Single(t) => match t {
                IrSingleType::Nil => {}
                IrSingleType::Bool => {}
                IrSingleType::Int => {}
                IrSingleType::Address(t) => {
                    t.subst_typevar(var, typ);
                }
                IrSingleType::BoxedArray(t) => {
                    t.subst_typevar(var, typ);
                }
                _ => unreachable!(),
            },
            IrType::Tuple(ts) => {
                for t in ts {
                    t.subst_typevar(var.clone(), typ.clone());
                }
            }
            IrType::Slice(_, t) => t.subst_typevar(var, typ),
            IrType::Ident(ident) => {
                if ident == &var {
                    *self = typ;
                }
            }
            IrType::Generic(ps, t) => {
                if ps.contains(&var) {
                    return;
                }
                t.subst_typevar(var, typ);
            }
            IrType::TypeApp(t, ps) => {
                t.subst_typevar(var.clone(), typ.clone());
                for p in ps {
                    p.subst_typevar(var.clone(), typ.clone());
                }
            }
            IrType::TypeTag => {}
            IrType::TypeArgument(_) => {}
        }
    }

    pub fn walk_ident(&mut self, walker: impl Fn(&mut String) -> IrType) {
        match self {
            IrType::Unknown => {}
            IrType::Single(t) => match t {
                IrSingleType::Nil => {}
                IrSingleType::Bool => {}
                IrSingleType::Int => {}
                IrSingleType::Address(t) => t.walk_ident(walker),
                IrSingleType::BoxedArray(t) => t.walk_ident(walker),
                _ => unreachable!(),
            },
            IrType::Tuple(ts) => {
                for t in ts {
                    t.walk_ident(&walker);
                }
            }
            IrType::Slice(_, t) => t.walk_ident(walker),
            IrType::Ident(ident) => {
                *self = walker(ident);
            }
            IrType::Generic(_, t) => {
                t.walk_ident(walker);
            }
            IrType::TypeApp(t, ps) => {
                t.walk_ident(&walker);
                for p in ps {
                    p.walk_ident(&walker);
                }
            }
            IrType::TypeTag => {}
            IrType::TypeArgument(_) => todo!(),
        }
    }

    pub fn size_of(&self, types: &HashMap<String, IrType>) -> Result<usize> {
        match self {
            IrType::Unknown => bail!("Cannot determine size of unknown type",),
            IrType::Single(_) => Ok(1),
            IrType::Tuple(vs) => Ok(vs
                .into_iter()
                .map(|v| v.size_of(types))
                .collect::<Result<Vec<_>>>()?
                .iter()
                .sum::<usize>()
                + 1), // +1 for a pointer to info table
            IrType::Slice(len, t) => Ok(len * t.size_of(types)? + 1),
            IrType::Ident(t) => {
                // FIXME: Really?
                if t == "typetag" {
                    return Ok(1);
                }

                types
                    .get(t)
                    .ok_or(anyhow::anyhow!(
                        "Cannot determine size of type {}, because it is not defined",
                        t
                    ))?
                    .size_of(types)
            }
            IrType::Generic(_, _) => {
                bail!("Cannot determine size of generic type, because it is not instantiated")
            }
            IrType::TypeApp(t, ps) => match t.as_ref() {
                IrType::Ident(i) => {
                    let t = types
                        .get(i)
                        .ok_or(anyhow::anyhow!(
                            "Cannot determine size of type {}, because it is not defined",
                            i
                        ))?
                        .clone();
                    match t {
                        IrType::Generic(qs, gen) => {
                            assert_eq!(qs.len(), ps.len());

                            let mut gen = gen.clone();
                            for (q, p) in qs.into_iter().zip(ps.into_iter()) {
                                gen.subst_typevar(q, p.clone());
                            }

                            gen.size_of(types)
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            IrType::TypeTag => Ok(1),
            // Generics will be a pointer in runtime phase, so size is same as a pointer
            IrType::TypeArgument(_t) => Ok(1),
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            IrType::Unknown => true,
            _ => false,
        }
    }

    pub fn is_typetag(&self) -> bool {
        match self {
            IrType::TypeTag => true,
            _ => false,
        }
    }

    pub fn as_addr(&self) -> Result<Box<IrType>> {
        match self {
            IrType::Single(IrSingleType::Address(t)) => Ok(t.clone()),
            _ => bail!("{:?} is not address", self),
        }
    }

    pub fn as_func(&self) -> Option<(Vec<IrType>, Box<IrType>)> {
        match self {
            IrType::Single(IrSingleType::Fn(args, ret)) => Some((args.clone(), ret.clone())),
            _ => None,
        }
    }

    pub fn as_slice(&self) -> Option<(usize, Box<IrType>)> {
        match self {
            IrType::Slice(len, t) => Some((*len, t.clone())),
            _ => None,
        }
    }

    pub fn as_element(&self) -> Option<IrType> {
        match self {
            IrType::Tuple(ts) if ts.len() == 1 => Some(ts[0].as_addr().unwrap().as_ref().clone()),
            _ => None,
        }
    }

    pub fn as_array_element(&self) -> Option<IrType> {
        match self {
            IrType::Single(IrSingleType::BoxedArray(t)) => Some(t.as_ref().clone()),
            IrType::Slice(_, t) => Some(t.as_ref().clone()),
            _ => None,
        }
    }

    pub fn unify(self, want: IrType) -> Result<IrType> {
        match (self, want) {
            (s, t) if s == t => Ok(s),
            (IrType::Unknown, t) => Ok(t),
            (s, IrType::Unknown) => Ok(s),
            (IrType::Single(s), IrType::Single(t)) => Ok(IrType::Single(s.unify(t)?)),
            (IrType::Tuple(ts), Self::Tuple(vs)) => {
                if ts.len() != vs.len() {
                    bail!("{:?} and {:?} are not unifiable", ts, vs);
                }

                let mut result = vec![];
                for (t, s) in ts.into_iter().zip(vs) {
                    result.push(t.unify(s)?);
                }

                Ok(IrType::Tuple(result))
            }
            // slice as an address
            (IrType::Slice(_, _), IrType::Single(IrSingleType::Address(s))) => {
                Ok(IrType::Single(IrSingleType::Address(s)))
            }
            // FIXME: Currently, we don't support type argument
            (IrType::TypeArgument(_), t) => Ok(t),
            (s, IrType::TypeArgument(_)) => Ok(s),
            (s, t) => {
                bail!(
                    "Type want {} but got {}",
                    t.to_element().show_compact(),
                    s.to_element().show_compact(),
                )
            }
        }
    }

    pub fn offset(self, index: usize, types: &HashMap<String, IrType>) -> Result<IrType> {
        match self {
            IrType::Single(IrSingleType::Address(t)) => Ok(t.as_ref().clone()),
            IrType::Slice(r, t) => {
                if index < r {
                    Ok(t.as_ref().clone())
                } else {
                    bail!("Out of offset, {} in {:?}", index, IrType::Slice(r, t))
                }
            }
            IrType::Tuple(ts) => {
                if index < ts.len() {
                    Ok(ts[index].clone())
                } else {
                    bail!("Out of offset, {} in {:?}", index, IrType::Tuple(ts))
                }
            }
            IrType::Ident(t) => {
                let ty = types[&t].clone();
                ty.offset(index, types)
            }
            IrType::TypeApp(t, ps) => match t.as_ref() {
                IrType::Ident(i) => {
                    let t = types
                        .get(i)
                        .ok_or(anyhow::anyhow!(
                            "Cannot determine size of type {}, because it is not defined",
                            i
                        ))?
                        .clone();
                    match t {
                        IrType::Generic(qs, gen) => {
                            assert_eq!(qs.len(), ps.len());

                            let mut gen = gen.clone();
                            for (q, p) in qs.into_iter().zip(ps.into_iter()) {
                                gen.subst_typevar(q, p);
                            }

                            gen.offset(index, types)
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            t => bail!("Type {} is not address", t.to_element().show()),
        }
    }

    pub fn offset_in_words(self, index: usize, types: &HashMap<String, IrType>) -> Result<usize> {
        let mut result = 1;
        for i in 0..index {
            result += self.clone().offset(i, types)?.size_of(types)?;
        }

        Ok(result)
    }
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());

#[derive(PartialEq, Debug, Clone)]
enum IrLexeme {
    Ident(String), // $ident
    Keyword(String),
    Argument(usize),
    Number(String),
    LParen,
    RParen,
}

fn run_lexer(input: &str) -> Vec<IrLexeme> {
    let mut tokens = vec![];
    let mut position = 0;

    while position < input.len() {
        if let Some(m) = SPACE_PATTERN.find(&input[position..]) {
            position += m.end();
            continue;
        }

        if &input[position..position + 1] == "(" {
            tokens.push(IrLexeme::LParen);
            position += 1;
            continue;
        }

        if &input[position..position + 1] == ")" {
            tokens.push(IrLexeme::RParen);
            position += 1;
            continue;
        }

        if let Some(m) = NUMBER_PATTERN.find(&input[position..]) {
            tokens.push(IrLexeme::Number(m.as_str().to_string()));

            position += m.end();
            continue;
        }

        if &input[position..position + 1] == "$" {
            if let Some(m) = NUMBER_PATTERN.find(&input[position + 1..]) {
                let index = m.as_str().parse::<usize>().unwrap();
                let token = IrLexeme::Argument(index);
                position += m.end() + 1;

                tokens.push(token);

                continue;
            }

            if let Some(m) = IDENT_PATTERN.find(&input[position + 1..]) {
                let ident = m.as_str().to_string();
                let token = IrLexeme::Ident(ident.clone());
                position += m.end() + 1;

                tokens.push(token);

                continue;
            }

            unreachable!("{:?}", &input[position..position + 20]);
        }

        if let Some(m) = IDENT_PATTERN.find(&input[position..]) {
            let name = m.as_str();
            tokens.push(IrLexeme::Keyword(name.to_string()));

            position += m.end();
            continue;
        }

        break;
    }

    tokens
}

struct IrParser<'s> {
    position: usize,
    tokens: &'s [IrLexeme],
}

impl IrParser<'_> {
    fn next(&mut self) -> &IrLexeme {
        let token = &self.tokens[self.position];
        self.position += 1;

        token
    }

    fn expect(&mut self, lexeme: IrLexeme) -> Result<()> {
        if self.tokens[self.position] == lexeme {
            self.position += 1;
            return Ok(());
        } else {
            bail!(
                "Expected {:?} but got {:?}",
                lexeme,
                self.tokens[self.position]
            );
        }
    }

    fn term(&mut self) -> Result<IrTerm> {
        let token = self.next();

        Ok(match token {
            IrLexeme::Ident(ident) => IrTerm::Ident(ident.to_string()),
            IrLexeme::Argument(arg) => IrTerm::Argument(*arg),
            IrLexeme::Keyword(ident) => {
                if ident == "nil" {
                    IrTerm::Nil
                } else if ident == "true" {
                    IrTerm::Bool(true)
                } else if ident == "false" {
                    IrTerm::Bool(false)
                } else {
                    bail!("Unknown keyword {:?}", ident);
                }
            }
            IrLexeme::Number(n) => {
                if let Ok(d) = n.parse::<i32>() {
                    IrTerm::Int(d)
                } else {
                    bail!("Invalid number {:?}", n);
                }
            }
            token => unreachable!("{:?}", token),
        })
    }

    fn element(&mut self) -> Result<IrElement> {
        if self.expect(IrLexeme::LParen).is_ok() {
            let name = match self.next() {
                IrLexeme::Keyword(i) => i.to_string(),
                _ => unreachable!(),
            };
            let mut elements = vec![];

            while self.tokens[self.position] != IrLexeme::RParen {
                elements.push(self.element()?);
            }

            self.expect(IrLexeme::RParen)?;

            Ok(IrElement::Block(IrBlock { name, elements }))
        } else {
            let term = self.term()?;

            Ok(IrElement::Term(term))
        }
    }
}

pub fn parse_ir(input: &str) -> Result<IrElement> {
    let tokens = run_lexer(input);
    let mut parser = IrParser {
        position: 0,
        tokens: &tokens,
    };

    parser.element()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_run_lexer() {
        let cases = vec![(
            r#"
(module
    (func $main (
        (let $x 10)
        (assign $x 20)
        (return $x)
    ))
)
"#,
            vec![
                IrLexeme::LParen,
                IrLexeme::Keyword("module".to_string()),
                IrLexeme::LParen,
                IrLexeme::Keyword("func".to_string()),
                IrLexeme::Ident("main".to_string()),
                IrLexeme::LParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("let".to_string()),
                IrLexeme::Ident("x".to_string()),
                IrLexeme::Number("10".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("assign".to_string()),
                IrLexeme::Ident("x".to_string()),
                IrLexeme::Number("20".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("return".to_string()),
                IrLexeme::Ident("x".to_string()),
                IrLexeme::RParen,
                IrLexeme::RParen,
                IrLexeme::RParen,
                IrLexeme::RParen,
            ],
        )];

        for (input, result) in cases {
            assert_eq!(result, run_lexer(input));
        }
    }

    #[test]
    fn test_parse_ir() {
        let cases = vec![(
            r#"
(module
    (func $main
        (let $x 10)
        (assign $x 20)
        (return $x)
    )
)
"#,
            IrElement::Block(IrBlock {
                name: "module".to_string(),
                elements: vec![IrElement::Block(IrBlock {
                    name: "func".to_string(),
                    elements: vec![
                        IrElement::Term(IrTerm::Ident("main".to_string())),
                        IrElement::Block(IrBlock {
                            name: "let".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string())),
                                IrElement::Term(IrTerm::Int(10)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "assign".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string())),
                                IrElement::Term(IrTerm::Int(20)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "return".to_string(),
                            elements: vec![IrElement::Term(IrTerm::Ident("x".to_string()))],
                        }),
                    ],
                })],
            }),
        )];

        for (input, result) in cases {
            let ast = parse_ir(input);

            assert!(ast.is_ok(), "Error:{:?}\n{}", ast, input);
            assert_eq!(result, ast.unwrap(), "{}", input);
        }
    }
}
