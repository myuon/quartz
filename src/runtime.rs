use anyhow::{bail, Result};

use crate::{
    ast::Statement,
    code_gen::{HeapData, OpCode},
};

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum UnsizedDataType {
    Nil,
    String(String),
    Closure(Vec<String>, Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Nil,
    Int(i32),
    HeapAddr(usize),
    StackAddr(usize),
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
            StackAddr(_) => "stack_addr".to_string(),
            Tuple(u, _) => format!("tuple({})", u),
            Object(_) => "object".to_string(),
        }
    }
}

#[derive(Debug)]
struct Runtime {
    pc: usize,
    program: Vec<OpCode>,
    stack: Vec<DataType>,
    heap: Vec<HeapData>,
    call_stack: Vec<usize>,
    ffi_functions: Vec<FFIFunction>,
}

impl Runtime {
    pub fn new(program: Vec<OpCode>, ffi_functions: Vec<FFIFunction>) -> Runtime {
        Runtime {
            pc: 0,
            program,
            // 関数内部の実行時には先頭に関数へのアドレスが入っているという規約のため、main関数内ではmainへの関数ポインタを1つ置いておく(使うことはないのでnilにしておく)
            stack: vec![],
            heap: vec![],
            call_stack: vec![],
            ffi_functions,
        }
    }

    fn push(&mut self, val: DataType) -> usize {
        let u = self.stack.len();
        self.stack.push(val);
        u
    }

    fn pop(&mut self, n: usize) -> DataType {
        let mut result = DataType::Nil;
        for _ in 0..n {
            result = self.stack.pop().unwrap();
        }

        result
    }

    // TODO: 空いてるところを探すようにする
    fn alloc(&mut self, val: HeapData) -> DataType {
        let p = self.heap.len();
        self.heap.push(val);
        DataType::HeapAddr(p)
    }

    fn free(&mut self, pointer: DataType) -> Result<()> {
        match pointer {
            DataType::HeapAddr(p) => {
                if p < self.heap.len() && !matches!(self.heap[p], HeapData::Nil) {
                    self.heap[p] = HeapData::Nil;

                    Ok(())
                } else {
                    bail!("Failed to free this object");
                }
            }
            _ => {
                bail!("Expected HeapAddr but found {:?}", pointer);
            }
        }
    }

    fn deref(&mut self, pointer: DataType) -> Result<HeapData> {
        match pointer {
            DataType::HeapAddr(p) => Ok(self.heap[p].clone()),
            _ => bail!("Expected pointer but found {:?}", pointer),
        }
    }

    fn is_end(&self) -> bool {
        self.pc == self.program.len()
    }

