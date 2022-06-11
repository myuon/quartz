use anyhow::Result;
use quartz_core::vm::QVMInstruction;

use crate::freelist::Freelist;

#[derive(Clone, Debug, Copy)]
pub enum AddrPlace {
    Heap,
    Unknown,
}

#[derive(Clone, Copy)]
pub enum Value {
    Int(i32, &'static str),
    Addr(usize, AddrPlace, &'static str),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0, arg1) => {
                write!(f, "{}:{}", arg0, arg1)
            }
            Self::Addr(arg0, _, arg1) => {
                write!(f, "*{}:{}", arg0, arg1)
            }
        }
    }
}

impl Value {
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Value::Int(i, "bool") if i == 0 => Some(false),
            Value::Int(i, "bool") if i == 1 => Some(true),
            _ => None,
        }
    }

    pub fn as_int(self) -> Option<i32> {
        match self {
            Value::Int(i, "int") => Some(i),
            _ => None,
        }
    }

    pub fn as_addr(self) -> Option<usize> {
        match self {
            Value::Addr(i, _, "addr") => Some(i),
            _ => None,
        }
    }

    pub fn as_named_int(self, name: &str) -> Option<i32> {
        match self {
            Value::Int(i, n) if n == name => Some(i),
            _ => None,
        }
    }

    pub fn as_named_addr(self, name: &str) -> Option<usize> {
        match self {
            Value::Addr(i, _, n) if n == name => Some(i),
            _ => None,
        }
    }

    pub fn metadata(&self) -> &'static str {
        match self {
            Value::Int(_, metadata) => metadata,
            Value::Addr(_, _, metadata) => metadata,
        }
    }

    pub fn nil() -> Value {
        Value::Addr(0, AddrPlace::Unknown, "nil")
    }

    pub fn bool(b: bool) -> Value {
        Value::Int(if b { 1 } else { 0 }, "bool")
    }

    pub fn int(i: i32) -> Value {
        Value::Int(i, "int")
    }

    pub fn addr(i: usize) -> Value {
        Value::Addr(i, AddrPlace::Heap, "addr")
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

#[derive(Debug)]
pub struct Runtime {
    stack: Vec<Value>,
    heap: Freelist,
    globals: Vec<i32>,
    code: Vec<QVMInstruction>,
    pc: usize,
    stack_pointer: usize,
    frame_pointer: usize,
}

impl Runtime {
    pub fn new(code: Vec<QVMInstruction>, globals: usize) -> Runtime {
        Runtime {
            stack: vec![],
            heap: Freelist::new(1_000_000),
            globals: vec![0; globals],
            code,
            pc: 0,
            stack_pointer: 0,
            frame_pointer: 0,
        }
    }

    fn run_gc(&mut self) {}

    fn pop(&mut self) -> Value {
        assert!(
            self.stack_pointer > 0,
            "{} at {:?}",
            self.stack_pointer,
            self.stack
        );
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
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
        self.stack[self.stack_pointer - offset]
    }

    pub fn run(&mut self) -> Result<()> {
        while self.pc < self.code.len() {
            println!(
                "{:?}\n{:?}\n{:?}\n",
                &self.heap.data[0..25],
                &self.stack[0..self.stack_pointer].iter().collect::<Vec<_>>(),
                &self.code[self.pc],
            );
            match self.code[self.pc] {
                QVMInstruction::Jump(_) => todo!(),
                QVMInstruction::Call => {
                    let r = self.pop();
                    assert_matches!(r, Value::Addr(_, AddrPlace::Heap, _));

                    self.push(Value::Int(self.pc as i32 + 1, "pc"));
                    self.pc = r.as_addr().unwrap();
                    self.push(Value::Int(self.frame_pointer as i32, "fp"));
                    self.frame_pointer = self.stack_pointer;
                    continue;
                }
                QVMInstruction::Return(args) => {
                    // exit this program
                    if self.frame_pointer == 0 {
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

                    let result = self.pop();
                    self.stack_pointer = self.frame_pointer;

                    let fp = self.load(1);
                    self.frame_pointer = fp.as_named_int("fp").unwrap() as usize;

                    let pc = self.load(2);
                    self.pc = pc.as_named_int("pc").unwrap() as usize;

                    self.stack[current_fp - (args + 2)] = result;
                    self.stack_pointer -= args + 1; // -2 words (fp, pc), +1 word (return value)
                    continue;
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
                QVMInstruction::Div => todo!(),
                QVMInstruction::Mod => todo!(),
                QVMInstruction::Eq => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value::Int(
                        if b.as_int().unwrap() == a.as_int().unwrap() {
                            1
                        } else {
                            0
                        },
                        "bool",
                    ));
                }
                QVMInstruction::Neq => todo!(),
                QVMInstruction::Lt => todo!(),
                QVMInstruction::Le => todo!(),
                QVMInstruction::And => todo!(),
                QVMInstruction::Or => todo!(),
                QVMInstruction::Not => todo!(),
                QVMInstruction::I32Const(c) => {
                    self.push(Value::int(c));
                }
                QVMInstruction::AddrConst(addr, _) => {
                    self.push(Value::addr(addr));
                }
                QVMInstruction::Load(kind) => {
                    let addr_value = self.pop();
                    assert!(matches!(addr_value, Value::Addr(_, _, "addr")));
                    let i = addr_value.as_addr().unwrap();

                    match kind {
                        "local" => {
                            assert!(
                                matches!(self.stack[self.frame_pointer - 1], Value::Int(_, "fp")),
                                "{} at {:?}",
                                self.frame_pointer,
                                self.stack
                            );

                            let v = self.stack[self.frame_pointer + i];
                            assert!(matches!(v.metadata(), "int" | "addr" | "&bytes"), "{:?}", v);
                            self.push(v);
                        }
                        "local_rev" => {
                            self.push(self.stack[self.stack_pointer - 1 - i]);
                        }
                        "heap" => {
                            self.push(self.heap.data[i]);
                        }
                        "global" => {
                            let value = self.globals[i];
                            self.push(Value::int(value));
                        }
                        _ => {
                            unreachable!();
                        }
                    };
                }
                QVMInstruction::Store(kind) => {
                    let addr_value = self.pop();
                    assert_matches!(addr_value, Value::Addr(_, AddrPlace::Heap, "addr"));
                    let r = addr_value.as_addr().unwrap();

                    match kind {
                        "local" => {
                            self.stack[self.stack_pointer - r] = self.pop();
                        }
                        "heap" => {
                            let value = self.pop();
                            assert_matches!(value.metadata(), "int" | "&bytes", "{:?}", value);

                            self.heap.data[r] = value;
                        }
                        "global" => {
                            let value = self.pop();
                            self.globals[r] = value.as_int().unwrap();
                        }
                        _ => {
                            unreachable!();
                        }
                    };
                }
                QVMInstruction::Pop(r) => {
                    for _ in 0..r {
                        self.pop();
                    }
                }
                QVMInstruction::LoadArg(r) => {
                    let arg = self.stack[self.frame_pointer - 3 - r];
                    assert_matches!(
                        arg.metadata(),
                        "int" | "addr" | "&bytes",
                        "{}",
                        arg.metadata()
                    );
                    self.push(arg);
                }
                QVMInstruction::JumpIfFalse(k) => {
                    let v = self.pop();
                    if v.as_bool().unwrap() == false {
                        self.pc = k;
                        continue;
                    }
                }
                QVMInstruction::Alloc => {
                    let size = self.pop();
                    let addr = self.heap.alloc(size.as_int().unwrap() as usize)?;
                    self.push(Value::Addr(addr, AddrPlace::Heap, "&bytes"));
                }
                QVMInstruction::Free(addr) => {
                    self.heap.free(self.heap.parse(addr)?)?;
                }
                //
                QVMInstruction::LabelAddrConst(_) => unreachable!(),
                QVMInstruction::LabelJumpIfFalse(_) => unreachable!(),
                QVMInstruction::PAdd => {
                    let a = self.pop();
                    let b = match self.pop() {
                        Value::Addr(b, AddrPlace::Heap, "&bytes") => b,
                        Value::Addr(b, AddrPlace::Heap, "addr") => b,
                        _ => {
                            unreachable!();
                        }
                    };
                    self.push(Value::addr(b + a.as_int().unwrap() as usize));
                }
            }

            self.pc += 1;
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
            AddrConst(4, String::new()),
            Call, // call main
            Return(0),
            // main:
            I32Const(1),  // a
            I32Const(10), // z
            AddrConst(0, String::new()),
            Load("local"), // load a
            LoadArg(0),    // load b
            Add,           // a + b
            Return(1),     // return
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
func main() {
    let x = _new(5);
    x[0] = 1;
    x[1] = 2;
    x[2] = _add(x[0], x[1]);

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
    ];

    for (input, result) in cases {
        let mut compiler = Compiler::new();
        let code = compiler.compile(input)?;

        let mut runtime = Runtime::new(code.clone(), compiler.code_generation.globals());
        println!("{}", input);
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let pop = runtime.pop();
        assert_eq!(pop.as_int(), Some(result), "{:?} {:?}", pop, result);
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
        let code = compiler.compile(input)?;

        let mut runtime = Runtime::new(code.clone(), compiler.code_generation.globals());
        println!("{}", input);
        for (n, inst) in runtime.code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
        runtime.run()?;
        let bytes = runtime.pop().as_named_addr("&bytes").unwrap();
        assert_eq!(
            String::from_utf8(
                runtime.heap.data[bytes..bytes + 3]
                    .iter()
                    .map(|u| u.as_int().unwrap() as u8)
                    .collect()
            )
            .unwrap(),
            "ABC".to_string(),
        );
    }

    Ok(())
}
