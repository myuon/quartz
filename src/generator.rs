use std::collections::{HashMap, HashSet};

use anyhow::{bail, Ok, Result};

use crate::{
    ast::Type,
    ir::{IrTerm, IrType},
    util::{ident::Ident, path::Path, sexpr_writer::SExprWriter},
    value::Value,
};

pub struct Generator {
    pub writer: SExprWriter,
    pub globals: HashSet<String>,
    pub types: HashMap<Ident, Type>,
    pub main_signature: Option<(Vec<IrType>, IrType)>,
    pub cwd: Path,
    pub strings: Vec<String>,
    pub entrypoint_symbol: String,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: SExprWriter::new(),
            globals: HashSet::new(),
            types: HashMap::new(),
            main_signature: None,
            cwd: Path::new(vec![]),
            strings: vec![],
            entrypoint_symbol: String::new(),
        }
    }

    pub fn set_globals(&mut self, globals: HashSet<Path>) {
        self.globals = globals
            .iter()
            .map(|p| p.as_joined_str("_"))
            .collect::<HashSet<_>>();
    }

    pub fn set_types(&mut self, types: HashMap<Ident, Type>) {
        self.types = types;
    }

    pub fn set_cwd(&mut self, cwd: Path) {
        self.cwd = cwd;
    }

    pub fn set_strings(&mut self, strings: Vec<String>) {
        self.strings = strings;
    }

    pub fn run(&mut self, module: &mut IrTerm) -> Result<()> {
        match module {
            IrTerm::Module { elements } => {
                self.module(elements)?;
            }
            _ => bail!("Expected module"),
        }

        Ok(())
    }

    fn module(&mut self, elements: &mut Vec<IrTerm>) -> Result<()> {
        self.entrypoint_symbol = format!(
            "{}_main",
            self.cwd
                .0
                .iter()
                .map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join("_")
        );

        self.writer.start();
        self.writer.write("module");

        self.decl(&mut IrTerm::Declare {
            name: "write_stdout".to_string(),
            params: vec![IrType::I32],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "debug_i32".to_string(),
            params: vec![IrType::I32],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "read_stdin".to_string(),
            params: vec![],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "abort".to_string(),
            params: vec![],
            result: IrType::I32,
        })?;

        self.writer.start();
        self.writer.write(r#"memory 1"#);
        self.writer.end();

        for term in elements {
            self.decl(term)?;
        }

        // builtin functions here
        self.decl(&mut IrTerm::Func {
            name: "mem_copy".to_string(),
            params: vec![
                ("source".to_string(), IrType::I32),
                ("target".to_string(), IrType::I32),
                ("length".to_string(), IrType::I32),
            ],
            result: Box::new(IrType::I32),
            body: vec![
                IrTerm::ident("target"),
                IrTerm::ident("source"),
                IrTerm::ident("length"),
                IrTerm::Instruction("memory.copy".to_string()),
                IrTerm::Return {
                    value: Box::new(IrTerm::nil()),
                },
            ],
        })?;
        self.decl(&mut IrTerm::Func {
            name: "mem_free".to_string(),
            params: vec![("address".to_string(), IrType::I32)],
            result: Box::new(IrType::I32),
            body: vec![IrTerm::Return {
                value: Box::new(IrTerm::nil()),
            }],
        })?;

        // prepare strings
        let var_strings = "quartz_std_strings_ptr";

        self.writer.start();
        self.writer.write("func $prepare_strings");

        self.writer.new_statement();
        self.writer
            .write(format!("(local $p {})", IrType::I32.to_string()));

        self.expr(&mut IrTerm::Call {
            callee: Box::new(IrTerm::ident("quartz_std_alloc".to_string())),
            args: vec![IrTerm::wrap_mult_sizeof(
                IrType::Address,
                IrTerm::i32(self.strings.len() as i32),
            )],
            source: None,
        })?;

        self.writer.new_statement();
        self.writer.write(format!("global.set ${}", var_strings));

        for (i, string) in self.strings.clone().iter().enumerate() {
            self.writer.new_statement();
            self.writer.write(format!(";; {:?}", string));

            self.expr(&mut IrTerm::Call {
                callee: Box::new(IrTerm::ident("quartz_std_new_empty_string")),
                args: vec![IrTerm::i32(string.len() as i32)],
                source: None,
            })?;

            self.writer.new_statement();
            self.writer.write("local.set $p");

            self.expr(&mut IrTerm::WriteMemory {
                type_: IrType::I32,
                address: Box::new(IrTerm::Load {
                    type_: IrType::I32,
                    address: Box::new(IrTerm::ident("p")),
                    offset: Box::new(IrTerm::i32(0)),
                }),
                value: string.bytes().map(|b| IrTerm::i32(b as i32)).collect(),
            })?;

            self.expr(&mut IrTerm::Store {
                type_: IrType::I32,
                address: Box::new(IrTerm::ident(var_strings)),
                offset: Box::new(IrTerm::Call {
                    callee: Box::new(IrTerm::ident("mult")),
                    args: vec![IrTerm::i32(i as i32), IrTerm::SizeOf { type_: IrType::I32 }],
                    source: None,
                }),
                value: Box::new(IrTerm::ident("p")),
            })?;
        }

        self.writer.end();

        self.decl(&mut IrTerm::Func {
            name: "load_string".to_string(),
            params: vec![("index".to_string(), IrType::I32)],
            result: Box::new(IrType::I32),
            body: vec![IrTerm::Return {
                value: Box::new(IrTerm::Load {
                    type_: IrType::I32,
                    address: Box::new(IrTerm::ident(var_strings)),
                    offset: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::ident("mult")),
                        args: vec![
                            IrTerm::ident("index"),
                            IrTerm::SizeOf { type_: IrType::I32 },
                        ],
                        source: None,
                    }),
                }),
            }],
        })?;

        let (_, result) = self.main_signature.clone().unwrap();

        self.decl(&mut IrTerm::Func {
            name: "start".to_string(),
            params: vec![],
            result: Box::new(result),
            body: vec![
                IrTerm::Assign {
                    lhs: "quartz_std_strings_count".to_string(),
                    rhs: Box::new(IrTerm::i32(self.strings.len() as i32)),
                },
                IrTerm::i32(2),
                IrTerm::Discard {
                    element: Box::new(IrTerm::Instruction("memory.grow".to_string())),
                },
                IrTerm::Call {
                    callee: Box::new(IrTerm::ident("prepare_strings")),
                    args: vec![],
                    source: None,
                },
                IrTerm::Return {
                    value: Box::new(IrTerm::Call {
                        callee: Box::new(IrTerm::ident(self.entrypoint_symbol.clone())),
                        args: vec![],
                        source: None,
                    }),
                },
            ],
        })?;

        self.writer.start();
        self.writer.write(r#"export "main" (func $start)"#);
        self.writer.end();
        self.writer.end();

        self.writer.finalize();

        Ok(())
    }

    fn decl(&mut self, decl: &mut IrTerm) -> Result<()> {
        match decl {
            IrTerm::Func {
                name,
                params,
                result,
                body,
            } => self.func(name, params, result, body),
            IrTerm::GlobalLet { name, type_, value } => self.global_let(name, type_, value),
            IrTerm::Module { elements } => {
                for element in elements {
                    self.decl(element)?;
                }

                Ok(())
            }
            IrTerm::Declare {
                name,
                params,
                result,
            } => {
                self.writer.start();
                self.writer.write("import");
                self.writer.write("\"env\"");
                self.writer.write(&format!("\"{}\"", name.as_str()));

                self.writer.start();
                self.writer.write("func");
                self.writer.write(&format!("${}", name.as_str()));
                for type_ in params {
                    self.writer.write(&format!("(param {})", type_.to_string()));
                }
                self.writer
                    .write(&format!("(result {})", result.to_string()));
                self.writer.end();
                self.writer.end();

                Ok(())
            }
            _ => bail!("Expected func or global let, got {:?}", decl),
        }
    }

    fn func(
        &mut self,
        name: &mut String,
        params: &mut Vec<(String, IrType)>,
        result: &mut IrType,
        body: &mut Vec<IrTerm>,
    ) -> Result<()> {
        if name == &self.entrypoint_symbol {
            self.main_signature = Some((
                params.iter().map(|(_, t)| t.clone()).collect(),
                result.clone(),
            ));
        }

        self.writer.start();
        self.writer.write("func");
        self.writer.write(&format!("${}", name.as_str()));
        for (name, type_) in params {
            self.writer
                .write(&format!("(param ${} {})", name.as_str(), type_.to_string()));
        }

        self.writer
            .write(&format!("(result {})", result.to_string()));

        let mut used = HashSet::new();
        for term in body.iter() {
            for (name, type_, _) in term.find_let() {
                if used.contains(&name) {
                    continue;
                }

                self.writer.start();
                self.writer
                    .write(&format!("local ${} {}", name.as_str(), type_.to_string()));
                self.writer.end();

                used.insert(name);
            }
        }
        for expr in body.iter_mut() {
            self.expr(expr)?;
        }

        // In WASM, even if you have exhaustive return in if block, you must provide explicit return at the end of the function
        // so we add a return here for some random value
        match result {
            IrType::Nil => {}
            IrType::I32 => {
                self.writer.new_statement();
                self.writer.write("unreachable");
            }
            IrType::I64 => {
                self.writer.new_statement();
                self.writer.write("unreachable");
            }
            IrType::Address => {
                self.expr(&mut IrTerm::nil())?;
            }
        }

        self.writer.new_statement();
        self.writer.write("return");

        self.writer.end();

        Ok(())
    }

    fn global_let(
        &mut self,
        name: &mut String,
        type_: &mut IrType,
        expr: &mut IrTerm,
    ) -> Result<()> {
        self.writer.start();
        self.writer.write("global");
        self.writer.write(&format!("${}", name.as_str()));
        self.writer.write(&format!("(mut {})", type_.to_string()));
        self.writer.start();
        self.expr(expr)?;
        self.writer.end();
        self.writer.end();

        Ok(())
    }

    fn write_value(&mut self, value: Value) {
        self.writer
            .write(&format!("{}.const {}", Value::wasm_type(), value.as_i64()));
    }

    fn expr(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Nil => {
                // self.write_value(Value::nil());
                self.writer.write(&format!("i32.const {}", 0));
            }
            IrTerm::I32(i) => {
                // self.write_value(Value::i32(*i));
                self.writer.write(&format!("i32.const {}", i));
            }
            IrTerm::I64(i) => {
                todo!("{}", i);
            }
            IrTerm::U32(i) => {
                // self.write_value(Value::i32(*i as i32));
                self.writer.write(&format!("i32.const {}", i));
            }
            IrTerm::Ident(i) => {
                if self.globals.contains(&i.clone()) {
                    self.writer.write(&format!("global.get ${}", i.as_str()));
                } else {
                    self.writer.write(&format!("local.get ${}", i.as_str()));
                }
            }
            IrTerm::String(p) => {
                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", p));

                self.writer.new_statement();
                self.writer.write("call $load_string");
            }
            IrTerm::Call { callee, args, .. } => self.call(callee, args)?,
            IrTerm::Seq { elements } => {
                for element in elements {
                    self.expr(element)?;
                }
            }
            IrTerm::Let {
                name,
                type_: _,
                value,
            } => {
                self.writer.new_statement();
                self.expr(value)?;
                self.expr_left_value(&mut IrTerm::Ident(name.clone()))?;
            }
            IrTerm::Return { value } => {
                self.writer.new_statement();
                self.expr(value)?;
                self.writer.new_statement();
                self.writer.write("return");
            }
            IrTerm::Assign { lhs: ident, rhs } => {
                self.expr(rhs)?;

                self.writer.new_statement();
                if self.globals.contains(&ident.clone()) {
                    self.writer.write("global.set");
                } else {
                    self.writer.write("local.set");
                }
                self.writer.write(&format!("${}", ident.as_str()));
            }
            IrTerm::If {
                cond, then, else_, ..
            } => {
                self.generate_if(None, cond, then, else_)?;
            }
            IrTerm::While {
                cond,
                body,
                cleanup,
            } => {
                /*  [while(cond) {block, cleanup}]

                    (block $exit
                        (loop $loop
                            (block $continue
                                (br_if $exit (i32.eqz (cond)))
                                ($body)
                            )

                            ($cleanup)
                            (br $loop)
                        )
                    )
                */

                self.writer.start();
                self.writer.write("block");
                self.writer.write("$exit");

                self.writer.start();
                self.writer.write("loop");
                self.writer.write("$loop");

                self.writer.start();
                self.writer.write("block");
                self.writer.write("$continue");

                self.expr(cond.as_mut())?;
                self.writer.new_statement();
                self.writer.write("i32.eqz");
                self.writer.new_statement();
                self.writer.write("br_if $exit");

                self.expr(body.as_mut())?;
                self.writer.end();

                if let Some(cleanup) = cleanup {
                    self.expr(cleanup.as_mut())?;
                }

                self.writer.new_statement();
                self.writer.write("br $loop");

                self.writer.end();
                self.writer.end();
            }
            IrTerm::Continue => {
                self.writer.new_statement();
                self.writer.write("br $continue");
            }
            IrTerm::Break => {
                self.writer.new_statement();
                self.writer.write("br $exit");
            }
            IrTerm::SizeOf { type_ } => {
                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", type_.sizeof()));
            }
            IrTerm::WriteMemory {
                type_,
                address,
                value,
            } => {
                for (i, v) in value.into_iter().enumerate() {
                    self.writer.new_statement();
                    self.expr(&mut IrTerm::Store {
                        type_: type_.clone(),
                        address: address.clone(),
                        offset: Box::new(IrTerm::wrap_mult_sizeof(
                            type_.clone(),
                            IrTerm::I32(i as i32),
                        )),
                        value: Box::new(v.clone()),
                    })?;
                }
            }
            IrTerm::Discard { element } => {
                self.writer.new_statement();
                self.expr(element)?;

                self.writer.new_statement();
                self.writer.write("drop");
            }
            IrTerm::And { lhs, rhs } => {
                self.generate_if(
                    Some(IrType::I32),
                    lhs.as_mut(),
                    rhs.as_mut(),
                    &mut IrTerm::i32(0),
                )?;
            }
            IrTerm::Or { lhs, rhs } => {
                self.generate_if(
                    Some(IrType::I32),
                    lhs.as_mut(),
                    &mut IrTerm::i32(1),
                    rhs.as_mut(),
                )?;
            }
            IrTerm::Load {
                type_,
                address,
                offset,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(offset)?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.writer.write(&format!("{}.load", type_.to_string()));
            }
            IrTerm::Store {
                type_,
                address,
                offset,
                value,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(offset)?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write(&format!("{}.store", type_.to_string()));
            }
            IrTerm::Instruction(i) => {
                self.writer.new_statement();
                self.writer.write(i);
            }
            _ => todo!("{:?}", expr),
        }

        Ok(())
    }

    fn expr_left_value(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Ident(i) => {
                self.writer.new_statement();
                if self.globals.contains(&i.clone()) {
                    self.writer.write("global.set");
                } else {
                    self.writer.write("local.set");
                }
                self.writer.write(&format!("${}", i.as_str()));
            }
            _ => todo!(),
        }

        Ok(())
    }

    fn call(&mut self, caller: &mut Box<IrTerm>, args: &mut Vec<IrTerm>) -> Result<()> {
        for arg in args {
            self.writer.new_statement();
            self.expr(arg)?;
        }

        self.writer.new_statement();

        match caller.as_ref() {
            IrTerm::Ident(ident) => match ident.as_str() {
                "add" => {
                    self.writer.write("i32.add");
                }
                "sub" => {
                    self.writer.write("i32.sub");
                }
                "mult" => {
                    self.writer.write("i32.mul");
                }
                "mult_u32" => {
                    self.writer.write("i32.mul");
                }
                "mult_i64" => {
                    self.writer.write("i64.mul");
                }
                "div" => {
                    self.writer.write("i32.div_s");
                }
                "mod" => {
                    self.writer.write("i32.rem_s");
                }
                "mod_u32" => {
                    self.writer.write("i32.rem_u");
                }
                "mod_i64" => {
                    self.writer.write("i64.rem_s");
                }
                "equal" => {
                    self.writer.write("i32.eq");
                }
                "not_equal" => {
                    self.writer.write("i32.ne");
                }
                "not" => {
                    self.writer.write("i32.eqz");
                }
                "lt" => {
                    self.writer.write("i32.lt_s");
                }
                "gt" => {
                    self.writer.write("i32.gt_s");
                }
                "lte" => {
                    self.writer.write("i32.le_s");
                }
                "gte" => {
                    self.writer.write("i32.ge_s");
                }
                "xor_u32" => {
                    self.writer.write("i32.xor");
                }
                "xor_i64" => {
                    self.writer.write("i64.xor");
                }
                "i32_to_i64" => {
                    self.writer.write("i64.extend_i32_s");
                }
                "i64_to_i32" => {
                    self.writer.write("i32.wrap_i64");
                }
                _ => {
                    self.writer.write(&format!("call ${}", ident.as_str()));
                }
            },
            _ => todo!(),
        }

        Ok(())
    }

    fn generate_if(
        &mut self,
        type_: Option<IrType>,
        cond: &mut IrTerm,
        then: &mut IrTerm,
        else_: &mut IrTerm,
    ) -> Result<()> {
        self.writer.new_statement();
        self.expr(cond)?;

        self.writer.start();
        self.writer.write(if let Some(t) = type_ {
            format!("if (result {})", t.to_string())
        } else {
            "if".to_string()
        });

        self.writer.start();
        self.writer.write("then");
        self.expr(then)?;
        self.writer.end();

        self.writer.start();
        self.writer.write("else");
        self.expr(else_)?;
        self.writer.end();

        self.writer.end();

        Ok(())
    }
}
