use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, Ok, Result};

use crate::{
    ast::Type,
    ir::{IrTerm, IrType},
    util::{ident::Ident, path::Path, sexpr_writer::SExprWriter},
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
            .write(r#"import "env" "write_stdout" (func $write_stdout (param i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer
            .write(r#"import "env" "debug_i32" (func $debug_i32 (param i32))"#);
        self.writer.end();

        self.writer.start();
        self.writer.write(r#"import "env" "abort" (func $abort)"#);
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
(global $_sp (mut i32) (i32.const 0))
(global $_bp (mut i32) (i32.const 0))

(func $alloc (param $size i32) (result i32)
    (local $addr i32)

    global.get $_sp
    local.tee $addr

    local.get $size
    i32.add
    global.set $_sp

    local.get $addr
    i32.const 8
    i32.mul
)

(func $mem_copy (param $source i32) (param $target i32) (param $length i32)
    local.get $target
    local.get $source
    local.get $length
    memory.copy
)
(func $mem_free (param $address i32)
)
"#,
        );

        // prepare strings
        self.writer.new_statement();
        self.writer
            .write("(global $_strings (mut i32) (i32.const 0))");

        self.writer.start();
        self.writer.write("func $prepare_strings");

        self.writer.new_statement();
        self.writer.write("(local $p i32)");

        self.writer.new_statement();
        self.writer
            .write(format!("i32.const {}", self.strings.len()));

        self.writer.new_statement();
        self.writer.write("call $alloc");

        self.writer.new_statement();
        self.writer.write("global.set $_strings");

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
                address: Box::new(IrTerm::GetField {
                    address: Box::new(IrTerm::ident("p")),
                    offset: 0,
                }),
                value: string.bytes().map(|b| IrTerm::i32(b as i32)).collect(),
            })?;

            self.writer.new_statement();
            self.writer.write("global.get $_strings");

            self.writer.new_statement();
            self.writer.write(format!("i32.const {}", i));

            self.writer.new_statement();
            self.writer.write("i32.const 8");

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

        self.writer.write(
            r#"
(func $load_string (param $index i32) (result i32)
    global.get $_strings
    local.get $index
    i32.const 8
    i32.mul
    i32.add
    i32.load
)
"#,
        );

        let (_, result) = self.main_signature.clone().unwrap();

        self.writer.write(&format!(
            r#"
(func $start {}
    call $prepare_strings
    call ${}
)
"#,
            if result.is_nil() {
                "".to_string()
            } else {
                format!("(result {})", result.to_string())
            },
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

        if !result.is_nil() {
            self.writer
                .write(&format!("(result {})", result.to_string()));
        }

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

    fn expr(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Nil => {
                self.writer.write(&format!("i32.const {}", -1));
            }
            IrTerm::I32(i) => {
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
                cond,
                type_,
                then,
                else_,
            } => {
                self.writer.new_statement();
                self.expr(cond)?;

                self.writer.start();
                self.writer.write("if");
                if !type_.is_nil() {
                    self.writer
                        .write(&format!("(result {})", type_.to_string()));
                }

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
            IrTerm::GetField { address, offset } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::I32(offset.clone() as i32))?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.writer.write("i32.load");
            }
            IrTerm::SetField {
                address,
                offset,
                value,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::I32(offset.clone() as i32))?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write("i32.store");
            }
            IrTerm::While { cond, body } => {
                /*  [while(cond) {block}]

                    (block $exit
                        (loop
                            (not (cond))
                            (br_if $exit)

                            (block)
                            (br 0)
                        )
                    )
                */

                self.writer.start();
                self.writer.write("block");
                self.writer.write("$exit");

                self.writer.start();
                self.writer.write("loop");
                self.writer.write("$loop");

                self.expr(cond.as_mut())?;
                self.writer.new_statement();
                self.writer.write("i32.eqz");
                self.writer.new_statement();
                self.writer.write("br_if $exit");

                self.expr(body.as_mut())?;
                self.writer.new_statement();
                self.writer.write("br $loop");

                self.writer.end();
                self.writer.end();
            }
            IrTerm::PointerAt {
                type_,
                address,
                index,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::Call {
                    callee: Box::new(IrTerm::ident("mult")),
                    args: vec![
                        index.as_ref().clone(),
                        IrTerm::SizeOf {
                            type_: type_.clone(),
                        },
                    ],
                    source: None,
                })?;

                self.writer.new_statement();
                self.writer.write(&format!("{}.add", type_.to_string()));

                self.writer.new_statement();
                self.writer.write(&format!("{}.load", type_.to_string()));
            }
            IrTerm::SetPointer { address, value } => {
                self.writer.new_statement();
                self.expr_left_value(address)?;

                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write("i32.store");
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
                    self.expr(&mut IrTerm::SetPointer {
                        address: Box::new(IrTerm::PointerAt {
                            type_: type_.clone(),
                            address: address.clone(),
                            index: Box::new(IrTerm::I32(i as i32)),
                        }),
                        value: Box::new(v.clone()),
                    })?;
                }
            }
            IrTerm::Continue => {
                self.writer.new_statement();
                self.writer.write("br $loop");
            }
            IrTerm::PointerOffset { address, offset } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(offset)?;

                self.writer.new_statement();
                self.writer.write("i32.add");
            }
            _ => todo!(),
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
            IrTerm::PointerAt {
                type_,
                address,
                index,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::Call {
                    callee: Box::new(IrTerm::ident("mult")),
                    args: vec![
                        index.as_ref().clone(),
                        IrTerm::SizeOf {
                            type_: type_.clone(),
                        },
                    ],
                    source: None,
                })?;

                self.writer.new_statement();
                self.writer.write(&format!("{}.add", type_.to_string()));
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
                "div" => {
                    self.writer.write("i32.div_s");
                }
                "mod" => {
                    self.writer.write("i32.rem_s");
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
                "or" => {
                    self.writer.write("i32.or");
                }
                "and" => {
                    self.writer.write("i32.and");
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
