use anyhow::Result;
use pretty_assertions::assert_eq;
use quartz_core::vm::QVMInstruction;

#[derive(Clone, Copy, Debug)]
pub struct Value(i32, &'static str);

/* StackFrame
    [argument*, return_address, fp, local*]
                                    ^ new fp
*/

#[derive(Debug)]
pub struct Runtime {
    stack: Vec<Value>,
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
            globals: vec![0; globals],
            code,
            pc: 0,
            stack_pointer: 0,
            frame_pointer: 0,
        }
    }

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
                "{:?} {:?}",
                &self.stack[0..self.stack_pointer].iter().collect::<Vec<_>>(),
                &self.code[self.pc],
            );
            match self.code[self.pc].clone() {
                QVMInstruction::GlobalGet(u) => {
                    let value = self.globals[u];
                    self.push(Value(value, "i32"));
                }
                QVMInstruction::GlobalSet(u) => {
                    let value = self.pop();
                    self.globals[u] = value.0;
                }
                QVMInstruction::Jump(_) => todo!(),
                QVMInstruction::Call(r) => {
                    self.push(Value((self.pc + 1) as i32, "pc"));
                    self.pc = r as usize;
                    self.push(Value(self.frame_pointer as i32, "fp"));
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
                    assert_eq!(result.1, "i32");
                    self.stack_pointer = self.frame_pointer;

                    let fp = self.load(1);
                    assert_eq!(fp.1, "fp");
                    self.frame_pointer = fp.0 as usize;

                    let pc = self.load(2);
                    assert_eq!(pc.1, "pc");
                    self.pc = pc.0 as usize;

                    self.stack[current_fp - (args + 2)] = result;
                    self.stack_pointer -= args + 1; // -2 words (fp, pc), +1 word (return value)
                    continue;
                }
                QVMInstruction::Add => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value(b.0 + a.0, "i32"));
                }
                QVMInstruction::Sub => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value(b.0 - a.0, "i32"));
                }
                QVMInstruction::Mult => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value(b.0 * a.0, "i32"));
                }
                QVMInstruction::Div => todo!(),
                QVMInstruction::Mod => todo!(),
                QVMInstruction::Eq => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value(if b.0 == a.0 { 1 } else { 0 }, "bool"));
                }
                QVMInstruction::Neq => todo!(),
                QVMInstruction::Lt => todo!(),
                QVMInstruction::Le => todo!(),
                QVMInstruction::And => todo!(),
                QVMInstruction::Or => todo!(),
                QVMInstruction::Not => todo!(),
                QVMInstruction::I32Const(c) => {
                    self.push(Value(c, "i32"));
                }
                QVMInstruction::Load(i) => {
                    assert_eq!(
                        self.stack[self.frame_pointer - 1].1,
                        "fp",
                        "{} at {:?}",
                        self.frame_pointer,
                        self.stack
                    );
                    let v = self.stack[self.frame_pointer + i];
                    assert_eq!(v.1, "i32");
                    self.push(v);
                }
                QVMInstruction::Store(r) => {
                    self.stack[self.stack_pointer - r] = self.pop();
                }
                QVMInstruction::Pop(r) => {
                    for _ in 0..r {
                        self.pop();
                    }
                }
                QVMInstruction::LoadArg(r) => {
                    let arg = self.stack[self.frame_pointer - 3 - r];
                    assert_eq!(arg.1, "i32");
                    self.push(arg);
                }
                QVMInstruction::LabelCall(_) => unreachable!(),
                QVMInstruction::JumpIfFalse(k) => {
                    let v = self.pop();
                    assert_eq!(v.1, "bool");
                    if v.0 == 0 {
                        self.pc = k;
                        continue;
                    }
                }
                QVMInstruction::Label(_) => todo!(),
                QVMInstruction::LabelJumpIfFalse(_) => todo!(),
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
            Call(3), // call main
            Return(0),
            // main:
            I32Const(1),  // a
            I32Const(10), // z
            Load(0),      // load a
            LoadArg(0),   // load b
            Add,          // a + b
            Return(1),    // return
        ],
        3,
    )];

    for (code, result) in cases {
        let mut runtime = Runtime::new(code, 0);
        runtime.run()?;
        assert_eq!(result, runtime.pop().0);
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
        assert_eq!(runtime.pop().0, result);
    }

    Ok(())
}
