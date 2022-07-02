use crate::vm::QVMInstruction;

pub struct VmOptimizer {}

impl VmOptimizer {
    pub fn new() -> VmOptimizer {
        VmOptimizer {}
    }

    pub fn optimize(&mut self, input: Vec<QVMInstruction>) -> (Vec<QVMInstruction>, Vec<usize>) {
        let mut code_map = vec![0; input.len()];
        let mut result = vec![];
        let mut position = 0;
        while position < input.len() {
            let new_position = result.len();

            if position < input.len() - 1
                && matches!(input[position], QVMInstruction::I32Const(_))
                && matches!(input[position + 1], QVMInstruction::PAdd)
            {
                code_map[position] = new_position;
                code_map[position + 1] = new_position;
                result.push(QVMInstruction::PAddIm(match input[position] {
                    QVMInstruction::I32Const(n) => n as usize,
                    _ => unreachable!(),
                }));
                position += 2;
                continue;
            }

            code_map[position] = new_position;
            result.push(input[position].clone());
            position += 1;
        }

        // jump address recalculation
        for i in 0..result.len() {
            match result[i] {
                QVMInstruction::Jump(n) => {
                    result[i] = QVMInstruction::Jump(code_map[n]);
                }
                QVMInstruction::JumpIf(n) => {
                    result[i] = QVMInstruction::JumpIf(code_map[n]);
                }
                QVMInstruction::JumpIfFalse(n) => {
                    result[i] = QVMInstruction::JumpIfFalse(code_map[n]);
                }
                QVMInstruction::Call => match &result[i - 1] {
                    QVMInstruction::AddrConst(n, s) => {
                        result[i - 1] = QVMInstruction::AddrConst(code_map[*n], s.clone());
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        (result, code_map)
    }
}
