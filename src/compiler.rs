use std::collections::HashMap;

use anyhow::Result;

use crate::{
    code_gen::gen_code,
    parser::run_parser,
    runtime::{FFIFunction, Runtime},
    stdlib::{create_ffi_table, typecheck_with_stdlib},
    vm::{HeapData, OpCode},
};

pub struct Compiler {
    stdlib: (HashMap<String, usize>, Vec<FFIFunction>),
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            stdlib: create_ffi_table(),
        }
    }

    pub fn compile(&self, input: &str) -> Result<(Vec<OpCode>, Vec<HeapData>)> {
        let mut m = run_parser(input)?;
        typecheck_with_stdlib(&mut m)?;

        let (mut program, static_area) = gen_code(m, self.stdlib.0.clone())?;

        // main(というか最後に宣言された関数)を呼ぶ
        program.push(OpCode::CopyStatic(static_area.len() - 1));
        program.push(OpCode::Call(0));

        Ok((program, static_area))
    }

    pub fn exec(&self, input: &str) -> Result<()> {
        let (program, static_area) = self.compile(input)?;
        let mut runtime = Runtime::new(program, static_area, self.stdlib.1.clone());
        runtime.execute()?;

        Ok(())
    }
}
