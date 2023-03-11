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

        self.writer.start();
        self.writer
            .write(r#"import "env" "write_stdout" (func $write_stdout (param i32) (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer
            .write(r#"import "env" "debug_i32" (func $debug_i32 (param i32) (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer
            .write(r#"import "env" "read_stdin" (func $read_stdin (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer
            .write(r#"import "env" "abort" (func $abort (result i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer.write(r#"memory 1"#);
        self.writer.end();

        for term in elements {
            self.decl(term)?;
        }

        // builtin functions here
        self.writer.write(
            r#"
(func $mem_copy (param $source i32) (param $target i32) (param $length i32) (result i32)
    local.get $target
    local.get $source
    local.get $length
    memory.copy
    i32.const 0
)
(func $mem_free (param $address i32) (result i32)
    i32.const 0
)
"#,
        );

        // prepare strings
        let var_strings = "quartz_std_strings_ptr";

        self.writer.start();
        self.writer.write("func $prepare_strings");

        self.writer.new_statement();
        self.writer.write("(local $p i32)");

        self.writer.new_statement();
        self.writer
            .write(format!("i32.const {}", self.strings.len()));

        self.writer.new_statement();
        self.writer.write(format!(
            "call ${}",
            Path::new(
                vec!["quartz", "std", "alloc"]
                    .into_iter()
                    .map(|i| Ident(i.to_string()))
                    .collect()
            )
            .as_joined_str("_")
        ));

        self.writer.new_statement();
        self.writer.write(format!("global.set ${}", var_strings));

        for (i, string) in self.strings.clone().iter().enumerate() {
            self.writer.new_statement();
            self.writer.write(format!(";; {:?}", string));

            self.writer.new_statement();
            self.writer.write(format!(
                "(call $quartz_std_new_empty_string (i32.const {}))",
                string.len()
            ));

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

            self.writer.new_statement();
            self.writer.write(format!("global.get ${}", var_strings));

            self.writer.new_statement();
            self.writer.write(format!("i32.const {}", i));

            self.writer.new_statement();
            self.writer.write("i32.const 4");

            self.writer.new_statement();
            self.writer.write("i32.mul");

            self.writer.new_statement();
            self.writer.write("i32.add");

            self.writer.new_statement();
            self.writer.write("local.get $p");

            self.writer.new_statement();
            self.writer.write("i32.store");
        }

        self.writer.end();

        self.writer.write(format!(
            r#"
(func $load_string (param $index i32) (result i32)
    global.get ${}
    local.get $index
    i32.const 4
    i32.mul
    i32.add
    i32.load
)
"#,
            var_strings,
        ));

        let (_, result) = self.main_signature.clone().unwrap();

        self.writer.write(&format!(
            r#"
(func $start {}
    i32.const {}
    global.set ${}

    (memory.grow (i32.const 2))
    drop

    call $prepare_strings
    call ${}
)
"#,
            if result.is_nil() {
                "".to_string()
            } else {
                format!("(result {})", result.to_string())
            },
            self.strings.len(),
            "quartz_std_strings_count",
            self.entrypoint_symbol
        ));

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
                self.writer.write("i32.const 0");
            }
            IrType::I64 => {
                self.writer.new_statement();
                self.writer.write("i64.const 0");
            }
            IrType::Address => {
                self.writer.new_statement();
                self.writer.write("i32.const 0");
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
                self.writer.new_statement();
                self.expr(cond)?;

                self.writer.start();
                self.writer.write("if");

                self.writer.start();
                self.writer.write("then");
                self.expr(then)?;
                self.writer.end();

                self.writer.start();
                self.writer.write("else");
                self.expr(else_)?;
                self.writer.end();

                self.writer.end();
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
                self.writer.new_statement();
                self.expr(lhs)?;

                self.writer.start();
                self.writer.write("if (result i32)");

                self.writer.start();
                self.writer.write("then");
                self.expr(rhs)?;
                self.writer.end();

                self.writer.start();
                self.writer.write("else");
                self.writer.new_statement();
                self.writer.write("i32.const 0");
                self.writer.end();

                self.writer.end();
            }
            IrTerm::Or { lhs, rhs } => {
                self.writer.new_statement();
                self.expr(lhs)?;

                self.writer.start();
                self.writer.write("if (result i32)");

                self.writer.start();
                self.writer.write("then");
                self.writer.new_statement();
                self.writer.write("i32.const 1");
                self.writer.end();

                self.writer.start();
                self.writer.write("else");
                self.expr(rhs)?;
                self.writer.end();

                self.writer.end();
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
}
