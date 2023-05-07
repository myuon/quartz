use std::collections::{HashMap, HashSet};

use anyhow::{bail, Ok, Result};

use crate::{
    ast::Type,
    compiler::{MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN, MODE_READABLE_WASM},
    ir::{IrTerm, IrType},
    util::{ident::Ident, path::Path, sexpr_writer::SExprWriter},
    value::Value,
};

pub struct Generator {
    pub writer: SExprWriter,
    pub globals: HashSet<String>,
    pub types: HashMap<Ident, (Vec<Type>, Type)>,
    pub main_signature: Option<(Vec<IrType>, IrType)>,
    pub entrypoint: Path,
    pub strings: Vec<String>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: SExprWriter::new(),
            globals: HashSet::new(),
            types: HashMap::new(),
            main_signature: None,
            entrypoint: Path::new(vec![]),
            strings: vec![],
        }
    }

    pub fn set_globals(&mut self, globals: HashSet<Path>) {
        self.globals = globals
            .iter()
            .map(|p| p.as_joined_str("_"))
            .collect::<HashSet<_>>();
    }

    pub fn set_types(&mut self, types: HashMap<Ident, (Vec<Type>, Type)>) {
        self.types = types;
    }

    pub fn set_entrypoint(&mut self, entrypoint: Path) {
        self.entrypoint = entrypoint;
    }

    pub fn set_strings(&mut self, strings: Vec<String>) {
        self.strings = strings;
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

        self.decl(&mut IrTerm::Declare {
            name: "write_stdout".to_string(),
            params: vec![IrType::Byte],
            result: IrType::I32,
        })?;

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
            params: vec![IrType::I32],
            result: IrType::Nil,
        })?;

        self.decl(&mut IrTerm::Declare {
            name: "read_handler".to_string(),
            params: vec![IrType::I32],
            result: IrType::I32,
        })?;

        self.writer.start();
        self.writer.write(r#"memory 200"#);
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
            name: "reflection_is_pointer".to_string(),
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
            name: "reflection_is_bool".to_string(),
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
            body: vec![
                IrTerm::Assign {
                    lhs: "quartz_std_alloc_ptr".to_string(),
                    rhs: Box::new(IrTerm::i32(data_section_offset as i32)),
                },
                IrTerm::Assign {
                    lhs: "quartz_std_strings_count".to_string(),
                    rhs: Box::new(IrTerm::i32(self.strings.len() as i32)),
                },
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
                    self.writer.write(";; nil");
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
                        " ;; {}",
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

                self.writer.new_statement();
                self.expr(offset)?;

                self.convert_stack_to_i32_2();

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

                self.writer.new_statement();
                self.expr(offset)?;

                self.convert_stack_to_i32_2();

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
}
