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
        name: String,
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
    AssignLocal {
        lhs: Box<IrTerm>,
        rhs: Box<IrTerm>,
    },
    AssignGlobal {
        lhs: Box<IrTerm>,
        rhs: Box<IrTerm>,
    },
    If {
        cond: Box<IrTerm>,
        type_: IrType,
        then: Box<IrTerm>,
        else_: Box<IrTerm>,
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
            IrTerm::Call { name, args } => {
                writer.start();
                writer.write("call");
                writer.write(name);
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
            IrTerm::AssignLocal { lhs, rhs } => {
                writer.start();
                writer.write("assign-local");
                lhs.to_string_writer(writer);
                rhs.to_string_writer(writer);
                writer.end();
            }
            IrTerm::AssignGlobal { lhs, rhs } => {
                writer.start();
                writer.write("assign-global");
                lhs.to_string_writer(writer);
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
            Type::Ident(_) => Ok(IrType::Address),
            _ => bail!("unknown type {}", type_.to_string()),
        }
    }

    pub fn to_term(&self) -> IrTerm {
        match self {
            IrType::Nil => IrTerm::nil(),
            IrType::I32 => IrTerm::Ident("i32".to_string()),
            IrType::Address => IrTerm::Ident("address".to_string()),
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
}
