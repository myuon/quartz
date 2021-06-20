use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    runtime::{DataType, UnsizedDataType},
};

#[derive(PartialEq, Debug, Clone)]
pub enum HeapData {
    Nil,
    String(String),
    Closure(Vec<OpCode>),
    Vec(Vec<DataType>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpCode {
    Push(DataType),
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
}

#[derive(Debug)]
struct CodeGenerator {
    variables: HashMap<String, usize>,
    codes: Vec<OpCode>,
    stack_count: usize,
    pop_count: usize,
    ffi_table: HashMap<String, usize>,
}

impl CodeGenerator {
    pub fn new(ffi_table: HashMap<String, usize>) -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            codes: vec![],
            stack_count: 0,
            pop_count: 0,
            ffi_table,
        }
    }

    fn push(&mut self, val: DataType) {
        self.codes.push(OpCode::Push(val));
        self.stack_count += 1;
        self.pop_count += 1;
    }

    fn alloc(&mut self, val: UnsizedDataType) -> Result<()> {
        match val {
            UnsizedDataType::Nil => {
                self.codes.push(OpCode::Alloc(HeapData::Nil));
            }
            UnsizedDataType::String(s) => {
                self.codes.push(OpCode::Alloc(HeapData::String(s)));
            }
            UnsizedDataType::Closure(args, body) => {
                let mut generator = CodeGenerator::new(self.ffi_table.clone());
                generator.variables = self.variables.clone();
                generator.stack_count = self.stack_count + 1;

                let arity = args.len();
                for a in args {
                    generator.variables.insert(a, generator.stack_count);
                    generator.stack_count += 1;
                }

                generator.statements(arity, body, true)?;

                self.codes
                    .push(OpCode::Alloc(HeapData::Closure(generator.codes)));
            }
        }
        self.stack_count += 1;
        self.pop_count += 1;
        Ok(())
    }

