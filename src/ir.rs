use anyhow::{bail, Result};

use crate::{ast::Type, util::sexpr_writer::SExprWriter};

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    I32(i32),
    Ident(String),
    Func {
        name: String,
        params: Vec<(String, IrType)>,
        result: Box<IrType>,
        body: Vec<IrTerm>,
    },
    GlobalLet {
        name: String,
        type_: IrType,
        value: Box<IrTerm>,
    },
    Call {
        callee: Box<IrTerm>,
        args: Vec<IrTerm>,
    },
    Seq {
        elements: Vec<IrTerm>,
    },
    Let {
        name: String,
        type_: IrType,
        value: Box<IrTerm>,
    },
    Return {
        value: Box<IrTerm>,
    },
    Assign {
        lhs: String,
        rhs: Box<IrTerm>,
    },
    If {
        cond: Box<IrTerm>,
        type_: IrType,
        then: Box<IrTerm>,
        else_: Box<IrTerm>,
    },
    SetField {
        address: Box<IrTerm>,
        offset: usize,
        value: Box<IrTerm>,
    },
    GetField {
        address: Box<IrTerm>,
        offset: usize,
    },
    PointerAt {
        address: Box<IrTerm>,
        offset: Box<IrTerm>,
    },
    SetPointer {
        address: Box<IrTerm>,
        value: Box<IrTerm>,
    },
    While {
        cond: Box<IrTerm>,
        body: Box<IrTerm>,
    },
    SizeOf {
        type_: IrType,
    },
    Module {
        elements: Vec<IrTerm>,
    },
}

impl IrTerm {
    pub fn nil() -> Self {
        IrTerm::Nil
    }

    pub fn ident(name: impl Into<String>) -> Self {
        IrTerm::Ident(name.into())
    }

    pub fn i32(value: i32) -> Self {
        IrTerm::I32(value)
    }

