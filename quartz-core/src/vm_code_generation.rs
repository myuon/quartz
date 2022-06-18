use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ir::{IrElement, IrTerm},
    vm::QVMInstruction,
};

macro_rules! unvec {
    ($e:expr, 2) => {{
        let v2 = $e.pop().unwrap();
        let v1 = $e.pop().unwrap();

        (v1, v2)
    }};
}

#[derive(Debug)]
struct VmFunctionGenerator {
    code: Vec<QVMInstruction>,
    local_pointer: usize,
    locals: HashMap<String, usize>,
    args: HashMap<String, usize>,
}

impl VmFunctionGenerator {
    pub fn new(labels: &mut HashMap<String, usize>, offset: usize) -> VmFunctionGenerator {
        VmFunctionGenerator {
            code: Vec::new(),
            local_pointer: 0,
            locals: HashMap::new(),
            args: HashMap::new(),
        }
    }

    fn expr(&mut self, expr: IrElement) -> Result<()> {
        todo!()
    }

    fn statement(&mut self, element: IrElement) -> Result<()> {
        let mut block = element.into_block()?;

        match block.name.as_str() {
            name => todo!("{:?}", name),
        }
    }

    pub fn statements(&mut self, statements: Vec<IrElement>) -> Result<()> {
        for statement in statements {
            self.statement(statement)?;
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
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(labels, offset);
        let mut name = true;
        for statement in body {
            if name {
                name = false;
                continue;
            }

            generator.statement(statement)?;
        }

        Ok(generator.code)
    }

    pub fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator = VmFunctionGenerator::new(labels, offset);
        generator.statement(IrElement::instruction(
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
                    block.elements[0].clone().into_term()?.into_ident()?,
                    block.elements,
                ));
            }
        }

        // call main
        labels.insert("main".to_string(), code.len());
        code.extend(self.call_main(&mut labels, code.len())?);

        for (name, function) in functions {
            labels.insert(name, code.len());
            code.extend(self.function(function, &mut labels, code.len())?);
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
