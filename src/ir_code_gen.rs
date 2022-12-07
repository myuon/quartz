use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{
    ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type},
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
        let mut elements = vec![];
        for statement in &mut func.body {
            elements.push(self.statement(statement)?);
        }

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
            Statement::Assign(lhs, rhs) => Ok(IrTerm::Assign {
                lhs: Box::new(self.expr(lhs)?),
                rhs: Box::new(self.expr(rhs)?),
            }),
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
                    type_: IrType::from_type(type_)?,
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
        }
    }

    fn expr(&mut self, expr: &mut Source<Expr>) -> Result<IrTerm> {
        match &mut expr.data {
            Expr::Ident(ident) => Ok(IrTerm::ident(ident.as_str())),
            Expr::Lit(lit) => match lit {
                Lit::I32(i) => Ok(IrTerm::i32(*i)),
            },
            Expr::Call(callee, args) => match &mut callee.data {
                Expr::Project(expr, type_, label) => match (type_, label.as_str()) {
                    (Type::Pointer(_), "at") => {
                        assert_eq!(args.len(), 1);

                        Ok(IrTerm::PointerAt {
                            address: Box::new(self.expr(expr)?),
                            offset: Box::new(self.expr(&mut args[0])?),
                        })
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
                for (field, expr) in fields {
                    let index = record_type
                        .iter()
                        .position(|(f, _)| f == field)
                        .ok_or(anyhow!("Field not found: {:?} in {:?}", field, record_type))?;
                    elements.push(IrTerm::SetField {
                        address: Box::new(IrTerm::ident(var)),
                        offset: index,
                        value: Box::new(self.expr(expr)?),
                    });
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

                Ok(IrTerm::GetField {
                    address: Box::new(self.expr(expr)?),
                    offset: index,
                })
            }
        }
    }
}
