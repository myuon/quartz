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
}

#[derive(Debug)]
struct VmFunctionGenerator {
    code: Vec<QVMInstruction>,
    arg_len: usize,
    local_pointer: usize,
    locals: HashMap<String, usize>,
}

impl VmFunctionGenerator {
    pub fn new(
        arg_len: usize,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> VmFunctionGenerator {
        VmFunctionGenerator {
            code: Vec::new(),
            arg_len,
            local_pointer: 0,
            locals: HashMap::new(),
        }
    }

    fn push_local(&mut self, name: String) {
        self.locals.insert(name, self.local_pointer);
        self.local_pointer += 1;
    }

    fn element(&mut self, element: IrElement) -> Result<()> {
        match element {
            IrElement::Term(term) => match term {
                IrTerm::Ident(v) => {
                    if let Some(u) = self.locals.get(&v) {
                        self.code
                            .push(QVMInstruction::AddrConst(*u, format!("local:{}", v)));
                        self.code.push(QVMInstruction::Load("local"));
                    } else {
                        self.code.push(match v.as_str() {
                            "_add" => QVMInstruction::Add,
                            "_sub" => QVMInstruction::Sub,
                            "_mult" => QVMInstruction::Mult,
                            "_eq" => QVMInstruction::Eq,
                            "_new" => QVMInstruction::Alloc,
                            "_padd" => QVMInstruction::PAdd,
                            "_gc" => QVMInstruction::RuntimeInstr("_gc".to_string()),
                            "_len" => QVMInstruction::RuntimeInstr("_len".to_string()),
                            _ => QVMInstruction::LabelAddrConst(v.clone()),
                        });
                    }
                }
                IrTerm::Nil => {
                    self.code
                        .push(QVMInstruction::AddrConst(0, "nil".to_string()));
                }
                IrTerm::Bool(_) => todo!(),
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
                        self.code.push(QVMInstruction::Call);
                    }
                    "assign" => {
                        let (left, right) = unvec!(block.elements, 2);
                        self.element(right)?;

                        match left {
                            IrElement::Term(IrTerm::Ident(v)) => {
                                if let Some(u) = self.locals.get(&v).cloned() {
                                    self.code.push(QVMInstruction::AddrConst(u, v.clone()));
                                    self.code.push(QVMInstruction::Store("local"));
                                } else {
                                    anyhow::bail!("assign: {} not found", v);
                                }
                            }
                            _ => {
                                self.element(left)?;
                            }
                        }
                    }
                    name => todo!("{:?}", name),
                };
            }
        }

        Ok(())
    }
}

pub struct VmGenerator {}

impl VmGenerator {
    pub fn new() -> VmGenerator {
        VmGenerator {}
    }

    pub fn function(
        &mut self,
        body: Vec<IrElement>,
        arg_len: usize,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(arg_len, labels, offset);

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

    pub fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(0, labels, offset);
        generator.element(IrElement::instruction(
            "call",
            vec![IrTerm::Ident("main".to_string())],
        ))?;

        Ok(generator.code)
    }

    pub fn generate(&mut self, element: IrElement) -> Result<Vec<QVMInstruction>> {
        let mut code = vec![];
        let mut labels = HashMap::new();
        let mut functions = vec![];

        // = first path

        // collect functions
        let block = element.into_block()?;
        assert_eq!(block.name, "module");

        for element in block.elements {
            let block = element.into_block()?;
            if block.name == "func" {
                assert_eq!(block.name, "func");

                functions.push((
                    block.elements[0].clone().into_term()?.into_ident()?, // name
                    block.elements[1].clone().into_term()?.into_int()?,   // arg length
                    block.elements,
                ));
            }
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
            }
        }

        Ok(code)
    }
}
