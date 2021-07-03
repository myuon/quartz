use std::collections::{HashMap, HashSet};

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    vm::{DataType, HeapData, OpCode, StackData},
};

#[derive(Debug, Clone)]
struct VarInfo {
    is_static: bool,
    address: usize,
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    static_addr: usize,
    // 型推論時にどうにかできるので本来は不要
    closure_id: usize,
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
    static_area: Vec<HeapData>,
    functions: HashMap<String, FunctionInfo>,
}

impl CodeGenerator {
    pub fn new(
        ffi_table: HashMap<String, usize>,
        closures: HashMap<usize, Vec<String>>,
    ) -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            closures,
            local: HashSet::new(),
            codes: vec![],
            stack_count: 0,
            pop_count: 0,
            ffi_table,
            non_local_variables: vec![],
            base_address: 0,
            static_area: vec![],
            functions: HashMap::new(),
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
                self.codes.push(OpCode::Push(StackData::Int(n)));
            }
            DataType::String(s) => {
                self.codes.push(OpCode::Alloc(HeapData::String(s)));
            }
            _ => {
                bail!("Invalid expr");
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

                if v.is_static {
                    self.codes.push(OpCode::CopyStatic(v.address));
                } else {
                    self.codes
                        .push(OpCode::Copy(self.stack_count - 1 - v.address));
                }

                self.stack_count += 1;
                self.pop_count += 1;

                Ok(())
            }
            Expr::Lit(lit) => {
                match lit {
                    Literal::Int(n) => self.push(StackData::Int(n)),
                    Literal::String(s) => self.alloc(DataType::String(s))?,
                    Literal::Nil => self.push(StackData::Nil),
                    Literal::Bool(b) => self.push(StackData::Bool(b)),
                };

                Ok(())
            }
            Expr::Fun(_, _, _) => bail!("Function expression is not supported"),
            Expr::Call(f, args) => {
                if self.functions.contains_key(&f) {
                    let addr = self.functions[&f].clone();

                    // push arguments
                    let arity = args.len();
                    for a in args {
                        self.expr(arity, a)?;
                    }

                    self.codes.push(OpCode::Call(addr.static_addr));
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

                // panic
                if &f == "_panic" {
                    ensure!(arity == 1, "Expected {} arguments but {} given", 1, arity);
                    self.codes.push(OpCode::Panic);
                    self.after_call(arity);

                    return Ok(());
                }

                if let Some(addr) = self.ffi_table.get(&f).cloned() {
                    self.codes.push(OpCode::FFICall(addr));

                    // TODO(safety): arity check
                    self.after_call(arity);

                    return Ok(());
                }

                bail!("Ident {} not found in call   ", f);
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?
                        .clone();
                    self.push(if p.is_static {
                        StackData::StaticAddr(p.address)
                    } else {
                        StackData::StackAddr(self.stack_count - 1 - p.address)
                    });

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
                // 関数宣言はstaticなものにコンパイルする必要があるのでここで特別扱いする
                Statement::Let(is_static, x, Expr::Fun(id, args, body)) => {
                    if !is_static {
                        bail!("A function in a function is not supported");
                    }

                    let mut generator =
                        CodeGenerator::new(self.ffi_table.clone(), self.closures.clone());
                    generator.variables = self.variables.clone();
                    generator.closures = self.closures.clone();
                    generator.stack_count = self.stack_count;
                    generator.base_address = self.stack_count;
                    generator.non_local_variables = self.closures[&id].clone();
                    generator.functions = self.functions.clone();

                    let arity = args.len();
                    for a in args {
                        generator.local.insert(a.clone());
                        generator.variables.insert(
                            a,
                            VarInfo {
                                is_static: false,
                                address: generator.stack_count,
                            },
                        );
                        generator.stack_count += 1;
                    }

                    generator.statements(arity, body, true)?;

                    self.closures.insert(id, generator.non_local_variables);
                    self.functions.extend(generator.functions);

                    let addr = self.static_area.len();
                    self.static_area.push(HeapData::Closure(generator.codes));
                    self.functions.insert(
                        x,
                        FunctionInfo {
                            static_addr: addr,
                            closure_id: id,
                        },
                    );
                }
                Statement::Let(is_static, x, e) => {
                    self.expr(arity, e.clone())?;
                    self.variables.insert(
                        x.clone(),
                        VarInfo {
                            is_static,
                            address: if is_static {
                                self.static_area.len()
                            } else {
                                self.stack_count - 1
                            },
                        },
                    );

                    if is_static {
                        let addr = self.static_area.len();
                        self.static_area.push(HeapData::Nil);
                        self.codes.push(OpCode::SetStatic(addr));

                        self.stack_count -= 1;
                        self.pop_count -= 1;
                    }
                }
                Statement::Return(e) => {
                    self.expr(arity, e.clone())?;
                    self.ret(arity);
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(arity, e.clone())?;
                }
                Statement::ReturnIf(expr, cond) => {
                    self.expr(arity, expr)?;
                    self.expr(arity, cond)?;
                    self.ret_if(arity);
                    self.stack_count -= 2;
                }
                Statement::If(cond, s1, s2) => {
                    let stack_count = self.stack_count;
                    let pop_count = self.pop_count;

                    let else_label = format!("else-{}", self.codes.len());
                    let end_if_label = format!("end-if-{}", self.codes.len());
                    self.expr(0, cond.as_ref().clone())?;
                    self.codes.push(OpCode::JumpIfNot(else_label.clone()));

                    // then block
                    self.statements(0, s1, false)?;
                    self.codes.push(OpCode::Jump(end_if_label.clone()));

                    // else block
                    self.codes.push(OpCode::Label(else_label));
                    self.statements(0, s2, false)?;

                    // endif
                    self.codes.push(OpCode::Label(end_if_label));

                    self.stack_count = stack_count;
                    self.pop_count = pop_count;
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

pub fn gen_code(
    module: Module,
    ffi_table: HashMap<String, usize>,
    closures: HashMap<usize, Vec<String>>,
) -> Result<(Vec<OpCode>, Vec<HeapData>)> {
    let mut generator = CodeGenerator::new(ffi_table, closures);
    generator.gen_code(module)?;

    Ok((generator.codes, generator.static_area))
}

#[cfg(test)]
mod tests {
    use crate::{create_ffi_table, parser::run_parser, typechecker::typechecker};

    use super::*;

    #[test]
    fn test_gen_code() {
        use OpCode::*;

        let cases = vec![
            (
                r#"let x = 10; let y = x; return y;"#,
                (
                    vec![
                        Push(StackData::Int(10)),
                        SetStatic(0),
                        CopyStatic(0),
                        SetStatic(1),
                        CopyStatic(1),
                        Return(1),
                    ],
                    vec![HeapData::Nil, HeapData::Nil],
                ),
            ),
            (
                r#"let x = 10; return &x;"#,
                (
                    vec![
                        Push(StackData::Int(10)),
                        SetStatic(0),
                        Push(StackData::StaticAddr(0)),
                        Return(1),
                    ],
                    vec![HeapData::Nil],
                ),
            ),
            (
                r#"1; 2; 3; 4;"#,
                (
                    vec![
                        Push(StackData::Int(1)),
                        Push(StackData::Int(2)),
                        Push(StackData::Int(3)),
                        Push(StackData::Int(4)),
                        Push(StackData::Nil),
                        Return(5),
                    ],
                    vec![],
                ),
            ),
            (
                r#"let f = fn(a) { return a; }; f(1);"#,
                (
                    vec![
                        Push(StackData::Int(1)),
                        Call(0),
                        Push(StackData::Nil),
                        Return(2),
                    ],
                    vec![HeapData::Closure(vec![Copy(0), Return(2)])],
                ),
            ),
            (
                r#"let x = 0; _passign(&x, 10); return x;"#,
                (
                    vec![
                        Push(StackData::Int(0)),
                        SetStatic(0),
                        Push(StackData::StaticAddr(0)),
                        Push(StackData::Int(10)),
                        PAssign,
                        Push(StackData::Nil),
                        CopyStatic(0),
                        Return(2),
                    ],
                    vec![HeapData::Nil],
                ),
            ),
            (
                r#"let x = 10; let f = fn (a,b,c,d,e) { return a; }; f(x,x,x,x,x);"#,
                (
                    vec![
                        Push(StackData::Int(10)),
                        SetStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        Call(1),
                        Push(StackData::Nil),
                        Return(2),
                    ],
                    vec![HeapData::Nil, HeapData::Closure(vec![Copy(4), Return(6)])],
                ),
            ),
            (
                r#"let x = 0; let f = fn (a) { return *a; };"#,
                (
                    vec![
                        Push(StackData::Int(0)),
                        SetStatic(0),
                        Push(StackData::Nil),
                        Return(1),
                    ],
                    vec![
                        HeapData::Nil,
                        HeapData::Closure(vec![Copy(0), Deref, Return(2)]),
                    ],
                ),
            ),
            (
                r#"let x = _tuple(1, 2, 3, 4, 5); return _get(x, 3);"#,
                (
                    vec![
                        Push(StackData::Int(1)),
                        Push(StackData::Int(2)),
                        Push(StackData::Int(3)),
                        Push(StackData::Int(4)),
                        Push(StackData::Int(5)),
                        Tuple(5),
                        SetStatic(0),
                        CopyStatic(0),
                        Push(StackData::Int(3)),
                        Get,
                        Return(1),
                    ],
                    vec![HeapData::Nil],
                ),
            ),
            (
                r#"let x = _object("x", 10, "y", "yes"); return _get(x, "x");"#,
                (
                    vec![
                        Alloc(HeapData::String("x".to_string())),
                        Push(StackData::Int(10)),
                        Alloc(HeapData::String("y".to_string())),
                        Alloc(HeapData::String("yes".to_string())),
                        Object(2),
                        SetStatic(0),
                        CopyStatic(0),
                        Alloc(HeapData::String("x".to_string())),
                        Get,
                        Return(1),
                    ],
                    vec![HeapData::Nil],
                ),
            ),
            (
                r#"
                    loop {
                        return 10 if 1;
                    };
                "#,
                (
                    vec![
                        Label("label-0".to_string()),
                        Push(StackData::Int(10)),
                        Push(StackData::Int(1)),
                        ReturnIf(2),
                        Pop(0),
                        Jump("label-0".to_string()),
                        Push(StackData::Nil),
                        Return(3),
                    ],
                    vec![],
                ),
            ),
            (
                r#"
                    loop {
                        _print("loop");
                    };
                "#,
                (
                    vec![
                        Label("label-0".to_string()),
                        Alloc(HeapData::String("loop".to_string())),
                        FFICall(1),
                        Pop(1),
                        Jump("label-0".to_string()),
                        Push(StackData::Nil),
                        Return(2),
                    ],
                    vec![],
                ),
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
                (
                    vec![
                        Push(StackData::Int(0)),
                        SetStatic(0),
                        Push(StackData::Int(0)),
                        SetStatic(1),
                        Push(StackData::Int(0)),
                        SetStatic(3),
                        Push(StackData::Int(0)),
                        Push(StackData::Int(0)),
                        Push(StackData::Int(0)),
                        Call(2),
                        Push(StackData::Nil),
                        Return(2),
                    ],
                    vec![
                        HeapData::Nil,
                        HeapData::Nil,
                        HeapData::Closure(vec![
                            Push(StackData::Int(0)),
                            Push(StackData::Int(0)),
                            CopyStatic(1),
                            CopyStatic(0),
                            CopyStatic(1),
                            Tuple(3),
                            Return(6),
                        ]),
                        HeapData::Nil,
                    ],
                ),
            ),
            (
                // method call chain
                r#"
                    let f = fn() {};
                    let g = fn() { f(); };
                    let h = fn() { g(); };
                    let x = 10;
                    h();
                "#,
                (
                    vec![
                        Push(StackData::Int(10)),
                        SetStatic(3),
                        Call(2),
                        Push(StackData::Nil),
                        Return(2),
                    ],
                    vec![
                        HeapData::Closure(vec![Push(StackData::Nil), Return(1)]),
                        HeapData::Closure(vec![Call(0), Push(StackData::Nil), Return(2)]),
                        HeapData::Closure(vec![Call(1), Push(StackData::Nil), Return(2)]),
                        HeapData::Nil,
                    ],
                ),
            ),
        ];

        for c in cases {
            let (ffi_table, _) = create_ffi_table();
            let m = run_parser(c.0).unwrap();
            let closures = typechecker(&m).unwrap();

            let result = gen_code(m, ffi_table, closures);
            assert!(result.is_ok(), "{:?} {}", result, c.0);
            assert_eq!(result.unwrap(), c.1, "{}", c.0);
        }
    }
}
