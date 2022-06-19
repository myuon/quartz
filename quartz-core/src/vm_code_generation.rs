use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ir::{IrElement, IrTerm},
    vm::QVMInstruction,
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
}

#[derive(Debug)]
struct VmFunctionGenerator<'s> {
    code: Vec<QVMInstruction>,
    arg_len: usize,
    local_pointer: usize,
    locals: HashMap<String, usize>,
    globals: &'s HashMap<String, usize>,
    labels: &'s mut HashMap<String, usize>,
    offset: usize,
}

impl<'s> VmFunctionGenerator<'s> {
    pub fn new(
        arg_len: usize,
        globals: &'s HashMap<String, usize>,
        labels: &'s mut HashMap<String, usize>,
        offset: usize,
    ) -> VmFunctionGenerator<'s> {
        VmFunctionGenerator {
            code: Vec::new(),
            arg_len,
            local_pointer: 0,
            locals: HashMap::new(),
            globals,
            labels,
            offset,
        }
    }

    fn push_local(&mut self, name: String) {
        self.locals.insert(name, self.local_pointer);
        self.local_pointer += 1;
    }

    fn register_label(&mut self, name: String) {
        self.labels.insert(name, self.offset + self.code.len());
    }

    fn element(&mut self, element: IrElement) -> Result<()> {
        match element {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v) => {
                    if let Some(u) = self.locals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, format!("local:{}", v)));
                        self.code.push(QVMInstruction::Load("local"));
                    } else if let Some(u) = self.globals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, format!("global:{}", v)));
                        self.code.push(QVMInstruction::Load("global"));
                    } else {
                        self.code.push(match v.as_str() {
                            "_add" => QVMInstruction::Add,
                            "_sub" => QVMInstruction::Sub,
                            "_mult" => QVMInstruction::Mult,
                            "_eq" => QVMInstruction::Eq,
                            "_neq" => QVMInstruction::NotEq,
                            "_new" => QVMInstruction::Alloc,
                            "_padd" => QVMInstruction::PAdd,
                            "_lt" => QVMInstruction::Lt,
                            "_gt" => QVMInstruction::Gt,
                            "_div" => QVMInstruction::Div,
                            "_mod" => QVMInstruction::Mod,
                            "_gc" => QVMInstruction::RuntimeInstr("_gc".to_string()),
                            "_panic" => QVMInstruction::RuntimeInstr("_panic".to_string()),
                            "_len" => QVMInstruction::RuntimeInstr("_len".to_string()),
                            "_deref" => QVMInstruction::Load("heap"),
                            "_println" => QVMInstruction::RuntimeInstr("_println".to_string()),
                            "_stringify" => QVMInstruction::RuntimeInstr("_stringify".to_string()),
                            "_copy" => QVMInstruction::RuntimeInstr("_copy".to_string()),
                            _ => QVMInstruction::LabelAddrConst(v.clone()),
                        });
                    }
                }
                IrTerm::Nil => {
                    self.code
                        .push(QVMInstruction::AddrConst(0, "nil".to_string()));
                }
                IrTerm::Bool(b) => {
                    self.code
                        .push(QVMInstruction::I32Const(if b { 1 } else { 0 }));
                }
                IrTerm::Int(n) => {
                    self.code.push(QVMInstruction::I32Const(n));
                }
                IrTerm::Argument(u) => {
                    self.code.push(QVMInstruction::LoadArg(u));
                }
                IrTerm::Keyword(_) => todo!(),
            },
            IrElement::Block(mut block) => {
                match block.name.as_str() {
                    "let" => {
                        let (name, expr) = unvec!(block.elements, 2);
                        let var_name = name.into_term()?.into_ident()?;
                        self.element(expr)?;
                        self.push_local(var_name);
                    }
                    "return" => {
                        let expr = unvec!(block.elements, 1);
                        self.element(expr)?;
                        self.code.push(QVMInstruction::Return(self.arg_len));
                    }
                    "call" => {
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
                        if matches!(self.code.last().unwrap(), QVMInstruction::LabelAddrConst(_)) {
                            self.code.push(QVMInstruction::Call);
                        }
                    }
                    "assign" => {
                        let (left, right) = unvec!(block.elements, 2);
                        self.element(right)?;

                        match left {
                            IrElement::Term(IrTerm::Ident(v)) => {
                                if let Some(u) = self.locals.get(&v).cloned() {
                                    self.code.push(QVMInstruction::AddrConst(u, v.clone()));
                                    self.code.push(QVMInstruction::Store("local"));
                                } else if let Some(u) = self.globals.get(&v).cloned() {
                                    self.code.push(QVMInstruction::AddrConst(u, v.clone()));
                                    self.code.push(QVMInstruction::Store("global"));
                                } else {
                                    anyhow::bail!("assign: {} not found", v);
                                }
                            }
                            _ => {
                                self.element(left)?;
                                self.code.push(QVMInstruction::Store("heap"));
                            }
                        }
                    }
                    "if" => {
                        let (cond, left, right) = unvec!(block.elements, 3);

                        let label = format!("if-{}", self.globals.len());
                        let label_then = format!("then-{}", self.globals.len());
                        let label_else = format!("else-{}", self.globals.len());
                        let label_end = format!("end-{}", self.globals.len());

                        self.register_label(label.clone());
                        self.element(cond)?;
                        self.code
                            .push(QVMInstruction::LabelJumpIfFalse(label_else.clone()));
                        self.register_label(label_then.clone());
                        self.element(left)?;
                        self.code
                            .push(QVMInstruction::LabelJumpIfFalse(label_end.clone()));
                        self.register_label(label_else.clone());
                        self.element(right)?;
                        self.register_label(label_end.clone());
                    }
                    "seq" => {
                        for elem in block.elements {
                            self.element(elem)?;
                        }
                    }
                    "while" => {
                        let (cond, body) = unvec!(block.elements, 2);

                        let label = format!("while-{}", self.globals.len());

                        self.register_label(label.clone());
                        self.element(body)?;
                        self.element(cond)?;
                        self.code.push(QVMInstruction::LabelJumpIf(label.clone()));
                    }
                    name => todo!("{:?}", name),
                };
            }
        }

        Ok(())
    }
}

