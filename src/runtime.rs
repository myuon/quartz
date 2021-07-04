use std::collections::HashMap;

use anyhow::{bail, Result};
use regex::Regex;

use crate::vm::{DataType, HeapData, OpCode, StackData};

#[derive(Debug)]
struct Runtime {
    pc: usize,
    program: Vec<OpCode>,
    stack: Vec<StackData>,
    heap: Vec<HeapData>,
    static_area: Vec<HeapData>,
    call_stack: Vec<usize>,
    ffi_functions: Vec<FFIFunction>,
    labels: HashMap<String, usize>,
}

impl Runtime {
    pub fn new(
        program: Vec<OpCode>,
        static_area: Vec<HeapData>,
        ffi_functions: Vec<FFIFunction>,
    ) -> Runtime {
        Runtime {
            pc: 0,
            program,
            stack: vec![],
            heap: vec![],
            static_area,
            call_stack: vec![],
            ffi_functions,
            labels: HashMap::new(),
        }
    }

    fn show_error(&self) -> String {
        format!(
            "= stack =\n{}\n= program =\n{}\n",
            self.show_error_in_stack(),
            self.show_error_in_program()
        )
    }

    fn show_error_in_stack(&self) -> String {
        self.stack
            .iter()
            .enumerate()
            .map(|(i, v)| {
                format!(
                    "{} | {:?}{}\n",
                    self.stack.len() - 1 - i,
                    v,
                    v.as_heap_addr()
                        .map(|p| format!("  => {:?}", self.heap[p]))
                        .unwrap_or(String::new())
                )
            })
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .concat()
    }

    fn show_error_in_program(&self) -> String {
        let pc = self.pc;
        let line_no_starts_at = pc.saturating_sub(10);
        self.program[line_no_starts_at..(pc + 10).min(self.program.len())]
            .iter()
            .enumerate()
            .map(|(i, v)| {
                format!(
                    "{}{:?}\n",
                    if pc == i + line_no_starts_at {
                        format!("{:?} | ", pc)
                    } else {
                        " ".repeat(format!("{:?}", pc).len() + 3)
                    },
                    v,
                )
            })
            .collect::<Vec<_>>()
            .concat()
    }

    fn push(&mut self, val: StackData) -> usize {
        let u = self.stack.len();
        self.stack.push(val);
        u
    }

    fn pop(&mut self, n: usize) -> StackData {
        let mut result = StackData::Nil;
        for _ in 0..n {
            result = self.stack.pop().unwrap();
        }

        result
    }

    // TODO: 空いてるところを探すようにする
    fn alloc(&mut self, val: HeapData) -> StackData {
        let p = self.heap.len();
        self.heap.push(val);
        StackData::HeapAddr(p)
    }

