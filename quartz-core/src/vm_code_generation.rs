use std::collections::HashMap;

use anyhow::{Context, Result};
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
struct InstructionWriter {
    code: Vec<QVMInstruction>,
    offset: usize,
}

impl InstructionWriter {
    pub fn push(&mut self, instruction: QVMInstruction) {
        self.code.push(instruction);
    }

    pub fn extend(&mut self, instructions: Vec<QVMInstruction>) {
        self.code.extend(instructions);
    }

    pub fn get_code_address(&self) -> usize {
        self.offset + self.code.len()
    }

    pub fn into_code(self) -> Vec<QVMInstruction> {
        self.code
    }

    pub fn last(&self) -> Option<&QVMInstruction> {
        self.code.last()
    }
}

#[derive(Debug)]
struct VmFunctionGenerator<'s> {
    writer: InstructionWriter,
    args: Args,
    local_pointer: usize,
    locals: HashMap<String, (usize, IrType)>,
    globals: &'s HashMap<String, (usize, IrType)>,
    labels: &'s mut HashMap<String, usize>,
    functions: &'s HashMap<String, IrType>,
    label_continue: Option<String>,
    source_map: HashMap<usize, String>,
    scope_local_pointers: Vec<usize>,
    current_continue_scope: Option<usize>,
    string_pointers: &'s Vec<usize>,
    expected_type: IrType,
}

