use std::collections::HashMap;

use anyhow::Result;
use log::info;

use crate::{
    ir::{IrElement, IrTerm, IrType},
    vm::{QVMInstruction, Variable},
    vm_optimizer::VmOptimizer,
};

macro_rules! unvec {
    ($e:expr, 1) => {{
        assert_eq!($e.len(), 1);
        let v1 = $e.pop().unwrap();

        (v1)
    }};
    ($e:expr, 2) => {{
        assert_eq!($e.len(), 2);
        let v2 = $e.pop().unwrap();
        let v1 = $e.pop().unwrap();

        (v1, v2)
    }};
    ($e:expr, 3) => {{
        assert_eq!($e.len(), 3);
        let v3 = $e.pop().unwrap();
        let v2 = $e.pop().unwrap();
        let v1 = $e.pop().unwrap();

        (v1, v2, v3)
    }};
    ($e:expr, 4) => {{
        assert_eq!($e.len(), 4);
        let v4 = $e.pop().unwrap();
        let v3 = $e.pop().unwrap();
        let v2 = $e.pop().unwrap();
        let v1 = $e.pop().unwrap();

        (v1, v2, v3, v4)
    }};
}

#[derive(Debug)]
struct Args(Vec<IrType>);

impl Args {
    pub fn arg_position(&self, arg: usize) -> Result<(usize, IrType)> {
        let mut index = 0;
        let mut result_type = IrType::nil();
        for typ in self.0.iter().rev().take(arg + 1) {
            index += typ.size_of();
            result_type = typ.clone();
        }

        Ok((index - 1, result_type))
    }

    pub fn total_word(&self) -> Result<usize> {
        let mut total = 0;
        for i in 0..self.0.len() {
            total += &self.0[i].size_of();
        }

        Ok(total)
    }
}

#[derive(Debug)]
struct VmFunctionGenerator<'s> {
    code: Vec<QVMInstruction>,
    args: Args,
    local_pointer: usize,
    locals: HashMap<String, (usize, IrType)>,
    globals: &'s HashMap<String, (usize, IrType)>,
    labels: &'s mut HashMap<String, usize>,
    offset: usize,
    label_continue: Option<String>,
    source_map: HashMap<usize, String>,
    scope_local_pointers: Vec<usize>,
    current_continue_scope: Option<usize>,
    string_pointers: &'s Vec<usize>,
}

