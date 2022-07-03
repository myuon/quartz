use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Variable {
    Local,
    Heap,
    Global,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum QVMInstruction {
    Nop,
    // stack manipulation
    Load(Variable),
    Store(Variable),
    Pop(usize),
    // heap manipulation
    Alloc,
    Free(usize),
    // function arguments
    LoadArg(usize),
    // control
    Jump(usize),
    JumpIf(usize),
    JumpIfFalse(usize),
    // functions
    Call,
    Return(usize, usize), // number of caller arguments, size of return value
    // arithmetic and logic
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    And,
    Or,
    Not,
    // pointer arithmetics
    PAdd,
    PAddIm(usize),
    Deref,
    // Runtime instructions for FFI
    RuntimeInstr(String),
    // constants
    I32Const(i32),
    BoolConst(bool), // I32Const can be used instead
    AddrConst(usize, Variable),
    //
    // Only used during generation phase
    LabelAddrConst(String),
    LabelJumpIfFalse(String),
    LabelJumpIf(String),
    LabelJump(String),
}

impl QVMInstruction {
    pub fn into_string(self) -> String {
        use QVMInstruction::*;

        match self {
            Load(s) => {
                format!("LOAD {:?}", s)
            }
            Store(s) => {
                format!("STORE {:?}", s)
            }
            Pop(n) => {
                format!("POP {}", n)
            }
            Alloc => {
                format!("ALLOC")
            }
            Free(n) => {
                format!("FREE {}", n)
            }
            LoadArg(n) => {
                format!("LOADARG {}", n)
            }
            Jump(n) => {
                format!("JUMP {}", n)
            }
            JumpIf(n) => {
                format!("JUMPIF {}", n)
            }
            JumpIfFalse(n) => {
                format!("JUMPIFNOT {}", n)
            }
            Call => {
                format!("CALL")
            }
            Return(n, s) => {
                format!("RETURN {} {}", n, s)
            }
            Add => {
                format!("ADD")
            }
            Sub => {
                format!("SUB")
            }
            Mult => {
                format!("MULT")
            }
            Div => {
                format!("DIV")
            }
            Mod => {
                format!("MOD")
            }
            Eq => {
                format!("EQ")
            }
            Neq => {
                format!("NEQ")
            }
            Lt => {
                format!("LT")
            }
            Le => {
                format!("LE")
            }
            Gt => {
                format!("GT")
            }
            And => {
                format!("AND")
            }
            Or => {
                format!("OR")
            }
            Not => {
                format!("NOT")
            }
            PAdd => {
                format!("PADD")
            }
            PAddIm(n) => {
                format!("PADDIM {}", n)
            }
            RuntimeInstr(s) => {
                format!("RUNTIMEINSTR {}", s)
            }
            I32Const(n) => {
                format!("I32CONST {}", n)
            }
            BoolConst(b) => {
                format!("BOOLCONST {}", b)
            }
            AddrConst(n, s) => {
                format!("ADDRCONST {} {:?}", n, s)
            }
            Nop => {
                format!("NOP")
            }
            Deref => {
                format!("DEREF")
            }
            LabelAddrConst(_) => unreachable!(),
            LabelJumpIfFalse(_) => unreachable!(),
            LabelJumpIf(_) => unreachable!(),
            LabelJump(_) => unreachable!(),
        }
    }
}

pub struct QVMSource {
    pub instructions: Vec<QVMInstruction>,
}

impl QVMSource {
    pub fn new(instructions: Vec<QVMInstruction>) -> Self {
        QVMSource { instructions }
    }

    pub fn into_string(self) -> String {
        self.instructions
            .into_iter()
            .map(|i| format!("{};", i.into_string()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