pub struct VmGenerator {
    globals: HashMap<String, usize>,
    global_pointer: usize,
}

impl VmGenerator {
    pub fn new() -> VmGenerator {
        VmGenerator {
            globals: HashMap::new(),
            global_pointer: 0,
        }
    }

    fn push_global(&mut self, name: String) {
        self.globals.insert(name, self.global_pointer);
        self.global_pointer += 1;
    }

    pub fn globals(&self) -> usize {
        self.globals.len()
    }

    pub fn function(
        &mut self,
        body: Vec<IrElement>,
        arg_len: usize,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(arg_len, &self.globals, labels, offset);

        let mut skip = 2;
        for statement in body {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            generator.element(statement)?;
        }

        Ok(generator.code)
    }

    pub fn variable(
        &mut self,
        name: String,
        expr: IrElement,
        offset: usize,
        labels: &mut HashMap<String, usize>,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(0, &self.globals, labels, offset);
        generator.element(expr)?;
        let mut code = generator.code;

        self.push_global(name.clone());
        code.push(QVMInstruction::AddrConst(
            self.globals[&name],
            format!("let_global:{}", name),
        ));
        code.push(QVMInstruction::Store("global"));

        Ok(code)
    }

    pub fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(0, &self.globals, labels, offset);
        generator.element(IrElement::block(
            "return",
            vec![IrElement::instruction(
                "call",
                vec![IrTerm::Ident("main".to_string())],
            )],
        ))?;

        Ok(generator.code)
    }

    pub fn generate(&mut self, element: IrElement) -> Result<Vec<QVMInstruction>> {
        let mut code = vec![];
        let mut labels = HashMap::new();

        let mut functions = vec![];
        let mut variables = vec![];

        // = first path

        // collect functions
        let block = element.into_block()?;
        assert_eq!(block.name, "module");

        for element in block.elements {
            let mut block = element.into_block()?;
            match block.name.as_str() {
                "func" => {
                    functions.push((
                        block.elements[0].clone().into_term()?.into_ident()?, // name
                        block.elements[1].clone().into_term()?.into_int()?,   // arg length
                        block.elements,
                    ));
                }
                "var" => {
                    let (name, expr) = unvec!(block.elements, 2);
                    variables.push((name.into_term()?.into_ident()?, expr));
                }
                _ => unreachable!("{:?}", block),
            }
        }

        for (name, expr) in variables {
            code.extend(self.variable(name, expr, code.len(), &mut labels)?);
        }

        // call main
        labels.insert("main".to_string(), code.len());
        code.extend(self.call_main(&mut labels, code.len())?);

        for (name, arg_len, function) in functions {
            labels.insert(name, code.len());
            code.extend(self.function(function, arg_len as usize, &mut labels, code.len())?);
        }

        // = second path

        // resolve labels
        for i in 0..code.len() {
            if let QVMInstruction::LabelAddrConst(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::AddrConst(*pc, format!("{}:", label));
                } else {
                    println!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJumpIfFalse(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::JumpIfFalse(*pc);
                } else {
                    println!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJumpIf(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::JumpIf(*pc);
                } else {
                    println!("{:?}", code);
                    anyhow::bail!("label {} not found", label);
                }
            }
        }

        Ok(code)
    }
}
