use std::{collections::HashMap, vec};

use anyhow::{anyhow, bail, Context, Result};
use rand::{distributions::Alphanumeric, prelude::Distribution};

use crate::{
    ast::{
        Decl, Expr, ForMode, Func, Lit, Module, Pattern, Statement, Type, UnwrapMode, VariadicCall,
    },
    compiler::{ErrorInSource, SourcePosition, MODE_TYPE_REP},
    ir::{IrTerm, IrType},
    util::{ident::Ident, path::Path, serial_id_map::SerialIdMap, source::Source},
    value::Value,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeRep {
    pub name: String,
    pub params: Vec<TypeRep>,
    pub fields: Vec<(String, TypeRep)>,
}

impl TypeRep {
    pub fn from_name(name: String, params: Vec<TypeRep>) -> TypeRep {
        TypeRep {
            name,
            params,
            fields: vec![],
        }
    }

    pub fn from_struct(
        name: String,
        params: Vec<IrType>,
        record_type: Vec<(String, IrType)>,
    ) -> Self {
        let params = params.into_iter().map(TypeRep::from_type).collect();

        let mut fields = vec![];
        for (name, type_) in record_type {
            fields.push((name, TypeRep::from_type(type_)));
        }

        TypeRep {
            name,
            params,
            fields,
        }
    }

    pub fn from_type(type_: IrType) -> TypeRep {
        match type_ {
            IrType::Nil => TypeRep::from_name("nil".to_string(), vec![]),
            IrType::I32 => TypeRep::from_name("i32".to_string(), vec![]),
            IrType::I64 => todo!(),
            IrType::Bool => TypeRep::from_name("bool".to_string(), vec![]),
            IrType::Address => TypeRep::from_name("address".to_string(), vec![]),
            IrType::Any => TypeRep::from_name("any".to_string(), vec![]),
            IrType::Byte => TypeRep::from_name("byte".to_string(), vec![]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrCodeGenerator {
    types: HashMap<Ident, (Vec<Type>, Type)>,
    current_path: Path,
    pub strings: SerialIdMap<String>,
    pub type_reps: SerialIdMap<TypeRep>,
    pub data_section_offset: usize,
}

impl IrCodeGenerator {
    pub fn new() -> Self {
        IrCodeGenerator {
            types: HashMap::new(),
            current_path: Path::empty(),
            strings: SerialIdMap::new(),
            type_reps: SerialIdMap::new(),
            data_section_offset: 0,
        }
    }

    pub fn set_types(&mut self, types: HashMap<Ident, (Vec<Type>, Type)>) {
        self.types = types;
    }

    pub fn run(&mut self, module: &mut Module) -> Result<IrTerm> {
        let mut decls = self.module(module)?;
        // generate_prepare_type_reps should be called before generate_prepare_strings, since it uses strings
        decls.push(self.generate_prepare_type_reps()?);
        decls.extend(self.generate_prepare_strings()?);
        decls.push(IrTerm::Func {
            name: "reflection_get_type_rep_id".to_string(),
            params: vec![("p".to_string(), IrType::Address)],
            result: Some(IrType::Address),
            body: vec![IrTerm::Return {
                value: Box::new(IrTerm::Load {
                    type_: IrType::Address,
                    address: Box::new(IrTerm::ident("p".to_string())),
                    offset: Box::new(IrTerm::i32(0)),
                    raw_offset: None,
                }),
            }],
        });
        decls.push(IrTerm::Func {
            name: "unsafe_load_ptr".to_string(),
            params: vec![
                ("p".to_string(), IrType::Address),
                ("offset".to_string(), IrType::I32),
            ],
            result: Some(IrType::Address),
            body: vec![IrTerm::Return {
                value: Box::new(IrTerm::Load {
                    type_: IrType::Address,
                    address: Box::new(IrTerm::ident("p".to_string())),
                    offset: Box::new(IrTerm::ident("offset".to_string())),
                    raw_offset: None,
                }),
            }],
        });

        Ok(IrTerm::Module { elements: decls })
    }

    fn module(&mut self, module: &mut Module) -> Result<Vec<IrTerm>> {
        let mut elements = vec![];

        for decl in &mut module.0 {
            match &mut decl.data {
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

    fn generate_prepare_type_reps(&mut self) -> Result<IrTerm> {
        let var_ptr = "quartz_std_type_reps_ptr";
        let mut type_reps = self.type_reps.keys.clone().into_iter().collect::<Vec<_>>();
        type_reps.sort_by(|(_, a), (_, b)| a.cmp(&b));

        let mut body = vec![];
        // quartz_std_type_reps_ptr = make[ptr[any]](${type_reps.len()});
        body.push(IrTerm::Assign {
            lhs: var_ptr.to_string(),
            rhs: Box::new(self.expr(&mut Source::unknown(Expr::Make(
                Type::Ptr(Box::new(Type::Any)),
                vec![Source::unknown(Expr::Lit(Lit::I32(type_reps.len() as i32)))],
            )))?),
        });

        for (type_rep, rep_id) in type_reps {
            let var_p = "p".to_string();

            // let p = write_type_rep(${rep});
            body.push(IrTerm::Let {
                name: var_p.clone(),
                type_: IrType::I32,
                value: Box::new(self.write_type_rep(type_rep)?),
            });

            // quartz_std_type_reps_ptr.at(${rep_id}) = p
            let lhs = self.generate_array_at(
                &Type::Any,
                IrTerm::ident(var_ptr.to_string()),
                IrTerm::i32(rep_id as i32),
            )?;
            body.push(self.assign(lhs, IrType::Address, IrTerm::ident(var_p.clone()))?);
        }

        body.push(IrTerm::Return {
            value: Box::new(IrTerm::nil()),
        });

        Ok(IrTerm::Func {
            name: "prepare_type_reps".to_string(),
            params: vec![],
            result: None,
            body,
        })
    }

    fn generate_prepare_strings(&mut self) -> Result<Vec<IrTerm>> {
        let mut terms = vec![];

        let var_strings = "quartz_std_strings_ptr";
        let mut body = vec![];
        // quartz_std_strings_ptr = make[ptr[string]](${strings.len()});
        body.push(IrTerm::Assign {
            lhs: var_strings.to_string(),
            rhs: Box::new(self.expr(&mut Source::unknown(Expr::Make(
                Type::Ptr(Box::new(Type::Ident(Ident("string".to_string())))),
                vec![Source::unknown(Expr::Lit(Lit::I32(
                    self.strings.keys.len() as i32,
                )))],
            )))?),
        });

        // avoid 0-8 for null pointer
        self.data_section_offset = 8;

        for (string, i) in self.strings.to_vec() {
            let string_len = string.len();
            // add 8 bytes padding for object header
            // In quartz, ptr.at will load with offset=8
            let string_memory_size = string_len + 8;
            let string = "\00\00\00\00\00\00\00\00".to_string() + &string;
            terms.push(IrTerm::Data {
                offset: self.data_section_offset,
                data: string,
            });

            // strings.at(i) = new_empty_string(${offset}, ${string.len()})
            let lhs = self.generate_array_at(
                &Type::Ident(Ident("string".to_string())),
                IrTerm::ident(var_strings.to_string()),
                IrTerm::i32(i as i32),
            )?;
            body.push(self.assign(
                lhs,
                IrType::Address,
                IrTerm::Call {
                    callee: Box::new(IrTerm::ident("quartz_std_new_string".to_string())),
                    args: vec![
                        IrTerm::i32(self.data_section_offset as i32),
                        IrTerm::i32(string_len as i32),
                    ],
                    source: None,
                },
            )?);

            self.data_section_offset += string_memory_size;
        }

        body.push(IrTerm::Return {
            value: Box::new(IrTerm::nil()),
        });

        terms.push(IrTerm::Func {
            name: "prepare_strings".to_string(),
            params: vec![],
            result: None,
            body,
        });

        Ok(terms)
    }

    fn func(&mut self, func: &mut Func) -> Result<IrTerm> {
        let elements = self.statements(&mut func.body.data)?;

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
        path.push(func.name.data.clone());

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
            Statement::Let(pattern, type_, expr) => match &mut pattern.data {
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
                    let lhs = lhs_pattern.data.as_ident().unwrap();
                    let rhs = rhs_pattern.data.as_ident().unwrap();

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
                                value: Box::new(self.generate_array_at(
                                    &lhs_type,
                                    IrTerm::Ident(var_name.clone()),
                                    IrTerm::i32(0),
                                )?),
                            },
                            IrTerm::Let {
                                name: rhs.0.clone(),
                                type_: IrType::Address,
                                value: Box::new(self.generate_array_at(
                                    &rhs_type,
                                    IrTerm::Ident(var_name.clone()),
                                    IrTerm::i32(1),
                                )?),
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
            Statement::Assign(lhs, rhs_type, rhs) => {
                let lhs = self.expr(lhs)?;
                let rhs = self.expr(rhs)?;

                self.assign(lhs, IrType::from_type(rhs_type)?, rhs)
            }
            Statement::If(cond, type_, then_block, else_block) => {
                let mut then_elements = vec![];
                for statement in &mut then_block.data {
                    then_elements.push(self.statement(statement)?);
                }

                let mut else_elements = vec![];
                if let Some(else_block) = else_block {
                    for statement in &mut else_block.data {
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
                for statement in &mut block.data {
                    elements.push(self.statement(statement)?);
                }

                Ok(IrTerm::While {
                    cond: Box::new(self.expr(cond)?),
                    body: Box::new(IrTerm::Seq { elements }),
                    cleanup: None,
                })
            }
            Statement::For(mode, ident, range, body) => match mode.clone().unwrap() {
                ForMode::Range => match &mut range.data {
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
                                    elements: self.statements(&mut body.data)?,
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
                ForMode::Vec(vec_elem_type) => {
                    // for v in vec { body }
                    // =>
                    // for i in 0..vec.length {
                    //   let v = vec.at(i);
                    //   body
                    // }
                    let var_index =
                        format!("index_{}_{}", ident.as_str(), range.start.unwrap_or(0));

                    let mut new_body = vec![Source::unknown(Statement::Let(
                        Source::unknown(Pattern::Ident(ident.clone())),
                        vec_elem_type.clone(),
                        Source::unknown(Expr::Call(
                            Box::new(Source::transfer(
                                Expr::Project(
                                    Box::new(range.clone()),
                                    Type::Vec(Box::new(vec_elem_type.clone())),
                                    Source::unknown(Path::ident(Ident("at".to_string()))),
                                ),
                                range,
                            )),
                            vec![Source::unknown(Expr::ident(Ident(var_index.clone())))],
                            None,
                            None,
                        )),
                    ))];
                    new_body.extend(body.data.clone());

                    self.statement(&mut Source::transfer(
                        Statement::For(
                            Some(ForMode::Range),
                            Ident(var_index.clone()),
                            Source::transfer(
                                Expr::Range(
                                    Box::new(Source::unknown(Expr::Lit(Lit::I32(0)))),
                                    Box::new(Source::unknown(Expr::Project(
                                        Box::new(range.clone()),
                                        Type::Vec(Box::new(vec_elem_type)),
                                        Source::unknown(Path::ident(Ident("length".to_string()))),
                                    ))),
                                ),
                                range,
                            ),
                            Source::transfer(new_body.clone(), &body),
                        ),
                        statement,
                    ))
                }
            },
            Statement::Continue => Ok(IrTerm::Continue),
            Statement::Break => Ok(IrTerm::Break),
        }
    }

    fn assign(&mut self, lhs: IrTerm, rhs_type: IrType, rhs: IrTerm) -> Result<IrTerm> {
        match lhs {
            IrTerm::Ident(ident) => Ok(IrTerm::Assign {
                lhs: ident,
                rhs: Box::new(rhs),
            }),
            IrTerm::Call { .. } => Ok(IrTerm::Store {
                type_: rhs_type,
                address: Box::new(lhs),
                offset: Box::new(IrTerm::i32(0)),
                value: Box::new(rhs),
                raw_offset: Some(if MODE_TYPE_REP { 8 } else { 0 }),
            }),
            IrTerm::Load {
                type_,
                address,
                offset,
                raw_offset,
            } => Ok(IrTerm::Store {
                type_,
                address,
                offset,
                value: Box::new(rhs),
                raw_offset,
            }),
            _ => bail!("invalid lhs for assignment: {}", lhs.to_string()),
        }
    }

    fn register_string(&mut self, s: String) -> IrTerm {
        let index = self.strings.get_or_insert(s);

        IrTerm::String(index)
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
                Lit::Nil(_) => Ok(IrTerm::nil()),
                Lit::Bool(b) => Ok(IrTerm::Bool(*b)),
                Lit::I32(i) => Ok(IrTerm::i32(*i)),
                Lit::I32Base2(i) => Ok(IrTerm::i32(*i)),
                Lit::U32(i) => Ok(IrTerm::u32(*i)),
                Lit::String(s, _) => Ok(self.register_string(s.clone())),
            },
            Expr::Not(expr) => {
                let expr = self.expr(expr)?;

                Ok(IrTerm::Call {
                    callee: Box::new(IrTerm::Ident("not".to_string())),
                    args: vec![expr],
                    source: None,
                })
            }
            Expr::BinOp(op, type_, arg1, arg2) => {
                use crate::ast::BinOp::*;

                let arg1 = self.expr(arg1)?;
                let arg2 = self.expr(arg2)?;

                match op {
                    Add => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "add"
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
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
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
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
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
                            } else {
                                bail!("invalid type for gte: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    Equal => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("equal".to_string())),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    NotEqual => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("not_equal".to_string())),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    BitOr => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "bit_or"
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
                                "bit_or_i64"
                            } else {
                                bail!("invalid type for bit_or: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    BitAnd => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "bit_and"
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
                                "bit_and_i64"
                            } else {
                                bail!("invalid type for bit_or: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    BitShiftL => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "bit_shift_left"
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
                                "bit_shift_left_i64"
                            } else {
                                bail!("invalid type for bit_or: {:?}", type_)
                            }
                            .to_string(),
                        )),
                        args: vec![arg1, arg2],
                        source: None,
                    }),
                    BitShiftR => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            if matches!(type_, Type::I32) {
                                "bit_shift_right"
                            } else if *type_ == Type::Ident(Ident("i64".to_string())) {
                                "bit_shift_right_i64"
                            } else {
                                bail!("invalid type for bit_or: {:?}", type_)
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
                    if label.data.0.len() == 1 {
                        match (type_, label.data.0[0].as_str()) {
                            (Type::Ptr(p), "at") => {
                                assert_eq!(args.len(), 1);

                                let ptr = self.expr(expr)?;
                                let offset = self.expr(&mut args[0])?;

                                Ok(self.generate_array_at(&p, ptr, offset)?)
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

                                let array = self.expr(&mut Source::transfer(
                                    Expr::Project(
                                        expr.clone(),
                                        Type::Array(p.clone(), *s),
                                        Source::unknown(Path::ident(Ident("data".to_string()))),
                                    ),
                                    expr,
                                ))?;
                                let offset = self.expr(&mut args[0])?;

                                Ok(self.generate_array_at(p, array, offset)?)
                            }
                            (Type::Vec(p), "at") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec![
                                                    "quartz",
                                                    "std",
                                                    if matches!(**p, Type::Byte) {
                                                        "vec_at_byte"
                                                    } else {
                                                        "vec_at"
                                                    },
                                                ]
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
                            (Type::Vec(p), "push") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec![
                                                    "quartz",
                                                    "std",
                                                    if matches!(**p, Type::Byte) {
                                                        "vec_push_byte"
                                                    } else {
                                                        "vec_push"
                                                    },
                                                ]
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
                            (Type::Vec(p), "extend") => {
                                assert_eq!(args.len(), 1);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec![
                                                    "quartz",
                                                    "std",
                                                    if matches!(**p, Type::Byte) {
                                                        "vec_extend_byte"
                                                    } else {
                                                        "vec_extend"
                                                    },
                                                ]
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
                            (Type::Vec(_), "slice") => {
                                assert_eq!(args.len(), 2);

                                Ok(self.expr(&mut Source::transfer(
                                    Expr::Call(
                                        Box::new(Source::transfer(
                                            Expr::path(Path::new(
                                                vec!["quartz", "std", "vec_slice"]
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
                            (Type::Map(_, _), "list_keys") => {
                                assert_eq!(args.len(), 0);

                                Ok(IrTerm::Call {
                                    callee: Box::new(self.expr(&mut Source::transfer(
                                        Expr::path(Path::new(vec![
                                            Ident("quartz".to_string()),
                                            Ident("std".to_string()),
                                            Ident("map_list_keys".to_string()),
                                        ])),
                                        expr,
                                    ))?),
                                    args: vec![self.expr(expr)?],
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
                            (type_, label) => {
                                bail!("invalid project: {:?}, {:?}.{}", type_, expr, label)
                            }
                        }
                    } else {
                        let mut args_with_self = vec![expr.as_ref().clone()];
                        args_with_self.extend(args.clone());

                        self.generate_call(
                            &mut Source::transfer(Expr::path(label.data.clone()), expr),
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
                    .1
                    .to_record()?;

                let expansion_term = if let Some(expansion) = expansion {
                    Some(self.expr(expansion)?)
                } else {
                    None
                };

                let mut record = vec![];
                for (i, type_) in record_type {
                    let ir_type = IrType::from_type(&type_.data)?;

                    if let Some((_, expr)) = fields.iter_mut().find(|(f, _)| f == &i) {
                        record.push((Some(i.0), ir_type, self.expr(expr)?));
                    } else {
                        record.push((Some(i.0), ir_type, expansion_term.clone().unwrap()));
                    }
                }

                Ok(self.generate_array(ident.data.as_str().to_string(), vec![], record)?)
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
                    elements.push((
                        Some(label.0.clone()),
                        IrType::from_type(&type_.data)?,
                        value,
                    ));
                }

                self.generate_array("struct".to_string(), vec![], elements)
            }
            Expr::Project(expr, type_, label) => {
                let record_type =
                    if let Ok(record) = type_.as_record_type() {
                        record.clone()
                    } else if let Ok(ident) = type_.clone().to_ident() {
                        self.types
                            .get(&ident)
                            .ok_or(anyhow!("Type not found: {:?}", type_))?
                            .clone()
                            .1
                            .to_record()?
                    } else {
                        return Err(anyhow!("invalid project: {:?} for {:?}", expr, type_)
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: expr.start.unwrap_or(0),
                                end: expr.end.unwrap_or(0),
                            }));
                    };

                let index = record_type
                    .iter()
                    .position(|(f, _)| Path::ident(f.clone()) == label.data)
                    .ok_or(anyhow!("Field not found: {:?} in {:?}", label, record_type))?;

                let root = self.expr(expr)?;
                Ok(self.generate_array_at(
                    &record_type[index].1.data,
                    root,
                    IrTerm::i32(index as i32),
                )?)
            }
            Expr::Make(type_, args) => match type_ {
                Type::Ptr(p) => {
                    assert_eq!(args.len(), 1);
                    let len = self.expr(&mut args[0])?;

                    Ok(self.allocate_heap_object(
                        TypeRep::from_name(
                            "ptr".to_string(),
                            vec![TypeRep::from_type(IrType::from_type(p)?)],
                        ),
                        IrType::from_type(p)?,
                        len,
                    )?)
                }
                Type::Array(elem, size) => {
                    let data_ptr = self.expr(&mut Source::unknown(Expr::Make(
                        Type::Ptr(elem.clone()),
                        vec![Source::unknown(Expr::Lit(Lit::I32(*size as i32)))],
                    )))?;

                    Ok(self.generate_array(
                        "array".to_string(),
                        vec![IrType::from_type(elem)?],
                        vec![
                            (
                                Some("data".to_string()),
                                IrType::from_type(&Type::Ptr(elem.clone()))?,
                                data_ptr,
                            ),
                            (
                                Some("length".to_string()),
                                IrType::from_type(&Type::I32)?,
                                IrTerm::i32(*size as i32),
                            ),
                        ],
                    )?)
                }
                Type::Vec(_) => {
                    let var_vec = format!(
                        "vec_{}",
                        Alphanumeric
                            .sample_iter(&mut rand::thread_rng())
                            .take(5)
                            .map(char::from)
                            .collect::<String>()
                    );

                    let mut elements = vec![IrTerm::Let {
                        name: var_vec.clone(),
                        type_: IrType::from_type(type_)?,
                        value: Box::new(IrTerm::Call {
                            callee: Box::new(IrTerm::Ident(
                                Path::new(
                                    vec!["quartz", "std", "vec_make"]
                                        .into_iter()
                                        .map(|s| Ident(s.to_string()))
                                        .collect(),
                                )
                                .as_joined_str("_"),
                            )),
                            args: vec![IrTerm::i32(5)],
                            source: None,
                        }),
                    }];
                    for arg in args {
                        elements.push(
                            self.statement(&mut Source::transfer(
                                Statement::Expr(
                                    Source::transfer(
                                        Expr::Call(
                                            Box::new(Source::unknown(Expr::path(Path::new(
                                                vec!["quartz", "std", "vec_push"]
                                                    .into_iter()
                                                    .map(|s| Ident(s.to_string()))
                                                    .collect(),
                                            )))),
                                            vec![
                                                Source::unknown(Expr::ident(Ident(
                                                    var_vec.clone(),
                                                ))),
                                                arg.clone(),
                                            ],
                                            None,
                                            None,
                                        ),
                                        arg,
                                    ),
                                    Type::Nil,
                                ),
                                arg,
                            ))?,
                        );
                    }
                    elements.push(IrTerm::Ident(var_vec));

                    Ok(IrTerm::Seq { elements })
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
            Expr::As(expr, source, target) => {
                let term = self.expr(expr)?;

                match (IrType::from_type(source)?, IrType::from_type(target)?) {
                    (IrType::I32, IrType::Address) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("i32_to_address".to_string())),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::Address, IrType::I32) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("address_to_i32".to_string())),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::I32, IrType::Byte) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("i32_to_byte".to_string())),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::Byte, IrType::I32) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("byte_to_i32".to_string())),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::Byte, IrType::Address) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("byte_to_address".to_string())),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::I32, IrType::I64) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            Path::new(
                                vec!["quartz", "std", "i32_to_i64"]
                                    .into_iter()
                                    .map(|s| Ident(s.to_string()))
                                    .collect(),
                            )
                            .as_joined_str("_"),
                        )),
                        args: vec![term],
                        source: None,
                    }),
                    (IrType::I64, IrType::I32) => Ok(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident(
                            Path::new(
                                vec!["quartz", "std", "i64_to_i32"]
                                    .into_iter()
                                    .map(|s| Ident(s.to_string()))
                                    .collect(),
                            )
                            .as_joined_str("_"),
                        )),
                        args: vec![term],
                        source: None,
                    }),
                    (_, IrType::Any) => Ok(term),
                    (IrType::Any, _) => Ok(term),
                    (source, target) if source == target => Ok(term),
                    (source, target) => {
                        return Err(anyhow!("unsupported as: {:?} -> {:?}", source, target)
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: expr.start.unwrap_or(0),
                                end: expr.end.unwrap_or(0),
                            }));
                    }
                }
            }
            Expr::SizeOf(type_) => {
                let type_ = IrType::from_type(type_)?;
                Ok(IrTerm::SizeOf { type_ })
            }
            Expr::Wrap(type_, expr) => {
                let expr = self.expr(expr)?;

                self.generate_array(
                    "optional".to_string(),
                    vec![IrType::from_type(type_)?],
                    vec![(None, IrType::from_type(type_)?, expr)],
                )
            }
            Expr::Unwrap(type_, mode, expr) => match mode.clone().unwrap() {
                UnwrapMode::Optional => {
                    let expr = self.expr(expr)?;
                    Ok(self.generate_array_at(type_, expr, IrTerm::i32(0))?)
                }
                UnwrapMode::Or => {
                    let expr = self.expr(expr)?;
                    Ok(self.generate_array_at(type_, expr, IrTerm::i32(1))?)
                }
            },
            Expr::Omit(_) => todo!(),
            Expr::EnumOr(lhs_type, rhs_type, lhs, rhs) => {
                let lhs_term = if let Some(lhs) = lhs {
                    let lhs_term = self.expr(lhs)?;
                    self.generate_array(
                        "optional".to_string(),
                        vec![IrType::from_type(lhs_type)?],
                        vec![(None, IrType::from_type(lhs_type)?, lhs_term)],
                    )?
                } else {
                    IrTerm::Nil
                };
                let rhs_term = if let Some(rhs) = rhs {
                    let rhs_term = self.expr(rhs)?;
                    self.generate_array(
                        "optional".to_string(),
                        vec![IrType::from_type(rhs_type)?],
                        vec![(None, IrType::from_type(rhs_type)?, rhs_term)],
                    )?
                } else {
                    IrTerm::Nil
                };

                self.generate_array(
                    "or".to_string(),
                    vec![IrType::from_type(lhs_type)?, IrType::from_type(rhs_type)?],
                    vec![
                        (Some("left".to_string()), IrType::Address, lhs_term),
                        (Some("right".to_string()), IrType::Address, rhs_term),
                    ],
                )
            }
            Expr::Try(expr) => {
                // expr.try
                // --> let try = expr;
                //     if try.right != nil { return _ or right }
                //     try.left!
                let var_name = format!("try_{}", expr.start.unwrap_or(0));
                let left = self.generate_array_at(
                    &Type::Ptr(Box::new(Type::Omit(0))),
                    IrTerm::Ident(var_name.clone()),
                    IrTerm::i32(0),
                )?;
                let right = self.generate_array_at(
                    &Type::Ptr(Box::new(Type::Omit(0))),
                    IrTerm::Ident(var_name.clone()),
                    IrTerm::i32(1),
                )?;

                let right_name = format!("try_{}_right", expr.start.unwrap_or(0));

                let mut elements = vec![];
                elements.push(IrTerm::Let {
                    name: var_name.clone(),
                    type_: IrType::Address,
                    value: Box::new(self.expr(expr)?),
                });
                elements.push(IrTerm::Let {
                    name: right_name.clone(),
                    type_: IrType::Address,
                    value: Box::new(right),
                });
                elements.push(IrTerm::If {
                    cond: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::Ident("not_equal".to_string())),
                        args: vec![IrTerm::ident(right_name.clone()), IrTerm::Nil],
                        source: None,
                    }),
                    type_: IrType::Nil,
                    then: Box::new(IrTerm::Return {
                        value: Box::new(self.generate_array(
                            "or".to_string(),
                            vec![IrType::Address, IrType::Address],
                            vec![
                                (Some("left".to_string()), IrType::Address, IrTerm::nil()),
                                (
                                    Some("right".to_string()),
                                    IrType::Address,
                                    IrTerm::ident(right_name),
                                ),
                            ],
                        )?),
                    }),
                    else_: Box::new(IrTerm::Seq { elements: vec![] }),
                });
                elements.push(self.generate_array_at(
                    &Type::Ptr(Box::new(Type::Omit(0))),
                    left.clone(),
                    IrTerm::i32(0),
                )?);

                Ok(IrTerm::Seq { elements })
            }
            Expr::Paren(p) => self.expr(p),
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
                    args: vec![IrTerm::I32(1)],
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

    fn generate_array(
        &mut self,
        rep_name: String,
        params: Vec<IrType>,
        elements: Vec<(Option<String>, IrType, IrTerm)>,
    ) -> Result<IrTerm> {
        let rep = TypeRep::from_struct(
            rep_name,
            params,
            elements
                .iter()
                .enumerate()
                .map(|(i, (l, t, _))| (l.clone().unwrap_or(format!("{}", i)), t.clone()))
                .collect::<Vec<_>>(),
        );

        let mut terms = vec![];
        for (index, (_, type_, elem)) in elements.into_iter().enumerate() {
            terms.push((index, type_.clone(), elem));
        }

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
            value: Box::new(self.allocate_heap_object(
                rep,
                IrType::Address,
                IrTerm::i32(terms.len() as i32),
            )?),
        }];
        for (offset, type_, element) in terms {
            array.push(IrTerm::Store {
                type_,
                address: Box::new(IrTerm::ident(var_name.clone())),
                offset: Box::new(
                    self.generate_mult_sizeof(&Type::I32, IrTerm::i32(offset as i32))?,
                ),
                value: Box::new(element),
                raw_offset: Some(if MODE_TYPE_REP { Value::sizeof() } else { 0 }),
            });
        }

        array.push(IrTerm::ident(var_name.clone()));

        Ok(IrTerm::Seq { elements: array })
    }

    fn generate_mult_sizeof(&mut self, type_: &Type, term: IrTerm) -> Result<IrTerm> {
        Ok(IrTerm::wrap_mult_sizeof(IrType::from_type(type_)?, term))
    }

    fn generate_array_at(
        &mut self,
        elem_type: &Type,
        array: IrTerm,
        index: IrTerm,
    ) -> Result<IrTerm> {
        Ok(IrTerm::Load {
            type_: IrType::from_type(elem_type)?,
            address: Box::new(array),
            offset: Box::new(self.generate_mult_sizeof(elem_type, index)?),
            raw_offset: Some(if MODE_TYPE_REP { Value::sizeof() } else { 0 }),
        })
    }

    fn allocate_heap_object(
        &mut self,
        rep: TypeRep,
        type_: IrType,
        size: IrTerm,
    ) -> Result<IrTerm> {
        let rep_id = self.type_reps.get_or_insert(rep);

        let var = format!(
            "object_{}",
            Alphanumeric
                .sample_iter(&mut rand::thread_rng())
                .take(5)
                .map(char::from)
                .collect::<String>()
        );

        let mut object = vec![IrTerm::Let {
            name: var.clone(),
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
                args: vec![IrTerm::Call {
                    callee: Box::new(IrTerm::ident("add".to_string())),
                    args: vec![
                        IrTerm::wrap_mult_sizeof(type_, size),
                        IrTerm::i32(if MODE_TYPE_REP { Value::sizeof() } else { 0 } as i32),
                    ],
                    source: None,
                }],
                source: None,
            }),
        }];
        // object header
        if MODE_TYPE_REP {
            object.push(IrTerm::Store {
                type_: IrType::Address,
                address: Box::new(IrTerm::ident(var.clone())),
                offset: Box::new(IrTerm::i32(0)),
                value: Box::new(IrTerm::i32(rep_id as i32)),
                raw_offset: None,
            });
        }

        object.push(IrTerm::ident(var.clone()));

        Ok(IrTerm::Seq { elements: object })
    }

    fn write_type_rep(&mut self, rep: TypeRep) -> Result<IrTerm> {
        // [nil, rep_name, size of params, *params, size of fields, *fields]
        let mut elements = vec![IrTerm::nil(), self.register_string(rep.name.clone())];

        elements.push(IrTerm::i32(rep.params.len() as i32));
        let mut type_rep_terms = vec![];
        for p in rep.params {
            type_rep_terms.push((None, IrType::Address, self.write_type_rep(p)?));
        }
        elements.push(self.generate_array("slice".to_string(), vec![], type_rep_terms)?);

        elements.push(IrTerm::i32(rep.fields.len() as i32));
        let mut field_terms = vec![];
        for (name, _) in rep.fields {
            field_terms.push((None, IrType::Address, self.register_string(name)));
        }
        elements.push(self.generate_array("slice".to_string(), vec![], field_terms)?);

        let var_name = format!(
            "type_rep_{}",
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
                args: vec![
                    self.generate_mult_sizeof(&Type::Any, IrTerm::i32(elements.len() as i32))?
                ],
                source: None,
            }),
        }];
        for (index, element) in elements.into_iter().enumerate() {
            array.push(IrTerm::Store {
                type_: IrType::Any,
                address: Box::new(IrTerm::ident(var_name.clone())),
                offset: Box::new(self.generate_mult_sizeof(&Type::I32, IrTerm::i32(index as i32))?),
                value: Box::new(element),
                raw_offset: None,
            });
        }

        array.push(IrTerm::ident(var_name.clone()));

        Ok(IrTerm::Seq { elements: array })
    }
}