    fn free(&mut self, pointer: StackData) -> Result<()> {
        match pointer {
            StackData::HeapAddr(p) => {
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

    fn deref_heap_data(&mut self, pointer: HeapData) -> Result<DataType> {
        match pointer {
            HeapData::Nil => Ok(DataType::Nil),
            HeapData::Bool(b) => Ok(DataType::Bool(b)),
            HeapData::Int(s) => Ok(DataType::Int(s)),
            HeapData::String(s) => Ok(DataType::String(s)),
            HeapData::Object(obj) => Ok(DataType::Object(obj)),
            HeapData::Tuple(_, obj) => Ok(DataType::Tuple(obj)),
            HeapData::Vec(obj) => Ok(DataType::Vec(obj)),
            HeapData::HeapAddr(p) => self.deref_heap_data(self.heap[p].clone()),
            v => panic!("Failed to deref: {:?}", v),
        }
    }

    fn deref(&mut self, pointer: StackData) -> Result<DataType> {
        match pointer {
            StackData::Nil => Ok(DataType::Nil),
            StackData::Bool(b) => Ok(DataType::Bool(b)),
            StackData::Int(s) => Ok(DataType::Int(s)),
            StackData::StackAddr(s) => self.deref(self.stack[s].clone()),
            StackData::HeapAddr(p) => self.deref_heap_data(self.heap[p].clone()),
            StackData::StaticAddr(s) => self.deref_heap_data(self.static_area[s].clone()),
        }
    }

    fn is_end(&self) -> bool {
        self.pc == self.program.len()
    }

    fn get_stack_addr_index(&self, u: usize) -> usize {
        self.stack.len() - 1 - u
    }

    fn expect_bool(&mut self, datatype: StackData) -> Result<bool> {
        match datatype {
            StackData::Bool(s) => return Ok(s),
            v => bail!("Expected a bool but found {:?}", v),
        }
    }

    fn expect_int(&mut self, datatype: StackData) -> Result<i32> {
        match datatype {
            StackData::Int(s) => return Ok(s),
            v => bail!("Expected a int but found {:?}", v),
        }
    }

    fn expect_string(&self, datatype: StackData) -> Result<String> {
        match &self.heap[self.expect_heap_addr(datatype)?] {
            HeapData::String(s) => return Ok(s.to_string()),
            v => bail!("Expected a string but found {:?}", v),
        }
    }

    fn expect_heap_addr(&self, datatype: StackData) -> Result<usize> {
        match datatype {
            StackData::HeapAddr(h) => Ok(h),
            v => bail!("Expected heap address but found {:?}", v),
        }
    }

    fn expect_vector(&self, datatype: StackData) -> Result<Vec<StackData>> {
        let h = self.expect_heap_addr(datatype)?;
        match &self.heap[h] {
            HeapData::Vec(v) => Ok(v.clone()),
            v => bail!("Expected vector but found {:?}", v),
        }
    }

    fn find_label_forward(&mut self, label: String) -> Result<usize> {
        let mut pc = self.pc;
        while pc < self.program.len() {
            if self.program[pc] == OpCode::Label(label.clone()) {
                return Ok(pc);
            }
            pc += 1;
        }

        bail!("Label {} not found", label);
    }

    pub fn execute(&mut self) -> Result<()> {
        while !self.is_end() {
            if option_env!("DEBUG") == Some("true") {
                println!(
                    "{:?}\n{:?}\n{:?}\n",
                    &self.program[self.pc..],
                    self.stack.iter().rev().collect::<Vec<_>>(),
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
                    let n = self.expect_bool(cond)?;
                    let ret = self.pop(1);

                    // if true
                    if n {
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
                        StackData::StackAddr(addr) => {
                            self.push(StackData::StackAddr(addr + p + 1));
                        }
                        s => {
                            self.push(s);
                        }
                    }
                }
                OpCode::Alloc(h) => {
                    let p = self.alloc(h);
                    self.push(p);
                }
                OpCode::FFICall(addr) => {
                    let (x, y) = self.ffi_functions[addr](self.stack.clone(), self.heap.clone());
                    self.stack = x;
                    self.heap = y;
                    self.pc += 1;
                    continue;
                }
                OpCode::Call(addr) => match self.static_area[addr].clone() {
                    HeapData::Closure(body) => {
                        self.call_stack.push(self.pc);

                        self.pc = self.program.len();
                        self.program.extend(body);
                        continue;
                    }
                    r => bail!(
                        "Expected a closure but found {:?} at {:?}\n\n{}",
                        r,
                        self.pc,
                        self.show_error()
                    ),
                },
                OpCode::PAssign => {
                    let val = self.pop(1);
                    let addr = self.pop(1);

                    match addr {
                        StackData::StackAddr(p) => {
                            let u = self.get_stack_addr_index(p);
                            self.stack[u] = val;
                        }
                        StackData::StaticAddr(p) => {
                            self.static_area[p] = val.clone().into_heap_data().ok_or(
                                anyhow::anyhow!("Cannot place the value on heap: {:?}", val),
                            )?;
                        }
                        v => bail!("Cannot deref: {:?}", v),
                    }
                }
                OpCode::Free => {
                    let val = self.pop(1);
                    self.free(val)?;
                }
                OpCode::Deref => {
                    let addr = self.pop(1);

                    match addr {
                        StackData::StackAddr(p) => {
                            let u = self.get_stack_addr_index(p);
                            self.push(self.stack[u].clone());
                        }
                        StackData::StaticAddr(p) => {
                            self.push(self.static_area[p].clone().as_stack_data().ok_or(
                                anyhow::anyhow!(
                                    "Cannot place the value on stack: {:?}",
                                    self.static_area[p]
                                ),
                            )?);
                        }
                        v => bail!("Cannot deref: {:?}", v),
                    }
                }
                OpCode::Tuple(u) => {
                    let mut tuple = vec![];
                    for _ in 0..u {
                        tuple.push(self.pop(1));
                    }

                    let r = self.alloc(HeapData::Tuple(u, tuple));
                    self.push(r);
                }
                OpCode::Object(u) => {
                    let mut object = vec![];
                    for _ in 0..u {
                        let value = self.pop(1);
                        let key = self.pop(1);
                        match self.deref(key)? {
                            DataType::String(s) => {
                                object.push((s, value));
                            }
                            k => bail!("Unexpected key type: {:?}", k),
                        }
                    }

                    let p = self.alloc(HeapData::Object(object));
                    self.push(p);
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

                    match (self.deref(tuple)?, self.deref(index)?) {
                        (DataType::Tuple(vs), DataType::Int(n)) => {
                            self.push(vs[n as usize].clone());
                        }
                        (DataType::Object(vs), DataType::String(key)) => {
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
                                None => bail!(
                                    "Cannot find key {} in {:?}\n{}",
                                    key,
                                    vs,
                                    self.show_error()
                                ),
                            }
                        }
                        (DataType::Vec(vs), DataType::Int(n)) => {
                            self.push(vs[n as usize].clone());
                        }
                        (t, i) => bail!("Unexpected type: _get({:?}, {:?})", t, i),
                    }
                }
                OpCode::Set => {
                    let value = self.pop(1);
                    let key = self.pop(1);
                    let obj = self.pop(1);

                    match obj {
                        StackData::HeapAddr(pointer) => match self.heap[pointer].clone() {
                            HeapData::Object(mut vs) => {
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

                                self.heap[pointer] = HeapData::Object(updated);
                            }
                            t => bail!("Expected object but found {:?}", t),
                        },
                        StackData::StaticAddr(addr) => match self.static_area[addr].clone() {
                            HeapData::HeapAddr(pointer) => match self.heap[pointer].clone() {
                                HeapData::Object(mut vs) => {
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

                                    self.heap[pointer] = HeapData::Object(updated);
                                }
                                t => bail!("Expected object but found {:?}", t),
                            },
                            t => bail!("Expected pointer but found {:?}", t),
                        },
                        t => bail!(
                            "Expected stack or static address but found {:?}\n{}",
                            t,
                            self.show_error()
                        ),
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
                            let start = StackData::Int(m.start() as i32);
                            let end = StackData::Int(m.end() as i32);
                            let p = self.alloc(HeapData::Tuple(2, vec![start, end]));
                            self.push(p);
                        }
                        None => {
                            self.push(StackData::Nil);
                        }
                    }
                }
                OpCode::VPush => {
                    let value = self.pop(1);
                    let vec = self.pop(1);

                    match vec {
                        StackData::StaticAddr(addr) => match &mut self.static_area[addr] {
                            HeapData::Vec(vs) => {
                                vs.push(value);
                            }
                            HeapData::HeapAddr(pointer) => match &mut self.heap[*pointer] {
                                HeapData::Vec(vs) => {
                                    vs.push(value);
                                }
                                t => bail!("Expected vector but found {:?}", t),
                            },
                            t => bail!("Expected vector but found {:?}", t),
                        },
                        StackData::HeapAddr(addr) => match &mut self.heap[addr] {
                            HeapData::Vec(vs) => {
                                vs.push(value);
                            }
                            t => bail!("Expected vector but found {:?}", t),
                        },
                        t => bail!("TypeError {:?}", t),
                    }
                }
                OpCode::Len => {
                    let v = self.pop(1);
                    match self.deref(v)? {
                        DataType::Vec(vs) => {
                            self.push(StackData::Int(vs.len() as i32));
                        }
                        DataType::String(vs) => {
                            self.push(StackData::Int(vs.len() as i32));
                        }
                        t => bail!("Expected vector but found {:?}", t),
                    }
                }
                OpCode::Label(s) => {
                    self.labels.insert(s, self.pc);
                }
                OpCode::Jump(u) => {
                    if self.labels.contains_key(&u) {
                        self.pc = self.labels[&u];
                        continue;
                    } else {
                        self.pc = self.find_label_forward(u).unwrap();
                    }
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
                OpCode::JumpIfNot(label) => {
                    let cond = self.pop(1);
                    let cond_case = self.expect_bool(cond)?;
                    if !cond_case {
                        self.pc = self.find_label_forward(label).unwrap();
                        continue;
                    }
                }
                OpCode::Panic => {
                    let msg = self.pop(1);
                    let msg = self.expect_string(msg)?;
                    println!("Panic: {}\n{}", msg, self.show_error());
                    return Ok(());
                }
                OpCode::SetStatic(addr) => {
                    let val = self.pop(1);

                    if let StackData::HeapAddr(h) = val {
                        self.static_area[addr] = HeapData::HeapAddr(h);
                    } else if let StackData::StaticAddr(h) = val {
                        self.static_area[addr] = HeapData::StaticAddr(h);
                    } else {
                        self.static_area[addr] = val
                            .clone()
                            .into_heap_data()
                            .ok_or(anyhow::anyhow!("Cannot place the value on heap: {:?}", val))?;
                    }
                }
                OpCode::CopyStatic(addr) => {
                    if let Some(d) = self.static_area[addr].clone().as_stack_data() {
                        self.push(d);
                    } else {
                        self.push(StackData::StaticAddr(addr));
                    }
                }
            }

            self.pc += 1;
        }

        Ok(())
    }
}

pub fn execute(
    program: Vec<OpCode>,
    static_area: Vec<HeapData>,
    ffi_functions: Vec<FFIFunction>,
) -> Result<StackData> {
    let mut runtime = Runtime::new(program, static_area, ffi_functions);
    runtime.execute()?;

    Ok(runtime.pop(1))
}

pub type FFIFunction = Box<fn(Vec<StackData>, Vec<HeapData>) -> (Vec<StackData>, Vec<HeapData>)>;

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
                DataType::String("こんにちは、世界".to_string()),
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
                r#"let x = _object("x", 10, "y", "yes"); _set(x, "y", 20); return _get(x, "y");"#,
                DataType::Int(20),
            ),
            (
                r#"let ch = _regex("^[a-zA-Z_][a-zA-Z0-9_]*", "abcABC9192_"); return _get(ch, 1);"#,
                DataType::Int(11),
            ),
            (r#"return _eq(1, 2);"#, DataType::Bool(false)),
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
                    _vpush(v, 10);
                    _vpush(v, 20);
                    _vpush(v, 30);

                    return _tuple(_len(v), _get(v, 1));
                "#,
                DataType::Tuple(vec![StackData::Int(20), StackData::Int(3)]),
            ),
            (
                r#"
                    let fib = fn (n) {
                        let a = 1;
                        let b = 1;
                        let count = 0;

                        loop {
                            if _eq(count, n) {
                                return b;
                            };

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
                DataType::Bool(true),
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
            (
                // A dangling variable example for a closure
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
                StackData::HeapAddr(0),
                Some(HeapData::Int(11)),
            ),
             */
             /*
            (
                r#"
                    let f = fn (input) {
                        let t = fn () {
                            return 10;
                        };

                        return t();
                    };

                    let g = fn (input) {
                        return f(input);
                    };

                    return g(0);
                "#,
                StackData::HeapAddr(4),
                Some(HeapData::Int(10)),
            ),
             */
            /*
            (
                r#"
                    let f = fn (x) {
                        let t = fn () {
                            return _len(x);
                        };
                        return t();
                    };
                    let z = 20;
                    return f("aaa");
                "#,
                StackData::HeapAddr(4),
                Some(HeapData::Int(3)),
            ),
             */
            (
                // if statement
                r#"
                    let f = fn (x) {
                        if _eq(x, 10) {
                            return 1;
                        } else {
                            return 0;
                        };
                    };
                    return f(10);
                "#,
                DataType::Int(1),
            ),
            (
                // if statement
                r#"
                    let f = fn (x) {
                        if _eq(x, 10) {
                            return 1;
                        } else {
                            return 0;
                        };
                    };
                    return f(11);
                "#,
                DataType::Int(0),
            ),
            (
                r#"
                    let assign_a = fn (obj, x) {
                        _set(*obj, "a", x);
                    };

                    let object = _object(
                        "a", 10,
                    );
                    assign_a(&object, 20);

                    if false {};

                    assign_a(&object, 30);

                    return _get(object, "a");
                "#,
                DataType::Int(30),
            ),
            (
                r#"
                    let assign_a = fn (obj, x) {
                        _set(*obj, "a", x);
                    };

                    let main = fn () {
                        let object = _object(
                            "a", 10,
                        );
                        assign_a(&object, 20);
    
                        return _get(object, "a");
                    };

                    return main();
                "#,
                DataType::Int(20),
            ),
            (
                r#"
                    let f = fn (a,b,c) {
                        if true {
                            return c;
                        };

                        return 0;
                    };

                    return f(1,2,3);
                "#,
                DataType::Int(3),
            ),
            (
                r#"
                    let c = 0;

                    loop {
                        if _eq(c, 10) {
                            return nil;
                        };

                        let a = 10;
                        let b = "hello";

                        _passign(&c, _add(c, 1));

                        continue;
                    };
                "#,
                DataType::Nil,
            ),
            (
                r#"
                    let f = fn (a) {
                        _passign(a, 10);
                    };

                    let c = 0;
                    let a = 0;

                    loop {
                        if _eq(c, 10) {
                            return nil;
                        };

                        f(&a);

                        _passign(&c, _add(c, 1));
                    };
                "#,
                DataType::Nil,
            ),
            (
                r#"
                    let f = fn () {
                        let a = 10;
                        let b = "yes";
    
                        if _eq(a, 10) {
                            return 0;
                        };
    
                        return 1;
                    };

                    return f();
                "#,
                DataType::Nil,
            ),
        ];

        let (ffi_table, ffi_functions) = create_ffi_table();

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let program = gen_code(m, ffi_table.clone());
            assert!(program.is_ok(), "{:?} {}", program, c.0);

            let (program, static_area) = program.unwrap();

            let mut runtime = Runtime::new(program, static_area, ffi_functions.clone());
            let result = runtime.execute();
            assert!(result.is_ok(), "{:?} {}", result, c.0);

            let result = runtime.pop(1);
            assert_eq!(runtime.deref(result).unwrap(), c.1, "{}", c.0);
        }
    }

