use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};

use crate::{
    ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type},
    compiler::ErrorInSource,
    ir::{IrTerm, IrType},
    util::source::Source,
};

#[derive(Debug, Clone)]
pub struct IrCodeGenerator {
    types: HashMap<Ident, Type>,
}

impl IrCodeGenerator {
    pub fn new() -> Self {
        IrCodeGenerator {
            types: HashMap::new(),
        }
    }

    pub fn set_types(&mut self, types: HashMap<Ident, Type>) {
        self.types = types;
    }

    pub fn run(&mut self, module: &mut Module) -> Result<IrTerm> {
        self.module(module)
    }

    fn module(&mut self, module: &mut Module) -> Result<IrTerm> {
        let mut elements = vec![];

        for decl in &mut module.0 {
            match decl {
                Decl::Func(func) => {
                    elements.push(self.func(func)?);
                }
                Decl::Let(ident, type_, expr) => {
                    elements.push(IrTerm::GlobalLet {
                        name: ident.0.clone(),
                        type_: IrType::from_type(type_)?,
                        value: Box::new(self.expr(expr)?),
                    });
                }
                Decl::Type(_, _) => (),
            }
        }

        Ok(IrTerm::Module { elements })
    }

    fn func(&mut self, func: &mut Func) -> Result<IrTerm> {
        let elements = self.statements(&mut func.body)?;

        let mut params = vec![];
        for (ident, type_) in &func.params {
            params.push((ident.0.clone(), IrType::from_type(type_)?));
        }

        Ok(IrTerm::Func {
            name: func.name.0.clone(),
            params,
            result: Box::new(IrType::from_type(&func.result)?),
            body: vec![IrTerm::Seq { elements }],
        })
    }

