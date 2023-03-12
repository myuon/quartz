use std::{collections::HashMap, vec};

use anyhow::{anyhow, bail, Context, Result};
use rand::{distributions::Alphanumeric, prelude::Distribution};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Pattern, Statement, Type, UnwrapMode, VariadicCall},
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
        let mut decls = self.module(module)?;
        decls.push(self.generate_prepare_strings()?);

        Ok(IrTerm::Module { elements: decls })
    }

    fn module(&mut self, module: &mut Module) -> Result<Vec<IrTerm>> {
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
                    elements.push(IrTerm::Module {
                        elements: self.module(module)?,
                    });

                    self.current_path = path;
                }
                Decl::Import(_) => {}
            }
        }

        Ok(elements)
    }

    pub fn generate_prepare_strings(&mut self) -> Result<IrTerm> {
        let var_strings = "quartz_std_strings_ptr";
        let mut body = vec![];
        // let p = 0;
        body.push(IrTerm::Let {
            name: "p".to_string(),
            type_: IrType::I32,
            value: Box::new(IrTerm::i32(0)),
        });
        // quartz_std_strings_ptr = make[ptr[string]](${strings.len()});
        body.push(IrTerm::Assign {
            lhs: var_strings.to_string(),
            rhs: Box::new(self.expr(&mut Source::unknown(Expr::Make(
                Type::Ptr(Box::new(Type::Ident(Ident("string".to_string())))),
                vec![Source::unknown(Expr::Lit(Lit::I32(
                    self.strings.len() as i32
                )))],
            )))?),
        });

        for (i, string) in self.strings.clone().iter().enumerate() {
            // let p = new_empty_string(${string.len()})
            body.push(IrTerm::Let {
                name: "p".to_string(),
                type_: IrType::I32,
                value: Box::new(self.expr(&mut Source::unknown(Expr::Call(
                    Box::new(Source::unknown(Expr::ident(Ident(
                        "quartz_std_new_empty_string".to_string(),
                    )))),
                    vec![Source::unknown(Expr::Lit(Lit::I32(string.len() as i32)))],
                    None,
                    None,
                )))?),
            });

            // write_memory(*p, ${string.byte()})
            body.push(IrTerm::WriteMemory {
                type_: IrType::I32,
                address: Box::new(IrTerm::Load {
                    type_: IrType::I32,
                    address: Box::new(IrTerm::ident("p")),
                    offset: Box::new(IrTerm::i32(0)),
                }),
                value: string.bytes().map(|b| IrTerm::i32(b as i32)).collect(),
            });

            // strings[i] = p
            body.push(IrTerm::Store {
                type_: IrType::I32,
                address: Box::new(IrTerm::ident(var_strings.to_string())),
                offset: Box::new(self.generate_mult_sizeof(
                    &Type::Ident(Ident("string".to_string())),
                    IrTerm::i32(i as i32),
                )?),
                value: Box::new(IrTerm::ident("p")),
            });
        }

        body.push(IrTerm::Return {
            value: Box::new(IrTerm::nil()),
        });

        Ok(IrTerm::Func {
            name: "prepare_strings".to_string(),
            params: vec![],
            result: None,
            body,
        })
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
        if let Some((name, type_)) = &func.variadic {
            params.push((name.0.clone(), IrType::from_type(type_)?));
        }

        let mut path = self.current_path.clone();
        path.push(func.name.clone());

        Ok(IrTerm::Func {
            name: path.as_joined_str("_"),
            params,
            result: Some(IrType::from_type(&func.result).context("func:result")?),
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
            Statement::Let(pattern, type_, expr) => match pattern {
                Pattern::Ident(ident) => Ok(IrTerm::Let {
                    name: ident.0.clone(),
                    type_: IrType::from_type(type_).context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: statement.start.unwrap_or(0),
                        end: statement.end.unwrap_or(0),
                    })?,
                    value: Box::new(self.expr(expr)?),
                }),
                Pattern::Or(lhs_pattern, rhs_pattern) => {
                    let (lhs_type, rhs_type) = match type_.clone() {
                        Type::Or(lhs, rhs) => (lhs, rhs),
                        _ => bail!("type of or pattern must be or type"),
                    };
                    let lhs = lhs_pattern.as_ident().unwrap();
                    let rhs = rhs_pattern.as_ident().unwrap();

                    let var_name = format!(
                        "var_{}",
                        Alphanumeric
                            .sample_iter(&mut rand::thread_rng())
                            .take(5)
                            .map(char::from)
                            .collect::<String>()
                    );

                    Ok(IrTerm::Seq {
                        elements: vec![
                            IrTerm::Let {
                                name: var_name.clone(),
                                type_: IrType::from_type(type_).context(ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: statement.start.unwrap_or(0),
                                    end: statement.end.unwrap_or(0),
                                })?,
                                value: Box::new(self.expr(expr)?),
                            },
                            IrTerm::Let {
                                name: lhs.0.clone(),
                                type_: IrType::Address,
                                value: Box::new(IrTerm::Load {
                                    type_: IrType::from_type(&lhs_type)?,
                                    address: Box::new(IrTerm::Ident(var_name.clone())),
                                    offset: Box::new(IrTerm::i32(0)),
                                }),
                            },
                            IrTerm::Let {
                                name: rhs.0.clone(),
                                type_: IrType::Address,
                                value: Box::new(IrTerm::Load {
                                    type_: IrType::from_type(&rhs_type)?,
                                    address: Box::new(IrTerm::Ident(var_name.clone())),
                                    offset: Box::new(IrTerm::i32(IrType::Address.sizeof() as i32)),
                                }),
                            },
                        ],
                    })
                }
                _ => todo!(),
            },
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

                Ok(IrTerm::Discard {
                    element: Box::new(expr),
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
            Statement::Break => Ok(IrTerm::Break),
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
            IrTerm::Call { .. } => Ok(IrTerm::Store {
                type_: IrType::Address,
                address: Box::new(lhs),
                offset: Box::new(IrTerm::i32(0)),
                value: Box::new(rhs),
            }),
            IrTerm::Load {
                type_,
                address,
                offset,
            } => Ok(IrTerm::Store {
                type_,
                address,
                offset,
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
                Lit::Nil => Ok(IrTerm::nil()),
                Lit::Bool(b) => Ok(IrTerm::I32(if *b { 1 } else { 0 })),
                Lit::I32(i) => Ok(IrTerm::i32(*i)),
                Lit::U32(i) => Ok(IrTerm::u32(*i)),
                Lit::I64(i) => Ok(IrTerm::i64(*i)),
                Lit::String(s) => {
                    let index = self.strings.len();
                    self.strings.push(s.clone());

                    Ok(IrTerm::String(index))
                }
            },
            Expr::BinOp(op, type_, arg1, arg2) => {
                use crate::ast::BinOp::*;

                let arg1 = self.expr(arg1)?;
                let arg2 = self.expr(arg2)?;

                match op {
                    Add => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "add"
                            } else if matches!(type_, Type::I64) {
                                "add_i64"
                            } else {
                                bail!("invalid type for add: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Sub => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "sub"
                            } else if matches!(type_, Type::I64) {
                                "sub_i64"
                            } else {
                                bail!("invalid type for sub: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Mul => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "mult"
                            } else if matches!(type_, Type::U32) {
                                "mult_u32"
                            } else if matches!(type_, Type::I64) {
                                "mult_i64"
                            } else {
                                bail!("invalid type for mul: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Div => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "div"
                            } else if matches!(type_, Type::I64) {
                                "div_i64"
                            } else {
                                bail!("invalid type for div: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Mod => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "mod"
                            } else if matches!(type_, Type::U32) {
                                "mod_u32"
                            } else if matches!(type_, Type::I64) {
                                "mod_i64"
                            } else {
                                bail!("invalid type for mod: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    And => Ok(IrTerm::And {
                        lhs: Box::new(arg1),
                        rhs: Box::new(arg2),
                    }),
                    Or => Ok(IrTerm::Or {
                        lhs: Box::new(arg1),
                        rhs: Box::new(arg2),
                    }),
                    Lt => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "lt"
                            } else if matches!(type_, Type::I64) {
                                "lt_i64"
                            } else {
                                bail!("invalid type for lt: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Lte => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "lte"
                            } else if matches!(type_, Type::I64) {
                                "lte_i64"
                            } else {
                                bail!("invalid type for lte: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Gt => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "gt"
                            } else if matches!(type_, Type::I64) {
                                "gt_i64"
                            } else {
                                bail!("invalid type for gt: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Gte => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "gte"
                            } else if matches!(type_, Type::I64) {
                                "gte_i64"
                            } else {
                                bail!("invalid type for gte: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                }
            }
            Expr::Call(callee, args, variadic, expansion) => match &mut callee.data {
                Expr::Project(expr, type_, label) => {
                    if label.0.len() == 1 {
                        match (type_, label.0[0].as_str()) {
                            (Type::Ptr(p), "at") => {
                                assert_eq!(args.len(), 1);

                                let offset = self.expr(&mut args[0])?;
                                Ok(IrTerm::Load {
                                    type_: IrType::from_type(p).context("method:ptr.at")?,
                                    address: Box::new(self.expr(expr)?),
                                    offset: Box::new(self.generate_mult_sizeof(p, offset)?),
                                })
                            }
                            (Type::Ptr(p), "offset") => {
                                assert_eq!(args.len(), 1);

                                let offset = self.expr(&mut args[0])?;
                                Ok(IrTerm::Call {
                                    callee: Box::new(IrTerm::Ident("add".to_string())),
                                    args: vec![
                                        self.expr(expr)?,
                                        self.generate_mult_sizeof(p, offset)?,
                                    ],
                                    source: None,
                                })
                            }
                            (Type::Array(p, s), "at") => {
                                assert_eq!(args.len(), 1);

                                let term = self.expr(&mut args[0])?;
                                Ok(IrTerm::Load {
                                    type_: IrType::from_type(p).context("method:array.at")?,
                                    address: Box::new(self.expr(&mut Source::transfer(
                                        Expr::Project(
                                            expr.clone(),
                                            Type::Array(p.clone(), *s),
                                            Path::ident(Ident("data".to_string())),
                                        ),
                                        expr,
                                    ))?),
                                    offset: Box::new(self.generate_mult_sizeof(p, term)?),
                                })
                            }
                            (Type::Vec(_), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec!["quartz", "std", "vec_at"]
                                                    .into_iter()
                                                    .map(|t| Ident(t.to_string()))
                                                    .collect(),
                                            )),
                                            expr,
                                        )),
                                        vec![expr.as_ref().clone(), args[0].clone()],
                                        None,
                                        None,
                                    ),
                                    expr,
                                ))?)
                            }
                            (Type::Vec(_), "push") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec!["quartz", "std", "vec_push"]
                                                    .into_iter()
                                                    .map(|t| Ident(t.to_string()))
                                                    .collect(),
                                            )),
                                            expr,
                                        )),
                                        vec![expr.as_ref().clone(), args[0].clone()],
                                        None,
                                        None,
                                    ),
                                    expr,
                                ))?)
                            }
                            (Type::Map(_, _), "insert") => {
                                assert_eq!(args.len(), 2);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec!["quartz", "std", "map_insert"]
                                                    .into_iter()
                                                    .map(|t| Ident(t.to_string()))
                                                    .collect(),
                                            )),
                                            expr,
                                        )),
                                        vec![
                                            expr.as_ref().clone(),
                                            args[0].clone(),
                                            args[1].clone(),
                                        ],
                                        None,
                                        None,
                                    ),
                                    expr,
                                ))?)
                            }
                            (Type::Map(_, _), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::Call {
                                    callee: Box::new(self.expr(&mut Source::transfer(
                                        Expr::path(Path::new(vec![
                                            Ident("quartz".to_string()),
                                            Ident("std".to_string()),
                                            Ident("map_at".to_string()),
                                        ])),
                                        expr,
                                    ))?),
                                    args: vec![self.expr(expr)?, self.expr(&mut args[0])?],
                                    source: None,
                                })
                            }
                            (Type::Map(_, _), "has") => {
                                assert_eq!(args.len(), 1);

                                Ok(IrTerm::Call {
                                    callee: Box::new(self.expr(&mut Source::transfer(
                                        Expr::path(Path::new(vec![
                                            Ident("quartz".to_string()),
                                            Ident("std".to_string()),
                                            Ident("map_has".to_string()),
                                        ])),
                                        expr,
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
                                    callee: Box::new(self.expr(&mut Source::transfer(
                                        Expr::path(Path::new(vec![
                                            Ident("i32".to_string()),
                                            Ident(label.to_string()),
                                        ])),
                                        expr,
                                    ))?),
                                    args: elements,
                                    source: None,
                                })
                            }
                            _ => bail!("invalid project: {:?}", expr),
                        }
                    } else {
                        let mut args_with_self = vec![expr.as_ref().clone()];
                        args_with_self.extend(args.clone());

                        self.generate_call(
                            &mut Source::transfer(Expr::path(label.clone()), expr),
                            &mut args_with_self,
                            variadic,
                            expansion,
                        )
                    }
                }
                _ => self.generate_call(callee, args, variadic, expansion),
            },
            Expr::Record(ident, fields, expansion) => {
                let record_type = self
                    .types
                    .get(&ident.data)
                    .ok_or(anyhow!("Type not found: {:?}", ident))?
                    .clone()
                    .to_record()?;

                let expansion_term = if let Some(expansion) = expansion {
                    Some(self.expr(expansion)?)
                } else {
                    None
                };

                let mut record = vec![];
                for (i, type_) in record_type {
                    let ir_type = IrType::from_type(&type_)?;

                    if let Some((_, expr)) = fields.iter_mut().find(|(f, _)| f == &i) {
                        record.push((ir_type, self.expr(expr)?));
                    } else {
                        record.push((ir_type, expansion_term.clone().unwrap()));
                    }
                }

                Ok(self.generate_array_enumerated(record)?)
            }
            Expr::AnonymousRecord(fields, type_) => {
                let mut elements = vec![];
                for (label, type_) in type_.as_record_type().unwrap() {
                    let value = self.expr(
                        &mut fields
                            .iter_mut()
                            .find(|(f, _)| f == label)
                            .ok_or(anyhow!("Field not found: {:?}", label))?
                            .1,
                    )?;
                    elements.push((IrType::from_type(type_)?, value));
                }

                self.generate_array_enumerated(elements)
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

                Ok(IrTerm::Load {
                    type_: IrType::from_type(&record_type[index].1)?,
                    address: Box::new(self.expr(expr)?),
                    offset: Box::new(IrTerm::i32(offset as i32)),
                })
            }
            Expr::Make(type_, args) => match type_ {
                Type::Ptr(p) => {
                    assert_eq!(args.len(), 1);
                    let len = self.expr(&mut args[0])?;

                    Ok(IrTerm::Call {
                        callee: Box::new(self.expr(&mut Source::transfer(
                            Expr::path(Path::new(vec![
                                Ident("quartz".to_string()),
                                Ident("std".to_string()),
                                Ident("alloc".to_string()),
                            ])),
                            &args[0],
                        ))?),
                        args: vec![self.generate_mult_sizeof(p, len)?],
                        source: None,
                    })
                }
                Type::Array(elem, size) => {
                    let data_ptr = self.expr(&mut Source::unknown(Expr::Make(
                        Type::Ptr(elem.clone()),
                        vec![Source::unknown(Expr::Lit(Lit::I32(*size as i32)))],
                    )))?;

                    Ok(self.generate_array_enumerated(vec![
                        (IrType::from_type(&Type::Ptr(elem.clone()))?, data_ptr),
                        (IrType::from_type(&Type::I32)?, IrTerm::i32(*size as i32)),
                    ])?)
                }
                Type::Vec(_) => {
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
                        args: vec![cap],
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
            Expr::Wrap(type_, expr) => {
                let expr = self.expr(expr)?;

                self.generate_array_enumerated(vec![(IrType::from_type(type_)?, expr)])
            }
            Expr::Unwrap(type_, mode, expr) => match mode.clone().unwrap() {
                UnwrapMode::Optional => {
                    let expr = self.expr(expr)?;

                    Ok(IrTerm::Load {
                        type_: IrType::from_type(type_)?,
                        address: Box::new(expr),
                        offset: Box::new(IrTerm::i32(0)),
                    })
                }
                UnwrapMode::Or => {
                    let expr = self.expr(expr)?;

                    Ok(IrTerm::Load {
                        type_: IrType::from_type(type_)?,
                        address: Box::new(expr),
                        offset: Box::new(IrTerm::i32(1)),
                    })
                }
            },
            Expr::Omit(_) => todo!(),
            Expr::EnumOr(lhs_type, rhs_type, lhs, rhs) => {
                let lhs_term = if let Some(lhs) = lhs {
                    let lhs_term = self.expr(lhs)?;
                    self.generate_array_enumerated(vec![(IrType::from_type(lhs_type)?, lhs_term)])?
                } else {
                    IrTerm::Nil
                };
                let rhs_term = if let Some(rhs) = rhs {
                    let rhs_term = self.expr(rhs)?;
                    self.generate_array_enumerated(vec![(IrType::from_type(rhs_type)?, rhs_term)])?
                } else {
                    IrTerm::Nil
                };

                self.generate_array_enumerated(vec![
                    (IrType::Address, lhs_term),
                    (IrType::Address, rhs_term),
                ])
            }
            Expr::Try(expr) => {
                // expr.try
                // --> let try = expr;
                //     if try.right != nil { return try }
                //     try.left!
                let var_name = format!("try_{}", expr.start.unwrap_or(0));
                let left = IrTerm::Load {
                    type_: IrType::Address,
                    address: Box::new(IrTerm::Ident(var_name.clone())),
                    offset: Box::new(IrTerm::i32(0)),
                };
                let right = IrTerm::Load {
                    type_: IrType::Address,
                    address: Box::new(IrTerm::Ident(var_name.clone())),
                    offset: Box::new(IrTerm::i32(IrType::Address.sizeof() as i32)),
                };

                let mut elements = vec![];
                elements.push(IrTerm::Let {
                    name: var_name.clone(),
                    type_: IrType::Address,
                    value: Box::new(self.expr(&mut *expr)?),
                });
                elements.push(IrTerm::If {
                    cond: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("not_equal".to_string())),
                        args: vec![right.clone(), IrTerm::Nil],
                        source: None,
                    }),
                    type_: IrType::Nil,
                    then: Box::new(IrTerm::Return {
                        value: Box::new(IrTerm::ident(var_name.clone())),
                    }),
                    else_: Box::new(IrTerm::Seq { elements: vec![] }),
                });
                elements.push(IrTerm::Load {
                    type_: IrType::Address,
                    address: Box::new(left.clone()),
                    offset: Box::new(IrTerm::i32(0)),
                });

                Ok(IrTerm::Seq { elements })
            }
        }
    }

    fn generate_call(
        &mut self,
        callee: &mut Source<Expr>,
        args: &mut Vec<Source<Expr>>,
        variadic: &mut Option<VariadicCall>,
        expansion: &mut Option<Box<Source<Expr>>>,
    ) -> Result<IrTerm> {
        if let Some(variadic_call) = variadic {
            let mut elements = vec![];
            for arg in &mut args[0..variadic_call.index] {
                elements.push(self.expr(arg)?);
            }

            let vec_name = format!("vec_{}", callee.start.unwrap_or(0));

            let mut variadic_terms = vec![IrTerm::Let {
                name: vec_name.clone(),
                type_: IrType::from_type(&Type::Vec(Box::new(variadic_call.element_type.clone())))?,
                value: Box::new(IrTerm::Call {
                    callee: Box::new(self.expr(&mut Source::transfer(
                        Expr::path(Path::new(vec![
                            Ident("quartz".to_string()),
                            Ident("std".to_string()),
                            Ident("vec_make".to_string()),
                        ])),
                        callee,
                    ))?),
                    args: vec![IrTerm::I32(args.len() as i32 - variadic_call.index as i32)],
                    source: None,
                }),
            }];
            for arg in &mut args[variadic_call.index..] {
                variadic_terms.push(IrTerm::Discard {
                    element: Box::new(IrTerm::Call {
                        callee: Box::new(self.expr(&mut Source::transfer(
                            Expr::path(Path::new(vec![
                                Ident("quartz".to_string()),
                                Ident("std".to_string()),
                                Ident("vec_push".to_string()),
                            ])),
                            arg,
                        ))?),
                        args: vec![IrTerm::Ident(vec_name.clone()), self.expr(arg)?],
                        source: None,
                    }),
                });
            }

            if let Some(expansion) = expansion {
                variadic_terms.push(IrTerm::Discard {
                    element: Box::new(IrTerm::Call {
                        callee: Box::new(self.expr(&mut Source::transfer(
                            Expr::path(Path::new(vec![
                                Ident("quartz".to_string()),
                                Ident("std".to_string()),
                                Ident("vec_extend".to_string()),
                            ])),
                            expansion,
                        ))?),
                        args: vec![IrTerm::Ident(vec_name.clone()), self.expr(expansion)?],
                        source: None,
                    }),
                });
            }

            variadic_terms.push(IrTerm::Ident(vec_name.clone()));

            elements.push(IrTerm::Seq {
                elements: variadic_terms,
            });

            Ok(IrTerm::Call {
                callee: Box::new(self.expr(callee)?),
                args: elements,
                source: None,
            })
        } else {
            assert_eq!(variadic, &mut None);
            assert_eq!(expansion, &mut None);

            let mut elements = vec![];
            for arg in args {
                elements.push(self.expr(arg)?);
            }

            Ok(IrTerm::Call {
                callee: Box::new(self.expr(callee)?),
                args: elements,
                source: Some(SourcePosition {
                    path: self.current_path.clone(),
                    start: callee.start.unwrap_or(0),
                    end: callee.end.unwrap_or(0),
                }),
            })
        }
    }

    fn generate_array(&mut self, elements: Vec<(usize, IrType, IrTerm)>) -> Result<IrTerm> {
        // generates:
        //
        // let array_x = alloc(10);
        // for i in elements {
        //     array_x.at(i) = elements.at(i);
        // }
        // array_x
        let var_name = format!(
            "array_{}",
            Alphanumeric
                .sample_iter(&mut rand::thread_rng())
                .take(5)
                .map(char::from)
                .collect::<String>()
        );

        let mut array = vec![IrTerm::Let {
            name: var_name.clone(),
            type_: IrType::Address,
            value: Box::new(IrTerm::Call {
                callee: Box::new(IrTerm::ident(
                    Path::new(vec![
                        Ident("quartz".to_string()),
                        Ident("std".to_string()),
                        Ident("alloc".to_string()),
                    ])
                    .as_joined_str("_"),
                )),
                args: vec![self.generate_mult_sizeof(
                    &Type::Ident(Ident("string".to_string())),
                    IrTerm::i32(elements.len() as i32),
                )?],
                source: None,
            }),
        }];
        for (offset, type_, element) in elements {
            array.push(IrTerm::Store {
                type_,
                address: Box::new(IrTerm::ident(var_name.clone())),
                offset: Box::new(IrTerm::i32(offset as i32)),
                value: Box::new(element),
            });
        }

        array.push(IrTerm::ident(var_name.clone()));

        Ok(IrTerm::Seq { elements: array })
    }

    fn generate_array_enumerated(&mut self, elements: Vec<(IrType, IrTerm)>) -> Result<IrTerm> {
        let mut terms = vec![];
        let mut offset = 0;
        for (type_, elem) in elements {
            terms.push((offset, type_.clone(), elem));

            offset += type_.sizeof();
        }

        self.generate_array(terms)
    }

    fn generate_mult_sizeof(&mut self, type_: &Type, term: IrTerm) -> Result<IrTerm> {
        Ok(IrTerm::wrap_mult_sizeof(IrType::from_type(type_)?, term))
    }
}
