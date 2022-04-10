#[derive(Clone, Debug)]
pub enum QVMInstruction {
    // locals
    Load(usize),  // relative position from stack pointer
    Store(usize), // relative position from stack pointer
    Pop(usize),
    // function arguments
    LoadArg(usize),
    // global variables
    GlobalGet(usize), // normal order
    GlobalSet(usize),
    // control
    Jump(usize),
    // functions
    Call(usize),
    Return,
    // arithmetic and logic
    Add,
    Sub,
    Mul,
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
}