    fn load(&mut self, ident: &String) -> Result<()> {
        let v = self
            .variables
            .get(ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        self.codes.push(OpCode::Copy(self.stack_count - 1 - *v));
        self.stack_count += 1;
        self.pop_count += 1;
        Ok(())
    }

    fn ret(&mut self, arity: usize) {
        let pop = self.pop_count + arity;

        self.codes.push(OpCode::Return(pop));
    }

    fn ret_if(&mut self, arity: usize) {
        let pop = self.pop_count + arity;

        self.codes.push(OpCode::ReturnIf(pop));
    }

    fn after_call(&mut self, arity: usize) {
        self.stack_count = self.stack_count + 1 - arity;
        self.pop_count = self.pop_count + 1 - arity;
    }

    fn expr(&mut self, expr: Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => {
                match lit {
                    Literal::Int(n) => self.push(DataType::Int(n)),
                    Literal::String(s) => self.alloc(UnsizedDataType::String(s))?,
                };

                Ok(())
            }
            Expr::Fun(args, body) => self.alloc(UnsizedDataType::Closure(args, body)),
            Expr::Call(f, args) => {
                let arity = args.len();
                for a in args {
                    self.expr(a)?;
                }

                // 特別な組み込み関数(stack, heapに干渉する必要があるものはここで)

                // ポインタ経由の代入: _passign(p,v) == (*p = v)
                if &f == "_passign" {
                    ensure!(arity == 2, "Expected 2 arguments but {:?} given", arity);
                    self.codes.push(OpCode::PAssign);
                    self.codes.push(OpCode::Push(DataType::Nil));
                    self.after_call(arity);

                    return Ok(());
                }

                // ヒープ領域の開放
                if &f == "_free" {
                    ensure!(arity == 1, "Expected 1 arguments but {:?} given", arity);
                    self.codes.push(OpCode::Free);
                    self.codes.push(OpCode::Push(DataType::Nil));
                    self.after_call(arity);

                    return Ok(());
                }

                // n-タプルの生成
                if &f == "_tuple" {
                    self.codes.push(OpCode::Tuple(arity));
                    self.after_call(arity);

                    return Ok(());
                }

                // objectの生成
                if &f == "_object" {
                    ensure!(
                        arity % 2 == 0,
                        "Expected even arguments but {:?} given",
                        arity
                    );
                    self.codes.push(OpCode::Object(arity / 2));
                    self.after_call(arity);

                    return Ok(());
                }

                // 値の取り出し
                if &f == "_get" {
                    ensure!(arity == 2, "Expected {} arguments but {} given", 2, arity);
                    self.codes.push(OpCode::Get);
                    self.after_call(arity);

                    return Ok(());
                }

                // 値の上書き
                if &f == "_set" {
                    ensure!(arity == 3, "Expected {} arguments but {} given", 3, arity);
                    self.codes.push(OpCode::Set);
                    self.codes.push(OpCode::Push(DataType::Nil));
                    self.after_call(arity);

                    return Ok(());
                }

                // if (compare and run)
                if &f == "_switch" {
                    ensure!(
                        arity >= 2,
                        "Expected {} or more than {} arguments but {} given",
                        2,
                        2,
                        arity
                    );
                    self.codes.push(OpCode::Switch(arity - 1));
                    self.after_call(arity);

                    return Ok(());
                }

                // regular expressions
                if &f == "_regex" {
                    ensure!(arity == 2, "Expected {} arguments but {} given", 2, arity);
                    self.codes.push(OpCode::Regex);
                    self.after_call(arity);

                    return Ok(());
                }

                // regular expressions
                if &f == "_vec" {
                    ensure!(arity == 0, "Expected {} arguments but {} given", 0, arity);
                    self.codes.push(OpCode::Alloc(HeapData::Vec(vec![])));
                    self.after_call(arity);

                    return Ok(());
                }

                // push to vector
                if &f == "_vpush" {
                    ensure!(arity == 2, "Expected {} arguments but {} given", 2, arity);
                    self.codes.push(OpCode::VPush);
                    self.codes.push(OpCode::Push(DataType::Nil));
                    self.after_call(arity);

                    return Ok(());
                }

                // length of a vector
                if &f == "_len" {
                    ensure!(arity == 1, "Expected {} arguments but {} given", 1, arity);
                    self.codes.push(OpCode::Len);
                    self.after_call(arity);

                    return Ok(());
                }

                if let Some(addr) = self.ffi_table.get(&f).cloned() {
                    self.codes.push(OpCode::FFICall(addr));

                    // TODO(safety): arity check
                    self.after_call(arity);

                    return Ok(());
                }

                let addr = self
                    .variables
                    .get(&f)
                    .ok_or(anyhow::anyhow!("Ident {} not found", f))?;
                self.codes.push(OpCode::Call(*addr));
                self.after_call(arity);

                Ok(())
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?
                        .clone();

                    self.push(DataType::StackAddr(self.stack_count - 1 - p));

                    Ok(())
                }
                _ => bail!("Cannot take the address of {:?}", expr),
            },
            Expr::Deref(expr) => {
                self.expr(expr.as_ref().clone())?;
                self.codes.push(OpCode::Deref);
                Ok(())
            }
            Expr::Loop(s) => {
                let label = format!("label-{}", self.codes.len());

                let p = self.stack_count;
                self.codes.push(OpCode::Label(label.clone()));
                self.statements(0, s, false)?;
                let q = self.stack_count;
                self.codes.push(OpCode::Pop(q - p));
                self.codes.push(OpCode::Jump(label));
                Ok(())
            }
        }
    }

    fn statements(&mut self, arity: usize, stmts: Vec<Statement>, do_return: bool) -> Result<()> {
        self.pop_count = 0;
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    self.expr(e.clone())?;
                    self.variables.insert(x.clone(), self.stack_count - 1);
                }
                Statement::Return(e) => {
                    self.expr(e.clone())?;
                    self.ret(arity);
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                }
                Statement::Panic => return Ok(()),
                Statement::ReturnIf(expr, cond) => {
                    self.expr(expr)?;
                    self.expr(cond)?;
                    self.ret_if(arity);
                    self.stack_count -= 2;
                }
            }
        }

        if do_return {
            // returnがない場合はreturn nil;と同等
            self.push(DataType::Nil);
            self.ret(arity);
        }
        Ok(())
    }

    fn module(&mut self, module: Module) -> Result<()> {
        self.statements(0, module.0, true)
    }

    pub fn gen_code(&mut self, module: Module) -> Result<()> {
        self.module(module)
    }
}