    fn to_string_writer(&self, writer: &mut SExprWriter) {
        match self {
            IrTerm::Nil => {
                writer.write("nil");
            }
            IrTerm::I32(i) => {
                writer.write(&i.to_string());
            }
            IrTerm::Ident(p) => {
                writer.write(p);
            }
            IrTerm::Func {
                name,
                params,
                result,
                body,
            } => {
                writer.start();
                writer.write("func");
                writer.write(name);

                for (name, type_) in params {
                    writer.start();
                    writer.write("param");
                    writer.write(name);
                    type_.to_term().to_string_writer(writer);
                    writer.end();
                }

                writer.start();
                writer.write("result");
                result.to_term().to_string_writer(writer);
                writer.end();

                for term in body {
                    term.to_string_writer(writer);
                }
                writer.end();
            }
            IrTerm::GlobalLet { name, type_, value } => {
                writer.start();
                writer.write("global-let");
                writer.write(name);
                type_.to_term().to_string_writer(writer);
                value.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Call { callee, args } => {
                writer.start();
                writer.write("call");
                callee.to_string_writer(writer);
                for arg in args {
                    arg.to_string_writer(writer);
                }
                writer.end();
            }
            IrTerm::Seq { elements } => {
                writer.start();
                writer.write("seq");
                for element in elements {
                    element.to_string_writer(writer);
                }
                writer.end();
            }
            IrTerm::Let { name, type_, value } => {
                writer.start();
                writer.write("let");
                writer.write(name);
                type_.to_term().to_string_writer(writer);
                value.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Return { value } => {
                writer.start();
                writer.write("return");
                value.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Assign { lhs, rhs } => {
                writer.start();
                writer.write("assign");
                writer.write(lhs);
                rhs.to_string_writer(writer);
                writer.end();
            }
            IrTerm::If {
                cond,
                type_,
                then,
                else_,
            } => {
                writer.start();
                writer.write("if");
                cond.to_string_writer(writer);
                type_.to_term().to_string_writer(writer);
                then.to_string_writer(writer);
                else_.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Module { elements } => {
                writer.start();
                writer.write("module");
                for element in elements {
                    element.to_string_writer(writer);
                }
                writer.end();
            }
            IrTerm::SetField {
                address,
                offset,
                value,
            } => {
                writer.start();
                writer.write("set-field");
                address.to_string_writer(writer);
                writer.write(&offset.to_string());
                value.to_string_writer(writer);
                writer.end();
            }
            IrTerm::GetField { address, offset } => {
                writer.start();
                writer.write("get-field");
                address.to_string_writer(writer);
                writer.write(&offset.to_string());
                writer.end();
            }
            IrTerm::While { cond, body } => {
                writer.start();
                writer.write("while");
                cond.to_string_writer(writer);
                body.to_string_writer(writer);
                writer.end();
            }
            IrTerm::PointerAt { address, offset } => {
                writer.start();
                writer.write("pointer-at");
                address.to_string_writer(writer);
                offset.to_string_writer(writer);
                writer.end();
            }
            IrTerm::SetPointer { address, value } => {
                writer.start();
                writer.write("set-pointer");
                address.to_string_writer(writer);
                value.to_string_writer(writer);
                writer.end();
            }
            IrTerm::SizeOf { type_ } => {
                writer.start();
                writer.write("size-of");
                type_.to_term().to_string_writer(writer);
                writer.end();
            }
        }
    }

    pub fn find_let(&self) -> Vec<(String, IrType, Box<IrTerm>)> {
        match self {
            IrTerm::Nil => vec![],
            IrTerm::I32(_) => vec![],
            IrTerm::Ident(_) => vec![],
            IrTerm::Call { callee: _, args } => {
                let mut result = vec![];
                for arg in args {
                    result.extend(arg.find_let());
                }
                result
            }
            IrTerm::Seq { elements } => {
                let mut result = vec![];
                for element in elements {
                    result.extend(element.find_let());
                }
                result
            }
            IrTerm::Let { name, type_, value } => {
                let mut result = vec![];
                result.push((name.clone(), type_.clone(), value.clone()));
                result.extend(value.find_let());
                result
            }
            IrTerm::Return { value } => {
                let mut result = vec![];
                result.extend(value.find_let());
                result
            }
            IrTerm::Assign { lhs, rhs } => {
                let mut result = vec![];
                result.extend(rhs.find_let());
                result
            }
            IrTerm::If {
                cond,
                type_,
                then,
                else_,
            } => {
                let mut result = vec![];
                result.extend(cond.find_let());
                result.extend(then.find_let());
                result.extend(else_.find_let());
                result
            }
            IrTerm::Module { elements } => todo!(),
            _ => vec![],
        }
    }

    pub fn to_string(&self) -> String {
        let mut writer = SExprWriter::new();
        self.to_string_writer(&mut writer);

        writer.buffer
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrType {
    Nil,
    I32,
    Address,
}

impl IrType {
    pub fn from_type(type_: &Type) -> Result<Self> {
        match type_ {
            Type::Nil => Ok(IrType::Nil),
            Type::I32 => Ok(IrType::I32),
            Type::Record(_) => Ok(IrType::Address),
            Type::Ident(_) => Ok(IrType::Address), // FIXME: could be other types
            Type::Pointer(_) => Ok(IrType::Address),
            Type::Array(_, _) => Ok(IrType::Address),
            _ => bail!("unknown type {}", type_.to_string()),
        }
    }

    pub fn to_term(&self) -> IrTerm {
        match self {
            IrType::Nil => IrTerm::nil(),
            IrType::I32 => IrTerm::Ident("i32".to_string()),
            IrType::Address => IrTerm::Ident("i32".to_string()),
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            IrType::Nil => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        self.to_term().to_string()
    }

    pub fn sizeof(&self) -> usize {
        match self {
            IrType::Nil => 32,
            IrType::I32 => 32,
            IrType::Address => 32,
        }
    }
}
