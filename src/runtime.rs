use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    code_gen::{HeapData, OpCode},
};

#[derive(PartialEq, Debug, Clone)]
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
}

impl DataType {
    pub fn type_of(&self) -> &str {
        use DataType::*;

        match self {
            Nil => "nil",
            Int(_) => "int",
            HeapAddr(_) => "heap_addr",
            StackAddr(_) => "stack_addr",
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
    // 関数が引数より外側のスタック領域に勝手にアクセスしないようにするためのやつ
    stack_guard: usize,
}

impl Runtime {
    pub fn new(program: Vec<OpCode>) -> Runtime {
        Runtime {
            pc: 0,
            program,
            stack: vec![],
            heap: vec![],
            call_stack: vec![],
            stack_guard: 0,
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
            println!("{:?}\n{:?}\n", &self.program[self.pc..], self.stack);

            match self.program[self.pc].clone() {
                OpCode::Push(v) => {
                    self.push(v);
                }
                OpCode::Return(r) => {
                    let ret = self.pop(1);
                    self.pop(r);
                    self.push(ret);

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

                    ensure!(
                        self.stack_guard <= target,
                        "Detected invalid stack access: {} is out of stack guard address {} in {:?}",
                        p,
                        self.stack_guard,
                        self.stack,
                    );
                    self.push(self.stack[target].clone());
                }
                OpCode::Alloc(h) => {
                    let p = self.alloc(h);
                    self.push(p);
                }
                OpCode::Call(t) => {
                    let addr = self.stack[self.stack.len() - 1].clone();
                    let closure = self.deref(addr)?;
                    match closure {
                        HeapData::Closure(body) => {
                            self.call_stack.push(self.pc);

                            self.pc = self.program.len();
                            self.program.extend(body);
                            self.stack_guard = self.stack.len() - 1 - t;
                            continue;
                        }
                        r => bail!("Expected a closure but found {:?}", r),
                    }
                }
                OpCode::PAssign => todo!(),
                OpCode::Free => todo!(),
                OpCode::Deref => todo!(),
            }

            self.pc += 1;
        }

        Ok(())
    }
}

pub fn execute(program: Vec<OpCode>) -> Result<DataType> {
    let mut runtime = Runtime::new(program);
    runtime.execute()?;

    Ok(runtime.pop(1))
}

pub type FFITable = HashMap<String, Box<fn(Vec<DataType>) -> DataType>>;

struct Interpreter {
    stack: Vec<DataType>,
    heap: Vec<UnsizedDataType>,
    variables: HashMap<String, usize>,
    ffi_table: FFITable,
}

impl Interpreter {
    pub fn new(ffi_table: FFITable) -> Interpreter {
        Interpreter {
            stack: vec![],
            heap: vec![],
            variables: HashMap::new(),
            ffi_table,
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
    fn alloc(&mut self, val: UnsizedDataType) -> DataType {
        let p = self.heap.len();
        self.heap.push(val);
        DataType::HeapAddr(p)
    }

    fn free(&mut self, pointer: DataType) -> Result<()> {
        match pointer {
            DataType::HeapAddr(p) => {
                if p < self.heap.len() && !matches!(self.heap[p], UnsizedDataType::Nil) {
                    self.heap[p] = UnsizedDataType::Nil;

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

    fn deref(&mut self, pointer: DataType) -> Result<UnsizedDataType> {
        match pointer {
            DataType::HeapAddr(p) => Ok(self.heap[p].clone()),
            _ => bail!("Expected pointer but found {:?}", pointer),
        }
    }

    fn load(&mut self, ident: &String) -> Result<DataType> {
        let v = self
            .variables
            .get(ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        Ok(self.stack[*v].clone())
    }

    fn statements(&mut self, arity: usize, stmts: Vec<Statement>) -> Result<()> {
        let mut pop_count = 0;
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    let val = self.expr(e.clone())?;
                    let p = self.push(val);
                    self.variables.insert(x.clone(), p);
                    pop_count += 1;
                }
                Statement::Return(e) => {
                    let val = self.expr(e.clone())?;
                    self.pop(pop_count + arity);
                    self.push(val);
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                }
                Statement::Panic => return Ok(()),
            }
        }

        // returnがない場合はreturn nil;と同等
        self.pop(pop_count + arity);
        self.push(DataType::Nil);
        Ok(())
    }

    // &Exprで十分？
    fn expr(&mut self, expr: Expr) -> Result<DataType> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => Ok(match lit {
                Literal::Int(n) => DataType::Int(n),
                Literal::String(s) => self.alloc(UnsizedDataType::String(s)),
            }),
            Expr::Fun(args, body) => Ok(self.alloc(UnsizedDataType::Closure(args, body))),
            Expr::Call(f, args) => {
                let arity = args.len();
                let mut vargs = vec![];
                for a in args {
                    vargs.push(self.expr(a)?);
                }

                // 特別な組み込み関数(stack, heapに干渉する必要があるものはここで)

                // ポインタ経由の代入: _passign(p,v) == (*p = v)
                if &f == "_passign" {
                    ensure!(arity == 2, "Expected 2 arguments but {:?} given", arity);

                    match vargs[0] {
                        DataType::StackAddr(p) => {
                            ensure!(
                                self.stack[p].type_of() == vargs[1].type_of(),
                                "Cannot assign to different types, left {:?} right {:?}",
                                self.stack[p],
                                vargs[1]
                            );

                            self.stack[p] = vargs[1].clone();
                        }
                        _ => {
                            bail!("Expected stack address but found {:?}", vargs[0]);
                        }
                    }

                    return Ok(DataType::Nil);
                }

                // ヒープ領域の開放
                if &f == "_free" {
                    ensure!(arity == 1, "Expected 1 arguments but {:?} given", arity);

                    self.free(vargs[0].clone())?;
                    return Ok(DataType::Nil);
                }

                if let Some(f) = self.ffi_table.get(&f) {
                    return Ok(f(vargs));
                }

                let faddr = self.load(&f)?;
                let closure = self.deref(faddr)?;
                match closure {
                    UnsizedDataType::Closure(vs, body) => {
                        ensure!(
                            vs.len() == arity,
                            "Expected {} arguments but {} given",
                            vs.len(),
                            arity,
                        );

                        let prev = self.variables.clone();
                        for (v, val) in vs.into_iter().zip(vargs) {
                            let p = self.push(val);
                            self.variables.insert(v, p);
                        }

                        self.statements(arity, body)?;

                        self.variables = prev;

                        Ok(self.pop(1))
                    }
                    r => bail!("Expected closure but found {:?}", r),
                }
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?;

                    Ok(DataType::StackAddr(*p))
                }
                _ => bail!("Cannot take the address of {:?}", expr),
            },
            Expr::Deref(expr) => match self.expr(expr.as_ref().clone())? {
                DataType::StackAddr(p) => Ok(self.stack[p].clone()),
                r => bail!("Cannot deref non-pointer value: {:?}", r),
            },
        }
    }

    pub fn module(&mut self, module: Module) -> Result<()> {
        // moduleは引数のない関数とみなす
        self.statements(0, module.0)
    }
}

pub fn interpret(ffi_table: FFITable, module: Module) -> Result<DataType> {
    let mut interpreter = Interpreter::new(ffi_table);
    interpreter.module(module)?;
    Ok(interpreter.pop(1))
}

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
        ];

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let code = gen_code(m).unwrap();
            let result = execute(code).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }

