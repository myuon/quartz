use crate::ast::Statement;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum DataType {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Closure(
        usize, // UID for closure
        Vec<String>,
        Vec<Statement>,
    ),
    Tuple(Vec<StackData>),
    Object(Vec<(String, StackData)>),
    Vec(Vec<StackData>),
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
    Label(String),
    Jump(String),
    ReturnIf(usize),
    Slice,
    JumpIfNot(String),
    Panic,
    SetStatic(usize),
    CopyStatic(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub enum HeapData {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Closure(Vec<OpCode>),
    Vec(Vec<StackData>),
    Tuple(usize, Vec<StackData>),
    Object(Vec<(String, StackData)>),
    Pointer(usize),
}

impl HeapData {
    pub fn as_stack_data(self) -> Option<StackData> {
        match self {
            HeapData::Nil => Some(StackData::Nil),
            HeapData::Bool(b) => Some(StackData::Bool(b)),
            HeapData::Int(i) => Some(StackData::Int(i)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum StackData {
    Nil,
    Bool(bool),
    Int(i32),
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

    pub fn into_heap_data(self) -> Option<HeapData> {
        match self {
            StackData::Nil => Some(HeapData::Nil),
            StackData::Int(i) => Some(HeapData::Int(i)),
            StackData::Bool(b) => Some(HeapData::Bool(b)),
            _ => None,
        }
    }
}