    pub fn execute(&mut self) -> Result<()> {
        while !self.is_end() {
            if option_env!("DEBUG") == Some("true") {
                println!(
                    "{:?}\n{:?}\n{:?}\n",
                    &self.program[self.pc..],
                    self.stack,
                    self.heap
                );
            }

            match self.program[self.pc].clone() {
                OpCode::Push(v) => {
                    self.push(v);
                }
                OpCode::Return(r) => {
                    if r > 0 {
                        let ret = self.pop(1);
                        self.pop(r - 1);
                        self.push(ret);
                    }

                    match self.call_stack.pop() {
                        Some(p) => {
                            self.pc = p + 1;
                            continue;
                        }
                        _ => return Ok(()),
                    }
                }
                OpCode::Copy(p) => {
                    let target = self.stack.len() - 1 - p;
                    self.push(self.stack[target].clone());
                }
                OpCode::Alloc(h) => {
                    let p = self.alloc(h);
                    self.push(p);
                }
                OpCode::FFICall(addr) => {
                    self.stack = self.ffi_functions[addr](self.stack.clone(), self.heap.clone());
                    self.pc += 1;
                    continue;
                }
                OpCode::Call(addr) => {
                    let closure = self.deref(self.stack[addr].clone())?;
                    match closure {
                        HeapData::Closure(body) => {
                            self.call_stack.push(self.pc);

                            self.pc = self.program.len();
                            self.program.extend(body);
                            continue;
                        }
                        r => bail!("Expected a closure but found {:?}", r),
                    }
                }
                OpCode::PAssign => {
                    let val = self.pop(1);
                    let pointer = self.pop(1);
                    match pointer {
                        DataType::StackAddr(p) => {
                            self.stack[p] = val;
                        }
                        r => bail!("Expected stack address but found {:?}", r),
                    }
                }
                OpCode::Free => {
                    let val = self.pop(1);
                    self.free(val)?;
                }
                OpCode::Deref => {
                    let addr = self.pop(1);
                    match addr {
                        DataType::StackAddr(p) => {
                            self.push(self.stack[p].clone());
                        }
                        r => bail!("Expected stack address but found {:?}", r),
                    }
                }
                OpCode::Tuple(u) => {
                    let mut tuple = vec![];
                    for _ in 0..u {
                        tuple.push(self.pop(1));
                    }

                    self.push(DataType::Tuple(u, tuple));
                }
                OpCode::Object(u) => {
                    let mut object = vec![];
                    for _ in 0..u {
                        let value = self.pop(1);
                        let key = self.pop(1);
                        match self.deref(key)? {
                            HeapData::String(s) => {
                                object.push((s, value));
                            }
                            k => bail!("Unexpected key type: {:?}", k),
                        }
                    }

                    self.push(DataType::Object(object));
                }
                OpCode::Get => {
                    let index = self.pop(1);
                    let tuple = self.pop(1);
                    match (tuple, index) {
                        (DataType::Tuple(_, vs), DataType::Int(n)) => {
                            self.push(vs[n as usize].clone());
                        }
                        (DataType::Object(vs), index) => match self.deref(index)? {
                            HeapData::String(key) => {
                                let mut value = None;
                                for (k, v) in vs.clone() {
                                    if k == key {
                                        value = Some(v);
                                        break;
                                    }
                                }

                                match value {
                                    Some(v) => {
                                        self.push(v.clone());
                                    }
                                    None => bail!("Cannot found key {} in {:?}", key, vs),
                                }
                            }
                            x => bail!("Key must be string but found {:?}", x),
                        },
                        (i, t) => bail!("Unexpected type: _get({:?}, {:?})", t, i),
                    }
                }
            }

            self.pc += 1;
        }

        Ok(())
    }
}

pub fn execute(program: Vec<OpCode>, ffi_functions: Vec<FFIFunction>) -> Result<DataType> {
    let mut runtime = Runtime::new(program, ffi_functions);
    runtime.execute()?;

    Ok(runtime.pop(1))
}

pub type FFIFunction = Box<fn(Vec<DataType>, Vec<HeapData>) -> Vec<DataType>>;

#[cfg(test)]
mod tests {
    use crate::{code_gen::gen_code, create_ffi_table, parser::run_parser};

    use super::*;

