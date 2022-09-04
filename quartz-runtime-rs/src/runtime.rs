use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use log::{debug, info};
use quartz_core::vm::{QVMInstruction, Variable};
use serde::{Deserialize, Serialize};

use crate::freelist::Freelist;

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum AddrPlace {
    Stack,
    Heap,
    Static,
    InfoTable,
}

impl AddrPlace {
    pub fn from_variable(variable: Variable) -> AddrPlace {
        match variable {
            Variable::Local => AddrPlace::Stack,
            Variable::Heap => AddrPlace::Heap,
            Variable::Global => AddrPlace::Static,
            Variable::StackAbsolute => AddrPlace::Stack,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueIntFlag {
    Int,       // default
    Len,       // length in heap
    Pc(usize), // program counter (with calling label offset)
    Fp,        // frame pointer
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueAddrFlag {
    Addr,   // default
    Nodata, // no data in heap
    Prev,   // prev in heap
    Next,   // next in heap
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32, ValueIntFlag),
    Addr(usize, AddrPlace, ValueAddrFlag),
}

#[allow(dead_code)]
impl Value {
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_int(self) -> Result<i32> {
        match self {
            Value::Int(i, ValueIntFlag::Int) => Some(i),
            _ => None,
        }
        .ok_or_else(|| anyhow::anyhow!("expected int but got {:?}", self))
    }

    pub fn as_addr(self) -> Option<usize> {
        match self {
            Value::Addr(i, _, ValueAddrFlag::Addr) => Some(i),
            _ => None,
        }
    }

    pub fn as_stack_addr(self) -> Option<usize> {
        match self {
            Value::Addr(i, AddrPlace::Stack, ValueAddrFlag::Addr) => Some(i),
            _ => None,
        }
    }

    pub fn as_heap_addr(self) -> Option<usize> {
        match self {
            Value::Addr(i, AddrPlace::Heap, ValueAddrFlag::Addr) => Some(i),
            _ => None,
        }
    }

    pub fn as_named_int(self, flag: ValueIntFlag) -> Option<i32> {
        match self {
            Value::Int(i, n) if n == flag => Some(i),
            _ => None,
        }
    }

    pub fn as_int_pc(self) -> Option<usize> {
        match self {
            Value::Int(i, ValueIntFlag::Pc(_)) => Some(i as usize),
            _ => None,
        }
    }

    pub fn as_named_addr(self, flag: ValueAddrFlag) -> Option<usize> {
        match self {
            Value::Addr(i, _, n) if n == flag => Some(i),
            _ => None,
        }
    }

    pub fn nil() -> Value {
        Value::Nil
    }

    pub fn is_nil(&self) -> bool {
        self == &Value::nil()
    }

    pub fn bool(b: bool) -> Value {
        Value::Bool(b)
    }

    pub fn int(i: i32) -> Value {
        Value::Int(i, ValueIntFlag::Int)
    }

    pub fn addr(i: usize, p: AddrPlace) -> Value {
        Value::Addr(i, p, ValueAddrFlag::Addr)
    }
}

macro_rules! assert_matches {
    ($e:expr, $p:pat $(,$t:expr)* $(,)?) => {
        assert!(matches!($e, $p) $(,$t)*)
    };
}

#[allow(dead_code)]
struct Array {
    length: usize,
    data: Vec<Value>,
}

impl Array {
    pub fn from_values(values: &[Value]) -> Result<Array> {
        assert!(!values.is_empty());
        let length = values[0]
            .clone()
            .as_named_int(ValueIntFlag::Len)
            .ok_or(anyhow::anyhow!("expected length but got {:?}", values[0]))?
            as usize;

        Ok(Array {
            length,
            data: values[1..length].to_vec(),
        })
    }
}

/* StackFrame
    [argument*, return_address, fp, local*]
                                    ^ new fp
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Runtime {
    pub(crate) stack: Vec<Value>,
    pub(crate) heap: Freelist,
    pub(crate) globals: Vec<i32>,
    pub(crate) code: Vec<QVMInstruction>,
    pub(crate) pc: usize,
    pub(crate) stack_pointer: usize,
    pub(crate) frame_pointer: usize,
    debugger_json_path: PathBuf,
    debug_mode: bool,
    labels: HashMap<usize, String>,
}

impl Runtime {
    pub fn new(code: Vec<QVMInstruction>, globals: usize) -> Runtime {
        Runtime {
            stack: vec![],
            heap: Freelist::new(100_000),
            globals: vec![0; globals],
            code,
            pc: 0,
            stack_pointer: 0,
            frame_pointer: 0,
            debugger_json_path: PathBuf::new(),
            debug_mode: false,
            labels: HashMap::new(),
        }
    }

    pub fn set_debug_mode(&mut self, debugger_json: PathBuf) {
        self.debug_mode = true;
        self.debugger_json_path = debugger_json;
    }

    pub fn set_labels(&mut self, labels: HashMap<usize, String>) {
        self.labels = labels;
    }

    pub fn new_from_debugger_json(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents).unwrap())
    }

    fn run_gc(&mut self) -> Result<()> {
        // 1. mark phase
        let mut marked = HashSet::new();

        // handling the root object in heap...?
        let mut root = vec![];
        for g in &self.globals {
            root.push(Value::addr(*g as usize, AddrPlace::Static));
        }
        for s in &self.stack[..self.stack_pointer] {
            root.push(s.clone());
        }

        while let Some(r) = root.pop() {
            match r {
                Value::Addr(i, AddrPlace::Heap, _) => {
                    if !marked.contains(&i) {
                        marked.insert(i);

                        // if the next addr is a new object, mark every elements in it
                        // QUESTION: checking the previous addr being an address to InfoTable is a correct way?
                        if let Ok(object) = self.heap.parse_from_data_pointer(i) {
                            for p in object.get_data_pointer()..object.get_end_pointer() {
                                debug!("adding {:?}", p);
                                root.push(self.heap.data[p].clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // 2. sweep phase
        let mut current = self.heap.root()?;
        while let Ok(next) = self.heap.find_next(&current) {
            if !next.is_collectable() {
                break;
            }

            let addr = next.get_data_pointer();
            if !marked.contains(&addr) {
                debug!("freeing {:?}", next);
                self.heap.free(next.clone())?;
            }

            current = next;
        }

        Ok(())
    }

    fn pop(&mut self) -> Value {
        assert!(
            self.stack_pointer > 0,
            "{} at {:?}",
            self.stack_pointer,
            self.stack
        );
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer].clone()
    }

    fn pop_many(&mut self, size: usize) -> Vec<Value> {
        let mut values = vec![];
        for _ in 0..size {
            values.push(self.pop());
        }
        values.reverse();

        values
    }

    fn push(&mut self, value: Value) {
        self.stack_pointer += 1;
        if self.stack.len() < self.stack_pointer {
            self.stack.push(value);
        } else {
            self.stack[self.stack_pointer - 1] = value;
        }
    }

    fn load(&mut self, offset: usize) -> Value {
        self.stack[self.stack_pointer - offset].clone()
    }

    pub(crate) fn debug_info(&self) -> String {
        format!(
            "sp:{}\n{:?}\n{}\n{}\n{} {:?}\n",
            self.stack_pointer,
            self.globals,
            &self
                .heap
                .debug_objects()
                .iter()
                .rev()
                .take(5)
                .rev()
                .map(|c| format!("{:?}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.debug_stacktrace(),
            self.pc,
            &self.code[self.pc]
        )
    }

    pub(crate) fn debug_stacktrace(&self) -> String {
        let mut stack_frames = vec![];
        let mut current_frame = vec![];
        let mut p = 0;
        let mut pc_prev = 0;
        for s in &self.stack[0..self.stack_pointer] {
            match s {
                Value::Int(_, ValueIntFlag::Pc(t)) => {
                    stack_frames.push((p - current_frame.len(), pc_prev, current_frame));
                    pc_prev = *t;
                    current_frame = vec![];
                }
                _ => {}
            }

            current_frame.push(s.clone());
            p += 1;
        }
        stack_frames.push((p - current_frame.len(), pc_prev, current_frame));

        format!(
            "{}",
            stack_frames
                .into_iter()
                .skip(1) // skipping data segment
                .map(|(p, t, ds)| format!(
                    "{}({}) {:?}",
                    p,
                    self.labels.get(&t).unwrap_or(&format!("{}", t)),
                    ds
                ))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    #[allow(dead_code)]
    fn read(&self, value: Value) -> &Value {
        self.read_with_offset(value, 0)
    }

    fn read_with_offset(&self, value: Value, offset: i32) -> &Value {
        match value {
            Value::Addr(addr, AddrPlace::Heap, _) => {
                &self.heap.data[(addr as i32 + offset) as usize]
            }
            Value::Addr(addr, AddrPlace::Stack, _) => &self.stack[(addr as i32 + offset) as usize],
            _ => todo!(),
        }
    }

    fn read_bytes_len(&self, value: Value) -> Result<usize> {
        Ok(self.read_array(value).context("read_bytes_len")?.length)
    }

    fn read_array(&self, value: Value) -> Result<Array> {
        match value {
            Value::Addr(addr, AddrPlace::Heap, _) => {
                let header = self.heap.parse_from_data_pointer(addr)?;

                Array::from_values(
                    &self.heap.data[header.get_data_pointer()..header.get_end_pointer()],
                )
            }
            Value::Addr(addr, AddrPlace::Stack, _) => {
                let length = self.stack[addr]
                    .clone()
                    .as_named_int(ValueIntFlag::Len)
                    .ok_or(anyhow::anyhow!(
                        "expected int:len but got {:?}",
                        self.stack[addr]
                    ))? as usize;

                Array::from_values(&self.stack[addr..addr + length])
            }
            t => todo!("{:?}", t),
        }
    }

    #[allow(dead_code)]
    fn read_values_by(&self, value: Value, size: usize) -> Result<Vec<Value>> {
        match value {
            Value::Addr(addr, AddrPlace::Heap, _) => {
                let header = self.heap.parse_from_data_pointer(addr)?;
                assert!(size <= header.len(), "{} {}", size, header.len());

                let mut bytes = vec![];
                for i in 0..size {
                    bytes.push(self.heap.data[addr + i].clone());
                }

                Ok(bytes)
            }
            Value::Addr(addr, AddrPlace::Stack, _) => {
                let len = self.stack[addr].clone().as_int().unwrap() as usize;
                assert!(size <= len, "{} {}", size, len);

                let mut bytes = vec![];
                for i in 0..size {
                    bytes.push(self.stack[addr + i].clone());
                }

                Ok(bytes)
            }
            _ => todo!(),
        }
    }

    fn read_string_at(&self, value: Value) -> Result<String> {
        let mut bytes = vec![];
        for v in self.read_array(value)?.data {
            bytes.push(v.as_int().unwrap() as u8);
        }

        String::from_utf8(bytes).map_err(|e| e.into())
    }

    pub fn step(&mut self) -> Result<()> {
        match self.code[self.pc].clone() {
            QVMInstruction::Call => {
                let r = self.pop();
                assert_matches!(r, Value::Int(_, _), "{:?}", r);

                let jump_to = r.as_int().unwrap() as usize;
                self.push(Value::Int(self.pc as i32 + 1, ValueIntFlag::Pc(jump_to)));
                self.pc = jump_to;
                self.push(Value::Int(self.frame_pointer as i32, ValueIntFlag::Fp));
                self.frame_pointer = self.stack_pointer;

                return Ok(());
            }
            QVMInstruction::Return(args, size) => {
                // exit this program
                if self.frame_pointer == 0 {
                    self.pc = self.code.len();
                    return Ok(());
                }

                /* Before:
                 * [..., argument*, pc, fp, local*, return_value]
                 *                          ^ fp    ^ sp
                 *
                 * After:
                 * [..., return_value]
                 *       ^ sp
                 *
                 */

                let current_fp = self.frame_pointer;

                assert!(size > 0);
                let mut results = vec![];
                for _ in 0..size {
                    results.push(self.pop());
                }
                self.stack_pointer = self.frame_pointer;

                let fp = self.load(1);
                self.frame_pointer = fp.as_named_int(ValueIntFlag::Fp).unwrap() as usize;

                let pc = self.load(2);
                self.pc = pc.as_int_pc().unwrap();

                for (i, r) in results.into_iter().rev().enumerate() {
                    self.stack[current_fp - (args + 2) + i] = r;
                }
                self.stack_pointer = self.stack_pointer - args - 2 + size; // -args, +size word (return value)

                return Ok(());
            }
            QVMInstruction::Add => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::int(b + a));
            }
            QVMInstruction::Sub => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::int(b - a));
            }
            QVMInstruction::Mult => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::int(b * a));
            }
            QVMInstruction::Div => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::int(b / a));
            }
            QVMInstruction::Mod => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::int(b % a));
            }
            QVMInstruction::Eq => {
                let a = self.pop();
                let b = self.pop();
                let r = match (a, b) {
                    // FIXME: should we just support other heterogeneous cases like nil?
                    (a, b) if a.is_nil() => b.is_nil(),
                    (a, b) if b.is_nil() => a.is_nil(),
                    (Value::Bool(s), Value::Bool(t)) => s == t,
                    (Value::Int(s, f), Value::Int(t, g)) if f == g => s == t,
                    (Value::Addr(s, f, j), Value::Addr(t, g, k)) if f == g && j == k => s == t,
                    (a, b) => todo!("{:?} == {:?}", a, b),
                };
                self.push(Value::bool(r));
            }
            QVMInstruction::Neq => {
                let a = self.pop();
                let b = self.pop();
                self.push(Value::bool(b.as_int().unwrap() != a.as_int().unwrap()));
            }
            QVMInstruction::Lt => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::bool(b < a));
            }
            QVMInstruction::Gt => {
                let a = self.pop().as_int().unwrap();
                let b = self.pop().as_int().unwrap();
                self.push(Value::bool(b > a));
            }
            QVMInstruction::Le => todo!(),
            QVMInstruction::And => {
                let a = self.pop().as_bool().unwrap();
                let b = self.pop().as_bool().unwrap();
                self.push(Value::bool(b && a));
            }
            QVMInstruction::Or => {
                let a = self.pop().as_bool().unwrap();
                let b = self.pop().as_bool().unwrap();
                self.push(Value::bool(b || a));
            }
            QVMInstruction::Not => {
                let a = self.pop().as_bool().unwrap();
                self.push(Value::bool(!a));
            }
            QVMInstruction::I32Const(c) => {
                self.push(Value::int(c));
            }
            QVMInstruction::I32LenConst(c) => {
                self.push(Value::Int(c as i32, ValueIntFlag::Len));
            }
            QVMInstruction::AddrConst(addr, variable) => match variable {
                Variable::Local => {
                    assert!(
                        self.stack[self.frame_pointer - 1]
                            .clone()
                            .as_named_int(ValueIntFlag::Fp)
                            .is_some(),
                        "{} at {:?}",
                        self.frame_pointer,
                        &self.stack[self.frame_pointer - 5..self.frame_pointer + 1]
                    );
                    assert!(
                        self.frame_pointer + addr < self.stack_pointer,
                        "{} {}",
                        self.frame_pointer + addr,
                        self.stack_pointer
                    );

                    // Calculate absolute index in stack
                    // This is mandatory because the relative index will be changed by the current call stack
                    self.push(Value::addr(self.frame_pointer + addr, AddrPlace::Stack));
                }
                _ => {
                    self.push(Value::addr(addr, AddrPlace::from_variable(variable)));
                }
            },
            QVMInstruction::Load(u) => {
                let addr_value = self.pop();
                assert!(addr_value.is_nil() || addr_value.clone().as_addr().is_some());

                match addr_value {
                    a if a.is_nil() => {
                        info!("{}", self.debug_info());
                        bail!("nil pointer exception");
                    }
                    Value::Addr(i, space, _) => match space {
                        AddrPlace::Stack => {
                            if u > 1 {
                                assert!(
                                    matches!(
                                        self.stack[i],
                                        Value::Addr(_, AddrPlace::InfoTable, _)
                                    ),
                                    "{:?} {}",
                                    self.stack[i],
                                    self.pc,
                                );
                            }

                            for j in 0..u {
                                self.push(self.stack[i + j].clone());
                            }
                        }
                        AddrPlace::Heap => {
                            for j in 0..u {
                                self.push(self.heap.data[i + j].clone());
                            }
                        }
                        AddrPlace::Static => {
                            for j in 0..u {
                                let value = self.globals[i + j];
                                self.push(Value::int(value));
                            }
                        }
                        t => unreachable!("{:?}", t),
                    },
                    _ => unreachable!(),
                }
            }
            QVMInstruction::Store(size) => {
                let mut values = vec![];
                for _ in 0..size {
                    values.push(self.pop());
                }
                values.reverse();

                let addr_value = self.pop();
                assert_matches!(
                    addr_value.clone().as_addr(),
                    Some(_),
                    "{:?} at {}",
                    addr_value,
                    self.pc
                );

                match addr_value {
                    Value::Addr(r, space, _) => match space {
                        AddrPlace::Stack => {
                            if size > 1 {
                                assert!(
                                    matches!(
                                        self.stack[r],
                                        Value::Addr(_, AddrPlace::InfoTable, _)
                                    ),
                                    "{:?} {}",
                                    self.stack[r],
                                    self.pc,
                                );
                            }

                            for j in 0..size {
                                self.stack[r + j] = values[j].clone();
                            }
                        }
                        AddrPlace::Heap => {
                            for j in 0..size {
                                self.heap.data[r + j] = values[j].clone();
                            }
                        }
                        AddrPlace::Static => {
                            for j in 0..size {
                                self.globals[r + j] = values[j].clone().as_int().unwrap();
                            }
                        }
                        _ => unreachable!("{:?}", addr_value),
                    },
                    _ => unreachable!(),
                }
            }
            QVMInstruction::Pop(r) => {
                for _ in 0..r {
                    self.pop();
                }
            }
            QVMInstruction::ArgConst(r) => {
                self.push(Value::addr(self.frame_pointer - 3 - r, AddrPlace::Stack));
            }
            QVMInstruction::Jump(k) => {
                self.pc = k;

                return Ok(());
            }
            QVMInstruction::JumpIf(k) => {
                let v = self.pop();
                if v.as_bool().unwrap() == true {
                    self.pc = k;

                    return Ok(());
                }
            }
            QVMInstruction::JumpIfFalse(k) => {
                let v = self.pop();
                if v.as_bool().unwrap() == false {
                    self.pc = k;

                    return Ok(());
                }
            }
            QVMInstruction::Alloc => {
                let size = self.pop().as_int().unwrap() as usize + 1;
                let addr = self.heap.alloc(size)?;
                self.heap.data[addr] = Value::Int(size as i32, ValueIntFlag::Len);

                self.push(Value::addr(addr, AddrPlace::Heap));
            }
            QVMInstruction::Free(addr) => {
                self.heap.free(self.heap.parse(addr)?)?;
            }
            QVMInstruction::PAdd => {
                let a = self.pop();
                let (b, v) = match self.pop() {
                    Value::Addr(b, v, ValueAddrFlag::Addr) => (b, v),
                    t => {
                        unreachable!(
                            "{}, {:?}, {:?} ({:?})",
                            self.pc,
                            t,
                            &self.stack[0..self.stack_pointer],
                            a,
                        );
                    }
                };
                self.push(Value::addr(b + a.as_int().unwrap() as usize, v));
            }
            QVMInstruction::PAddIm(a) => {
                let (b, v) = match self.pop() {
                    Value::Addr(b, v, ValueAddrFlag::Addr) => (b, v),
                    t => {
                        unreachable!("{}, {:?}, ({:?})", self.pc, t, a);
                    }
                };
                self.push(Value::addr(b + a, v));
            }
            QVMInstruction::RuntimeInstr(ref label) => match label.as_str() {
                "_gc" => {
                    self.run_gc()?;
                    self.push(Value::nil());
                }
                "_println" => {
                    let string_data = self.pop();

                    let s = self.read_string_at(string_data)?;
                    println!("[quartz] {}", s);

                    self.push(Value::nil());
                }
                "_len" => {
                    let p = self.pop();
                    let size = self.read_bytes_len(p)?;

                    self.push(Value::int(size as i32 - 1));
                }
                "_copy" => {
                    let target_offset = self.pop().as_int().unwrap() as usize;
                    let target = self.pop();
                    let source_offset = self.pop().as_int().unwrap() as usize;
                    let source = self.pop();
                    let size = self.pop().as_int().unwrap() as usize;

                    let source_array = self.read_array(source)?;
                    self.read_array(target.clone())?;

                    match target {
                        Value::Addr(target_addr, AddrPlace::Heap, _) => {
                            for i in 0..size {
                                // NOTE: need to handle the pointer to info table
                                self.heap.data[target_addr + target_offset + i + 1] =
                                    source_array.data[source_offset + i].clone();
                            }
                        }
                        Value::Addr(target_addr, AddrPlace::Stack, _) => {
                            for i in 0..size {
                                // NOTE: need to handle the pointer to info table
                                self.stack[target_addr + target_offset + i + 1] =
                                    source_array.data[source_offset + i].clone();
                            }
                        }
                        _ => unreachable!(),
                    }

                    self.push(Value::nil());
                }
                "_panic" => {
                    panic!("====== PANIC CALLED ======");
                }
                "_debug" => {
                    println!("{}", self.debug_info());
                    self.push(Value::nil());
                }
                "_start_debugger" => {
                    self.push(Value::nil());

                    if self.debug_mode {
                        // Increment PC ahead to process this instruction.
                        self.pc += 1;

                        let mut file = File::create("./quartz-debugger.json").unwrap();
                        file.write_all(&serde_json::to_vec_pretty(&self).unwrap())
                            .unwrap();

                        std::process::exit(0);
                    }
                }
                "_check_sp" => {
                    debug!("{}", self.debug_info());

                    let sp = self.pop();
                    assert_eq!(
                        sp.as_int().unwrap() as usize,
                        self.stack_pointer - self.frame_pointer
                    );
                }
                "_coerce" => {
                    let actual_size = self.pop().as_int()? as usize;
                    let expected_size = self.pop().as_int()? as usize;
                    assert!(expected_size >= actual_size);
                    let values = self.pop_many(actual_size);

                    self.push(Value::addr(expected_size, AddrPlace::InfoTable));
                    if values.len() > 1 {
                        for v in values.into_iter().skip(1) {
                            self.push(v);
                        }

                        for _ in 0..expected_size - actual_size {
                            self.push(Value::nil());
                        }
                    } else {
                        self.push(values[0].clone());

                        for _ in 0..expected_size - actual_size - 1 {
                            self.push(Value::nil());
                        }
                    }
                }
                "_is_nil" => {
                    let value = self.pop();
                    if value.is_nil() {
                        self.push(Value::bool(true));
                    } else {
                        assert!(value.as_addr().is_some());
                        self.push(Value::bool(false));
                    }
                }
                t => {
                    unreachable!("{}", t);
                }
            },
            QVMInstruction::BoolConst(b) => {
                self.push(Value::bool(b));
            }
            QVMInstruction::Ref => {
                self.push(Value::addr(self.stack_pointer, AddrPlace::Stack));
            }
            QVMInstruction::InfoConst(i) => {
                self.push(Value::addr(i, AddrPlace::InfoTable));
            }
            QVMInstruction::Nop => {}
            QVMInstruction::Copy => match self.pop() {
                Value::Addr(addr, AddrPlace::Stack, _) => {
                    let value = self.stack[addr].clone();
                    if let Value::Addr(size, AddrPlace::InfoTable, _) = value {
                        for i in 0..=size {
                            self.push(self.stack[addr + i].clone());
                        }
                    } else {
                        self.push(value);
                    }
                }
                value => {
                    self.push(value);
                }
            },
            QVMInstruction::NilConst => {
                self.push(Value::nil());
            }
            QVMInstruction::LabelI32Const(_) => unreachable!(),
            QVMInstruction::LabelJumpIfFalse(_) => unreachable!(),
            QVMInstruction::LabelJumpIf(_) => unreachable!(),
            QVMInstruction::LabelJump(_) => todo!(),
        }

        self.pc += 1;

        Ok(())
    }

    pub fn step_out(&mut self) -> Result<()> {
        while self.pc < self.code.len() {
            debug!("{}", self.debug_info());
            let is_return = matches!(self.code[self.pc], QVMInstruction::Return(_, _));

            self.step()?;

            if is_return {
                break;
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while self.pc < self.code.len() {
            debug!("{}", self.debug_info());

            self.step()?;
        }

        Ok(())
    }
}

#[test]
fn runtime_run_hand_coded() -> Result<()> {
    use QVMInstruction::*;

    let cases = vec![(
        /*
            func main(b): int {
                let a = 1;
                let z = 10;
                let c = a + b;
                return c;
            }

            main(2);
        */
        vec![
            // entrypoint:
            I32Const(2),
            I32Const(4),
            Call, // call main
            Return(1, 1),
            // main:
            I32Const(1),                   // a
            I32Const(10),                  // z
            AddrConst(0, Variable::Local), // a + b
            Load(1),                       // load a
            ArgConst(0),                   // load b
            Load(1),
            Add,          // a + b
            Return(1, 1), // return
        ],
        3,
    )];

    for (code, result) in cases {
        let mut runtime = Runtime::new(code, 0);
        runtime.run()?;
        assert_eq!(result, runtime.pop().as_int().unwrap());
    }

    Ok(())
}

#[test]
fn runtime_run() -> Result<()> {
    use pretty_assertions::assert_eq;
    use quartz_core::compiler::Compiler;

    let cases = vec![
        (r#"func main() { return 10; }"#, 10),
        (r#"func main() { return _add(1, 20); }"#, 21),
        (
            r#"
func calc(b: int): int {
    let a = 1;
    let z = 10;
    let c = _add(a, b);
    return c;
}

func main(): int {
    return calc(2);
}
"#,
            3,
        ),
        (
            r#"
let a = 5;

func f() {
    a = _add(a, 10);
    return nil;
}

func main(): int {
    f();
    return a;
}
        "#,
            15,
        ),
        (
            r#"
func factorial(n: int) {
    if _eq(n,0) {
        return 1;
    } else {
        return _mult(n, factorial(_sub(n,1)));
    };
}

func main() {
    return factorial(5);
}
"#,
            120,
        ),
        (
            r#"
func main(): int {
    let x = make[array[int,5]](0);
    x(0) = 1;
    x(1) = 2;
    x(2) = x(0) + x(1);

    return x(2);
}
"#,
            3,
        ),
        (
            r#"
func main() {
    1;
    2;

    return 0;
}
"#,
            0,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

func main() {
    let p = Point {
        x: 1,
        y: 2,
    };

    return p.y;
}
"#,
            2,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

method Point sum(self): int {
    return _add(self.x, self.y);
}

func main() {
    let p = Point {
        x: 1,
        y: 2,
    };

    return p.sum();
}
"#,
            3,
        ),
        (
            r#"
func main() {
    let p = "Hello, World!";

    return p.bytes()(7);
}
"#,
            'W' as i32,
        ),
        (
            r#"
func main() {
    let p = "Hello, World!";

    return p.len();
}
"#,
            13,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

method Point sum(self): int {
    return _add(self.x, self.y);
}

func main() {
    let p = Point { x:0, y:0 };
    p.x = 1;
    p.y = 2;

    return p.sum();
}
"#,
            3,
        ),
        (
            r#"
func main() {
    let sum = 0;
    let n = 0;
    while _lt(n, 10) {
        sum = sum + n;
        n = n + 1;
    };

    return sum;
}
"#,
            45,
        ),
        (
            r#"
struct Modify {
    a: int,
}

method Modify f(self, c: int) {
    self.a = self.a + c;

    return nil;
}

func main() {
    let m = Modify { a: 10 };
    m.f(20);

    return m.a;
}
"#,
            30,
        ),
        (
            r#"
func main() {
    let result = 1;

    while false {
        result = 0;
    };

    return result;
}
"#,
            1,
        ),
        (
            r#"
func main() {
    let n = 0;
    let result = 0;

    while _lt(n, 10) {
        let k = n;
        if _eq(k, 0) {
            let p = 1;

            result = p;
        } else {
            result = result + n;
        };

        n = n + 1;
    };

    return result;
}
"#,
            46,
        ),
        (
            r#"
struct Child {
    n: int,
}

struct Nested {
    child: Child,
    m: int,
}

method Nested f(self): int {
    self.child.n = self.child.n + 1;
    return self.child.n + self.m;
}

func main(): int {
    let nested = Nested {
        child: Child {
            n: 10,
        },
        m: 20,
    };

    return nested.f();
}
"#,
            31,
        ),
        (
            r#"
struct Child {
    n: int,
}

func new(k: int): Child {
    return Child {
        n: k,
    };
}

func main(): int {
    let child = new(10);

    return child.n;
}
"#,
            10,
        ),
        (
            r#"
struct Child {
    n: int,
}

func id(n: int): int {
    return n;
}

struct Nested {
    child: Child,
    m: int,
}


func main(): int {
    let nested = Nested {
        child: Child {
            n: 10,
        },
        m: 20,
    };

    return id(nested.child.n);
}
"#,
            10,
        ),
        (
            r#"
struct Child {
    n: int,
}

method Child getN(self): int {
    return self.n;
}

struct Nested {
    child: Child,
    m: int,
}

func main(): int {
    let nested = Nested {
        child: Child {
            n: 10,
        },
        m: 20,
    };

    return nested.child.getN();
}
"#,
            10,
        ),
        (
            r#"
struct Foo {
    value: int?,
}

func main() {
    let foo = Foo {
        value: 100,
    };
    let bar = Foo {
        value: nil,
    };

    if _is_nil(bar.value) {
        return 10;
    } else {
        return 20;
    };
}
"#,
            10,
        ),
        (
            r#"
struct Nat {
    succ: ref Nat,
}

method Nat add(self, m: Nat): Nat {
    if _is_nil(self.succ) {
        return m;
    } else {
        return Nat {
            succ: ref self.succ.add(m)
        };
    };
}

method Nat to_int(self): int {
    if _is_nil(self.succ) {
        return 0;
    } else {
        return self.succ.to_int() + 1;
    };
}

func main() {
    let zero = Nat {
        succ: nil,
    };
    let two = Nat {
        succ: ref Nat {
            succ: ref zero,
        },
    };
    let three = Nat {
        succ: ref two,
    };

    return two.add(three).to_int();
}
"#,
            5,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

struct CoerceNil {
    point: Point?,
}

struct CoerceRef {
    point: ref Point,
}

func main() {
    let c1 = CoerceNil {
        point: nil,
    };
    let c2 = CoerceRef {
        point: nil,
    };

    if _is_nil(c1.point) {
        return 1;
    } else {
        return 0;
    };
}
"#,
            1,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

method Point sum(self): int {
    return self.x + self.y;
}

func main() {
    let p = Point {
        x: 10,
        y: 20,
    };

    return Point::sum(ref p);
}
"#,
            30,
        ),
        (
            r#"
struct Point {
    x: int,
    y: int,
}

method Point add_x(self) {
    self.x = self.x + 10;
}

method Point sum(self): int {
    self.add_x();
    return self.x + self.y;
}

func main() {
    let p = Point {
        x: 10,
        y: 20,
    };

    return p.sum();
}
"#,
            40,
        ),
        (
            r#"
func concat_array(a: array[int], b: array[int]): array[int] {
    let p = make[array[int]](_len(a) + _len(b), 0);
    let i = 0;
    while (i < _len(a)) {
        p(i) = a(i);
        i = i + 1;
    };

    let i = 0;
    while (i < _len(b)) {
        p(_len(a) + i) = b(i);
        i = i + 1;
    };

    return p;
}

func main() {
    let p1 = make[array[int]](5, 0);
    p1(0) = 1;
    p1(1) = 2;

    let p2 = make[array[int]](5, 0);
    p2(0) = 3;
    p2(1) = 4;

    let p = concat_array(p1, p2);
    assert_eq_int(p(0), 1);
    assert_eq_int(p(1), 2);
    assert_eq_int(p(5), 3);
    assert_eq_int(p(6), 4);

    return _len(p);
}
"#,
            10,
        ),
        (
            r#"
struct Q {
    a: int,
}

struct P {
    x: Q?,
}

method P get_x(self): int {
    return self.x!.a;
}

func main() {
    let p = P {
        x: Q {
            a: 10,
        },
    };

    return p.get_x();
}
"#,
            10,
        ),
        (
            r#"
func int_or(t: int): int? {
    return t as int?;
}

func main() {
    let p = nil as int?;
    p = int_or(100);

    if _is_nil(p) {
        return 0;
    } else {
        return p!;
    };
}
"#,
            100,
        ),
        (
            r#"
func main() {
    let s = "hello";
    let t = ", world";
    if s.concat(t).eq("hello, world") {
        return 1;
    } else {
        return 0;
    };
}
"#,
            1,
        ),
        (
            r#"
struct Point {
    x: int,
}

func f(): Point {
    return Point {
        x: 10,
    };
}

func main() {
    return f().x;
}
"#,
            10,
        ),
    ];

    for (input, result) in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input.to_string(), "main".to_string())?;

        let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
        println!("{}", input);
        println!("{}", compiler.ir_result.clone().unwrap().show());
        println!("{}", compiler.show_qasmv(&code));
        runtime.run()?;
        let pop = runtime.pop();
        assert_eq!(
            pop.clone().as_int().unwrap(),
            result,
            "{:?} {:?}",
            pop,
            result
        );
    }

    Ok(())
}

#[test]
fn runtime_run_env() -> Result<()> {
    use quartz_core::compiler::Compiler;

    let cases = vec![
        r#"
func main() {
    let p = "ABC";

    return p.bytes();
}
"#,
    ];

    for input in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input.to_string(), "main".to_string())?;

        let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
        println!("{}", input);
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let addr = runtime.pop();
        let bytes = runtime.read_array(addr)?;
        assert_eq!(
            String::from_utf8(
                bytes
                    .data
                    .iter()
                    .map(|u| u.clone().as_int().unwrap() as u8)
                    .collect()
            )
            .unwrap(),
            "ABC".to_string(),
        );
    }

    Ok(())
}

