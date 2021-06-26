use std::collections::{HashMap, HashSet};

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    vm::{DataType, HeapData, OpCode, StackData},
};

#[derive(Debug, Clone)]
struct VarInfo {
    address: usize,
    // 現状は関数をletによって束縛しないとcallできないのでとりあえず変数の情報として覚えておく
    // 実際は型情報にIDを含めそこから引くべき
    closure: Option<usize>,
}

#[derive(Debug)]
struct CodeGenerator {
    variables: HashMap<String, VarInfo>,
    closures: HashMap<usize, Vec<String>>,
    local: HashSet<String>,
    codes: Vec<OpCode>,
    ffi_table: HashMap<String, usize>,
    stack_count: usize,
    pop_count: usize,
    non_local_variables: Vec<String>, // for closure
    base_address: usize,              // base address for closure
}

impl CodeGenerator {
    pub fn new(ffi_table: HashMap<String, usize>) -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            closures: HashMap::new(),
            local: HashSet::new(),
            codes: vec![],
            stack_count: 0,
            pop_count: 0,
            ffi_table,
            non_local_variables: vec![],
            base_address: 0,
        }
    }

    fn push(&mut self, val: StackData) {
        self.codes.push(OpCode::Push(val));
        self.stack_count += 1;
        self.pop_count += 1;
    }

    fn alloc(&mut self, val: DataType) -> Result<()> {
        match val {
            DataType::Nil => {
                self.codes.push(OpCode::Alloc(HeapData::Nil));
            }
            DataType::Int(n) => {
                self.codes.push(OpCode::Alloc(HeapData::Int(n)));
            }
            DataType::String(s) => {
                self.codes.push(OpCode::Alloc(HeapData::String(s)));
            }
            DataType::Closure(id, args, body) => {
                let mut generator = CodeGenerator::new(self.ffi_table.clone());
                generator.variables = self.variables.clone();
                generator.closures = self.closures.clone();
                generator.stack_count = self.stack_count;
                generator.base_address = self.stack_count;

                let arity = args.len();
                for a in args {
                    generator.local.insert(a.clone());
                    generator.variables.insert(
                        a,
                        VarInfo {
                            address: generator.stack_count,
                            closure: None,
                        },
                    );
                    generator.stack_count += 1;
                }

                generator.statements(arity, body, true)?;

                self.codes
                    .push(OpCode::Alloc(HeapData::Closure(generator.codes)));
                self.closures.insert(id, generator.non_local_variables);
            }
        }
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

    fn expr(&mut self, arity: usize, expr: Expr) -> Result<()> {
        match expr {
            Expr::Var(ident) => {
                let v = self
                    .variables
                    .get(&ident)
                    .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

                let is_local = self.local.contains(&ident);

                if !is_local {
                    if !self.non_local_variables.contains(&ident) {
                        self.non_local_variables.push(ident.clone());
                    }
                }

                self.codes.push(OpCode::Copy(if is_local {
                    self.stack_count - 1 - v.address
                } else {
                    // localではない場合はclosureからouter variableにアクセスする場合
                    // stack_countとbase_addressの差分で関数開始時のアドレスまで戻れるので、そこからouter variableのindexまでさらに戻る
                    (self.stack_count - self.base_address)
                        + (self.non_local_variables.len()
                            - 1
                            - self
                                .non_local_variables
                                .iter()
                                .position(|p| p == &ident)
                                .unwrap())
                }));
                self.stack_count += 1;
                self.pop_count += 1;

                Ok(())
            }
            Expr::Lit(lit) => {
                match lit {
                    Literal::Int(n) => self.alloc(DataType::Int(n))?,
                    Literal::String(s) => self.alloc(DataType::String(s))?,
                };

                Ok(())
            }
            Expr::Fun(pos, args, body) => {
                self.alloc(DataType::Closure(pos, args, body))?;

                Ok(())
            }
            Expr::Call(f, args) => {
                if self.variables.contains_key(&f) {
                    let addr = self.variables[&f].clone();

                    // push outer_variables
                    // FIXME: _switchなど一部変数を宣言せずにclosureを使う場面があるのでそのような場面は一旦outer variableは利用しないものと仮定してスルーする
                    let outer_variables = addr
                        .closure
                        .map(|addr| self.closures[&addr].clone())
                        .unwrap_or(vec![]);

                    for v in outer_variables {
                        self.codes.push(OpCode::Copy(
                            self.stack_count - 1 - self.variables[&v].address,
                        ));
                        self.stack_count += 1;
                        self.pop_count += 1;
                    }

                    // push arguments
                    let arity = args.len();
                    for a in args {
                        self.expr(arity, a)?;
                    }

                    self.codes
                        .push(OpCode::Call(self.stack_count - 1 - addr.address));
                    self.after_call(arity);

                    return Ok(());
                }

                let arity = args.len();
                for a in args {
                    self.expr(arity, a)?;
                }

                // 特別な組み込み関数(stack, heapに干渉する必要があるものはここで)

                // ポインタ経由の代入: _passign(p,v) == (*p = v)
                if &f == "_passign" {
                    ensure!(arity == 2, "Expected 2 arguments but {:?} given", arity);
                    self.codes.push(OpCode::PAssign);
                    self.codes.push(OpCode::Push(StackData::Nil));
                    self.after_call(arity);

                    return Ok(());
                }

                // ヒープ領域の開放
                if &f == "_free" {
                    ensure!(arity == 1, "Expected 1 arguments but {:?} given", arity);
                    self.codes.push(OpCode::Free);
                    self.codes.push(OpCode::Push(StackData::Nil));
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
                    self.codes.push(OpCode::Push(StackData::Nil));
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
                    self.codes.push(OpCode::Push(StackData::Nil));
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

                // slice of string
                if &f == "_slice" {
                    ensure!(arity == 3, "Expected {} arguments but {} given", 3, arity);
                    self.codes.push(OpCode::Slice);
                    self.after_call(arity);

                    return Ok(());
                }

                if let Some(addr) = self.ffi_table.get(&f).cloned() {
                    self.codes.push(OpCode::FFICall(addr));

                    // TODO(safety): arity check
                    self.after_call(arity);

                    return Ok(());
                }

                bail!("Ident {} not found", f);
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?
                        .clone();
                    self.push(StackData::StackRevAddr(self.stack_count - 1 - p.address));

                    Ok(())
                }
                _ => bail!("Cannot take the address of {:?}", expr),
            },
            Expr::Deref(expr) => {
                self.expr(arity, expr.as_ref().clone())?;
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
                    self.expr(arity, e.clone())?;
                    self.variables.insert(
                        x.clone(),
                        VarInfo {
                            address: self.stack_count - 1,
                            closure: match e {
                                Expr::Fun(t, _, _) => Some(t),
                                _ => None,
                            },
                        },
                    );
                    self.local.insert(x.clone());
                }
                Statement::Return(e) => {
                    self.expr(arity, e.clone())?;
                    self.ret(arity);
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(arity, e.clone())?;
                }
                Statement::Panic => return Ok(()),
                Statement::ReturnIf(expr, cond) => {
                    self.expr(arity, expr)?;
                    self.expr(arity, cond)?;
                    self.ret_if(arity);
                    self.stack_count -= 2;
                }
            }
        }

        if do_return {
            // returnがない場合はreturn nil;と同等
            self.push(StackData::Nil);
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
                vec![Alloc(HeapData::Int(10)), Copy(0), Copy(0), Return(3)],
            ),
            (
                r#"let x = 10; return &x;"#,
                vec![
                    Alloc(HeapData::Int(10)),
                    Push(StackData::StackRevAddr(0)),
                    Return(2),
                ],
            ),
            (
                r#"1; 2; 3; 4;"#,
                vec![
                    Alloc(HeapData::Int(1)),
                    Alloc(HeapData::Int(2)),
                    Alloc(HeapData::Int(3)),
                    Alloc(HeapData::Int(4)),
                    Push(StackData::Nil),
                    Return(5),
                ],
            ),
            (
                r#"let f = fn(a) { return a; }; f(1);"#,
                vec![
                    Alloc(HeapData::Closure(vec![Copy(0), Return(2)])),
                    Alloc(HeapData::Int(1)),
                    Call(1),
                    Push(StackData::Nil),
                    Return(3),
                ],
            ),
            (
                r#"let x = 0; _passign(&x, 10); return x;"#,
                vec![
                    Alloc(HeapData::Int(0)),
                    Push(StackData::StackRevAddr(0)),
                    Alloc(HeapData::Int(10)),
                    PAssign,
                    Push(StackData::Nil),
                    Copy(1),
                    Return(3),
                ],
            ),
            (
                r#"let x = 10; let f = fn (a,b,c,d,e) { return a; }; f(x,x,x,x,x);"#,
                vec![
                    Alloc(HeapData::Int(10)),
                    Alloc(HeapData::Closure(vec![Copy(4), Return(6)])),
                    Copy(1),
                    Copy(2),
                    Copy(3),
                    Copy(4),
                    Copy(5),
                    Call(5),
                    Push(StackData::Nil),
                    Return(4),
                ],
            ),
            (
                r#"let x = 0; let f = fn (a) { return *a; };"#,
                vec![
                    Alloc(HeapData::Int(0)),
                    Alloc(HeapData::Closure(vec![Copy(0), Deref, Return(2)])),
                    Push(StackData::Nil),
                    Return(3),
                ],
            ),
            (
                r#"let x = _tuple(1, 2, 3, 4, 5); return _get(x, 3);"#,
                vec![
                    Alloc(HeapData::Int(1)),
                    Alloc(HeapData::Int(2)),
                    Alloc(HeapData::Int(3)),
                    Alloc(HeapData::Int(4)),
                    Alloc(HeapData::Int(5)),
                    Tuple(5),
                    Copy(0),
                    Alloc(HeapData::Int(3)),
                    Get,
                    Return(2),
                ],
            ),
            (
                r#"let x = _object("x", 10, "y", "yes"); return _get(x, "x");"#,
                vec![
                    Alloc(HeapData::String("x".to_string())),
                    Alloc(HeapData::Int(10)),
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
                    Alloc(HeapData::Int(10)),
                    Alloc(HeapData::Int(1)),
                    ReturnIf(2),
                    Pop(0),
                    Jump("label-0".to_string()),
                    Push(StackData::Nil),
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
                    Push(StackData::Nil),
                    Return(2),
                ],
            ),
            (
                // outer scope variable
                r#"
                    let outer1 = 0;
                    let outer2 = 0;
                    let f = fn (a,b,c) {
                        let inner1 = 0;
                        let inner2 = 0;
                        return _tuple(outer2, outer1, outer2);
                    };
                    let outer3 = 0;

                    f(0,0,0);
                "#,
                vec![
                    Alloc(HeapData::Int(0)),
                    Alloc(HeapData::Int(0)),
                    Alloc(HeapData::Closure(vec![
                        Alloc(HeapData::Int(0)),
                        Alloc(HeapData::Int(0)),
                        Copy(5),
                        Copy(6),
                        Copy(8),
                        Tuple(3),
                        Return(6),
                    ])),
                    Alloc(HeapData::Int(0)),
                    Copy(2),
                    Copy(4),
                    Alloc(HeapData::Int(0)),
                    Alloc(HeapData::Int(0)),
                    Alloc(HeapData::Int(0)),
                    Call(6),
                    Push(StackData::Nil),
                    Return(8),
                ],
            ),
        ];

        for c in cases {
            let (ffi_table, _) = create_ffi_table();
            let m = run_parser(c.0).unwrap();

            let result = gen_code(m, ffi_table);
            assert!(result.is_ok(), "{:?} {:?}", result, c.0);
            assert_eq!(result.unwrap(), c.1, "{:?}", c.0);
        }
    }
}