    #[test]
    fn test_runtime_with_env() {
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
                vec![StackData::Int(3)],
                vec![],
            ),
            (
                // no return functionでもローカル変数は全てpopされること
                r#"let x = 1; x;"#,
                vec![StackData::Nil],
                vec![],
            ),
            (
                // just panic
                r#"let x = 1; _panic("panic");"#,
                vec![],
                vec![HeapData::String("panic".to_string())],
            ),
            (
                // 関数呼び出しの際には引数がstackに積まれ、その後returnするときにそれらがpopされて値が返却される
                r#"
                    let f = fn (a,b,c,d,e) { return "hello"; };
                    return f(1,2,3,4,5);
                "#,
                vec![StackData::HeapAddr(0)],
                vec![HeapData::String("hello".to_string())],
            ),
            (
                // take the address of string
                r#"let x = "hello, world"; let y = &x; _panic("");"#,
                vec![],
                vec![
                    HeapData::String("hello, world".to_string()),
                    HeapData::String("".to_string()),
                ],
            ),
            (
                r#"let x = "hello, world"; _free(x); _panic("");"#,
                vec![],
                vec![HeapData::Nil, HeapData::String("".to_string())],
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
                vec![StackData::HeapAddr(1)],
                vec![
                    HeapData::String("".to_string()),
                    HeapData::String("hello, world".to_string()),
                ],
            ),
        ];

        let (ffi_table, ffi_functions) = create_ffi_table();

        for case in cases {
            let m = run_parser(case.0).unwrap();
            let (program, static_area) = gen_code(m, ffi_table.clone()).unwrap();
            let mut interpreter = Runtime::new(program, static_area, ffi_functions.clone());
            interpreter.execute().unwrap();
            assert_eq!(interpreter.stack, case.1, "{}", case.0);
            assert_eq!(interpreter.heap, case.2);
        }
    }
}
