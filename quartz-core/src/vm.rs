#[derive(Clone, Debug)]
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
    And,
    Or,
    Not,
    // constants
    I32Const(i32),
    AddrConst(usize),
    //
    // Only used during generation phase
    LabelAddrConst(String),
    LabelJumpIfFalse(String),
}
