use anyhow::{bail, Result};

use crate::ast::Type;

#[derive(PartialEq, Debug, Clone)]
pub enum IrElement {
    Term(IrTerm),
    Block(IrBlock),
}

impl IrElement {
    pub fn nil() -> Self {
        IrElement::Term(IrTerm::Nil)
    }

    pub fn ident(name: impl Into<String>) -> Self {
        IrElement::Term(IrTerm::Ident(name.into()))
    }

    pub fn i32(value: i32) -> Self {
        IrElement::Term(IrTerm::I32(value))
    }

    pub fn block(name: impl Into<String>, elements: Vec<IrElement>) -> Self {
        IrElement::Block(IrBlock {
            name: name.into(),
            elements,
        })
    }

    pub fn i_func(name: impl Into<String>, elements: Vec<IrElement>) -> Self {
        IrElement::block(
            "func",
            vec![IrElement::ident(name), IrElement::block("body", elements)],
        )
    }

    pub fn i_global_let(name: impl Into<String>, type_: IrType, value: IrElement) -> Self {
        IrElement::block(
            "global_let",
            vec![
                IrElement::ident(name),
                IrElement::Term(type_.to_term()),
                IrElement::block("value", vec![value]),
            ],
        )
    }

    pub fn i_call(name: impl Into<String>, args: Vec<IrElement>) -> Self {
        IrElement::block(
            "call",
            vec![IrElement::ident(name), IrElement::block("args", args)],
        )
    }

    pub fn i_seq(elements: Vec<IrElement>) -> Self {
        IrElement::block("seq", elements)
    }

    pub fn i_let(name: impl Into<String>, type_: IrType, value: IrElement) -> Self {
        IrElement::block(
            "let",
            vec![
                IrElement::ident(name),
                IrElement::Term(type_.to_term()),
                value,
            ],
        )
    }

    pub fn i_return(value: IrElement) -> Self {
        IrElement::block("return", vec![value])
    }

    pub fn i_assign_local(lhs: IrElement, rhs: IrElement) -> Self {
        IrElement::block("assign_local", vec![lhs, rhs])
    }

    pub fn i_assign_global(lhs: IrElement, rhs: IrElement) -> Self {
        IrElement::block("assign_global", vec![lhs, rhs])
    }

    pub fn i_if(cond: IrElement, type_: IrType, then: IrElement, else_: IrElement) -> Self {
        IrElement::block(
            "if",
            vec![
                cond,
                IrElement::Term(type_.to_term()),
                IrElement::block("then", vec![then]),
                else_,
            ],
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    I32(i32),
    Address(usize),
    Ident(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct IrBlock {
    pub name: String,
    pub elements: Vec<IrElement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrType {
    I32,
    Address,
}

impl IrType {
    pub fn from_type(type_: &Type) -> Result<Self> {
        match type_ {
            Type::I32 => Ok(IrType::I32),
            Type::Record(_) => Ok(IrType::Address),
            Type::Ident(_) => Ok(IrType::Address),
            _ => bail!("unknown type {}", type_.to_string()),
        }
    }

    pub fn to_term(&self) -> IrTerm {
        match self {
            IrType::I32 => IrTerm::Ident("i32".to_string()),
            IrType::Address => IrTerm::Ident("address".to_string()),
        }
    }
}