    #[test]
    fn test_interpreter_stack() {
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
                vec![UnsizedDataType::Closure(
                    vec![],
                    vec![
                        Statement::Let("y".to_string(), Expr::Lit(Literal::Int(1))),
                        Statement::Let("z".to_string(), Expr::Lit(Literal::Int(2))),
                        Statement::Return(Expr::Call(
                            "_add".to_string(),
                            vec![Expr::Var("y".to_string()), Expr::Var("z".to_string())],
                        )),
                    ],
                )],
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
                    UnsizedDataType::Closure(
                        vec![
                            "a".to_string(),
                            "b".to_string(),
                            "c".to_string(),
                            "d".to_string(),
                            "e".to_string(),
                        ],
                        vec![Statement::Return(Expr::Lit(Literal::String(
                            "hello".to_string(),
                        )))],
                    ),
                    UnsizedDataType::String("hello".to_string()),
                ],
            ),
            (
                // take the address of string
                r#"let x = "hello, world"; let y = &x; panic;"#,
                vec![DataType::HeapAddr(0), DataType::StackAddr(0)],
                vec![UnsizedDataType::String("hello, world".to_string())],
            ),
            (
                r#"let x = "hello, world"; _free(x); panic;"#,
                vec![DataType::HeapAddr(0)],
                vec![UnsizedDataType::Nil],
            ),
        ];

        for case in cases {
            let mut interpreter = Interpreter::new(create_ffi_table());
            let m = run_parser(case.0).unwrap();
            interpreter.module(m).unwrap();
            assert_eq!(interpreter.stack, case.1);
            assert_eq!(interpreter.heap, case.2);
        }
    }
}