impl<'s> VmFunctionGenerator<'s> {
    pub fn new(
        writer: InstructionWriter,
        args: Vec<IrType>,
        globals: &'s HashMap<String, (usize, IrType)>,
        labels: &'s mut HashMap<String, usize>,
        functions: &'s HashMap<String, IrType>,
        string_pointers: &'s Vec<usize>,
        expected_type: IrType,
    ) -> VmFunctionGenerator<'s> {
        VmFunctionGenerator {
            writer,
            args: Args(args),
            local_pointer: 0,
            locals: HashMap::new(),
            globals,
            labels,
            functions,
            label_continue: None,
            source_map: HashMap::new(),
            scope_local_pointers: Vec::new(),
            current_continue_scope: None,
            string_pointers,
            expected_type,
        }
    }

    fn push_local(&mut self, name: String, typ: IrType) {
        let size = typ.size_of();
        self.locals.insert(name, (self.local_pointer, typ));
        self.local_pointer += size;
    }

    fn register_label(&mut self, name: String) {
        self.labels.insert(name, self.writer.get_code_address());
    }

    fn new_source_map(&mut self, s: impl Into<String>) {
        self.source_map
            .insert(self.writer.get_code_address(), s.into());
    }

    fn resolve_symbol(&self, v: &str) -> (QVMInstruction, IrType) {
        match v {
            "_add" => (
                QVMInstruction::Add,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::int()),
            ),
            "_sub" => (
                QVMInstruction::Sub,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::int()),
            ),
            "_mult" => (
                QVMInstruction::Mult,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::int()),
            ),
            "_eq" => (
                QVMInstruction::Eq,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::bool()),
            ),
            "_neq" => (
                QVMInstruction::Neq,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::bool()),
            ),
            "_new" => (
                QVMInstruction::Alloc,
                IrType::func(vec![IrType::int()], IrType::addr_unknown()),
            ),
            "_padd" => (
                QVMInstruction::PAdd,
                IrType::func(
                    vec![IrType::addr_of(IrType::unknown()), IrType::int()],
                    IrType::int(),
                ),
            ),
            "_lt" => (
                QVMInstruction::Lt,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::bool()),
            ),
            "_gt" => (
                QVMInstruction::Gt,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::bool()),
            ),
            "_div" => (
                QVMInstruction::Div,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::int()),
            ),
            "_mod" => (
                QVMInstruction::Mod,
                IrType::func(vec![IrType::int(), IrType::int()], IrType::int()),
            ),
            "_not" => (
                QVMInstruction::Not,
                IrType::func(vec![IrType::bool()], IrType::bool()),
            ),
            "_or" => (
                QVMInstruction::Or,
                IrType::func(vec![IrType::bool(), IrType::bool()], IrType::bool()),
            ),
            "_and" => (
                QVMInstruction::And,
                IrType::func(vec![IrType::bool(), IrType::bool()], IrType::bool()),
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
                IrType::func(
                    vec![IrType::tuple(vec![IrType::addr_of(IrType::unknown())])],
                    IrType::int(),
                ),
            ),
            "_println" => (
                QVMInstruction::RuntimeInstr("_println".to_string()),
                IrType::func(vec![IrType::int()], IrType::nil()),
            ),
            "_copy" => (
                QVMInstruction::RuntimeInstr("_copy".to_string()),
                IrType::func(
                    vec![
                        IrType::addr_of(IrType::unknown()),
                        IrType::addr_of(IrType::unknown()),
                    ],
                    IrType::nil(),
                ),
            ),
            "_debug" => (
                QVMInstruction::RuntimeInstr("_debug".to_string()),
                IrType::nil(),
            ),
            "_byte_to_int" => (
                QVMInstruction::Nop,
                IrType::func(vec![IrType::byte()], IrType::int()),
            ),
            "_start_debugger" => (
                QVMInstruction::RuntimeInstr("_start_debugger".to_string()),
                IrType::nil(),
            ),
            "_check_sp" => (
                QVMInstruction::RuntimeInstr("_check_sp".to_string()),
                IrType::nil(),
            ),
            "_is_nil" => (
                QVMInstruction::RuntimeInstr("_is_nil".to_string()),
                IrType::func(vec![IrType::addr_of(IrType::unknown())], IrType::bool()),
            ),
            _ => (
                QVMInstruction::LabelI32Const(v.to_string()),
                self.functions
                    .get(v)
                    .ok_or(anyhow::anyhow!("Unknown symbol {}", v))
                    .unwrap()
                    .clone(),
            ),
        }
    }

    // compile to an address (lvar)
    fn element_addr(&mut self, element: IrElement) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v) => {
                    if let Some((u, t)) = self.locals.get(&v) {
                        self.writer
                            .push(QVMInstruction::AddrConst(*u, Variable::Local));

                        Ok(IrType::addr_of(t.clone()))
                    } else if let Some((u, t)) = self.globals.get(&v) {
                        self.writer
                            .push(QVMInstruction::AddrConst(*u, Variable::Global));

                        Ok(IrType::addr_of(t.clone()))
                    } else {
                        // resolve a function
                        let (code, typ) = self.resolve_symbol(v.as_str());
                        self.writer.push(code);

                        Ok(IrType::addr_of(typ))
                    }
                }
                IrTerm::Argument(v) => {
                    let (position, t) = self.args.arg_position(v)?;
                    self.writer.push(QVMInstruction::ArgConst(position));

                    Ok(IrType::addr_of(t.clone()))
                }
                _ => unreachable!("{}", element.show()),
            },
            IrElement::Block(mut block) => match block.name.as_str() {
                "call" => Ok(IrType::addr_of(self.element(element)?)),
                // FIXME: is this true?
                "deref" => {
                    self.new_source_map(element.show_compact());

                    let element = unvec!(block.elements, 1);
                    let typ = self.element(element)?;
                    Ok(typ.as_addr().unwrap().as_ref().clone())
                }
                "offset" => {
                    self.new_source_map(element.show_compact());

                    let (elem, offset_element) = unvec!(block.elements, 2);
                    let offset = offset_element.into_term()?.into_int()? as usize;
                    let typ = self.element_addr(elem)?;
                    let inner_addr_typ = if typ.is_unknown() {
                        IrType::unknown()
                    } else {
                        typ.as_addr().unwrap().as_ref().clone()
                    };
                    self.writer.push(QVMInstruction::I32Const(
                        inner_addr_typ.clone().offset_in_words(offset)? as i32,
                    ));
                    self.writer.push(QVMInstruction::PAdd);

                    Ok(IrType::addr_of(if inner_addr_typ.is_unknown() {
                        IrType::unknown()
                    } else {
                        inner_addr_typ
                            .offset(offset)
                            .context(format!("{}", element.show()))?
                    }))
                }
                "addr_offset" => {
                    self.new_source_map(element.show_compact());

                    let (elem, offset_element) = unvec!(block.elements, 2);
                    let offset = offset_element.into_term()?.into_int()? as usize;
                    let typ = self.element(elem)?;
                    let inner_addr_typ = typ.as_addr().unwrap();
                    self.writer.push(QVMInstruction::I32Const(
                        inner_addr_typ.clone().offset_in_words(offset)? as i32,
                    ));
                    self.writer.push(QVMInstruction::PAdd);
                    Ok(IrType::addr_of(
                        inner_addr_typ
                            .offset(offset)
                            .context(format!("{}", element.show()))?,
                    ))
                }
                "index" => {
                    self.new_source_map(element.show_compact());

                    let (element, offset) = unvec!(block.elements, 2);
                    let typ = self.element_addr(element.clone())?;
                    IrType::int()
                        .unify(self.element(offset)?)
                        .context(format!("{}", element.show()))?;
                    self.writer.push(QVMInstruction::I32Const(1)); // +1 for a pointer to info table
                    self.writer.push(QVMInstruction::Add);
                    self.writer.push(QVMInstruction::PAdd);

                    let elem_typ = typ.as_addr().unwrap().as_element_sized().unwrap();

                    Ok(IrType::addr_of(elem_typ))
                }
                "addr_index" => {
                    self.new_source_map(element.show_compact());
                    let (element, offset) = unvec!(block.elements, 2);

                    let typ = self.element(element)?;

                    self.element(offset.clone())?
                        .unify(IrType::int())
                        .context(format!("{}", offset.show()))?;
                    self.writer.push(QVMInstruction::I32Const(1));
                    self.writer.push(QVMInstruction::Add);
                    self.writer.push(QVMInstruction::PAdd);

                    let elem_typ = typ.as_addr().unwrap().as_ref().clone();

                    Ok(IrType::addr_of(elem_typ))
                }
                _ => unreachable!("{}", element.show()),
            },
        }
    }

    fn element(&mut self, element: IrElement) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v) => {
                    if let Some((u, t)) = self.locals.get(&v) {
                        self.writer
                            .push(QVMInstruction::AddrConst(*u, Variable::Local));
                        self.writer.push(QVMInstruction::Load(t.size_of()));

                        Ok(t.clone())
                    } else if let Some((u, t)) = self.globals.get(&v) {
                        self.writer
                            .push(QVMInstruction::AddrConst(*u, Variable::Global));
                        self.writer.push(QVMInstruction::Load(t.size_of()));

                        Ok(t.clone())
                    } else {
                        // resolve an embedded instruction
                        let (code, typ) = self.resolve_symbol(v.as_str());
                        self.writer.push(code);

                        Ok(IrType::addr_of(typ))
                    }
                }
                IrTerm::Nil => {
                    self.writer.push(QVMInstruction::NilConst);
                    Ok(IrType::nil())
                }
                IrTerm::Bool(b) => {
                    self.writer.push(QVMInstruction::BoolConst(b));
                    Ok(IrType::bool())
                }
                IrTerm::Int(n) => {
                    self.writer.push(QVMInstruction::I32Const(n));
                    Ok(IrType::int())
                }
                IrTerm::Argument(u) => {
                    let (index, t) = self.args.arg_position(u)?;

                    self.writer.push(QVMInstruction::ArgConst(index));
                    self.writer.push(QVMInstruction::Load(t.size_of()));
                    Ok(t)
                }
                IrTerm::Info(u) => {
                    self.writer.push(QVMInstruction::InfoConst(u));

                    Ok(IrType::unknown())
                }
            },
            IrElement::Block(mut block) => {
                match block.name.as_str() {
                    "let" => {
                        self.new_source_map(element.show_compact());
                        let (name, expr) = unvec!(block.elements, 2);
                        let var_name = name.into_term()?.into_ident()?;

                        let typ = self.element(expr)?;
                        self.push_local(var_name, typ);
                        Ok(IrType::nil())
                    }
                    "return" => {
                        self.new_source_map(element.show_compact());
                        let expr = unvec!(block.elements, 1);
                        let typ = self.element(expr)?;
                        self.writer.push(QVMInstruction::Return(
                            self.args.total_word()?,
                            typ.size_of(),
                        ));

                        self.expected_type
                            .clone()
                            .unify(typ)
                            .context(format!("[return] {}", element.show()))?;

                        Ok(IrType::unknown())
                    }
                    "call" => {
                        self.new_source_map(element.show_compact());
                        let callee = block.elements[0].clone();

                        let mut arg_types = vec![];
                        for elem in block.elements.into_iter().skip(1) {
                            arg_types.push(self.element(elem)?);
                        }

                        let callee_type = self.element_addr(callee.clone())?;
                        let callee_typ = callee_type
                            .unify(IrType::addr_of(IrType::func(arg_types, IrType::unknown())))
                            .context(format!("{}", callee.show()))?;
                        let (_, ret_type) = callee_typ.as_addr().unwrap().as_func().unwrap();

                        // If the last instruction is not LabelAddrConst, it will be a builtin operation and no need to run CALL operation
                        if matches!(
                            self.writer.last().unwrap(),
                            QVMInstruction::LabelI32Const(_)
                        ) {
                            self.writer.push(QVMInstruction::Call);
                        }

                        Ok(ret_type.as_ref().clone())
                    }
                    "assign" => {
                        self.new_source_map(element.show_compact());
                        let (lhs, rhs) = unvec!(block.elements, 2);
                        let typ = self
                            .element_addr(lhs.clone())?
                            .unify(IrType::addr_unknown())
                            .context(format!("[assign] {}", lhs.show()))?;
                        let addr_inner_typ = typ
                            .as_addr()
                            .map(|t| t.as_ref().clone())
                            .unwrap_or(IrType::unknown());
                        let mut rhs_type = self.element(rhs.clone())?;

                        // NOTE: addr type can be used like anytype, so just skip unification
                        // FIXME: Is this true?
                        if !addr_inner_typ.as_addr().is_ok() {
                            rhs_type = rhs_type.unify(addr_inner_typ).context(format!(
                                "[assign:rhs] {}\n{}",
                                typ.to_element().show(),
                                element.show()
                            ))?;
                        }
                        self.writer.push(QVMInstruction::Store(rhs_type.size_of()));

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
                        self.element(cond.clone())?
                            .unify(IrType::bool())
                            .context(format!("{}", cond.show()))?;
                        self.writer
                            .push(QVMInstruction::LabelJumpIfFalse(label_else.clone()));

                        // then block
                        self.new_source_map(IrElement::block("if:then", vec![]).show_compact());
                        let typ = self.element(left)?;
                        self.writer
                            .push(QVMInstruction::LabelJump(label_end.clone()));

                        // else block
                        self.new_source_map(IrElement::block("if:else", vec![]).show_compact());
                        self.register_label(label_else.clone());
                        self.element(right.clone())?
                            .unify(typ)
                            .context(format!("{}", right.show()))?;

                        // endif
                        self.register_label(label_end.clone());
                        self.new_source_map(IrElement::block("if:end", vec![]).show_compact());

                        Ok(IrType::unknown())
                    }
                    "seq" => {
                        let mut ret = IrType::unknown();
                        self.new_source_map(IrElement::block("seq", vec![]).show_compact());
                        for elem in block.elements {
                            ret = self.element(elem)?;
                        }

                        Ok(ret)
                    }
                    "while" => {
                        self.current_continue_scope = Some(self.local_pointer);

                        self.new_source_map(IrElement::block("while", vec![]).show_compact());
                        let (cond, body) = unvec!(block.elements, 2);

                        let label = format!("while-{}-{}", self.globals.len(), self.labels.len());
                        let label_cond =
                            format!("while-cond-{}-{}", self.globals.len(), self.labels.len());
                        self.label_continue = Some(label_cond.clone());

                        self.writer
                            .push(QVMInstruction::LabelJump(label_cond.clone()));

                        self.new_source_map(IrElement::block("while:body", vec![]).show_compact());
                        self.register_label(label.clone());

                        // additional check
                        self.element(IrElement::Term(IrTerm::Int(self.local_pointer as i32)))?
                            .unify(IrType::int())?;
                        self.element(IrElement::Term(IrTerm::Ident("_check_sp".to_string())))?;

                        self.element(body)?.unify(IrType::nil())?;

                        self.register_label(label_cond.clone());
                        self.new_source_map(IrElement::block("while:cond", vec![]).show_compact());
                        self.element(cond)?.unify(IrType::bool())?;

                        self.writer.push(QVMInstruction::LabelJumpIf(label.clone()));
                        self.label_continue = None;
                        self.new_source_map(IrElement::block("while:end", vec![]).show_compact());

                        Ok(IrType::nil())
                    }
                    "continue" => {
                        self.new_source_map(element.show_compact());

                        // pop local variables just like end_scope
                        let p = self.current_continue_scope.unwrap();
                        self.writer
                            .push(QVMInstruction::Pop(self.local_pointer - p));
                        self.writer.push(QVMInstruction::LabelJump(
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
                        self.writer
                            .push(QVMInstruction::Pop(self.local_pointer - p));
                        self.local_pointer = p;

                        Ok(IrType::nil())
                    }
                    "pop" => {
                        self.new_source_map(element.show_compact());
                        let typ = unvec!(block.elements, 1);
                        let typ = IrType::from_element(&typ)?;
                        self.writer.push(QVMInstruction::Pop(typ.size_of()));

                        Ok(IrType::nil())
                    }
                    "tuple" => {
                        self.new_source_map(IrElement::block("data", vec![]).show_compact());

                        let typ = IrType::from_element(&block.elements[0])?;
                        self.writer.push(QVMInstruction::InfoConst(typ.size_of()));

                        let mut types = vec![];
                        for (i, elem) in block.elements.into_iter().skip(1).enumerate() {
                            types.push(
                                self.element(elem.clone())?
                                    .unify(
                                        typ.clone()
                                            .offset(i)
                                            .context(format!("{}", element.show()))?,
                                    )
                                    .context(format!("{}", elem.show()))?,
                            );
                        }

                        Ok(IrType::tuple(types))
                    }
                    "slice" => {
                        self.new_source_map(IrElement::block("slice", vec![]).show_compact());

                        let (len, typ, value) = unvec!(block.elements, 3);

                        let len = len.into_term()?.into_int()? as usize;
                        let typ = IrType::from_element(&typ)?;
                        let slice_typ = IrType::slice(len, Box::new(typ.clone()));

                        self.writer
                            .push(QVMInstruction::InfoConst(slice_typ.size_of()));

                        for _ in 0..len {
                            self.element(value.clone())?
                                .unify(typ.clone())
                                .context(format!("{}", value.show()))?;
                        }

                        Ok(slice_typ)
                    }
                    // FIXME: integrate with _copy and load and _deref?
                    "copy" => {
                        self.new_source_map(element.show_compact());
                        let (size, elem) = unvec!(block.elements, 2);

                        self.element(elem)?;
                        self.writer
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    "string" => {
                        self.new_source_map(element.show_compact());

                        let string = unvec!(block.elements, 1);
                        let n = string.into_term()?.into_int()? as usize;

                        self.writer.push(QVMInstruction::InfoConst(2));
                        self.writer.push(QVMInstruction::AddrConst(
                            self.string_pointers[n],
                            Variable::StackAbsolute,
                        ));

                        Ok(IrType::tuple(vec![IrType::addr_unknown()]))
                    }
                    "deref" => {
                        self.new_source_map(element.show_compact());
                        let element = unvec!(block.elements, 1);
                        let typ = self
                            .element(element.clone())?
                            .unify(IrType::addr_unknown())
                            .context(format!("{}", element.show()))?;
                        self.writer
                            .push(QVMInstruction::Load(typ.as_addr().unwrap().size_of()));

                        Ok(typ
                            .as_addr()
                            .map(|t| t.as_ref().clone())
                            .unwrap_or(IrType::unknown()))
                    }
                    "coerce" => {
                        self.new_source_map(element.show_compact());
                        let (actual_size, expected_size, element) = unvec!(block.elements, 3);
                        let actual_size = actual_size.into_term()?.into_int()? as usize;
                        let expected_size = expected_size.into_term()?.into_int()? as usize;

                        let typ = self.element(element)?;
                        assert_eq!(typ.size_of(), actual_size);
                        self.writer.extend(vec![
                            QVMInstruction::I32Const(expected_size as i32),
                            QVMInstruction::I32Const(actual_size as i32),
                            QVMInstruction::RuntimeInstr("_coerce".to_string()),
                        ]);

                        Ok(IrType::unknown())
                    }
                    "address" => {
                        self.new_source_map(element.show_compact());
                        let element = unvec!(block.elements, 1);
                        let typ = self
                            .element_addr(element.clone())?
                            .unify(IrType::addr_unknown())
                            .context(format!("{}", element.show()))?;

                        Ok(typ)
                    }
                    "offset" => {
                        self.new_source_map(element.show_compact());
                        let (expr, offset_element) = unvec!(block.elements, 2);
                        let typ = self
                            .element_addr(expr)?
                            .unify(IrType::addr_unknown())
                            .context(format!("{}", element.show()))?;

                        let offset = offset_element.into_term()?.into_int()? as usize;
                        let inner_addr_typ = typ.as_addr().unwrap();
                        self.writer.push(QVMInstruction::I32Const(
                            inner_addr_typ.clone().offset_in_words(offset)? as i32,
                        ));
                        self.writer.push(QVMInstruction::PAdd);

                        self.writer.push(QVMInstruction::Load(typ.size_of()));

                        Ok(inner_addr_typ
                            .offset(offset)
                            .context(format!("[offset] {}", element.show()))?)
                    }
                    "addr_offset" => {
                        self.new_source_map(element.show_compact());
                        let (expr, offset_element) = unvec!(block.elements, 2);
                        let typ = self
                            .element(expr)?
                            .unify(IrType::addr_unknown())
                            .context(format!("{}", element.show()))?;
                        let inner_addr_typ = typ.as_addr().unwrap();

                        let offset = offset_element.into_term()?.into_int()? as usize;
                        self.writer.push(QVMInstruction::I32Const(
                            inner_addr_typ.clone().offset_in_words(offset)? as i32,
                        ));
                        self.writer.push(QVMInstruction::PAdd);

                        let result_typ = inner_addr_typ.offset(offset)?;
                        self.writer.push(QVMInstruction::Load(result_typ.size_of()));

                        Ok(result_typ)
                    }
                    "index" => {
                        self.new_source_map(element.show_compact());
                        let (element, offset) = unvec!(block.elements, 2);

                        let typ = self.element_addr(element.clone())?;
                        self.element(offset)?
                            .unify(IrType::int())
                            .context(format!("{}", element.show()))?;
                        self.writer.push(QVMInstruction::I32Const(1));
                        self.writer.push(QVMInstruction::Add);
                        self.writer.push(QVMInstruction::PAdd);

                        let elem_typ = typ.as_addr().unwrap().as_element_sized().unwrap();

                        self.writer.push(QVMInstruction::Load(elem_typ.size_of()));

                        Ok(elem_typ)
                    }
                    "addr_index" => {
                        self.new_source_map(element.show_compact());
                        let (element, offset) = unvec!(block.elements, 2);

                        let typ = self.element(element.clone())?;

                        self.element(offset.clone())?
                            .unify(IrType::int())
                            .context(format!("[addr_index] {}", offset.show()))?;
                        self.writer.push(QVMInstruction::I32Const(1));
                        self.writer.push(QVMInstruction::Add);
                        self.writer.push(QVMInstruction::PAdd);

                        let elem_typ = typ.as_addr().unwrap().as_element_sized().clone().unwrap();

                        self.writer.push(QVMInstruction::Load(elem_typ.size_of()));

                        Ok(elem_typ)
                    }
                    "size_of" => {
                        self.new_source_map(element.show_compact());
                        let typ = unvec!(block.elements, 1);
                        let typ = IrType::from_element(&typ)?;
                        self.writer
                            .push(QVMInstruction::I32Const(typ.size_of() as i32));

                        Ok(IrType::int())
                    }
                    "alloc" => {
                        self.new_source_map(element.show_compact());
                        let (typ, len) = unvec!(block.elements, 2);
                        let typ = IrType::from_element(&typ)?;

                        self.writer
                            .push(QVMInstruction::I32Const(typ.size_of() as i32));
                        self.element(IrElement::i_call(
                            "_mult",
                            vec![IrElement::int(typ.size_of() as i32), len],
                        ))?;
                        self.writer.push(QVMInstruction::Alloc);
                        self.local_pointer += 1;

                        Ok(IrType::boxed_array(typ))
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
    function_types: HashMap<String, IrType>,
}

impl VmGenerator {
    pub fn new() -> VmGenerator {
        VmGenerator {
            globals: HashMap::new(),
            global_pointer: 0,
            entrypoint: "main".to_string(),
            function_types: HashMap::new(),
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
        ret: IrType,
    ) -> Result<(Vec<QVMInstruction>, HashMap<usize, String>)> {
        let mut generator = VmFunctionGenerator::new(
            InstructionWriter {
                code: vec![],
                offset,
            },
            args.clone(),
            &self.globals,
            labels,
            &self.function_types,
            string_pointers,
            ret.clone(),
        );

        for statement in body {
            generator
                .element(statement.clone())
                .context(format!("{}", statement.show()))?;
        }

        // if the last statement was not return, insert a new "return nil" statement
        if !matches!(generator.writer.last(), Some(QVMInstruction::Return(_, _))) {
            generator.writer.push(QVMInstruction::NilConst);
            generator
                .writer
                .push(QVMInstruction::Return(Args(args).total_word()?, 1));
        }

        Ok((generator.writer.into_code(), generator.source_map))
    }

    pub fn variable(
        &mut self,
        name: String,
        typ: IrType,
        expr: IrElement,
        offset: usize,
        labels: &mut HashMap<String, usize>,
        string_pointers: &Vec<usize>,
    ) -> Result<Vec<QVMInstruction>> {
        self.push_global(name.clone(), typ.clone());
        let (g, _) = self
            .globals
            .get(&name)
            .ok_or(anyhow::anyhow!(
                "{} is not found in global, {:?}",
                name,
                self.globals
            ))?
            .clone();

        let size = typ.size_of();
        let mut code = vec![QVMInstruction::AddrConst(g, Variable::Global)];
        let mut generator = VmFunctionGenerator::new(
            InstructionWriter {
                code: vec![],
                offset,
            },
            vec![],
            &self.globals,
            labels,
            &self.function_types,
            string_pointers,
            typ.clone(),
        );
        generator.element(expr)?.unify(typ)?;
        code.extend(generator.writer.into_code());
        code.push(QVMInstruction::Store(size));

        Ok(code)
    }

    pub fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
        string_pointers: &Vec<usize>,
        ret: IrType,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(
            InstructionWriter {
                code: vec![],
                offset,
            },
            vec![],
            &self.globals,
            labels,
            &self.function_types,
            &string_pointers,
            ret.clone(),
        );
        generator
            .element(IrElement::block(
                "return",
                vec![IrElement::instruction(
                    "call",
                    vec![IrTerm::Ident(self.entrypoint.to_string())],
                )],
            ))?
            .unify(ret)?;

        Ok(generator.writer.into_code())
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
                    string_pointers.push(code.len());
                    for i in block.elements {
                        code.push(QVMInstruction::I32Const(i.into_term()?.into_int()?));
                    }
                }
                "func" => {
                    let args_block = block.elements[1]
                        .clone()
                        .into_block()
                        .context(format!("{}", IrElement::Block(block.clone()).show()))?;
                    assert_eq!(args_block.name, "args");
                    let return_block = block.elements[2].clone().into_block()?;
                    assert_eq!(return_block.name, "return");

                    self.function_types.insert(
                        block.elements[0].clone().into_term()?.into_ident()?,
                        IrType::func(
                            args_block
                                .elements
                                .into_iter()
                                .map(|b| IrType::from_element(&b))
                                .collect::<Result<_, _>>()?,
                            IrType::from_element(&return_block.elements[0])?,
                        ),
                    );
                    functions.push((
                        block.elements[0].clone().into_term()?.into_ident()?, // name
                        block.elements.into_iter().skip(3).collect::<Vec<_>>(), // body
                    ));
                }
                "var" => {
                    let (name, typ, expr) = unvec!(block.elements, 3);
                    variables.push((
                        name.into_term()?.into_ident()?,
                        IrType::from_element(&typ)?,
                        expr,
                    ));
                }
                _ => unreachable!("{:?}", block),
            }
        }

        for (name, typ, expr) in variables {
            code.extend(self.variable(
                name,
                typ,
                expr,
                code.len(),
                &mut labels,
                &string_pointers,
            )?);
        }

        // call main
        labels.insert(self.entrypoint.to_string(), code.len());

        let (_, ret) = self.function_types["main"].as_func().unwrap();
        code.extend(self.call_main(
            &mut labels,
            code.len(),
            &string_pointers,
            ret.as_ref().clone(),
        )?);

        for (name, body) in functions {
            let (args, ret) = self.function_types.get(&name).unwrap().as_func().unwrap();
            labels.insert(name.clone(), code.len());

            let (code_generated, source_map_generated) = self
                .function(
                    body,
                    args,
                    &mut labels,
                    code.len(),
                    &string_pointers,
                    ret.as_ref().clone(),
                )
                .context(format!("[function] {}", name))?;
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

#[cfg(test)]
mod tests {
    use crate::ir::parse_ir;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_element() -> Result<()> {
        let cases = vec![
            (
                r#"
(seq
    (let $x (tuple (tuple $int (address $int)) 100 nil))
    (assign (offset $x 0) 200)
)
"#,
                "(offset $x 1))",
                "(address $int)",
            ),
            (
                r#"
(seq
    (let $x (slice 10 $int 0))
    (assign (index $x 4) 15)
    (assign (index $x 5) 20)
)
"#,
                "(index $x 7)",
                "$int",
            ),
            (
                r#"
(seq
    (let $x (alloc $int 10))
    (assign (addr_index $x 4) 20)
)
"#,
                "$x",
                "(address (array $int))",
            ),
        ];

        for (input, evaluation, typ) in cases {
            let globals = HashMap::new();
            let mut labels = HashMap::new();
            let functions = HashMap::new();
            let strings = vec![];

            let mut generator = VmFunctionGenerator::new(
                InstructionWriter {
                    code: vec![],
                    offset: 0,
                },
                vec![],
                &globals,
                &mut labels,
                &functions,
                &strings,
                IrType::unknown(),
            );

            let ir = parse_ir(input)?;
            generator.element(ir)?.unify(IrType::nil())?;
            let eval = parse_ir(evaluation)?;
            let typ = IrType::from_element(&parse_ir(typ)?)?;
            assert_eq!(typ, generator.element(eval)?);
        }

        Ok(())
    }
}
