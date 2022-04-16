#[derive(Clone, Debug)]
pub enum QVMInstruction {
    // locals
    Load(usize),
    Store(usize),
    Pop(usize),
    // function arguments
    LoadArg(usize),
    // global variables
    GlobalGet(usize),
    GlobalSet(usize),
    // control
    Jump(usize),
    JumpIfFalse(usize),
    // functions
    Call(usize),
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
    //
    Label(String),
    //
    // Only used during generation phase
    LabelCall(String),
    LabelJumpIfFalse(String),
}
