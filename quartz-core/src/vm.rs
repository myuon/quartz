pub enum QVMInstruction {
    // local, global variables
    Get(usize), // reverse order
    Set(usize),
    GlobalGet(usize), // normal order
    GlobalSet(usize),
    // control
    Jump(usize),
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
}
