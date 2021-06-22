use std::collections::HashMap;

use anyhow::{bail, Result};
use regex::Regex;

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

#[derive(Debug)]
struct Runtime {
    pc: usize,
    program: Vec<OpCode>,
    stack: Vec<DataType>,
    heap: Vec<HeapData>,
    call_stack: Vec<usize>,
    ffi_functions: Vec<FFIFunction>,
    labels: HashMap<String, usize>,
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
            labels: HashMap::new(),
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

    fn get_stack_addr_index(&self, u: usize) -> usize {
        self.stack.len() - 1 - u
    }

    fn stack_addr_as_mut(&mut self, datatype: DataType) -> Result<&mut DataType> {
        match datatype {
            DataType::StackRevAddr(r) => {
                let r = self.stack.len() - 1 - r;
                Ok(&mut self.stack[r])
            }
            DataType::StackNormalAddr(r) => Ok(&mut self.stack[r]),
            v => bail!("Expected a stack address but found {:?}", v),
        }
    }

    fn get_stack_addr(&self, u: usize) -> &DataType {
        let index = self.get_stack_addr_index(u);
        &self.stack[index]
    }

    fn expect_int(&self, datatype: DataType) -> Result<i32> {
        match datatype {
            DataType::Int(s) => return Ok(s),
            v => bail!("Expected a int but found {:?}", v),
        }
    }

    fn expect_string(&self, datatype: DataType) -> Result<String> {
        match &self.heap[self.expect_heap_addr(datatype)?] {
            HeapData::String(s) => return Ok(s.to_string()),
            v => bail!("Expected a string but found {:?}", v),
        }
    }

    fn expect_heap_addr(&self, datatype: DataType) -> Result<usize> {
        match datatype {
            DataType::HeapAddr(h) => Ok(h),
            v => bail!("Expected heap address but found {:?}", v),
        }
    }

    fn expect_stack_index(&self, datatype: DataType) -> Result<usize> {
        match datatype {
            DataType::StackRevAddr(h) => Ok(self.get_stack_addr_index(h)),
            DataType::StackNormalAddr(h) => Ok(h),
            v => bail!("Expected stack address but found {:?}", v),
        }
    }

