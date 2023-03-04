use anyhow::{bail, Result};

use crate::{ast::Type, compiler::SourcePosition, util::sexpr_writer::SExprWriter};

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    I32(i32),
    I64(i64),
    Ident(String),
    String(usize),
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
        source: Option<SourcePosition>,
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
    While {
        cond: Box<IrTerm>,
        body: Box<IrTerm>,
        cleanup: Option<Box<IrTerm>>,
    },
    SizeOf {
        type_: IrType,
    },
    WriteMemory {
        type_: IrType,
        address: Box<IrTerm>,
        value: Vec<IrTerm>,
    },
    Module {
        elements: Vec<IrTerm>,
    },
    Continue,
    Break,
    Discard {
        element: Box<IrTerm>,
    },
    And {
        lhs: Box<IrTerm>,
        rhs: Box<IrTerm>,
    },
    Or {
        lhs: Box<IrTerm>,
        rhs: Box<IrTerm>,
    },
    Store {
        type_: IrType,
        address: Box<IrTerm>,
        offset: Box<IrTerm>,
        value: Box<IrTerm>,
    },
    Load {
        type_: IrType,
        address: Box<IrTerm>,
        offset: Box<IrTerm>,
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

    pub fn i64(value: i64) -> Self {
        IrTerm::I64(value)
    }

    fn to_string_writer(&self, writer: &mut SExprWriter) {
        match self {
            IrTerm::Nil => {
                writer.write("nil");
            }
            IrTerm::I32(i) => {
                writer.write(&i.to_string());
            }
            IrTerm::I64(i) => {
                writer.write(&i.to_string());
            }
            IrTerm::Ident(p) => {
                writer.write(p);
            }
            IrTerm::String(p) => {
                writer.start();
                writer.write("string");
                writer.write(&p.to_string());
                writer.end();
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
            IrTerm::Call { callee, args, .. } => {
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
            IrTerm::While {
                cond,
                body,
                cleanup,
            } => {
                writer.start();
                writer.write("while");
                cond.to_string_writer(writer);
                body.to_string_writer(writer);
                if let Some(cleanup) = cleanup {
                    cleanup.to_string_writer(writer);
                }
                writer.end();
            }
            IrTerm::SizeOf { type_ } => {
                writer.start();
                writer.write("size-of");
                type_.to_term().to_string_writer(writer);
                writer.end();
            }
            IrTerm::WriteMemory {
                type_,
                address,
                value,
            } => {
                writer.start();
                writer.write("write-memory");
                type_.to_term().to_string_writer(writer);
                address.to_string_writer(writer);

                writer.start();
                writer.write("value");
                for v in value {
                    v.to_string_writer(writer);
                }
                writer.end();
                writer.end();
            }
            IrTerm::Continue => {
                writer.start();
                writer.write("continue");
                writer.end();
            }
            IrTerm::Break => {
                writer.start();
                writer.write("break");
                writer.end();
            }
            IrTerm::Discard { element } => {
                writer.start();
                writer.write("discard");
                element.to_string_writer(writer);
                writer.end();
            }
            IrTerm::And { lhs, rhs } => {
                writer.start();
                writer.write("and");
                lhs.to_string_writer(writer);
                rhs.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Or { lhs, rhs } => {
                writer.start();
                writer.write("or");
                lhs.to_string_writer(writer);
                rhs.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Load {
                type_,
                address,
                offset,
            } => {
                writer.start();
                writer.write("load");
                type_.to_term().to_string_writer(writer);
                address.to_string_writer(writer);
                offset.to_string_writer(writer);
                writer.end();
            }
            IrTerm::Store {
                type_,
                address,
                offset,
                value,
            } => {
                writer.start();
                writer.write("store");
                type_.to_term().to_string_writer(writer);
                address.to_string_writer(writer);
                offset.to_string_writer(writer);
                value.to_string_writer(writer);
                writer.end();
            }
        }
    }

    #[allow(unused_variables)]
    pub fn find_let(&self) -> Vec<(String, IrType, Box<IrTerm>)> {
        match self {
            IrTerm::Nil => vec![],
            IrTerm::I32(_) => vec![],
            IrTerm::I64(_) => vec![],
            IrTerm::Ident(_) => vec![],
            IrTerm::String(_) => vec![],
            IrTerm::Call {
                callee: _, args, ..
            } => {
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
            IrTerm::While {
                cond,
                body,
                cleanup,
            } => {
                let mut result = vec![];
                result.extend(cond.find_let());
                result.extend(body.find_let());
                if let Some(cleanup) = cleanup {
                    result.extend(cleanup.find_let());
                }
                result
            }
            IrTerm::Module { elements } => todo!(),
            IrTerm::Func {
                name,
                params,
                result,
                body,
            } => todo!(),
            IrTerm::GlobalLet { name, type_, value } => todo!(),
            IrTerm::SizeOf { type_ } => vec![],
            IrTerm::WriteMemory {
                type_,
                address,
                value,
            } => vec![],
            IrTerm::Continue => vec![],
            IrTerm::Break => vec![],
            IrTerm::Discard { element } => {
                let mut result = vec![];
                result.extend(element.find_let());
                result
            }
            IrTerm::And { lhs, rhs } => {
                let mut result = vec![];
                result.extend(lhs.find_let());
                result.extend(rhs.find_let());
                result
            }
            IrTerm::Or { lhs, rhs } => {
                let mut result = vec![];
                result.extend(lhs.find_let());
                result.extend(rhs.find_let());
                result
            }
            IrTerm::Load {
                type_,
                address,
                offset,
            } => {
                let mut result = vec![];
                result.extend(address.find_let());
                result.extend(offset.find_let());
                result
            }
            IrTerm::Store {
                type_,
                address,
                offset,
                value,
            } => {
                let mut result = vec![];
                result.extend(address.find_let());
                result.extend(offset.find_let());
                result.extend(value.find_let());
                result
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut writer = SExprWriter::new();
        self.to_string_writer(&mut writer);

        writer.buffer
    }

    pub fn wrap_mult_sizeof(ir_type: IrType, term: IrTerm) -> IrTerm {
        IrTerm::Call {
            callee: Box::new(IrTerm::Ident("mult".to_string())),
            args: vec![term, IrTerm::SizeOf { type_: ir_type }],
            source: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrType {
    Nil,
    I32,
    I64,
    Address,
}

impl IrType {
    pub fn from_type(type_: &Type) -> Result<Self> {
        match type_ {
            Type::Nil => Ok(IrType::Nil),
            Type::Bool => Ok(IrType::I32),
            Type::I32 => Ok(IrType::I32),
            Type::I64 => Ok(IrType::I64),
            Type::Record(_) => Ok(IrType::Address),
            Type::Ident(_) => Ok(IrType::Address), // FIXME: could be other types
            Type::Ptr(_) => Ok(IrType::Address),
            Type::Array(_, _) => Ok(IrType::Address),
            Type::Byte => Ok(IrType::I32),
            Type::Vec(_) => Ok(IrType::Address),
            Type::Optional(_) => Ok(IrType::Address),
            Type::Func(_, _) => todo!(),
            Type::VariadicFunc(_, _, _) => todo!(),
            Type::Range(_) => todo!(),
            Type::Omit(_) => {
                bail!("Found omit type in IrType::from_type");
            }
            Type::Map(_, _) => Ok(IrType::Address),
            Type::Or(_, _) => Ok(IrType::Address),
        }
    }

    pub fn to_term(&self) -> IrTerm {
        match self {
            IrType::Nil => IrTerm::nil(),
            IrType::I32 => IrTerm::Ident("i32".to_string()),
            IrType::I64 => IrTerm::Ident("i64".to_string()),
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
            IrType::Nil => 4,
            IrType::I32 => 4,
            IrType::I64 => 8,
            IrType::Address => 4,
        }
    }
}
