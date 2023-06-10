use std::collections::HashSet;

use anyhow::{bail, Ok, Result};

use crate::{
    compiler::{
        MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN, MODE_OPTIMIZE_CONSTANT_FOLDING, MODE_READABLE_WASM,
    },
    ir::{IrTerm, IrType},
    ir_code_gen::TypeRep,
    util::{path::Path, sexpr_writer::SExprWriter},
    value::Value,
};

pub struct Generator {
    pub writer: SExprWriter,
    pub globals: HashSet<String>,
    pub main_signature: Option<(Vec<IrType>, IrType)>,
    pub entrypoint: Path,
    pub strings: Vec<String>,
    pub type_reps: Vec<TypeRep>,
    pub start: Vec<IrTerm>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: SExprWriter::new(),
            globals: HashSet::new(),
            main_signature: None,
            entrypoint: Path::new(vec![]),
            strings: vec![],
            type_reps: vec![],
            start: vec![],
        }
    }

    pub fn set_globals(&mut self, globals: HashSet<Path>) {
        self.globals = globals
            .iter()
            .map(|p| p.as_joined_str("_"))
            .collect::<HashSet<_>>();
    }

    pub fn set_entrypoint(&mut self, entrypoint: Path) {
        self.entrypoint = entrypoint;
    }

    pub fn set_strings(&mut self, strings: Vec<String>) {
        self.strings = strings;
    }

    pub fn set_type_reps(&mut self, type_reps: Vec<TypeRep>) {
        self.type_reps = type_reps;
    }

    pub fn set_start(&mut self, start: Vec<IrTerm>) {
        self.start = start;
    }

    pub fn run(&mut self, module: &mut IrTerm, data_section_offset: usize) -> Result<()> {
        match module {
            IrTerm::Module { elements } => {
                self.module(elements, data_section_offset)?;
            }
            _ => bail!("Expected module"),
        }

        Ok(())
    }

    fn module(&mut self, elements: &mut Vec<IrTerm>, data_section_offset: usize) -> Result<()> {
        let entrypoint_symbol = self.entrypoint.as_joined_str("_");

        self.writer.start();
        self.writer.write("module");

        // wasi must be declared first
        self.writer.start();
        self.writer.write(r#"import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer.write(r#"import "wasi_unstable" "path_open" (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer.write(
            r#"import "wasi_unstable" "fd_close" (func $fd_close (param i32) (result i32))"#,
        );
        self.writer.end();

        self.decl(&mut IrTerm::Declare {
            name: "debug_i32".to_string(),
            params: vec![IrType::I32],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "debug".to_string(),
            params: vec![IrType::Any],
            result: IrType::Address,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "read_stdin".to_string(),
            params: vec![],
            result: IrType::Byte,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "abort".to_string(),
            params: vec![],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "create_handler".to_string(),
            params: vec![],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "open_handler_stream".to_string(),
            params: vec![IrType::I32, IrType::Byte],
            result: IrType::Nil,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "open_handler_initialize".to_string(),
            params: vec![IrType::I32, IrType::I32],
            result: IrType::Nil,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "read_handler".to_string(),
            params: vec![IrType::I32],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "write_handler".to_string(),
            params: vec![IrType::I32, IrType::Byte],
            result: IrType::Address,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "i64_to_string_at".to_string(),
            params: vec![IrType::I32, IrType::I32, IrType::I32],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "get_args_len".to_string(),
            params: vec![],
            result: IrType::I32,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "get_args_at".to_string(),
            params: vec![IrType::I32],
            result: IrType::Byte,
        })?;

        self.decl(&mut IrTerm::Func {
            name: "_fd_write".to_string(),
            params: vec![
                ("fd".to_string(), IrType::I32),
                ("ciovec".to_string(), IrType::Address),
                ("ptr_size".to_string(), IrType::Address),
            ],
            result: Some(IrType::I32),
            body: vec![
                // fd
                IrTerm::Instruction("local.get $fd".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                // ciovec.ptr
                IrTerm::Instruction("local.get $ciovec".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                // ciovec.len
                IrTerm::Instruction("i32.const 1".to_string()),
                // ptr_size
                IrTerm::Instruction("local.get $ptr_size".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                // call $fd_write
                IrTerm::Instruction("call $fd_write".to_string()),
                IrTerm::Instruction("i64.extend_i32_s".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shl".to_string()),
                IrTerm::Instruction("return".to_string()),
            ],
        })?;

        self.decl(&mut IrTerm::Func {
            name: "set_ciovec".to_string(),
            params: vec![
                ("ptr".to_string(), IrType::Address),
                ("x".to_string(), IrType::Address),
                ("y".to_string(), IrType::I32),
            ],
            result: Some(IrType::Nil),
            body: vec![
                // $ptr
                IrTerm::Instruction("local.get $ptr".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                // $x
                IrTerm::Instruction("local.get $x".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                IrTerm::Instruction("i32.store".to_string()),
                // $ptr
                IrTerm::Instruction("local.get $ptr".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                // $y
                IrTerm::Instruction("local.get $y".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shr_u".to_string()),
                IrTerm::Instruction("i32.wrap_i64".to_string()),
                IrTerm::Instruction("i32.store offset=4".to_string()),
                // return nil
                IrTerm::Instruction("i64.const 1".to_string()),
                IrTerm::Instruction("return".to_string()),
            ],
        })?;

        self.decl(&mut IrTerm::Func {
            name: "_path_open".to_string(),
            params: vec![
                ("dirfd".to_string(), IrType::I32),
                ("dirflags".to_string(), IrType::I32),
                ("path".to_string(), IrType::Address),
                ("path_len".to_string(), IrType::I32),
                ("o_flags".to_string(), IrType::I32),
                ("fs_rights_base".to_string(), IrType::I32),
                ("fs_rights_inheriting".to_string(), IrType::I32),
                ("fs_flags".to_string(), IrType::I32),
                ("fd".to_string(), IrType::Address),
            ],
            result: Some(IrType::I32),
            body: {
                let mut body = vec![];
                body.extend(vec![IrTerm::Instruction("local.get $dirfd".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction("local.get $dirflags".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction("local.get $path".to_string())]);
                body.extend(Generator::instructions_ptr_to_wasmptr());

                body.extend(vec![IrTerm::Instruction("local.get $path_len".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction("local.get $o_flags".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction(
                    "local.get $fs_rights_base".to_string(),
                )]);
                body.extend(Generator::instructions_i32_to_wasmi64());

                body.extend(vec![IrTerm::Instruction(
                    "local.get $fs_rights_inheriting".to_string(),
                )]);
                body.extend(Generator::instructions_i32_to_wasmi64());

                body.extend(vec![IrTerm::Instruction("local.get $fs_flags".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction("local.get $fd".to_string())]);
                body.extend(Generator::instructions_ptr_to_wasmptr());

                body.extend(vec![IrTerm::Instruction("call $path_open".to_string())]);
                body.extend(Generator::instructions_wasmi32_to_i32());

                body.extend(vec![IrTerm::Instruction("return".to_string())]);

                body
            },
        })?;

        self.decl(&mut IrTerm::Func {
            name: "_fd_close".to_string(),
            params: vec![("fd".to_string(), IrType::I32)],
            result: Some(IrType::Nil),
            body: {
                let mut body = vec![];
                body.extend(vec![IrTerm::Instruction("local.get $fd".to_string())]);
                body.extend(Generator::instructions_i32_to_wasmi32());

                body.extend(vec![IrTerm::Instruction("call $fd_close".to_string())]);
                body.extend(Generator::instructions_wasmi32_to_i32());

                body
            },
        })?;

        self.writer.start();
        self.writer.write(r#"memory (export "memory") 50000"#);
        self.writer.end();

        for term in elements {
            self.decl(term)?;
        }

        // builtin functions here
        self.decl(&mut IrTerm::GlobalLet {
            name: "_value_i32_1".to_string(),
            type_: IrType::I32,
            value: Box::new(IrTerm::i32(1)),
        })?;

        self.decl(&mut IrTerm::GlobalLet {
            name: "_value_i64_1".to_string(),
            type_: IrType::Address,
            value: Box::new(IrTerm::i32(0)), // generate 0
        })?;

        self.decl(&mut IrTerm::GlobalLet {
            name: "_raw_i64_1".to_string(),
            type_: IrType::Address,
            value: Box::new(IrTerm::i32(0)), // generate 0
        })?;

        self.decl(&mut IrTerm::GlobalLet {
            name: "_raw_i64_2".to_string(),
            type_: IrType::Address,
            value: Box::new(IrTerm::i32(0)), // generate 0
        })?;

        self.decl(&mut IrTerm::Func {
            name: "_reflection_is_pointer".to_string(),
            params: vec![("value".to_string(), IrType::Any)],
            result: Some(IrType::Bool),
            body: vec![
                IrTerm::Instruction("local.get $value".to_string()),
                IrTerm::Instruction("i64.const 1".to_string()),
                IrTerm::Instruction("i64.and".to_string()),
                IrTerm::Instruction("i64.const 32".to_string()),
                IrTerm::Instruction("i64.shl".to_string()),
                IrTerm::Instruction("i64.const 2".to_string()),
                IrTerm::Instruction("i64.or".to_string()),
                IrTerm::Instruction("return".to_string()),
            ],
        })?;

        self.decl(&mut IrTerm::Func {
            name: "_reflection_is_bool".to_string(),
            params: vec![("value".to_string(), IrType::Any)],
            result: Some(IrType::Bool),
            body: vec![
                IrTerm::Instruction("local.get $value".to_string()),
                IrTerm::Instruction("i64.const 2".to_string()),
                IrTerm::Instruction("i64.and".to_string()),
                IrTerm::Instruction("i64.const 31".to_string()),
                IrTerm::Instruction("i64.shl".to_string()),
                IrTerm::Instruction("i64.const 2".to_string()),
                IrTerm::Instruction("i64.or".to_string()),
                IrTerm::Instruction("return".to_string()),
            ],
        })?;

        self.writer.start();
        self.writer
            .write(r#"func $i32_mul (param $a i64) (param $b i64) (result i64)"#);

        self.writer.new_statement();
        self.writer.write("local.get $a");

        self.writer.new_statement();
        self.writer.write("local.get $b");

        self.convert_stack_to_i32_2();

        self.writer.new_statement();
        self.writer.write("i32.mul");

        self.convert_stack_from_i32_1();

        self.writer.end();

        let (_, result) = self.main_signature.clone().unwrap();

        self.decl(&mut IrTerm::Func {
            name: "start".to_string(),
            params: vec![],
            result: Some(result),
            body: {
                let mut v = vec![
                    IrTerm::Assign {
                        lhs: "quartz_std_alloc_ptr".to_string(),
                        rhs: Box::new(IrTerm::i32(data_section_offset as i32)),
                    },
                    IrTerm::Assign {
                        lhs: "quartz_std_strings_count".to_string(),
                        rhs: Box::new(IrTerm::i32(self.strings.len() as i32)),
                    },
                    IrTerm::Assign {
                        lhs: "quartz_std_type_reps_count".to_string(),
                        rhs: Box::new(IrTerm::i32(self.type_reps.len() as i32)),
                    },
                ];
                v.extend(self.start.clone());
                v.extend(vec![
                    IrTerm::Call {
                        callee: Box::new(IrTerm::ident("prepare_strings")),
                        args: vec![],
                        source: None,
                    },
                    IrTerm::Call {
                        callee: Box::new(IrTerm::ident("prepare_type_reps")),
                        args: vec![],
                        source: None,
                    },
                    IrTerm::Return {
                        value: Box::new(IrTerm::Call {
                            callee: Box::new(IrTerm::ident(entrypoint_symbol.clone())),
                            args: vec![],
                            source: None,
                        }),
                    },
                ]);

                v
            },
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
            IrTerm::Data { offset, data } => {
                self.writer.start();
                self.writer.write("data");
                self.writer.write(format!("(i32.const {})", *offset));
                self.writer.write(format!("{:?}", data));
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
        result: &mut Option<IrType>,
        body: &mut Vec<IrTerm>,
    ) -> Result<()> {
        if name == &self.entrypoint.as_joined_str("_") {
            if let Some(result) = result {
                self.main_signature = Some((
                    params.iter().map(|(_, t)| t.clone()).collect(),
                    result.clone(),
                ));
            }
        }

        self.writer.start();
        self.writer.write("func");
        self.writer.write(&format!("${}", name.as_str()));
        for (name, type_) in params {
            self.writer.write(&format!(
                "(param ${} {})",
                name.as_str(),
                Value::from_ir_type(type_)
            ));
        }

        if let Some(result) = result {
            self.writer
                .write(&format!("(result {})", Value::from_ir_type(result)));
        }

        let mut used = HashSet::new();
        for term in body.iter() {
            for (name, type_, _) in term.find_let() {
                if used.contains(&name) {
                    continue;
                }

                self.writer.start();
                self.writer.write(&format!(
                    "local ${} {}",
                    name.as_str(),
                    Value::from_ir_type(&type_)
                ));
                self.writer.end();

                used.insert(name);
            }
        }
        for expr in body.iter_mut() {
            self.expr(expr)?;
        }

        // In WASM, even if you have exhaustive return in if block, you must provide explicit return at the end of the function
        // so we add a return here for some random value
        if let Some(result) = result {
            match result {
                IrType::Nil => {}
                IrType::Byte => {
                    self.writer.new_statement();
                    self.writer.write("unreachable");
                }
                IrType::Bool => {
                    self.writer.new_statement();
                    self.writer.write("unreachable");
                }
                IrType::I32 => {
                    self.writer.new_statement();
                    self.writer.write("unreachable");
                }
                IrType::I64 => {
                    self.writer.new_statement();
                    self.writer.write("unreachable");
                }
                IrType::Address => {
                    self.writer.new_statement();
                    self.expr(&mut IrTerm::nil())?;

                    self.writer.new_statement();
                    self.writer.write("return");
                }
                IrType::Any => {
                    self.writer.new_statement();
                    self.writer.write("unreachable");
                }
            }
        }

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
        self.writer.new_statement();
        self.writer
            .write(&format!("{}.const {}", Value::wasm_type(), value.as_i64()));
    }

    fn expr(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Nil => {
                if MODE_READABLE_WASM {
                    self.writer.new_statement();
                    self.writer.write(" ;; nil");
                }

                self.write_value(Value::nil());
            }
            IrTerm::I32(i) => {
                if MODE_READABLE_WASM {
                    self.writer.new_statement();
                    self.writer.write(&format!(" ;; {}", i));
                }

                self.write_value(Value::i32(*i));
            }
            IrTerm::I64(i) => {
                let hi = (*i >> 32) as i32;
                let lo = *i as i32;

                self.write_value(Value::i32(hi));
                self.write_value(Value::i32(lo));

                self.writer.new_statement();
                self.writer.write("call $quartz_std_i64_new");
            }
            IrTerm::U32(i) => {
                if MODE_READABLE_WASM {
                    self.writer.new_statement();
                    self.writer.write(&format!(";; {}", i));
                }

                self.write_value(Value::i32(*i as i32));
            }
            IrTerm::Bool(b) => {
                if MODE_READABLE_WASM {
                    self.writer.new_statement();
                    self.writer.write(&format!(" ;; {}", b));
                }

                self.write_value(Value::bool(*b));
            }
            IrTerm::Ident(i) => {
                self.writer.new_statement();
                if self.globals.contains(&i.clone()) {
                    self.writer.write(&format!("global.get ${}", i.as_str()));
                } else {
                    self.writer.write(&format!("local.get ${}", i.as_str()));
                }
            }
            IrTerm::String(p) => {
                if MODE_READABLE_WASM {
                    let s = &self.strings[*p];

                    self.writer.new_statement();
                    self.writer.write(&format!(
                        " ;; string: {}",
                        if s.len() > 30 {
                            format!("{:?}...", &s[0..30])
                        } else {
                            format!("{:?}", s)
                        }
                    ));
                }

                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", p));

                self.convert_stack_from_i32_1();

                self.writer.new_statement();
                self.writer.write("call $quartz_std_load_string");
            }
            IrTerm::TypeRep(p) => {
                if MODE_READABLE_WASM {
                    let s = &self.type_reps[*p];

                    self.writer.new_statement();
                    self.writer.write(&format!(" ;; type rep: {:?}", s));
                }

                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", p));

                self.convert_stack_from_i32_1();

                self.writer.new_statement();
                self.writer.write("call $quartz_std_get_type_rep_address");
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
                self.convert_stack_to_bool();

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
                let size = type_.sizeof();

                if MODE_READABLE_WASM {
                    self.writer.new_statement();
                    self.writer.write(format!(";; {} (sizeof)", size));
                }

                self.writer.new_statement();
                self.write_value(Value::i32(size));
            }
            IrTerm::Discard { element } => {
                self.writer.new_statement();
                self.expr(element)?;

                self.writer.new_statement();
                self.writer.write("drop");
            }
            IrTerm::And { lhs, rhs } => {
                self.generate_if(
                    Some(IrType::Bool),
                    lhs.as_mut(),
                    rhs.as_mut(),
                    &mut IrTerm::Bool(false),
                )?;
            }
            IrTerm::Or { lhs, rhs } => {
                self.generate_if(
                    Some(IrType::Bool),
                    lhs.as_mut(),
                    &mut IrTerm::Bool(true),
                    rhs.as_mut(),
                )?;
            }
            IrTerm::Load {
                type_,
                address,
                offset,
                raw_offset,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.convert_stack_to_i32_1();

                self.optimize_i32(offset)?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                // Only 32-bit addresses are supported
                // self.convert_stack_from_i32_1();

                let load_size = type_.sizeof() * 8;

                self.writer.new_statement();
                self.writer.write(&format!(
                    "{}.load{}",
                    Value::wasm_type(),
                    if load_size == 64 {
                        String::new()
                    } else {
                        format!("{}_u", load_size)
                    }
                ));
                if let Some(raw_offset) = raw_offset {
                    self.writer.write(&format!("offset={}", raw_offset));
                }

                match type_ {
                    IrType::Byte => {
                        self.writer.new_statement();
                        self.writer.write("i64.const 32");

                        self.writer.new_statement();
                        self.writer.write("i64.shl");

                        self.writer.new_statement();
                        self.writer.write("i64.const 4");

                        self.writer.new_statement();
                        self.writer.write("i64.xor");
                    }
                    _ => {}
                }
            }
            IrTerm::Store {
                type_,
                address,
                offset,
                value,
                raw_offset,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.convert_stack_to_i32_1();

                self.optimize_i32(offset)?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                // Only 32-bit addresses are supported
                // self.convert_stack_from_i32_1();

                self.writer.new_statement();
                self.expr(value)?;

                let store_size = type_.sizeof() * 8;

                // FIXME: adhoc fix for 64-bit store
                if store_size != 64 {
                    self.writer.new_statement();
                    self.writer.write("i64.const 32");

                    self.writer.new_statement();
                    self.writer.write("i64.shr_u");
                }

                self.writer.new_statement();
                self.writer.write(&format!(
                    "{}.store{}",
                    Value::wasm_type(),
                    if store_size == 64 {
                        String::new()
                    } else {
                        format!("{}", store_size)
                    }
                ));
                if let Some(raw_offset) = raw_offset {
                    self.writer.write(&format!("offset={}", raw_offset));
                }
            }
            IrTerm::Instruction(i) => {
                self.writer.new_statement();
                self.writer.write(i);
            }
            IrTerm::Comment(c) => {
                self.writer.new_statement();
                self.writer.write(format!(";; {:?}", c));
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
                "add" | "add_u32" => {
                    if MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN {
                        self.writer.write("i64.add");
                    } else {
                        self.generate_op_arithmetic("add");
                    }
                }
                "sub" => {
                    if MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN {
                        self.writer.write("i64.sub");
                    } else {
                        self.generate_op_arithmetic("sub");
                    }
                }
                "mult" | "mult_u32" => {
                    if MODE_READABLE_WASM {
                        self.writer.write("call $i32_mul");
                    } else {
                        self.generate_op_arithmetic("mul");
                    }
                }
                "mult_i64" => {
                    self.generate_op_arithmetic_i64("i64.mul");
                }
                "div" => {
                    self.generate_op_arithmetic("div_s");
                }
                "div_u32" => {
                    self.generate_op_arithmetic("div_u");
                }
                "div_i64" => {
                    self.generate_op_arithmetic_i64("i64.div_s");
                }
                "mod" => {
                    self.generate_op_arithmetic("rem_s");
                }
                "mod_u32" => {
                    self.generate_op_arithmetic("rem_u");
                }
                "mod_i64" => {
                    self.generate_op_arithmetic_i64("i64.rem_s");
                }
                "equal" => {
                    self.writer.write("i64.eq");
                    self.convert_stack_from_bool_1();
                }
                "not_equal" => {
                    self.generate_op_comparison("ne");
                }
                "not" => {
                    self.convert_stack_to_bool();
                    self.writer.write("i32.eqz");
                    self.convert_stack_from_bool_1();
                }
                "lt" => {
                    self.generate_op_comparison("lt_s");
                }
                "gt" => {
                    self.generate_op_comparison("gt_s");
                }
                "gt_u32" => {
                    self.generate_op_comparison("gt_u");
                }
                "lte" => {
                    self.generate_op_comparison("le_s");
                }
                "gte" => {
                    self.generate_op_comparison("ge_s");
                }
                "xor_u32" => {
                    self.generate_op_arithmetic("xor");
                }
                "xor_i64" => {
                    self.generate_op_arithmetic_i64("i64.xor");
                }
                "i32_to_address" => self.convert_value_i32_to_address_1(),
                "address_to_i32" => self.convert_value_address_to_i32_1(),
                "i32_to_byte" => self.convert_value_i32_to_byte_1(),
                "byte_to_i32" => self.convert_value_byte_to_i32_1(),
                "byte_to_address" => self.convert_value_byte_to_address_1(),
                "bit_shift_left" => {
                    self.generate_op_arithmetic("shl");
                }
                "bit_shift_left_u32" => {
                    self.generate_op_arithmetic("shl");
                }
                "bit_shift_right" => {
                    self.generate_op_arithmetic("shr_s");
                }
                "bit_or" => {
                    self.generate_op_arithmetic("or");
                }
                "bit_and" => {
                    self.generate_op_arithmetic("and");
                }
                "bit_or_i64" => {
                    self.generate_op_arithmetic_i64("i64.or");
                }
                "bit_shift_left_i64" => {
                    self.generate_op_arithmetic_i64("i64.shl");
                }
                "bit_shift_right_i64" => {
                    self.generate_op_arithmetic_i64("i64.shr_s");
                }
                _ => {
                    self.writer.write(&format!("call ${}", ident.as_str()));
                }
            },
            _ => todo!(),
        }

        Ok(())
    }

    fn convert_value_i32_to_address_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 1");

        self.writer.new_statement();
        self.writer.write("i64.xor");
    }

    fn convert_value_address_to_i32_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 1");

        self.writer.new_statement();
        self.writer.write("i64.xor");
    }

    fn convert_value_i32_to_byte_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 4");

        self.writer.new_statement();
        self.writer.write("i64.xor");
    }

    fn convert_value_byte_to_i32_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 4");

        self.writer.new_statement();
        self.writer.write("i64.xor");
    }

    fn convert_value_byte_to_address_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 4");

        self.writer.new_statement();
        self.writer.write("i64.xor");

        self.writer.new_statement();
        self.writer.write("i64.const 1");

        self.writer.new_statement();
        self.writer.write("i64.xor");
    }

    fn convert_stack_to_i32_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shr_u");

        self.writer.new_statement();
        self.writer.write("i32.wrap_i64");
    }

    fn convert_stack_to_i32_2(&mut self) {
        self.writer.new_statement();
        self.writer.write("global.set $_value_i32_1");

        self.convert_stack_to_i32_1();

        self.writer.new_statement();
        self.writer.write("global.get $_value_i32_1");

        self.convert_stack_to_i32_1();
    }

    fn convert_stack_from_i32_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.extend_i32_s");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shl");
    }

    fn convert_stack_from_bool_1(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.extend_i32_s");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shl");

        self.writer.new_statement();
        self.writer.write("i64.const 2");

        self.writer.new_statement();
        self.writer.write("i64.or");
    }

    fn convert_stack_to_bool(&mut self) {
        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shr_u");

        self.writer.new_statement();
        self.writer.write("i64.const 1");

        self.writer.new_statement();
        self.writer.write("i64.and");

        self.writer.new_statement();
        self.writer.write("i32.wrap_i64");
    }

    // FIXME: need GC lock
    // For GC locks, keep value n, which is the number of values to lock (not a standard value)
    fn load_i64(&mut self) {
        self.writer.new_statement();
        self.writer.write("global.set $_value_i64_1");

        self.writer.new_statement();
        self.writer.write("global.get $_value_i64_1");

        self.convert_stack_to_i32_1();

        // load hi address
        self.writer.new_statement();
        self.writer.write("i64.load offset=8");

        // load lo address
        self.writer.new_statement();
        self.writer.write("global.get $_value_i64_1");

        self.convert_stack_to_i32_1();

        self.writer.new_statement();
        self.writer.write("i64.load offset=16");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shr_u");

        self.writer.new_statement();
        self.writer.write("i64.add");
    }

    fn generate_op_arithmetic_i64(&mut self, op: &str) {
        self.load_i64();

        self.writer.new_statement();
        self.writer.write("global.set $_raw_i64_1");

        self.load_i64();

        self.writer.new_statement();
        self.writer.write("global.get $_raw_i64_1");

        self.writer.new_statement();
        self.writer.write(op);

        self.writer.new_statement();
        self.writer.write("global.set $_raw_i64_1");

        // prepare hi
        self.writer.new_statement();
        self.writer.write("global.get $_raw_i64_1");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shr_u");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shl");

        // prepare lo
        self.writer.new_statement();
        self.writer.write("global.get $_raw_i64_1");

        self.writer.new_statement();
        self.writer.write("i64.const 32");

        self.writer.new_statement();
        self.writer.write("i64.shl");

        self.writer.new_statement();
        self.writer.write("call $quartz_std_i64_new");
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
        self.convert_stack_to_bool();

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

    fn generate_op_arithmetic(&mut self, code: &str) {
        self.convert_stack_to_i32_2();
        self.writer.write(format!("i32.{}", code));
        self.convert_stack_from_i32_1();
    }

    fn generate_op_comparison(&mut self, code: &str) {
        self.convert_stack_to_i32_2();
        self.writer.write(format!("i32.{}", code));
        self.convert_stack_from_bool_1();
    }

    pub fn fold_consts(&mut self, term: &mut IrTerm) {
        match term {
            IrTerm::Module { elements } => {
                for element in elements {
                    self.fold_consts(element);
                }
            }
            IrTerm::Nil => {}
            IrTerm::Bool(_) => {}
            IrTerm::Byte(_) => {}
            IrTerm::I32(_) => {}
            IrTerm::U32(_) => {}
            IrTerm::I64(_) => {}
            IrTerm::Ident(_) => {}
            IrTerm::String(_) => {}
            IrTerm::Func { body, .. } => {
                for b in body {
                    self.fold_consts(b);
                }
            }
            IrTerm::GlobalLet { value, .. } => {
                self.fold_consts(value);
            }
            IrTerm::Call { callee, args, .. } => {
                self.fold_consts(callee);
                for arg in args.iter_mut() {
                    self.fold_consts(arg);
                }

                if let IrTerm::Ident(op) = &**callee {
                    if op == "add" {
                        if let (IrTerm::I32(a), IrTerm::I32(b)) = (&args[0], &args[1]) {
                            *term = IrTerm::I32(a + b);
                        }
                    } else if op == "sub" {
                        if let (IrTerm::I32(a), IrTerm::I32(b)) = (&args[0], &args[1]) {
                            *term = IrTerm::I32(a - b);
                        }
                    } else if op == "mult" {
                        if let (IrTerm::I32(a), IrTerm::I32(b)) = (&args[0], &args[1]) {
                            *term = IrTerm::I32(a * b);
                        }
                    } else if op == "div" {
                        if let (IrTerm::I32(a), IrTerm::I32(b)) = (&args[0], &args[1]) {
                            *term = IrTerm::I32(a / b);
                        }
                    }
                }
            }
            IrTerm::Seq { elements } => {
                for element in elements {
                    self.fold_consts(element);
                }
            }
            IrTerm::Let { value, .. } => {
                self.fold_consts(value);
            }
            IrTerm::Return { value } => {
                self.fold_consts(value);
            }
            IrTerm::Assign { rhs, .. } => {
                self.fold_consts(rhs);
            }
            IrTerm::If {
                cond, then, else_, ..
            } => {
                self.fold_consts(cond);
                self.fold_consts(then);
                self.fold_consts(else_);
            }
            IrTerm::While {
                cond,
                body,
                cleanup,
            } => {
                self.fold_consts(cond);
                self.fold_consts(body);
                if let Some(cleanup) = cleanup {
                    self.fold_consts(cleanup);
                }
            }
            IrTerm::SizeOf { type_ } => {
                let size = type_.sizeof();
                *term = IrTerm::I32(size as i32);
            }
            IrTerm::Continue => {}
            IrTerm::Break => {}
            IrTerm::Discard { element } => {
                self.fold_consts(element);
            }
            IrTerm::And { lhs, rhs } => {
                self.fold_consts(lhs);
                self.fold_consts(rhs);
            }
            IrTerm::Or { lhs, rhs } => {
                self.fold_consts(lhs);
                self.fold_consts(rhs);
            }
            IrTerm::Store {
                address,
                offset,
                value,
                ..
            } => {
                self.fold_consts(address);
                self.fold_consts(offset);
                self.fold_consts(value);
            }
            IrTerm::Load {
                address, offset, ..
            } => {
                self.fold_consts(address);
                self.fold_consts(offset);
            }
            IrTerm::Instruction(_) => {}
            IrTerm::Declare { .. } => {}
            IrTerm::Data { .. } => {}
            IrTerm::Comment(_) => {}
            IrTerm::TypeRep(_) => {}
        }
    }

    fn optimize_i32(&mut self, term: &mut IrTerm) -> Result<()> {
        let mut optimized = false;
        if MODE_OPTIMIZE_CONSTANT_FOLDING {
            if let IrTerm::I32(v) = term {
                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", v));

                optimized = true;
            }
        }
        if !optimized {
            self.writer.new_statement();
            self.expr(term)?;

            self.convert_stack_to_i32_1();
        }

        Ok(())
    }

    fn fragment_i32_to_wasmi32() -> Vec<&'static str> {
        vec!["i64.const 32", "i64.shr_u", "i32.wrap_i64"]
    }

    fn instructions_i32_to_wasmi32() -> Vec<IrTerm> {
        Generator::fragment_i32_to_wasmi32()
            .into_iter()
            .map(|s| IrTerm::Instruction(s.to_string()))
            .collect()
    }

    fn fragment_wasmi32_to_i32() -> Vec<&'static str> {
        vec!["i64.extend_i32_s", "i64.const 32", "i64.shl"]
    }

    fn instructions_wasmi32_to_i32() -> Vec<IrTerm> {
        Generator::fragment_wasmi32_to_i32()
            .into_iter()
            .map(|s| IrTerm::Instruction(s.to_string()))
            .collect()
    }

    fn fragment_ptr_to_wasmptr() -> Vec<&'static str> {
        vec!["i64.const 32", "i64.shr_u", "i32.wrap_i64"]
    }

    fn instructions_ptr_to_wasmptr() -> Vec<IrTerm> {
        Generator::fragment_ptr_to_wasmptr()
            .into_iter()
            .map(|s| IrTerm::Instruction(s.to_string()))
            .collect()
    }

    fn fragment_i32_to_wasmi64() -> Vec<&'static str> {
        vec![
            "i64.const 32",
            "i64.shr_u",
            "i32.wrap_i64",
            "i64.extend_i32_s",
        ]
    }

    fn instructions_i32_to_wasmi64() -> Vec<IrTerm> {
        Generator::fragment_i32_to_wasmi64()
            .into_iter()
            .map(|s| IrTerm::Instruction(s.to_string()))
            .collect()
    }
}