    fn expect_vector(&self, datatype: DataType) -> Result<Vec<DataType>> {
        let h = self.expect_heap_addr(datatype)?;
        match &self.heap[h] {
            HeapData::Vec(v) => Ok(v.clone()),
            v => bail!("Expected vector but found {:?}", v),
        }
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
                OpCode::Pop(v) => {
                    self.pop(v);
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
                OpCode::ReturnIf(r) => {
                    let cond = self.pop(1);
                    let n = self.expect_int(cond)?;
                    let ret = self.pop(1);

                    // if true
                    if n == 0 {
                        if r > 0 {
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
                }
                OpCode::Copy(p) => {
                    let target = self.stack.len() - 1 - p;
                    match self.stack[target].clone() {
                        DataType::StackRevAddr(addr) => {
                            self.push(DataType::StackRevAddr(addr + p + 1));
                        }
                        s => {
                            self.push(s);
                        }
                    }
                }
                OpCode::CopyAbsolute(p) => match self.stack[p].clone() {
                    DataType::StackRevAddr(r) => {
                        // ローカルでない変数がStackRevAddrだった場合はポインタ値を再計算する必要がある
                        self.push(DataType::StackNormalAddr(p - r - 1));
                    }
                    t => {
                        self.push(t);
                    }
                },
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
                    let closure = self.deref(self.stack[self.stack.len() - 1 - addr].clone())?;
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
                OpCode::CallAbsolute(addr) => {
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
                    *self.stack_addr_as_mut(pointer)? = val;
                }
                OpCode::Free => {
                    let val = self.pop(1);
                    self.free(val)?;
                }
                OpCode::Deref => {
                    let addr = self.pop(1);
                    let val = self.stack_addr_as_mut(addr)?.clone();
                    self.push(val);
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

                    if let Ok(vec) = self.expect_vector(tuple.clone()) {
                        let index = self.expect_int(index)?;
                        self.push(vec[index as usize].clone());
                        self.pc += 1;
                        continue;
                    }

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
                                    None => bail!("Cannot find key {} in {:?}", key, vs),
                                }
                            }
                            x => bail!("Key must be string but found {:?}", x),
                        },
                        (t, i) => bail!("Unexpected type: _get({:?}, {:?})", t, i),
                    }
                }
                OpCode::Set => {
                    let value = self.pop(1);
                    let key = self.pop(1);
                    let obj = self.pop(1);

                    match obj {
                        DataType::StackRevAddr(addr) => match self.get_stack_addr(addr).clone() {
                            DataType::Object(mut vs) => {
                                let key = self.expect_string(key)?;
                                let updated = (move || {
                                    for (i, (k, _)) in vs.clone().into_iter().enumerate() {
                                        if k == key {
                                            vs[i] = (k, value);
                                            return Ok(vs);
                                        }
                                    }

                                    bail!("Cannot find key {} in {:?}", key, vs);
                                })()?;

                                self.stack[addr] = DataType::Object(updated);
                            }
                            t => bail!("Expected object but found {:?}", t),
                        },
                        t => bail!("Expected stack address but found {:?}", t),
                    }
                }
                OpCode::Regex => {
                    let arg0 = self.pop(1);
                    let arg1 = self.pop(1);
                    let input = self.expect_string(arg0)?;
                    let pattern = self.expect_string(arg1)?;
                    let m = Regex::new(&pattern)?.find(&input);
                    match m {
                        Some(m) => {
                            self.push(DataType::Tuple(
                                2,
                                vec![
                                    DataType::Int(m.start() as i32),
                                    DataType::Int(m.end() as i32),
                                ],
                            ));
                        }
                        None => {
                            self.push(DataType::Nil);
                        }
                    }
                }
                OpCode::Switch(u) => {
                    let mut args = vec![];
                    for _ in 0..u {
                        args.push(self.pop(1));
                    }
                    args.reverse();

                    let cond = self.pop(1);
                    let cond_case = self.expect_int(cond)?;

                    self.push(args[cond_case as usize].clone());
                }
                OpCode::VPush => {
                    let value = self.pop(1);
                    let vec = self.pop(1);

                    let addr = self.expect_stack_index(vec)?;
                    let addr = self.expect_heap_addr(self.stack[addr].clone())?;
                    match &mut self.heap[addr] {
                        HeapData::Vec(vs) => {
                            vs.push(value);
                        }
                        t => bail!("Expected vector but found {:?}", t),
                    }
                }
                OpCode::Len => {
                    let v = self.pop(1);
                    match self.deref(v)? {
                        HeapData::Vec(vs) => {
                            self.push(DataType::Int(vs.len() as i32));
                        }
                        HeapData::String(vs) => {
                            self.push(DataType::Int(vs.len() as i32));
                        }
                        t => bail!("Expected vector but found {:?}", t),
                    }
                }
                OpCode::Loop => todo!(),
                OpCode::Label(s) => {
                    self.labels.insert(s, self.pc);
                }
                OpCode::Jump(u) => {
                    self.pc = self.labels.get(&u).unwrap().clone();
                    continue;
                }
                OpCode::Slice => {
                    let end = self.pop(1);
                    let end = self.expect_int(end)? as usize;

                    let start = self.pop(1);
                    let start = self.expect_int(start)? as usize;

                    let input_addr = self.pop(1);
                    let input_addr = self.expect_heap_addr(input_addr)?;

                    match self.heap[input_addr].clone() {
                        HeapData::String(input) => {
                            let d = self.alloc(HeapData::String((input[start..end]).to_string()));
                            self.push(d);
                        }
                        s => bail!("Expected string but {:?} found", s),
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
            (
                r#"let x = _object("x", 10, "y", "yes"); _set(&x, "y", 20); return _get(x, "y");"#,
                DataType::Int(20),
            ),
            (
                r#"let ch = _regex("^[a-zA-Z_][a-zA-Z0-9_]*", "abcABC9192_"); return ch;"#,
                DataType::Tuple(2, vec![DataType::Int(0), DataType::Int(11)]),
            ),
            (
                r#"
                    let x = 10;
                    let f = _switch(
                        0,
                        fn () { return 0; },
                        fn () { return 1; },
                    );
                    return f();
                "#,
                DataType::Int(0),
            ),
            (
                // 0 = true, 1 = false
                r#"return _eq(1, 2);"#,
                DataType::Int(1),
            ),
            (
                // _passign for local variables
                r#"let x = 0; _passign(&x, _add(x, 10)); return x;"#,
                DataType::Int(10),
            ),
            (
                // _passign for local variables in a function
                r#"
                    let f = fn () {
                        let x = 0;
                        _passign(&x, _add(x, 1));

                        return x;
                    };
                    return f();
                "#,
                DataType::Int(1),
            ),
            (
                // vector
                r#"
                    let v = _vec();
                    _vpush(&v, 10);
                    _vpush(&v, 20);
                    _vpush(&v, 30);

                    return _tuple(_len(v), _get(v, 1));
                "#,
                DataType::Tuple(2, vec![DataType::Int(20), DataType::Int(3)]),
            ),
            (
                r#"
                    let fib = fn (n) {
                        let a = 1;
                        let b = 1;
                        let count = 0;

                        loop {
                            return b if _eq(count, n);

                            let next = _add(a, b);
                            _passign(&a, b);
                            _passign(&b, next);
                            _passign(&count, _add(count, 1));
                        };
                    };

                    return fib(5);
                "#,
                DataType::Int(13),
            ),
            (
                r#"return _eq(_slice("hello, world", 5, 10), ", wor");"#,
                DataType::Int(0),
            ),
            (
                r#"
                    let x = 0;
                    let f = fn () {
                        return _add(x, 1);
                    };
                    let y = 0;

                    return f();
                "#,
                DataType::Int(1),
            ),
            (
                r#"
                    let x = 0;
                    let f = fn () {
                        _passign(&x, _add(x, 1));
                        return _add(x, 1);
                    };
                    let y = 0;

                    return f();
                "#,
                DataType::Int(2),
            ),
            (
                r#"
                    let x = 10;
                    let y = &x;
                    let f = fn () {
                        return _add(*y, 1);
                    };
                    let z = 0;

                    return f();
                "#,
                DataType::Int(11),
            ),
            /*
            // dangling variableの例
            // closureが実装できていないのでこのようにローカル変数のライフタイムを関数の外に広げようとすると壊れるので注意
            (
                r#"
                    let f = fn () {
                        let x = 20;
                        let say_name = fn () {
                            return x;
                        };
                        return say_name;
                    };
                    let g = f();

                    return g();
                "#,
                DataType::Int(11),
            ),
            */
            (
                r#"
                    let f = fn (input) {
                        let t = fn () {
                            return 10;
                        };
                        t();
                    };
                    
                    let g = fn (input) {
                        return f(input);
                    };
                    
                    g(0);
                "#,
                DataType::Nil,
            ),
        ];

        let (ffi_table, ffi_functions) = create_ffi_table();

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let code = gen_code(m, ffi_table.clone()).unwrap();
            let result = {
                let r = execute(code, ffi_functions.clone());
                assert!(r.is_ok(), "Result: {:?}, input: {:?}", r, c.0);
                r.unwrap()
            };
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }

    #[test]
    fn test_runtime_with_env() {
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
                vec![DataType::HeapAddr(0), DataType::StackRevAddr(0)],
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
