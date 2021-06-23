use crate::ast::Statement;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum DataType {
    Nil,
    Int(i32),
    String(String),
    Closure(Vec<String>, Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpCode {
    Push(StackData),
    Pop(usize),
    Return(usize),
    Copy(usize),
    CopyAbsolute(usize), // copy from absolute index
    Alloc(HeapData),
    Call(usize),
    CallAbsolute(usize), // call from absolute index
    FFICall(usize),
    PAssign,
    Free,
    Deref,
    Tuple(usize),
    Object(usize),
    Get,
    Set,
    Regex,
    Switch(usize),
    VPush,
    Len,
    Loop,
    Label(String),
    Jump(String),
    ReturnIf(usize),
    Slice,
}

#[derive(PartialEq, Debug, Clone)]
pub enum HeapData {
    Nil,
    Int(i32),
    String(String),
    Closure(Vec<OpCode>),
    Vec(Vec<StackData>),
    Tuple(usize, Vec<StackData>),
    Object(Vec<(String, StackData)>),
    Pointer(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub enum StackData {
    Nil,
    HeapAddr(usize),        // in normal order
    StackRevAddr(usize), // in reverse order, 0-origin, excluding itself, for addresses of local variables
    StackNormalAddr(usize), // in normal order, for addresses of out-of-scope variables
}