/* after _new & GC implementation
#[test]
fn runtime_run_gc() -> Result<()> {
    use quartz_core::compiler::Compiler;

    let cases = vec![
        (
            r#"
            func f(arr: array[int]): int {
                return arr(0);
            }

            func g(): int {
                let arr = make[array[int]](5, 0);
                arr(0) = 1;
                arr(1) = 2;
                arr(2) = 3;
                arr(3) = 4;
                return f(arr);
            }

            func main() {
                let preserved = make[array[int]](3, 0);
                preserved(0) = 5;
                preserved(1) = 6;
                preserved(2) = 7;
                let p = g();

                _gc;
                return p;
            }
        "#,
            1,
            1, // arr being collected
        ),
        (
            r#"
            func f() {
                // cyclic reference
                let link = _new(2);
                link[0] = _padd(link, 1);
                link[1] = _padd(link, 0);

                return nil;
            }

            func main() {
                f();
                _gc;

                return 0;
            }
        "#,
            0,
            0, // link being collected
        ),
        (
            r#"
            func f() {
                // cyclic reference
                let link = _new(2);
                link[0] = _padd(link, 1);
                link[1] = _padd(link, 0);

                let data = [link];

                return data;
            }

            func main() {
                let d = f();
                _gc;

                return 0;
            }
        "#,
            0,
            2, // data and link NOT being collected
        ),
    ];

    for (input, result, remaining_object_result) in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input, "main".to_string())?;

        let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
        println!("{}", input);
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let pop = runtime.pop();
        assert_eq!(
            pop.clone().as_int().unwrap(),
            result,
            "{:?} {:?}",
            pop,
            result
        );

        let mut remaining_object = 0;
        let mut current = runtime.heap.root()?;
        while let Ok(next) = runtime.heap.find_next(&current) {
            if !next.is_collectable() {
                break;
            }

            remaining_object += 1;
            current = next;
        }

        assert_eq!(remaining_object_result, remaining_object);
    }

    Ok(())
}
*/