    fn statements(&mut self, statements: &mut Vec<Statement>) -> Result<Vec<IrTerm>> {
        let mut elements = vec![];
        for statement in statements {
            elements.push(self.statement(statement)?);
        }
        Ok(elements)
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<IrTerm> {
        match statement {
            Statement::Let(ident, type_, expr) => Ok(IrTerm::Let {
                name: ident.0.clone(),
                type_: IrType::from_type(type_)?,
                value: Box::new(self.expr(expr)?),
            }),
            Statement::Return(expr) => Ok(IrTerm::Return {
                value: Box::new(self.expr(expr)?),
            }),
            Statement::Expr(expr) => Ok(self.expr(expr)?),
            Statement::Assign(lhs, rhs) => self.assign(lhs, rhs),
            Statement::If(cond, type_, then_block, else_block) => {
                let mut then_elements = vec![];
                for statement in then_block {
                    then_elements.push(self.statement(statement)?);
                }

                let mut else_elements = vec![];
                if let Some(else_block) = else_block {
                    for statement in else_block {
                        else_elements.push(self.statement(statement)?);
                    }
                }

                Ok(IrTerm::If {
                    cond: Box::new(self.expr(cond)?),
                    type_: IrType::from_type(type_).context(ErrorInSource {
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?,
                    then: Box::new(IrTerm::Seq {
                        elements: then_elements,
                    }),
                    else_: Box::new(IrTerm::Seq {
                        elements: else_elements,
                    }),
                })
            }
            Statement::While(cond, block) => {
                let mut elements = vec![];
                for statement in block {
                    elements.push(self.statement(statement)?);
                }

                Ok(IrTerm::While {
                    cond: Box::new(self.expr(cond)?),
                    body: Box::new(IrTerm::Seq { elements }),
                })
            }
            Statement::For(ident, range, body) => match &mut range.data {
                Expr::Range(start, end) => {
                    let mut elements = self.statements(body)?;
                    elements.push(IrTerm::Assign {
                        lhs: ident.0.clone(),
                        rhs: Box::new(IrTerm::Call {
                            callee: Box::new(IrTerm::Ident("add".to_string())),
                            args: vec![IrTerm::Ident(ident.0.clone()), IrTerm::I32(1)],
                        }),
                    });

                    Ok(IrTerm::Seq {
                        elements: vec![
                            IrTerm::Let {
                                name: ident.as_str().to_string(),
                                type_: IrType::I32,
                                value: Box::new(self.expr(start.as_mut())?),
                            },
                            IrTerm::While {
                                cond: Box::new(IrTerm::Call {
                                    callee: Box::new(IrTerm::Ident("lt".to_string())),
                                    args: vec![
                                        IrTerm::Ident(ident.as_str().to_string()),
                                        self.expr(end.as_mut())?,
                                    ],
                                }),
                                body: Box::new(IrTerm::Seq { elements }),
                            },
                        ],
                    })
                }
                _ => bail!("invalid range expression, {:?}", range),
            },
        }
    }

    fn assign(&mut self, lhs: &mut Source<Expr>, rhs: &mut Source<Expr>) -> Result<IrTerm> {
        let lhs = self.expr(lhs)?;
        let rhs = self.expr(rhs)?;
        match lhs {
            IrTerm::Ident(ident) => Ok(IrTerm::Assign {
                lhs: ident,
                rhs: Box::new(rhs),
            }),
            IrTerm::PointerAt { .. } => Ok(IrTerm::SetPointer {
                address: Box::new(lhs),
                value: Box::new(rhs),
            }),
            IrTerm::GetField { address, offset } => Ok(IrTerm::SetField {
                address,
                offset,
                value: Box::new(rhs),
            }),
            _ => bail!("invalid lhs for assignment: {}", lhs.to_string()),
        }
    }

    fn expr(&mut self, expr: &mut Source<Expr>) -> Result<IrTerm> {
        match &mut expr.data {
            Expr::Ident(ident) => Ok(IrTerm::ident(ident.as_str())),
            Expr::Lit(lit) => match lit {
                Lit::I32(i) => Ok(IrTerm::i32(*i)),
                Lit::String(s) => Ok(IrTerm::Seq {
                    elements: vec![
                        self.statement(&mut Statement::Let(
                            Ident("_string".to_string()),
                            self.type_name(&Ident("string".to_string()))?.clone(),
                            Source::unknown(Expr::Record(
                                Ident("string".to_string()),
                                vec![
                                    (
                                        Ident("length".to_string()),
                                        Source::unknown(Expr::Lit(Lit::I32(s.len() as i32))),
                                    ),
                                    (
                                        Ident("data".to_string()),
                                        Source::unknown(Expr::Call(
                                            Box::new(Source::unknown(Expr::Ident(Ident(
                                                "alloc".to_string(),
                                            )))),
                                            vec![Source::unknown(Expr::Lit(Lit::I32(
                                                s.len() as i32
                                            )))],
                                        )),
                                    ),
                                ],
                            )),
                        ))?,
                        IrTerm::WriteMemory {
                            type_: IrType::I32,
                            address: Box::new(self.expr(&mut Source::unknown(Expr::Project(
                                Box::new(Source::unknown(Expr::Ident(Ident(
                                    "_string".to_string(),
                                )))),
                                Type::Ident(Ident("string".to_string())),
                                Ident("data".to_string()),
                            )))?),
                            value: s.bytes().map(|b| IrTerm::i32(b as i32)).collect(),
                        },
                        IrTerm::Ident("_string".to_string()),
                    ],
                }),
            },
            Expr::Call(callee, args) => match &mut callee.data {
                Expr::Project(expr, type_, label) => match (type_, label.as_str()) {
                    (Type::Pointer(p), "at") => {
                        assert_eq!(args.len(), 1);

                        Ok(IrTerm::PointerAt {
                            type_: IrType::from_type(p)?,
                            address: Box::new(self.expr(expr)?),
                            offset: Box::new(self.expr(&mut args[0])?),
                        })
                    }
                    (Type::Array(p, _), "at") => {
                        assert_eq!(args.len(), 1);

                        Ok(IrTerm::PointerAt {
                            type_: IrType::from_type(p)?,
                            address: Box::new(self.expr(expr)?),
                            offset: Box::new(self.expr(&mut args[0])?),
                        })
                    }
                    (Type::Vec(p), "at") => {
                        assert_eq!(args.len(), 1);

                        Ok(IrTerm::PointerAt {
                            type_: IrType::from_type(p)?,
                            address: Box::new(self.expr(expr)?),
                            offset: Box::new(self.expr(&mut args[0])?),
                        })
                    }
                    (Type::Vec(_), "push") => {
                        assert_eq!(args.len(), 1);

                        Ok(
                            self.statement(&mut Statement::Expr(Source::unknown(Expr::Call(
                                Box::new(Source::unknown(Expr::Ident(Ident(
                                    "vec_push".to_string(),
                                )))),
                                vec![expr.as_ref().clone(), args[0].clone()],
                            ))))?,
                        )
                    }
                    _ => {
                        let mut elements = vec![];
                        elements.push(self.expr(expr)?);
                        for arg in args {
                            elements.push(self.expr(arg)?);
                        }

                        Ok(IrTerm::Call {
                            callee: Box::new(
                                self.expr(&mut Source::unknown(Expr::Ident(label.clone())))?,
                            ),
                            args: elements,
                        })
                    }
                },
                _ => {
                    let mut elements = vec![];
                    for arg in args {
                        elements.push(self.expr(arg)?);
                    }

                    Ok(IrTerm::Call {
                        callee: Box::new(self.expr(callee.as_mut())?),
                        args: elements,
                    })
                }
            },
            Expr::Record(ident, fields) => {
                /* example
                    let x = Point { x: 10, y: 20 };

                    (seq
                        (let $addr (call $alloc 2))
                        (call $set_field $addr 0 10)
                        (call $set_field $addr 1 20)
                        $addr
                    )
                */

                let record_type = self
                    .types
                    .get(ident)
                    .ok_or(anyhow!("Type not found: {:?}", ident))?
                    .clone()
                    .to_record()?;

                let var = "_record";

                let mut elements = vec![];
                elements.push(IrTerm::Let {
                    name: var.to_string(),
                    type_: IrType::Address,
                    value: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("alloc".to_string())),
                        args: vec![IrTerm::i32(record_type.len() as i32)],
                    }),
                });

                let mut offset = 0;
                for (field, expr) in fields {
                    let (_, type_) = record_type
                        .iter()
                        .find(|(f, _)| f == field)
                        .ok_or(anyhow!("Field not found: {:?} in {:?}", field, record_type))?;
                    elements.push(IrTerm::SetField {
                        address: Box::new(IrTerm::ident(var)),
                        offset,
                        value: Box::new(self.expr(expr)?),
                    });

                    offset += IrType::from_type(type_)?.sizeof();
                }

                elements.push(IrTerm::ident(var));

                Ok(IrTerm::Seq { elements })
            }
            Expr::Project(expr, type_, label) => {
                let record_type = self
                    .types
                    .get(&type_.clone().to_ident()?)
                    .ok_or(anyhow!("Type not found: {:?}", type_))?
                    .clone()
                    .to_record()?;

                let index = record_type
                    .iter()
                    .position(|(f, _)| f == label)
                    .ok_or(anyhow!("Field not found: {:?} in {:?}", label, record_type))?;

                let mut offset = 0;
                for (field, _) in record_type.iter().take(index) {
                    let (_, type_) = record_type
                        .iter()
                        .find(|(f, _)| f == field)
                        .ok_or(anyhow!("Field not found: {:?} in {:?}", field, record_type))?;
                    offset += IrType::from_type(type_)?.sizeof();
                }

                Ok(IrTerm::GetField {
                    address: Box::new(self.expr(expr)?),
                    offset,
                })
            }
            Expr::Make(type_, _) => match type_ {
                Type::Array(elem, size) => Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident("alloc".to_string())),
                    args: vec![IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("mult".to_string())),
                        args: vec![
                            IrTerm::SizeOf {
                                type_: IrType::from_type(elem)?,
                            },
                            IrTerm::i32(*size as i32),
                        ],
                    }],
                }),
                Type::Vec(type_) => Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident("vec_make".to_string())),
                    args: vec![
                        IrTerm::i32(5),
                        IrTerm::SizeOf {
                            type_: IrType::from_type(type_)?,
                        },
                    ],
                }),
                _ => bail!("unsupported type for make: {:?}", type_),
            },
            Expr::Range(_, _) => todo!(),
            Expr::As(expr, _) => self.expr(expr),
        }
    }

    fn type_name(&self, ident: &Ident) -> Result<&Type> {
        self.types
            .get(ident)
            .ok_or(anyhow!("Type not found: {:?}", ident))
    }
}
