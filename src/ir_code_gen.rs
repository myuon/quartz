use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use crate::{
    ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type, VarType},
    ir::{IrElement, IrType},
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

    pub fn run(&mut self, module: &mut Module) -> Result<IrElement> {
        self.module(module)
    }

    fn module(&mut self, module: &mut Module) -> Result<IrElement> {
        let mut elements = vec![];

        for decl in &mut module.0 {
            elements.push(self.decl(decl)?);
        }

        Ok(IrElement::block("module", elements))
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<IrElement> {
        match decl {
            Decl::Func(func) => self.func(func),
            Decl::Let(ident, type_, expr) => Ok(IrElement::i_global_let(
                ident.as_str(),
                IrType::from_type(type_)?,
                self.expr(expr)?,
            )),
            Decl::Type(_, _) => Ok(IrElement::nil()),
        }
    }

    fn func(&mut self, func: &mut Func) -> Result<IrElement> {
        let mut elements = vec![];
        for statement in &mut func.body {
            elements.push(self.statement(statement)?);
        }

        Ok(IrElement::i_func(func.name.as_str(), elements))
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<IrElement> {
        match statement {
            Statement::Let(ident, type_, expr) => Ok(IrElement::i_let(
                ident.as_str(),
                IrType::from_type(type_)?,
                self.expr(expr)?,
            )),
            Statement::Return(expr) => Ok(IrElement::i_return(self.expr(expr)?)),
            Statement::Expr(expr) => Ok(self.expr(expr)?),
            Statement::Assign(var_type, lhs, rhs) => match var_type {
                Some(VarType::Local) => Ok(IrElement::i_assign_local(
                    IrElement::ident(lhs.as_str()),
                    self.expr(rhs)?,
                )),
                Some(VarType::Global) => Ok(IrElement::i_assign_global(
                    IrElement::ident(lhs.as_str()),
                    self.expr(rhs)?,
                )),
                None => bail!("Invalid assignment"),
            },
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

                Ok(IrElement::i_if(
                    self.expr(cond)?,
                    IrType::from_type(type_)?,
                    IrElement::block("then", then_elements),
                    IrElement::block("else", else_elements),
                ))
            }
        }
    }

    fn expr(&mut self, expr: &mut Expr) -> Result<IrElement> {
        match expr {
            Expr::Ident(ident) => Ok(IrElement::ident(ident.as_str())),
            Expr::Lit(lit) => match lit {
                Lit::I32(i) => Ok(IrElement::i32(*i)),
            },
            Expr::Call(callee, args) => {
                let mut elements = vec![];
                for arg in args {
                    elements.push(self.expr(arg)?);
                }

                Ok(IrElement::i_call(callee.as_str(), elements))
            }
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
                elements.push(IrElement::i_let(
                    var,
                    IrType::Address,
                    IrElement::i_call("alloc", vec![IrElement::i32(record_type.len() as i32)]),
                ));
                for (field, expr) in fields {
                    let index = record_type
                        .iter()
                        .position(|(f, _)| f == field)
                        .ok_or(anyhow!("Field not found: {:?} in {:?}", field, record_type))?;
                    elements.push(IrElement::i_call(
                        "set_field",
                        vec![
                            IrElement::ident(var),
                            IrElement::i32(index as i32),
                            self.expr(expr)?,
                        ],
                    ));
                }

                Ok(IrElement::i_seq(elements))
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

                Ok(IrElement::i_call(
                    "get_field",
                    vec![self.expr(expr)?, IrElement::i32(index as i32)],
                ))
            }
        }
    }
}
