use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use log::debug;
use quartz_core::vm::{QVMInstruction, Variable};
use serde::{Deserialize, Serialize};

use crate::freelist::Freelist;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueIntFlag {
    Int, // default
    Len, // length in heap
    Pc,  // program counter
    Fp,  // frame pointer
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueAddrFlag {
    Addr,   // default
    Nodata, // no data in heap
    Prev,   // prev in heap
    Next,   // next in heap
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32, ValueIntFlag),
    Addr(usize, AddrPlace, ValueAddrFlag),
}

impl Value {
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_int(self) -> Option<i32> {
        match self {
            Value::Int(i, ValueIntFlag::Int) => Some(i),
            _ => None,
        }
    }

    pub fn as_addr(self) -> Option<usize> {
        match self {
            Value::Addr(i, _, ValueAddrFlag::Addr) => Some(i),
            _ => None,
        }
    }

    #[allow(dead_code)]
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

    pub fn as_named_addr(self, flag: ValueAddrFlag) -> Option<usize> {
        match self {
            Value::Addr(i, _, n) if n == flag => Some(i),
            _ => None,
        }
    }

    pub fn nil() -> Value {
        Value::Nil
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
        }
    }

    pub fn set_debug_mode(&mut self, debugger_json: PathBuf) {
        self.debug_mode = true;
        self.debugger_json_path = debugger_json;
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
            "{}\n{:?}\n{} {:?}\n",
            &self
                .heap
                .debug_objects()
                .iter()
                .map(|c| format!("{:?}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            &self.stack[0..self.stack_pointer].iter().collect::<Vec<_>>(),
            self.pc,
            &self.code[self.pc]
        )
    }

    pub fn step(&mut self) -> Result<()> {
        match self.code[self.pc].clone() {
            QVMInstruction::Call => {
                let r = self.pop();
                assert_matches!(r, Value::Int(_, _), "{:?}", r);

                self.push(Value::Int(self.pc as i32 + 1, ValueIntFlag::Pc));
                self.pc = r.as_int().unwrap() as usize;
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
                self.pc = pc.as_named_int(ValueIntFlag::Pc).unwrap() as usize;

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
                self.push(Value::bool(b.as_int().unwrap() == a.as_int().unwrap()));
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
            QVMInstruction::AddrConst(addr, variable) => match variable {
                Variable::Local => {
                    assert!(
                        self.stack[self.frame_pointer - 1]
                            .clone()
                            .as_named_int(ValueIntFlag::Fp)
                            .is_some(),
                        "{} at {:?}",
                        self.frame_pointer,
                        self.stack
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
                assert!(addr_value.clone().as_addr().is_some());

                match addr_value {
                    Value::Addr(i, space, _) => match space {
                        AddrPlace::Stack => {
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
            QVMInstruction::Store => {
                let value = self.pop();
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
                            self.stack[r] = value;
                        }
                        AddrPlace::Heap => {
                            self.heap.data[r] = value;
                        }
                        AddrPlace::Static => {
                            self.globals[r] = value.as_int().unwrap();
                        }
                        _ => unreachable!(),
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
                let size = self.pop();
                let addr = self.heap.alloc(size.as_int().unwrap() as usize)?;
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
                        unreachable!(
                            "{}, {:?}, {:?} ({:?})",
                            self.pc,
                            t,
                            &self.stack[0..self.stack_pointer],
                            a,
                        );
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
                    let value = self.pop().as_addr().unwrap();
                    let addr = self.heap.data[value].clone().as_addr().unwrap();
                    let header = self.heap.parse_from_data_pointer(addr)?;

                    let mut bytes = vec![];
                    for i in 0..header.len() {
                        bytes.push(self.heap.data[addr + i].clone().as_int().unwrap() as u8);
                    }

                    self.push(Value::nil());
                    println!("[quartz] {}", String::from_utf8(bytes).unwrap());
                }
                "_len" => {
                    let value = self.pop().as_heap_addr().unwrap();
                    let header = self.heap.parse_from_data_pointer(value).unwrap();
                    self.push(Value::int(header.len() as i32));
                }
                "_copy" => {
                    let target = self.pop().as_addr().unwrap();
                    let source = self.pop().as_addr().unwrap();
                    let len = self.pop().as_int().unwrap() as usize;
                    let target_offset = self.pop().as_int().unwrap() as usize;

                    for i in 0..len {
                        let value = self.heap.data[source + i].clone();
                        self.heap.data[target + i + target_offset] = value;
                    }

                    self.push(Value::nil());
                }
                "_panic" => {
                    panic!("====== PANIC CALLED ======\n{:?}", self.stack);
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
                _ => {
                    unreachable!();
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
func main(): byte {
    let x = _new(5);
    x[0] = _int_to_byte(1);
    x[1] = _int_to_byte(2);
    x[2] = _int_to_byte(_add(_byte_to_int(x[0]), _byte_to_int(x[1])));

    return x[2];
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

func (self: Point) sum(): int {
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
func main(): int {
    let p = [1,2,3,4];

    return p[2];
}
"#,
            3,
        ),
        (
            r#"
func main() {
    let p = "Hello, World!";

    return p.bytes()[7];
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

func (p: Point) sum(): int {
    return _add(p.x, p.y);
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

func (m: Modify) f(c: int) {
    m.a = m.a + c;

    return nil;
}

func main() {
    let m = Modify { a: 10 };
    m.f(20);

    return m.a;
}
"#,
            // FIXME: 10?(immutable) 30?(mutable)
            10,
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

func f(n: Nested): int {
    n.child.n = n.child.n + 1;
    return n.child.n + n.m;
}

func main(): int {
    let nested = Nested {
        child: Child {
            n: 10,
        },
        m: 20,
    };

    return f(nested);
}
"#,
            31,
        ),
        (
            r#"
struct Child {
    n: int,
}

func make(k: int): Child {
    return Child {
        n: k,
    };
}

func main(): int {
    let child = make(10);

    return child.n;
}
"#,
            10,
        ),
    ];

    for (input, result) in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input, "main".to_string())?;

        let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
        println!("{}", input);
        println!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let pop = runtime.pop();
        assert_eq!(pop.clone().as_int(), Some(result), "{:?} {:?}", pop, result);
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

    return p.data;
}
"#,
    ];

    for input in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input, "main".to_string())?;

        let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
        println!("{}", input);
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let bytes = runtime.pop().as_addr().unwrap();
        assert_eq!(
            String::from_utf8(
                runtime.heap.data[bytes..bytes + 3]
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

#[test]
fn runtime_run_gc() -> Result<()> {
    use quartz_core::compiler::Compiler;

    let cases = vec![
        (
            r#"
            func f(arr: ints): int {
                return arr[0];
            }

            func g(): int {
                let arr = [1,2,3,4];
                return f(arr);
            }

            func main() {
                let preserved = [5,6,7];
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
        assert_eq!(pop.clone().as_int(), Some(result), "{:?} {:?}", pop, result);

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
