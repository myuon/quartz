use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum QVMInstruction {
    // stack manipulation
    Load(&'static str),
    Store(&'static str),
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
    Return(usize), // number of caller arguments
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
    // Runtime instructions for FFI
    RuntimeInstr(String),
    // constants
    I32Const(i32),
    BoolConst(bool), // I32Const can be used instead
    AddrConst(usize, String),
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
                format!("load {}", s)
            }
            Store(s) => {
                format!("store {}", s)
            }
            Pop(n) => {
                format!("pop {}", n)
            }
            Alloc => {
                format!("alloc")
            }
            Free(n) => {
                format!("free {}", n)
            }
            LoadArg(n) => {
                format!("loadarg {}", n)
            }
            Jump(n) => {
                format!("jump {}", n)
            }
            JumpIf(n) => {
                format!("jumpif {}", n)
            }
            JumpIfFalse(n) => {
                format!("jumpiffalse {}", n)
            }
            Call => {
                format!("call")
            }
            Return(n) => {
                format!("return {}", n)
            }
            Add => {
                format!("add")
            }
            Sub => {
                format!("sub")
            }
            Mult => {
                format!("mult")
            }
            Div => {
                format!("div")
            }
            Mod => {
                format!("mod")
            }
            Eq => {
                format!("eq")
            }
            Neq => {
                format!("neq")
            }
            Lt => {
                format!("lt")
            }
            Le => {
                format!("le")
            }
            Gt => {
                format!("gt")
            }
            And => {
                format!("and")
            }
            Or => {
                format!("or")
            }
            Not => {
                format!("not")
            }
            PAdd => {
                format!("padd")
            }
            RuntimeInstr(s) => {
                format!("runtime {}", s)
            }
            I32Const(n) => {
                format!("i32const {}", n)
            }
            BoolConst(b) => {
                format!("boolconst {}", b)
            }
            AddrConst(n, s) => {
                format!("addrconst {} {}", n, s)
            }
            LabelAddrConst(s) => {
                format!("labeladdrconst {}", s)
            }
            LabelJumpIfFalse(s) => {
                format!("labeljumpiffalse {}", s)
            }
            LabelJumpIf(s) => {
                format!("labeljumpif {}", s)
            }
            LabelJump(s) => {
                format!("labeljump {}", s)
            }
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
