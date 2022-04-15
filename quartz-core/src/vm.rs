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
    // Only used during generation phase
    PlaceholderLabel(String),
}