pub fn gen_code(module: Module, ffi_table: HashMap<String, usize>) -> Result<Vec<OpCode>> {
    let mut generator = CodeGenerator::new(ffi_table);
    generator.gen_code(module)?;

    Ok(generator.codes)
}

#[cfg(test)]
mod tests {
    use crate::{create_ffi_table, parser::run_parser};

    use super::*;

    #[test]
    fn test_gen_code() {
        use OpCode::*;

        let cases = vec![
            (
                r#"let x = 10; let y = x; return y;"#,
                vec![Push(DataType::Int(10)), Copy(0), Copy(0), Return(3)],
            ),
            (
                r#"let x = 10; return &x;"#,
                vec![
                    Push(DataType::Int(10)),
                    Push(DataType::StackAddr(0)),
                    Return(2),
                ],
            ),
            (
                r#"1; 2; 3; 4;"#,
                vec![
                    Push(DataType::Int(1)),
                    Push(DataType::Int(2)),
                    Push(DataType::Int(3)),
                    Push(DataType::Int(4)),
                    Push(DataType::Nil),
                    Return(5),
                ],
            ),
            (
                r#"let f = fn(a) { return a; }; f(1);"#,
                vec![
                    Alloc(HeapData::Closure(vec![Copy(0), Return(2)])),
                    Push(DataType::Int(1)),
                    Call(0),
                    Push(DataType::Nil),
                    Return(3),
                ],
            ),
            (
                r#"let x = 0; _passign(&x, 10); return x;"#,
                vec![
                    Push(DataType::Int(0)),
                    Push(DataType::StackAddr(0)),
                    Push(DataType::Int(10)),
                    PAssign,
                    Push(DataType::Nil),
                    Copy(1),
                    Return(3),
                ],
            ),
            (
                r#"let x = 10; let f = fn (a,b,c,d,e) { return a; }; f(x,x,x,x,x);"#,
                vec![
                    Push(DataType::Int(10)),
                    Alloc(HeapData::Closure(vec![Copy(4), Return(6)])),
                    Copy(1),
                    Copy(2),
                    Copy(3),
                    Copy(4),
                    Copy(5),
                    Call(1),
                    Push(DataType::Nil),
                    Return(4),
                ],
            ),
            (
                r#"let x = 0; let f = fn (a) { return *a; };"#,
                vec![
                    Push(DataType::Int(0)),
                    Alloc(HeapData::Closure(vec![Copy(0), Deref, Return(2)])),
                    Push(DataType::Nil),
                    Return(3),
                ],
            ),
            (
                r#"let x = _tuple(1, 2, 3, 4, 5); return _get(x, 3);"#,
                vec![
                    Push(DataType::Int(1)),
                    Push(DataType::Int(2)),
                    Push(DataType::Int(3)),
                    Push(DataType::Int(4)),
                    Push(DataType::Int(5)),
                    Tuple(5),
                    Copy(0),
                    Push(DataType::Int(3)),
                    Get,
                    Return(2),
                ],
            ),
            (
                r#"let x = _object("x", 10, "y", "yes"); return _get(x, "x");"#,
                vec![
                    Alloc(HeapData::String("x".to_string())),
                    Push(DataType::Int(10)),
                    Alloc(HeapData::String("y".to_string())),
                    Alloc(HeapData::String("yes".to_string())),
                    Object(2),
                    Copy(0),
                    Alloc(HeapData::String("x".to_string())),
                    Get,
                    Return(2),
                ],
            ),
            (
                r#"
                    loop {
                        return 10 if 1;
                    };
                "#,
                vec![
                    Label("label-0".to_string()),
                    Push(DataType::Int(10)),
                    Push(DataType::Int(1)),
                    ReturnIf(2),
                    Pop(0),
                    Jump("label-0".to_string()),
                    Push(DataType::Nil),
                    Return(3),
                ],
            ),
            (
                r#"
                    loop {
                        _print("loop");
                    };
                "#,
                vec![
                    Label("label-0".to_string()),
                    Alloc(HeapData::String("loop".to_string())),
                    FFICall(1),
                    Pop(1),
                    Jump("label-0".to_string()),
                    Push(DataType::Nil),
                    Return(2),
                ],
            ),
        ];

        for c in cases {
            let (ffi_table, _) = create_ffi_table();
            let m = run_parser(c.0).unwrap();
            let result = gen_code(m, ffi_table).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
