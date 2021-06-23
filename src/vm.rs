use crate::ast::Statement;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum UnsizedDataType {
    Nil,
    String(String),
    Closure(Vec<String>, Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpCode {
    Push(DataType),
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
    Vec(Vec<DataType>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Nil,
    Int(i32),
    HeapAddr(usize),        // in normal order
    StackRevAddr(usize), // in reverse order, 0-origin, excluding itself, for addresses of local variables
    StackNormalAddr(usize), // in normal order, for addresses of out-of-scope variables
    Tuple(usize, Vec<DataType>),
    Object(Vec<(String, DataType)>),
}

impl DataType {
    pub fn type_of(&self) -> String {
        use DataType::*;

        match self {
            Nil => "nil".to_string(),
            Int(_) => "int".to_string(),
            HeapAddr(_) => "heap_addr".to_string(),
            StackRevAddr(_) => "stack_addr(local)".to_string(),
            StackNormalAddr(_) => "stack_addr".to_string(),
            Tuple(u, _) => format!("tuple({})", u),
            Object(_) => "object".to_string(),
        }
    }
}