    #[test]
    fn test_runtime() {
        let cases = vec![
            (
                r#"let x = 10; let y = x; let z = y; return z;"#,
                DataType::Int(10),
            ),
            (
                r#"let x = 10; let f = fn (a,b) { return a; }; return f(x,x);"#,
                DataType::Int(10),
            ),
            (
                // shadowingが起こる例
                r#"let x = 10; let f = fn (x) { let x = 5; return x; }; f(x); return x;"#,
                DataType::Int(10),
            ),
            (
                // 日本語
                r#"let u = "こんにちは、世界"; return u;"#,
                DataType::HeapAddr(0),
            ),
            (
                // early return
                r#"return 1; return 2; return 3;"#,
                DataType::Int(1),
            ),
            (
                // using ffi table
                r#"return _add(1,2);"#,
                DataType::Int(3),
            ),
            (
                // no return function
                r#"1; 2; 3;"#,
                DataType::Nil,
            ),
            (
                // ローカル変数や関数の引数のrefをとることはdangling pointerを生成するのでやってはいけない
                // refで渡ってきたものをderefするのは安全
                r#"let x = 10; let f = fn (a) { return *a; }; return f(&x);"#,
                DataType::Int(10),
            ),
            (
                r#"
                    let x = 10;
                    let f = fn (p) {
                        _passign(p, 30);
                    };
                    f(&x);
                    return x;
                "#,
                DataType::Int(30),
            ),
            (
                r#"
                    let f = fn (a,b) {
                        return _add(a,b);
                    };
                    return f(1,2);
                "#,
                DataType::Int(3),
            ),
            (
                r#"
                    let t = _tuple(1, 2, 3, 4, 5);
                    return _get(t, 2);
                "#,
                DataType::Int(3),
            ),
            (
                r#"
                    let t = _object("x", 1, "y", "hell");
                    return _get(t, "x");
                "#,
                DataType::Int(1),
            ),
            (
                r#"
                    let new = fn () {
                        return _object("x", 1);
                    };
                    let x = new();
                    return _get(x, "x");
                "#,
                DataType::Int(1),
            ),
        ];

        let (ffi_table, ffi_functions) = create_ffi_table();

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let code = gen_code(m, ffi_table.clone()).unwrap();
            let result = execute(code, ffi_functions.clone()).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }

    #[test]
    fn test_interpreter_stack() {
        use OpCode::*;

        let cases = vec![
            (
                r#"
                    let f = fn () {
                        let y = 1;
                        let z = 2;
                        return _add(y,z);
                    };
                    return f();
                "#,
                vec![DataType::Int(3)],
                vec![HeapData::Closure(vec![
                    Push(DataType::Int(1)),
                    Push(DataType::Int(2)),
                    Copy(1),
                    Copy(1),
                    FFICall(0),
                    Return(3),
                ])],
            ),
            (
                // no return functionでもローカル変数は全てpopされること
                r#"let x = 1; x;"#,
                vec![DataType::Nil],
                vec![],
            ),
            (
                // just panic
                r#"let x = 1; panic;"#,
                vec![DataType::Int(1)],
                vec![],
            ),
            (
                // 関数呼び出しの際には引数がstackに積まれ、その後returnするときにそれらがpopされて値が返却される
                r#"
                    let f = fn (a,b,c,d,e) { return "hello"; };
                    return f(1,2,3,4,5);
                "#,
                vec![DataType::HeapAddr(1)],
                vec![
                    HeapData::Closure(vec![
                        Alloc(HeapData::String("hello".to_string())),
                        Return(6),
                    ]),
                    HeapData::String("hello".to_string()),
                ],
            ),
            (
                // take the address of string
                r#"let x = "hello, world"; let y = &x; panic;"#,
                vec![DataType::HeapAddr(0), DataType::StackAddr(0)],
                vec![HeapData::String("hello, world".to_string())],
            ),
            (
                r#"let x = "hello, world"; _free(x); panic;"#,
                vec![DataType::HeapAddr(0), DataType::Nil],
                vec![HeapData::Nil],
            ),
            (
                r#"
                    let x = "";
                    let f = fn (p) {
                        _passign(p, "hello, world");
                    };
                    f(&x);
                    return x;
                "#,
                vec![DataType::HeapAddr(2)],
                vec![
                    HeapData::String("".to_string()),
                    HeapData::Closure(vec![
                        Copy(0),
                        Alloc(HeapData::String("hello, world".to_string())),
                        PAssign,
                        Push(DataType::Nil),
                        Push(DataType::Nil),
                        Return(3),
                    ]),
                    HeapData::String("hello, world".to_string()),
                ],
            ),
        ];

        let (ffi_table, ffi_functions) = create_ffi_table();

        for case in cases {
            let m = run_parser(case.0).unwrap();
            let program = gen_code(m, ffi_table.clone()).unwrap();
            let mut interpreter = Runtime::new(program, ffi_functions.clone());
            interpreter.execute().unwrap();
            assert_eq!(interpreter.stack, case.1);
            assert_eq!(interpreter.heap, case.2);
        }
    }
}
