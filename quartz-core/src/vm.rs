#[derive(Clone, Debug, PartialEq)]
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
    NotEq,
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
