use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Statement, Type},
    compiler::{ErrorInSource, SourcePosition},
    ir::{IrTerm, IrType},
    util::{ident::Ident, path::Path, source::Source},
};

#[derive(Debug, Clone)]
pub struct IrCodeGenerator {
    types: HashMap<Ident, Type>,
    current_path: Path,
    pub strings: Vec<String>,
}

impl IrCodeGenerator {
    pub fn new() -> Self {
        IrCodeGenerator {
            types: HashMap::new(),
            current_path: Path::empty(),
            strings: vec![],
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
                    let mut path = self.current_path.clone();
                    path.push(ident.clone());

                    elements.push(IrTerm::GlobalLet {
                        name: path.as_joined_str("_"),
                        type_: IrType::from_type(type_).context("globalLet")?,
                        value: Box::new(self.expr(expr)?),
                    });
                }
                Decl::Type(_, _) => (),
                Decl::Module(name, module) => {
                    let path = self.current_path.clone();
                    self.current_path.extend(name);
                    elements.push(self.module(module)?);

                    self.current_path = path;
                }
                Decl::Import(_) => {}
            }
        }

        Ok(IrTerm::Module { elements })
    }

    fn func(&mut self, func: &mut Func) -> Result<IrTerm> {
        let elements = self.statements(&mut func.body)?;

        let mut params = vec![];
        for (ident, type_) in &func.params {
            if ident.as_str() == "self" {
                params.push((
                    ident.0.clone(),
                    IrType::from_type(&Type::Ident(self.current_path.0[0].clone()))
                        .context("func:params self")?,
                ));
            } else {
                params.push((
                    ident.0.clone(),
                    IrType::from_type(type_).context("func:params")?,
                ));
            }
        }

        let mut path = self.current_path.clone();
        path.push(func.name.clone());

        Ok(IrTerm::Func {
            name: path.as_joined_str("_"),
            params,
            result: Box::new(IrType::from_type(&func.result).context("func:result")?),
            body: vec![IrTerm::Seq { elements }],
        })
    }

    fn statements(&mut self, statements: &mut Vec<Source<Statement>>) -> Result<Vec<IrTerm>> {
        let mut elements = vec![];
        for statement in statements {
            elements.push(self.statement(statement)?);
        }
        Ok(elements)
    }

    fn statement(&mut self, statement: &mut Source<Statement>) -> Result<IrTerm> {
        match &mut statement.data {
            Statement::Let(ident, type_, expr) => Ok(IrTerm::Let {
                name: ident.0.clone(),
                type_: IrType::from_type(type_).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: statement.start.unwrap_or(0),
                    end: statement.end.unwrap_or(0),
                })?,
                value: Box::new(self.expr(expr)?),
            }),
            Statement::Return(expr) => Ok(IrTerm::Return {
                value: Box::new(self.expr(expr)?),
            }),
            Statement::Expr(expr, type_) => {
                let expr = self.expr(expr)?;

                if type_.is_omit() {
                    return Err(anyhow!("omit type ({}) is not allowed", type_.to_string())
                        .context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: statement.start.unwrap_or(0),
                            end: statement.end.unwrap_or(0),
                        }));
                }

                Ok(if type_.is_nil() {
                    expr
                } else {
                    IrTerm::Discard {
                        element: Box::new(expr),
                    }
                })
            }
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
                        path: None,
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
                    cleanup: None,
                })
            }
            Statement::For(ident, range, body) => match &mut range.data {
                Expr::Range(start, end) => Ok(IrTerm::Seq {
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
                                source: None,
                            }),
                            body: Box::new(IrTerm::Seq {
                                elements: self.statements(body)?,
                            }),
                            cleanup: Some(Box::new(IrTerm::Assign {
                                lhs: ident.0.clone(),
                                rhs: Box::new(IrTerm::Call {
                                    callee: Box::new(IrTerm::Ident("add".to_string())),
                                    args: vec![IrTerm::Ident(ident.0.clone()), IrTerm::I32(1)],
                                    source: None,
                                }),
                            })),
                        },
                    ],
                }),
                _ => bail!("invalid range expression, {:?}", range),
            },
            Statement::Continue => Ok(IrTerm::Continue),
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
            IrTerm::Call { .. } => Ok(IrTerm::SetPointer {
                address: Box::new(lhs),
                value: Box::new(rhs),
            }),
            _ => bail!("invalid lhs for assignment: {}", lhs.to_string()),
        }
    }

    fn expr(&mut self, expr: &mut Source<Expr>) -> Result<IrTerm> {
        match &mut expr.data {
            Expr::Ident {
                ident,
                resolved_path,
            } => {
                let resolved_path = resolved_path.clone().unwrap_or(Path::ident(ident.clone()));

                Ok(IrTerm::ident(resolved_path.as_joined_str("_")))
            }
            Expr::Self_ => Ok(IrTerm::Ident("self".to_string())),
            Expr::Path {
                path,
                resolved_path,
            } => {
                let resolved_path = resolved_path.clone().unwrap_or(path.data.clone());

                Ok(IrTerm::ident(resolved_path.as_joined_str("_")))
            }
            Expr::Lit(lit) => match lit {
                Lit::Nil => Ok(IrTerm::I32(0)),
                Lit::Bool(b) => Ok(IrTerm::I32(if *b { 1 } else { 0 })),
                Lit::I32(i) => Ok(IrTerm::i32(*i)),
                Lit::I64(i) => Ok(IrTerm::i64(*i)),
                Lit::String(s) => {
                    let index = self.strings.len();
                    self.strings.push(s.clone());

                    Ok(IrTerm::String(index))
                }
            },
            Expr::BinOp(op, type_, arg1, arg2) => {
                use crate::ast::BinOp::*;

                match op {
                    Mul => {
                        let arg1 = self.expr(arg1)?;
                        let arg2 = self.expr(arg2)?;

                        Ok(IrTerm::Call {
                            callee: Box::new(IrTerm::Ident(
                                if matches!(type_, Type::I32) {
                                    "mult"
                                } else if matches!(type_, Type::I64) {
                                    "mult_i64"
                                } else {
                                    bail!("invalid type for mul: {:?}", type_)
                                }
                                .to_string(),
                            )),
                            args: vec![arg1, arg2],
                            source: None,
                        })
                    }
                    Mod => {
                        let arg1 = self.expr(arg1)?;
                        let arg2 = self.expr(arg2)?;

                        Ok(IrTerm::Call {
                            callee: Box::new(IrTerm::Ident(
                                if matches!(type_, Type::I32) {
                                    "mod"
                                } else if matches!(type_, Type::I64) {
                                    "mod_i64"
                                } else {
                                    bail!("invalid type for mod: {:?}", type_)
                                }
                                .to_string(),
                            )),
                            args: vec![arg1, arg2],
                            source: None,
                        })
                    }
                    And => {
                        let arg1 = self.expr(arg1)?;
                        let arg2 = self.expr(arg2)?;

                        Ok(IrTerm::And {
                            lhs: Box::new(arg1),
                            rhs: Box::new(arg2),
                        })
                    }
                    Or => {
                        let arg1 = self.expr(arg1)?;
                        let arg2 = self.expr(arg2)?;

                        Ok(IrTerm::Or {
                            lhs: Box::new(arg1),
                            rhs: Box::new(arg2),
                        })
                    }
                }
            }
            Expr::Call(callee, args) => match &mut callee.data {
                Expr::Project(expr, type_, label) => {
                    if label.0.len() == 1 {
                        match (type_, label.0[0].as_str()) {
                            (Type::Ptr(p), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::PointerAt {
                                    type_: IrType::from_type(p).context("method:ptr.at")?,
                                    address: Box::new(self.expr(expr)?),
                                    index: Box::new(self.expr(&mut args[0])?),
                                })
                            }
                            (Type::Ptr(p), "offset") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::PointerOffset {
                                    address: Box::new(self.expr(expr)?),
                                    offset: Box::new(IrTerm::Call {
                                        callee: Box::new(IrTerm::Ident("mult".to_string())),
                                        args: vec![
                                            self.expr(&mut args[0])?,
                                            IrTerm::SizeOf {
                                                type_: IrType::from_type(&p)
                                                    .context("method:ptr.offset")?,
                                            },
                                        ],
                                        source: None,
                                    }),
                                })
                            }
                            (Type::Array(p, s), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::PointerAt {
                                    type_: IrType::from_type(p).context("method:array.at")?,
                                    address: Box::new(self.expr(&mut Source::unknown(
                                        Expr::Project(
                                            expr.clone(),
                                            Type::Array(p.clone(), *s),
                                            Path::ident(Ident("data".to_string())),
                                        ),
                                    ))?),
                                    index: Box::new(self.expr(&mut args[0])?),
                                })
                            }
                            (Type::Vec(_), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::unknown(Expr::Call(
                                    Box::new(Source::unknown(Expr::path(Path::new(
                                        vec!["quartz", "std", "vec_at"]
                                            .into_iter()
                                            .map(|t| Ident(t.to_string()))
                                            .collect(),
                                    )))),
                                    vec![expr.as_ref().clone(), args[0].clone()],
                                )))?)
                            }
                            (Type::Vec(_), "push") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.statement(&mut Source::unknown(Statement::Expr(
                                    Source::unknown(Expr::Call(
                                        Box::new(Source::unknown(Expr::path(Path::new(
                                            vec!["quartz", "std", "vec_push"]
                                                .into_iter()
                                                .map(|t| Ident(t.to_string()))
                                                .collect(),
                                        )))),
                                        vec![expr.as_ref().clone(), args[0].clone()],
                                    )),
                                    Type::Nil,
                                )))?)
                            }
                            (Type::Map(_, _), "insert") => {
                                assert_eq!(args.len(), 2);

                                Ok(self.statement(&mut Source::unknown(Statement::Expr(
                                    Source::unknown(Expr::Call(
                                        Box::new(Source::unknown(Expr::path(Path::new(
                                            vec!["quartz", "std", "map_insert"]
                                                .into_iter()
                                                .map(|t| Ident(t.to_string()))
                                                .collect(),
                                        )))),
                                        vec![
                                            expr.as_ref().clone(),
                                            args[0].clone(),
                                            args[1].clone(),
                                        ],
                                    )),
                                    Type::Nil,
                                )))?)
                            }
                            (Type::Map(_, _), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::Call {
                                    callee: Box::new(self.expr(&mut Source::unknown(
                                        Expr::path(Path::new(vec![
                                            Ident("quartz".to_string()),
                                            Ident("std".to_string()),
                                            Ident("map_at".to_string()),
                                        ])),
                                    ))?),
                                    args: vec![self.expr(expr)?, self.expr(&mut args[0])?],
                                    source: None,
                                })
                            }
                            (Type::I32, label) => {
                                let mut elements = vec![];
                                elements.push(self.expr(expr)?);
                                for arg in args {
                                    elements.push(self.expr(arg)?);
                                }

                                Ok(IrTerm::Call {
                                    callee: Box::new(self.expr(&mut Source::unknown(
                                        Expr::path(Path::new(vec![
                                            Ident("i32".to_string()),
                                            Ident(label.to_string()),
                                        ])),
                                    ))?),
                                    args: elements,
                                    source: None,
                                })
                            }
                            _ => bail!("invalid project: {:?}", expr),
                        }
                    } else {
                        let mut elements = vec![];
                        elements.push(self.expr(expr)?);
                        for arg in args {
                            elements.push(self.expr(arg)?);
                        }

                        Ok(IrTerm::Call {
                            callee: Box::new(
                                self.expr(&mut Source::unknown(Expr::path(label.clone())))?,
                            ),
                            args: elements,
                            source: None,
                        })
                    }
                }
                _ => {
                    let mut elements = vec![];
                    for arg in args {
                        elements.push(self.expr(arg)?);
                    }

                    Ok(IrTerm::Call {
                        callee: Box::new(self.expr(callee.as_mut())?),
                        args: elements,
                        source: Some(SourcePosition {
                            path: self.current_path.clone(),
                            start: callee.start.unwrap_or(0),
                            end: callee.end.unwrap_or(0),
                        }),
                    })
                }
            },
            Expr::Record(ident, fields, expansion) => {
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
                    .get(&ident.data)
                    .ok_or(anyhow!("Type not found: {:?}", ident))?
                    .clone()
                    .to_record()?;

                let var = format!("_record_{}", ident.start.unwrap_or(0));

                let mut elements = vec![];
                elements.push(IrTerm::Let {
                    name: var.clone(),
                    type_: IrType::Address,
                    value: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("alloc".to_string())),
                        args: vec![IrTerm::i32(record_type.len() as i32)],
                        source: None,
                    }),
                });

                let mut offset = 0;
                for (i, type_) in record_type {
                    if let Some((_, expr)) = fields.iter_mut().find(|(f, _)| f == &i) {
                        elements.push(IrTerm::SetField {
                            address: Box::new(IrTerm::ident(var.clone())),
                            offset,
                            value: Box::new(self.expr(expr)?),
                        });
                    } else {
                        if let Some(expansion) = expansion {
                            elements.push(IrTerm::SetField {
                                address: Box::new(IrTerm::ident(var.clone())),
                                offset,
                                value: Box::new(self.expr(expansion)?),
                            });
                        } else {
                            return Err(anyhow!("Field not found: {:?}", i).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: expr.start.unwrap_or(0),
                                    end: expr.end.unwrap_or(0),
                                },
                            ));
                        }
                    }

                    offset += IrType::from_type(&type_)?.sizeof();
                }

                elements.push(IrTerm::ident(var));

                Ok(IrTerm::Seq { elements })
            }
            Expr::AnonymousRecord(fields, type_) => {
                let mut elements = vec![];
                elements.push(IrTerm::Let {
                    name: "_record".to_string(),
                    type_: IrType::Address,
                    value: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("alloc".to_string())),
                        args: vec![IrTerm::i32(fields.len() as i32)],
                        source: None,
                    }),
                });

                let mut offset = 0;
                for (label, type_) in type_.as_record_type().unwrap() {
                    elements.push(IrTerm::SetField {
                        address: Box::new(IrTerm::ident("_record".to_string())),
                        offset,
                        value: Box::new(
                            self.expr(
                                &mut fields
                                    .iter_mut()
                                    .find(|(f, _)| f == label)
                                    .ok_or(anyhow!("Field not found: {:?}", label))?
                                    .1,
                            )?,
                        ),
                    });

                    offset += IrType::from_type(&type_)?.sizeof();
                }

                elements.push(IrTerm::ident("_record".to_string()));

                Ok(IrTerm::Seq { elements })
            }
            Expr::Project(expr, type_, label) => {
                let record_type = if let Ok(record) = type_.as_record_type() {
                    record.clone()
                } else if let Ok(ident) = type_.clone().to_ident() {
                    self.types
                        .get(&ident)
                        .ok_or(anyhow!("Type not found: {:?}", type_))?
                        .clone()
                        .to_record()?
                } else {
                    return Err(
                        anyhow!("invalid project: {:?}", expr).context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        }),
                    );
                };

                let index = record_type
                    .iter()
                    .position(|(f, _)| &Path::ident(f.clone()) == label)
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
            Expr::Make(type_, args) => match type_ {
                Type::Array(_elem, size) => Ok(self.expr(&mut Source::unknown(Expr::Record(
                    Source::unknown(Ident("array".to_string())),
                    vec![
                        (
                            Ident("data".to_string()),
                            Source::unknown(Expr::Call(
                                Box::new(Source::unknown(Expr::ident(Ident("alloc".to_string())))),
                                vec![Source::unknown(Expr::Lit(Lit::I32(*size as i32)))],
                            )),
                        ),
                        (
                            Ident("length".to_string()),
                            Source::unknown(Expr::Lit(Lit::I32(*size as i32))),
                        ),
                    ],
                    None,
                )))?),
                Type::Vec(type_) => {
                    let cap = if args.is_empty() {
                        IrTerm::i32(5)
                    } else {
                        self.expr(&mut args[0])?
                    };

                    Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            Path::new(
                                vec!["quartz", "std", "vec_make"]
                                    .into_iter()
                                    .map(|s| Ident(s.to_string()))
                                    .collect(),
                            )
                            .as_joined_str("_"),
                        )),
                        args: vec![
                            cap,
                            IrTerm::SizeOf {
                                type_: IrType::from_type(type_)?,
                            },
                        ],
                        source: None,
                    })
                }
                Type::Map(_, _) => Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident(
                        Path::new(
                            vec!["quartz", "std", "map_make"]
                                .into_iter()
                                .map(|s| Ident(s.to_string()))
                                .collect(),
                        )
                        .as_joined_str("_"),
                    )),
                    args: vec![],
                    source: None,
                }),
                _ => bail!("unsupported type for make: {:?}", type_),
            },
            Expr::Range(_, _) => todo!(),
            Expr::As(expr, _) => self.expr(expr),
            Expr::SizeOf(type_) => {
                let type_ = IrType::from_type(type_)?;
                Ok(IrTerm::SizeOf { type_ })
            }
            Expr::Equal(lhs, rhs) => {
                let lhs = self.expr(lhs)?;
                let rhs = self.expr(rhs)?;

                Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident("equal".to_string())),
                    args: vec![lhs, rhs],
                    source: None,
                })
            }
            Expr::NotEqual(lhs, rhs) => {
                let lhs = self.expr(lhs)?;
                let rhs = self.expr(rhs)?;

                Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident("not_equal".to_string())),
                    args: vec![lhs, rhs],
                    source: None,
                })
            }
            Expr::Wrap(expr) => {
                let var = format!("_wrap_{}", expr.start.unwrap_or(0));
                let expr = self.expr(expr)?;

                Ok(IrTerm::Seq {
                    elements: vec![
                        IrTerm::Let {
                            name: var.clone(),
                            type_: IrType::Address,
                            value: Box::new(IrTerm::Call {
                                callee: Box::new(IrTerm::Ident("alloc".to_string())),
                                args: vec![IrTerm::i32(1)],
                                source: None,
                            }),
                        },
                        IrTerm::SetField {
                            address: Box::new(IrTerm::ident(var.clone())),
                            offset: 0,
                            value: Box::new(expr),
                        },
                        IrTerm::ident(var),
                    ],
                })
            }
            Expr::Unwrap(expr) => {
                let expr = self.expr(expr)?;

                Ok(IrTerm::GetField {
                    address: Box::new(expr),
                    offset: 0,
                })
            }
        }
    }

    fn type_name(&self, ident: &Ident) -> Result<&Type> {
        self.types
            .get(ident)
            .ok_or(anyhow!("Type not found: {:?}", ident))
    }
}
