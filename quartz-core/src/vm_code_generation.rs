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

    pub fn pop(&mut self) -> Option<QVMInstruction> {
        self.code.pop()
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
            "_new" => (QVMInstruction::Alloc, todo!()),
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
                IrType::func(vec![IrType::addr_of(IrType::unknown())], IrType::int()),
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
            "_int_to_byte" => (QVMInstruction::Nop, todo!()),
            "_byte_to_int" => (QVMInstruction::Nop, todo!()),
            "_nil_to_ref" => (QVMInstruction::Nop, todo!()),
            "_start_debugger" => (
                QVMInstruction::RuntimeInstr("_start_debugger".to_string()),
                IrType::nil(),
            ),
            "_check_sp" => (
                QVMInstruction::RuntimeInstr("_check_sp".to_string()),
                IrType::nil(),
            ),
            _ => (
                QVMInstruction::LabelI32Const(v.to_string()),
                self.functions[v].clone(),
            ),
        }
    }

    // compile to an address (lvar)
    fn element_addr(&mut self, element: IrElement, expected_type: IrType) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v, _) => {
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
                "call" => Ok(IrType::addr_of(self.element(element, IrType::unknown())?)),
                // FIXME: is this true?
                "deref" => {
                    self.new_source_map(element.show_compact());

                    let (_, element) = unvec!(block.elements, 2);
                    let typ = self.element(element, IrType::unknown())?;
                    Ok(typ.as_addr().unwrap().as_ref().clone())
                }
                "offset" => {
                    self.new_source_map(element.show_compact());

                    let (_, elem, offset_element) = unvec!(block.elements, 3);
                    let offset = offset_element.into_term()?.into_int()? as usize;
                    let typ = self.element_addr(elem, IrType::unknown())?;
                    self.writer.push(QVMInstruction::I32Const(offset as i32));
                    self.writer.push(QVMInstruction::PAdd);
                    Ok(IrType::addr_of(
                        (typ.as_addr().context(format!("{}", element.show()))?)
                            .offset(offset - 1)
                            .context(format!("{}", element.show()))?,
                    ))
                }
                "addr_offset" => {
                    self.new_source_map(element.show_compact());

                    let (_, elem, offset_element) = unvec!(block.elements, 3);
                    let offset = offset_element.into_term()?.into_int()? as usize;
                    let typ = self.element(elem, IrType::unknown())?;
                    self.writer.push(QVMInstruction::I32Const(offset as i32));
                    self.writer.push(QVMInstruction::PAdd);
                    Ok(IrType::addr_of(
                        (typ.as_addr()?)
                            .offset(offset - 1)
                            .context(format!("{}", element.show()))?,
                    ))
                }
                "index" => {
                    self.new_source_map(element.show_compact());

                    let (_, element, offset) = unvec!(block.elements, 3);
                    let typ = self.element_addr(element, IrType::unknown())?;
                    self.element(offset, IrType::int())?;
                    self.writer.push(QVMInstruction::I32Const(1)); // +1 for a pointer to info table
                    self.writer.push(QVMInstruction::Add);
                    self.writer.push(QVMInstruction::PAdd);
                    Ok(IrType::addr_of(typ))
                }
                _ => unreachable!("{}", element.show()),
            },
        }
    }

    fn element(&mut self, element: IrElement, expected_type: IrType) -> Result<IrType> {
        match element.clone() {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v, _) => {
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
                        let (typ, name, expr) = unvec!(block.elements, 3);
                        let var_name = name.into_term()?.into_ident()?;
                        let typ = IrType::from_element(&typ)?;

                        let typ = self.element(expr, typ)?;
                        self.push_local(var_name, typ);
                        Ok(IrType::nil())
                    }
                    "return" => {
                        self.new_source_map(element.show_compact());
                        let (size, expr) = unvec!(block.elements, 2);
                        let size = size.into_term()?.into_int()? as usize;
                        self.expected_type = (self.expected_type.clone())
                            .unify(self.element(expr, self.expected_type.clone())?)
                            .context(format!("{}", element.show()))?;
                        self.writer
                            .push(QVMInstruction::Return(self.args.total_word()?, size));

                        Ok(IrType::unknown())
                    }
                    "call" => {
                        self.new_source_map(element.show_compact());
                        let callee = block.elements[0].clone();

                        let mut arg_types = vec![];
                        for elem in block.elements.into_iter().skip(1) {
                            arg_types.push(self.element(elem, IrType::unknown())?);
                        }

                        let callee_typ = self.element_addr(
                            callee,
                            IrType::addr_of(IrType::func(arg_types, IrType::unknown())),
                        )?;
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
                        let (size, lhs, rhs) = unvec!(block.elements, 3);
                        let typ = self.element_addr(lhs, IrType::addr_unknown())?;
                        self.element(
                            rhs,
                            typ.as_addr()
                                .map(|t| t.as_ref().clone())
                                .unwrap_or(IrType::unknown()),
                        )?;

                        self.writer
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
                        self.element(cond, IrType::bool())?;
                        self.writer
                            .push(QVMInstruction::LabelJumpIfFalse(label_else.clone()));

                        // then block
                        self.new_source_map(IrElement::block("if:then", vec![]).show_compact());
                        let typ = self.element(left, IrType::unknown())?;
                        self.writer
                            .push(QVMInstruction::LabelJump(label_end.clone()));

                        // else block
                        self.new_source_map(IrElement::block("if:else", vec![]).show_compact());
                        self.register_label(label_else.clone());
                        self.element(right, typ)?;

                        // endif
                        self.register_label(label_end.clone());
                        self.new_source_map(IrElement::block("if:end", vec![]).show_compact());

                        Ok(IrType::unknown())
                    }
                    "seq" => {
                        let mut ret = IrType::unknown();
                        self.new_source_map(IrElement::block("seq", vec![]).show_compact());
                        for elem in block.elements {
                            ret = self.element(elem, IrType::unknown())?;
                        }

                        Ok(expected_type.unify(ret)?)
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
                        self.element(
                            IrElement::Term(IrTerm::Int(self.local_pointer as i32)),
                            IrType::int(),
                        )?;
                        self.element(
                            IrElement::Term(IrTerm::Ident("_check_sp".to_string(), 1)),
                            IrType::nil(),
                        )?;

                        self.element(body, IrType::nil())?;

                        self.register_label(label_cond.clone());
                        self.new_source_map(IrElement::block("while:cond", vec![]).show_compact());
                        self.element(cond, IrType::bool())?;

                        self.writer.push(QVMInstruction::LabelJumpIf(label.clone()));
                        self.label_continue = None;
                        self.new_source_map(IrElement::block("while:end", vec![]).show_compact());

                        Ok(IrType::unknown())
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
                        let n = unvec!(block.elements, 1);
                        self.writer
                            .push(QVMInstruction::Pop(n.into_term()?.into_int()? as usize));

                        Ok(IrType::nil())
                    }
                    "data" => {
                        self.new_source_map(IrElement::block("data", vec![]).show_compact());

                        let size = block.elements[0].clone().into_term()?.into_int()? as usize;
                        self.writer.push(QVMInstruction::InfoConst(size));

                        let mut types = vec![];
                        for (i, elem) in block.elements.into_iter().skip(1).enumerate() {
                            types.push(
                                self.element(
                                    elem,
                                    expected_type
                                        .clone()
                                        .offset(i)
                                        .context(format!("{}", element.show()))?,
                                )?,
                            );
                        }

                        Ok(IrType::tuple(types))
                    }
                    // FIXME: integrate with _copy and load and _deref?
                    "copy" => {
                        self.new_source_map(element.show_compact());
                        let (size, elem) = unvec!(block.elements, 2);

                        self.element(elem, IrType::unknown())?; // FIXME: use proper type
                        self.writer
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(IrType::unknown())
                    }
                    "unload" => {
                        self.new_source_map(element.show_compact());
                        let element = unvec!(block.elements, 1);
                        self.element(element.clone(), IrType::addr_unknown())?;
                        let code = self.writer.pop();
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
                        self.writer.push(QVMInstruction::AddrConst(
                            self.string_pointers[n],
                            Variable::StackAbsolute,
                        ));

                        Ok(IrType::addr_of(IrType::tuple(vec![IrType::addr_unknown()])))
                    }
                    "deref" => {
                        self.new_source_map(element.show_compact());
                        let (size, element) = unvec!(block.elements, 2);
                        let typ = self.element(element, IrType::addr_unknown())?;
                        self.writer
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

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

                        let typ = self.element(element, IrType::unknown())?; // FIXME: use proper type
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
                        let typ = self.element_addr(element, IrType::addr_unknown())?;

                        Ok(IrType::addr_of(typ))
                    }
                    "offset" => {
                        self.new_source_map(element.show_compact());
                        let (size, expr, offset_element) = unvec!(block.elements, 3);
                        let typ = self.element_addr(expr, IrType::addr_unknown())?;

                        let offset = offset_element.into_term()?.into_int()? as usize;
                        self.writer.push(QVMInstruction::I32Const(offset as i32));
                        self.writer.push(QVMInstruction::PAdd);
                        self.writer
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(expected_type
                            .unify(
                                typ.as_addr()
                                    .unwrap()
                                    .offset(
                                        // FIXME: Currently, offset positioning starts from 1
                                        offset - 1,
                                    )
                                    .context(format!("{}", element.show()))?,
                            )
                            .context(format!("{}", element.show()))?)
                    }
                    "addr_offset" => {
                        self.new_source_map(element.show_compact());
                        let (_, expr, offset_element) = unvec!(block.elements, 3);
                        let typ = self.element(expr, IrType::addr_unknown())?;

                        let offset = offset_element.into_term()?.into_int()? as usize;
                        self.writer.push(QVMInstruction::I32Const(offset as i32));
                        self.writer.push(QVMInstruction::PAdd);

                        let result_typ = expected_type
                            .unify(
                                typ.as_addr()
                                    .unwrap()
                                    .offset(
                                        // FIXME: Currently, offset positioning starts from 1
                                        offset - 1,
                                    )
                                    .context(format!("{}", element.show()))?,
                            )
                            .context(format!("{}", element.show()))?;
                        self.writer.push(QVMInstruction::Load(result_typ.size_of()));

                        Ok(result_typ)
                    }
                    "index" => {
                        self.new_source_map(element.show_compact());
                        let (size, element, offset) = unvec!(block.elements, 3);
                        self.element_addr(element, IrType::addr_unknown())?;
                        self.element(offset, IrType::int())?;
                        self.writer.push(QVMInstruction::I32Const(1));
                        self.writer.push(QVMInstruction::Add);
                        self.writer.push(QVMInstruction::PAdd);
                        self.writer
                            .push(QVMInstruction::Load(size.into_term()?.into_int()? as usize));

                        Ok(expected_type)
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
            generator.element(statement, ret.clone())?;
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
        generator.element(expr, typ)?;
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
        generator.element(
            IrElement::block(
                "return",
                vec![
                    IrElement::Term(IrTerm::Int(1)),
                    IrElement::instruction(
                        "call",
                        vec![IrTerm::Ident(self.entrypoint.to_string(), 1)],
                    ),
                ],
            ),
            ret,
        )?;

        Ok(generator.writer.into_code())
    }

    pub fn generate(
        &mut self,
        element: IrElement,
    ) -> Result<(Vec<QVMInstruction>, HashMap<usize, String>)> {
        println!("{}", element.show());
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
            labels.insert(name, code.len());

            let (code_generated, source_map_generated) = self.function(
                body,
                args,
                &mut labels,
                code.len(),
                &string_pointers,
                ret.as_ref().clone(),
            )?;
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
