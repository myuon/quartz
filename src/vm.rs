use crate::ast::Statement;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum DataType {
    Nil,
    Int(i32),
    String(String),
    Closure(
        usize, // UID for closure
        Vec<String>,
        Vec<Statement>,
    ),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpCode {
    Push(StackData),
    Pop(usize),
    Return(usize),
    Copy(usize),
    Alloc(HeapData),
    Call(usize),
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
    HeapAddr(usize),  // in normal order
    StackAddr(usize), // in reverse order, 0-origin, excluding itself, for addresses of local variables
    StaticAddr(usize),
}

impl StackData {
    pub fn as_heap_addr(&self) -> Option<usize> {
        match self {
            &Self::HeapAddr(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_static_addr(&self) -> Option<usize> {
        match self {
            &StackData::StaticAddr(u) => Some(u),
            _ => None,
        }
    }
}