impl<'s> VmFunctionGenerator<'s> {
    pub fn new(
        args: Vec<IrType>,
        globals: &'s HashMap<String, (usize, IrType)>,
        labels: &'s mut HashMap<String, usize>,
        offset: usize,
        string_pointers: &'s Vec<usize>,
    ) -> VmFunctionGenerator<'s> {
        VmFunctionGenerator {
            code: Vec::new(),
            args: Args(args),
            local_pointer: 0,
            locals: HashMap::new(),
            globals,
            labels,
            offset,
            label_continue: None,
            source_map: HashMap::new(),
            scope_local_pointers: Vec::new(),
            current_continue_scope: None,
            string_pointers,
        }
    }

    fn push_local(&mut self, name: String, size: usize, typ: IrType) {
        self.locals.insert(name, (self.local_pointer, typ));
        self.local_pointer += size;
    }

    fn register_label(&mut self, name: String) {
        self.labels.insert(name, self.offset + self.code.len());
    }

    fn new_source_map(&mut self, s: impl Into<String>) {
        self.source_map
            .insert(self.offset + self.code.len(), s.into());
    }

    fn builtin_instructions(&self, v: &str) -> (QVMInstruction, IrType) {
        match v {
            "_add" => (
                QVMInstruction::Add,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::int())),
            ),
            "_sub" => (
                QVMInstruction::Sub,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::int())),
            ),
            "_mult" => (
                QVMInstruction::Mult,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::int())),
            ),
            "_eq" => (
                QVMInstruction::Eq,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::bool())),
            ),
            "_neq" => (
                QVMInstruction::Neq,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::bool())),
            ),
            "_new" => (QVMInstruction::Alloc, todo!()),
            "_padd" => (
                QVMInstruction::PAdd,
                IrType::func(vec![IrType::addr(), IrType::int()], Box::new(IrType::int())),
            ),
            "_lt" => (
                QVMInstruction::Lt,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::bool())),
            ),
            "_gt" => (
                QVMInstruction::Gt,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::bool())),
            ),
            "_div" => (
                QVMInstruction::Div,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::int())),
            ),
            "_mod" => (
                QVMInstruction::Mod,
                IrType::func(vec![IrType::int(), IrType::int()], Box::new(IrType::int())),
            ),
            "_not" => (
                QVMInstruction::Not,
                IrType::func(vec![IrType::bool()], Box::new(IrType::bool())),
            ),
            "_or" => (
                QVMInstruction::Or,
                IrType::func(
                    vec![IrType::bool(), IrType::bool()],
                    Box::new(IrType::bool()),
                ),
            ),
            "_and" => (
                QVMInstruction::And,
                IrType::func(
                    vec![IrType::bool(), IrType::bool()],
                    Box::new(IrType::bool()),
                ),
            ),
            "_gc" => (
                QVMInstruction::RuntimeInstr("_gc".to_string()),
                IrType::nil(),
            ),
            "_panic" => (
                QVMInstruction::RuntimeInstr("_panic".to_string()),
                IrType::nil(),
            ),
            "_len" => (
                QVMInstruction::RuntimeInstr("_len".to_string()),
                IrType::func(vec![IrType::addr()], Box::new(IrType::int())),
            ),
            "_println" => (
                QVMInstruction::RuntimeInstr("_println".to_string()),
                IrType::func(vec![IrType::int()], Box::new(IrType::nil())),
            ),
            "_copy" => (
                QVMInstruction::RuntimeInstr("_copy".to_string()),
                IrType::func(
                    vec![IrType::addr(), IrType::addr()],
                    Box::new(IrType::nil()),
                ),
            ),
            "_debug" => (
                QVMInstruction::RuntimeInstr("_debug".to_string()),
                IrType::nil(),
            ),
            "_int_to_byte" => (QVMInstruction::Nop, todo!()),
            "_byte_to_int" => (QVMInstruction::Nop, todo!()),
            "_nil_to_ref" => (QVMInstruction::Nop, todo!()),
            "_start_debugger" => (
                QVMInstruction::RuntimeInstr("_start_debugger".to_string()),
                IrType::nil(),
            ),
            "_check_sp" => (
                QVMInstruction::RuntimeInstr("_check_sp".to_string()),
                todo!(),
            ),
            _ => (QVMInstruction::LabelI32Const(v.to_string()), IrType::nil()),
        }
    }

    // compile to an address (lvar)
    fn element_addr(&mut self, element: IrElement) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v, _) => {
                    if let Some((u, t)) = self.locals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, Variable::Local));
                    } else if let Some((u, t)) = self.globals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, Variable::Global));
                    } else {
                        self.code.push(self.builtin_instructions(v.as_str()).0);
                    }
                }
                IrTerm::Argument(v, _) => {
                    let (position, t) = self.args.arg_position(v)?;
                    self.code.push(QVMInstruction::ArgConst(position));
                }
                _ => unreachable!("{}", element.show()),
            },
            IrElement::Block(mut block) => match block.name.as_str() {
                "call" => {
                    self.element(element)?;
                }
                // FIXME: is this true?
                "load" | "deref" => {
                    self.new_source_map(element.show_compact());

                    let (_, element) = unvec!(block.elements, 2);
                    self.element(element)?;
                }
                "offset" => {
                    self.new_source_map(element.show_compact());

                    let (_, element, offset) = unvec!(block.elements, 3);
                    self.element_addr(element)?;
                    self.element(offset)?;
                    self.code.push(QVMInstruction::PAdd);
                }
                _ => unreachable!("{}", element.show()),
            },
        };

        Ok(IrType::addr())
    }

    fn element(&mut self, element: IrElement) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v, size) => {
                    if let Some((u, t)) = self.locals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, Variable::Local));
                        self.code.push(QVMInstruction::Load(size));

                        Ok(t.clone())
                    } else if let Some((u, t)) = self.globals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, Variable::Global));
                        self.code.push(QVMInstruction::Load(size));

                        Ok(t.clone())
                    } else {
                        let (code, t) = self.builtin_instructions(v.as_str());

                        self.code.push(code);
                        Ok(t)
                    }
                }
                IrTerm::Nil => {
                    self.code.push(QVMInstruction::NilConst);
                    Ok(IrType::nil())
                }
                IrTerm::Bool(b) => {
                    self.code.push(QVMInstruction::BoolConst(b));
                    Ok(IrType::bool())
                }
                IrTerm::Int(n) => {
                    self.code.push(QVMInstruction::I32Const(n));
                    Ok(IrType::int())
                }
                IrTerm::Argument(u, size) => {
                    let (index, t) = self.args.arg_position(u)?;

                    self.code.push(QVMInstruction::ArgConst(index));
                    self.code.push(QVMInstruction::Load(size));
                    Ok(t)
                }
                IrTerm::Info(u) => {
                    self.code.push(QVMInstruction::InfoConst(u));

                    Ok(IrType::unknown())
                }
            },
            IrElement::Block(mut block) => {
                match block.name.as_str() {
                    "let" => {
                        self.new_source_map(element.show_compact());
                        let (typ, name, expr) = unvec!(block.elements, 3);
                        let var_name = name.into_term()?.into_ident()?;
                        let size = IrElement::size_of_as_ir_type(&typ)?;

                        self.element(expr)?;
                        self.push_local(var_name, size, IrType::from_element(&typ)?);
                        Ok(IrType::nil())
                    }
                    "return" => {
                        self.new_source_map(element.show_compact());
                        let (size, expr) = unvec!(block.elements, 2);
                        let size = size.into_term()?.into_int()? as usize;
                        self.element(expr)?;
                        self.code
                            .push(QVMInstruction::Return(self.args.total_word()?, size));
                        Ok(IrType::unknown())
                    }
                    "call" => {
                        self.new_source_map(element.show_compact());
                        let mut callee = None;
                        let mut first = true;
                        for elem in block.elements {
                            if first {
                                first = false;
                                callee = Some(elem);
                                continue;
                            }

                            self.element(elem)?;
                        }
                        self.element(callee.unwrap())?;

                        // If the last instruction is not LabelAddrConst, it will be a builtin operation and no need to run CALL operation
                        if matches!(self.code.last().unwrap(), QVMInstruction::LabelI32Const(_)) {
                            self.code.push(QVMInstruction::Call);
                        }

                        Ok(IrType::unknown())
                    }
                    "assign" => {
                        self.new_source_map(element.show_compact());
                        let (size, lhs, rhs) = unvec!(block.elements, 3);
                        self.element_addr(lhs)?;
                        self.element(rhs)?;
                        self.code
                            .push(QVMInstruction::Store(size.into_term()?.into_int()? as usize));

                        Ok(IrType::nil())
                    }
                    "if" => {
                        self.new_source_map(IrElement::block("if", vec![]).show_compact());
                        let (cond, left, right) = unvec!(block.elements, 3);

                        // FIXME: Area these labels really unqiue?
                        let index = self.labels.len();
                        let label = format!("if-{}-{}", self.globals.len(), index);
                        let label_else = format!("else-{}-{}", self.globals.len(), index);
                        let label_end = format!("end-{}-{}", self.globals.len(), index);

                        // condition
                        self.register_label(label.clone());
                        self.new_source_map(IrElement::block("if:cond", vec![]).show_compact());
                        self.element(cond)?;
                        self.code
                            .push(QVMInstruction::LabelJumpIfFalse(label_else.clone()));

                        // then block
                        self.new_source_map(IrElement::block("if:then", vec![]).show_compact());
                        self.element(left)?;
                        self.code.push(QVMInstruction::LabelJump(label_end.clone()));

                        // else block
                        self.new_source_map(IrElement::block("if:else", vec![]).show_compact());
                        self.register_label(label_else.clone());
                        self.element(right)?;

                        // endif
                        self.register_label(label_end.clone());
                        self.new_source_map(IrElement::block("if:end", vec![]).show_compact());

                        Ok(IrType::unknown())
                    }
                    "seq" => {
                        self.new_source_map(IrElement::block("seq", vec![]).show_compact());
                        for elem in block.elements {
                            self.element(elem)?;
                        }

                        Ok(IrType::unknown())
                    }
                    "while" => {
                        self.current_continue_scope = Some(self.local_pointer);

                        self.new_source_map(IrElement::block("while", vec![]).show_compact());
                        let (cond, body) = unvec!(block.elements, 2);

                        let label = format!("while-{}-{}", self.globals.len(), self.labels.len());
                        let label_cond =
                            format!("while-cond-{}-{}", self.globals.len(), self.labels.len());
                        self.label_continue = Some(label_cond.clone());

                        self.code
                            .push(QVMInstruction::LabelJump(label_cond.clone()));

                        self.new_source_map(IrElement::block("while:body", vec![]).show_compact());
                        self.register_label(label.clone());

                        // additional check
                        self.element(IrElement::Term(IrTerm::Int(self.local_pointer as i32)))?;
                        self.element(IrElement::Term(IrTerm::Ident("_check_sp".to_string(), 1)))?;

                        self.element(body)?;

                        self.register_label(label_cond.clone());
                        self.new_source_map(IrElement::block("while:cond", vec![]).show_compact());
                        self.element(cond)?;

                        self.code.push(QVMInstruction::LabelJumpIf(label.clone()));
                        self.label_continue = None;
                        self.new_source_map(IrElement::block("while:end", vec![]).show_compact());

                        Ok(IrType::unknown())
                    }
                    "continue" => {
                        self.new_source_map(element.show_compact());

                        // pop local variables just like end_scope
                        let p = self.current_continue_scope.unwrap();
                        self.code.push(QVMInstruction::Pop(self.local_pointer - p));
                        self.code.push(QVMInstruction::LabelJump(
                            self.label_continue.clone().unwrap(),
                        ));

                        Ok(IrType::unknown())
                    }
                    "begin_scope" => {
                        self.new_source_map(element.show_compact());
                        self.scope_local_pointers.push(self.local_pointer);

                        Ok(IrType::nil())
                    }
                    "end_scope" => {
                        self.new_source_map(element.show_compact());
                        let p = self.scope_local_pointers.pop().unwrap();
                        self.code.push(QVMInstruction::Pop(self.local_pointer - p));
                        self.local_pointer = p;

                        Ok(IrType::nil())
                    }
                    "pop" => {
                        self.new_source_map(element.show_compact());
                        let n = unvec!(block.elements, 1);
                        self.code
                            .push(QVMInstruction::Pop(n.into_term()?.into_int()? as usize));

                        Ok(IrType::nil())
                    }
                    "data" => {
                        self.new_source_map(IrElement::block("data", vec![]).show_compact());

                        let mut size = 0;
                        for (i, elem) in block.elements.into_iter().enumerate() {
                            if i == 0 {
                                size = elem.into_term()?.into_int()? as usize;

                                self.code.push(QVMInstruction::InfoConst(size));
                            } else {
                                self.element(elem)?;
                            }
                        }

                        Ok(IrType::tuple(size, vec![]))
                    }
                    // FIXME: integrate with _copy and load and _deref?
                    "copy" => {
                        self.new_source_map(element.show_compact());
                        let (size, elem) = unvec!(block.elements, 2);

                        self.element(elem)?;
                        self.code
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    "load" => {
                        self.new_source_map(element.show_compact());
                        let (size, element) = unvec!(block.elements, 2);
                        self.element(element.clone())?;
                        self.code
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    "unload" => {
                        self.new_source_map(element.show_compact());
                        let element = unvec!(block.elements, 1);
                        self.element(element.clone())?;
                        let code = self.code.pop();
                        assert!(
                            matches!(code, Some(QVMInstruction::Load(_))),
                            "{:?} {}",
                            code,
                            element.show()
                        );

                        Ok(IrType::unknown())
                    }
                    "string" => {
                        self.new_source_map(element.show_compact());
                        let string = unvec!(block.elements, 1);
                        let n = string.into_term()?.into_int()? as usize;
                        self.code.push(QVMInstruction::AddrConst(
                            self.string_pointers[n],
                            Variable::StackAbsolute,
                        ));

                        Ok(IrType::unknown())
                    }
                    "deref" => {
                        self.new_source_map(element.show_compact());
                        let (size, element) = unvec!(block.elements, 2);
                        self.element(element)?;
                        self.code
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    "coerce" => {
                        self.new_source_map(element.show_compact());
                        let (actual_size, expected_size, element) = unvec!(block.elements, 3);
                        let actual_size = actual_size.into_term()?.into_int()? as usize;
                        let expected_size = expected_size.into_term()?.into_int()? as usize;

                        self.element(element)?;
                        self.code.extend(vec![
                            QVMInstruction::I32Const(expected_size as i32),
                            QVMInstruction::I32Const(actual_size as i32),
                            QVMInstruction::RuntimeInstr("_coerce".to_string()),
                        ]);

                        Ok(IrType::unknown())
                    }
                    "address" => {
                        self.new_source_map(element.show_compact());
                        let element = unvec!(block.elements, 1);
                        self.element_addr(element)?;

                        Ok(IrType::unknown())
                    }
                    "offset" => {
                        self.new_source_map(element.show_compact());
                        let (size, element, offset) = unvec!(block.elements, 3);
                        self.element_addr(element)?;
                        self.element(offset)?;
                        self.code.push(QVMInstruction::PAdd);
                        self.code
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    name => todo!("{:?}", name),
                }
            }
        }
    }
}

pub struct VmGenerator {
    globals: HashMap<String, (usize, IrType)>,
    global_pointer: usize,
    entrypoint: String,
}

impl VmGenerator {
    pub fn new() -> VmGenerator {
        VmGenerator {
            globals: HashMap::new(),
            global_pointer: 0,
            entrypoint: "main".to_string(),
        }
    }

    pub fn set_entrypoint(&mut self, name: String) {
        self.entrypoint = name;
    }

    fn push_global(&mut self, name: String, typ: IrType) {
        self.globals.insert(name, (self.global_pointer, typ));
        self.global_pointer += 1;
    }

    pub fn globals(&self) -> usize {
        self.globals.len()
    }

    pub fn function(
        &mut self,
        body: Vec<IrElement>,
        args: Vec<IrType>,
        labels: &mut HashMap<String, usize>,
        offset: usize,
        string_pointers: &Vec<usize>,
    ) -> Result<(Vec<QVMInstruction>, HashMap<usize, String>)> {
        let mut generator =
            VmFunctionGenerator::new(args, &self.globals, labels, offset, string_pointers);

        let mut skip = 2;
        for statement in body {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            generator.element(statement)?;
        }

        Ok((generator.code, generator.source_map))
    }

    pub fn variable(
        &mut self,
        name: String,
        size: usize,
        expr: IrElement,
        offset: usize,
        labels: &mut HashMap<String, usize>,
        string_pointers: &Vec<usize>,
    ) -> Result<Vec<QVMInstruction>> {
        let (g, t) = self.globals[&name].clone();
        self.push_global(name.clone(), t);

        let mut code = vec![QVMInstruction::AddrConst(g, Variable::Global)];

        let mut generator =
            VmFunctionGenerator::new(vec![], &self.globals, labels, offset, string_pointers);
        generator.element(expr)?;
        code.extend(generator.code);
        code.push(QVMInstruction::Store(size));

        Ok(code)
    }

    pub fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
        string_pointers: &Vec<usize>,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator =
            VmFunctionGenerator::new(vec![], &self.globals, labels, offset, &string_pointers);
        generator.element(IrElement::block(
            "return",
            vec![
                IrElement::Term(IrTerm::Int(1)),
                IrElement::instruction("call", vec![IrTerm::Ident(self.entrypoint.to_string(), 1)]),
            ],
        ))?;

        Ok(generator.code)
    }

    pub fn generate(
        &mut self,
        element: IrElement,
    ) -> Result<(Vec<QVMInstruction>, HashMap<usize, String>)> {
        let mut code = vec![];
        let mut labels = HashMap::new();

        let mut functions = vec![];
        let mut variables = vec![];

        let mut source_map: HashMap<usize, String> = HashMap::new();

        // = first path

        // collect functions
        let block = element.into_block()?;
        assert_eq!(block.name, "module");

        let mut string_pointers = vec![];

        for element in block.elements {
            let mut block = element.into_block()?;
            match block.name.as_str() {
                "text" => {
                    // text segment must be processed at first
                    // +1 for skipping the size value
                    string_pointers.push(code.len() + 1);
                    for i in block.elements {
                        code.push(QVMInstruction::I32Const(i.into_term()?.into_int()?));
                    }
                }
                "func" => {
                    let type_block = block.elements[1].clone().into_block()?;
                    assert_eq!(type_block.name, "args");

                    functions.push((
                        block.elements[0].clone().into_term()?.into_ident()?, // name
                        type_block
                            .elements
                            .into_iter()
                            .map(|b| IrType::from_element(&b))
                            .collect::<Result<_, _>>()?, // arg types
                        block.elements,
                    ));
                }
                "var" => {
                    let (size, name, expr) = unvec!(block.elements, 3);
                    variables.push((
                        name.into_term()?.into_ident()?,
                        size.into_term()?.into_int()? as usize,
                        expr,
                    ));
                }
                _ => unreachable!("{:?}", block),
            }
        }

        for (name, size, expr) in variables {
            code.extend(self.variable(
                name,
                size,
                expr,
                code.len(),
                &mut labels,
                &string_pointers,
            )?);
        }

        // call main
        labels.insert(self.entrypoint.to_string(), code.len());
        code.extend(self.call_main(&mut labels, code.len(), &string_pointers)?);

        for (name, args, function) in functions {
            labels.insert(name, code.len());

            let (code_generated, source_map_generated) =
                self.function(function, args, &mut labels, code.len(), &string_pointers)?;
            code.extend(code_generated);
            source_map.extend(source_map_generated);
        }

        // = second path

        // resolve labels
        for i in 0..code.len() {
            if let QVMInstruction::LabelI32Const(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::I32Const(*pc as i32);
                } else {
                    info!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJumpIfFalse(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::JumpIfFalse(*pc);
                } else {
                    info!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJumpIf(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::JumpIf(*pc);
                } else {
                    info!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJump(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::Jump(*pc);
                } else {
                    info!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            }
        }

        // optimize
        {
            let mut optimizer = VmOptimizer::new();
            let (optimized_code, code_map) = optimizer.optimize(code);

            let mut new_source_map: HashMap<usize, String> = HashMap::new();
            for (k, v) in source_map {
                if let Some(p) = code_map.get(k) {
                    new_source_map
                        .entry(*p)
                        .and_modify(|e| {
                            *e += v.as_str();
                        })
                        .or_insert(v.to_string());
                }
            }

            code = optimized_code;
            source_map = new_source_map;
        }

        Ok((code, source_map))
    }
}
