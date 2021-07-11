use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Module, Statement},
    vm::{DataType, HeapData, OpCode, StackData},
};

#[derive(Debug)]
struct CodeGenerator {
    variables: HashMap<String, usize>,
    statics: HashMap<String, usize>,
    ffi_table: HashMap<String, usize>,
    codes: Vec<OpCode>,
    static_area: Vec<HeapData>,
    current_loop_label: Option<String>,
    stack_count: usize, // 現在のstack frameから見たスタックポインタの相対位置
}

impl CodeGenerator {
    pub fn new(ffi_table: HashMap<String, usize>) -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            statics: HashMap::new(),
            codes: vec![],
            ffi_table,
            static_area: vec![],
            current_loop_label: None,
            stack_count: 0,
        }
    }

    fn push(&mut self, val: DataType) {
        match val {
            DataType::Nil => {
                self.codes.push(OpCode::Push(StackData::Nil));
            }
            DataType::Bool(b) => {
                self.codes.push(OpCode::Push(StackData::Bool(b)));
            }
            DataType::Int(n) => {
                self.codes.push(OpCode::Push(StackData::Int(n)));
            }
            DataType::String(s) => {
                self.codes.push(OpCode::Alloc(HeapData::String(s)));
            }
            _ => todo!(),
        }

        self.stack_count += 1;
    }

    fn pop(&mut self) {
        self.codes.push(OpCode::Pop(1));
        self.stack_count -= 1;
    }

    fn set_static(&mut self, addr: usize) {
        self.codes.push(OpCode::SetStatic(addr));
        self.stack_count -= 1;
    }

    fn ret(&mut self) {
        self.codes.push(OpCode::Return(self.stack_count));
    }

    fn after_call(&mut self, arity: usize) {
        self.stack_count = self.stack_count - arity;
    }

    fn load(&mut self, name: String) -> Result<()> {
        if let Some(addr) = self.statics.get(&name) {
            self.codes.push(OpCode::CopyStatic(*addr));
            self.stack_count += 1;

            return Ok(());
        }
        if let Some(addr) = self.variables.get(&name) {
            self.codes.push(OpCode::Copy(*addr));
            self.stack_count += 1;

            return Ok(());
        }

        bail!("Ident {} not found", name);
    }

    fn load_address(&mut self, name: String) -> Result<()> {
        if let Some(addr) = self.statics.get(&name) {
            self.codes.push(OpCode::Push(StackData::StaticAddr(*addr)));
            self.stack_count += 1;

            return Ok(());
        }
        if let Some(addr) = self.variables.get(&name) {
            self.codes.push(OpCode::Push(StackData::StackAddr(*addr)));
            self.stack_count += 1;

            return Ok(());
        }

        bail!("Ident {} not found", name);
    }

    fn expr(&mut self, expr: Expr) -> Result<()> {
        match expr {
            Expr::Var(ident) => {
                return self.load(ident);
            }
            Expr::Lit(lit) => {
                self.push(lit.into_datatype());

                return Ok(());
            }
            Expr::Fun(_, _, _) => bail!("Function expression is not supported"),
            Expr::Call(f, args) => {
                if self.statics.contains_key(&f) {
                    let p = self.stack_count;
                    let addr = self.statics[&f].clone();

                    self.codes.push(OpCode::Push(StackData::StaticAddr(addr)));
                    self.stack_count += 1;

                    // push arguments
                    let arity = args.len();
                    for a in args {
                        self.expr(a)?;
                    }

                    self.codes.push(OpCode::Call(arity));
                    self.after_call(arity);

                    // 関数呼び出しによって戻り値がstackに積まれるので1つ分だけ増えている必要がある
                    assert_eq!(p + 1, self.stack_count, "{:?}", self);

                    return Ok(());
                }

                if let Some(addr) = self.ffi_table.get(&f).cloned() {
                    let p = self.stack_count;
                    self.codes.push(OpCode::Push(StackData::StaticAddr(addr)));
                    self.stack_count += 1;

                    // push arguments
                    let arity = args.len();
                    for a in args {
                        self.expr(a)?;
                    }

                    self.codes.push(OpCode::FFICall(arity));
                    self.after_call(arity);

                    // 関数呼び出しによって戻り値がstackに積まれるので1つ分だけ増えている必要がある
                    assert_eq!(p + 1, self.stack_count, "{:?}", self);

                    return Ok(());
                }

                // push arguments
                let arity = args.len();
                for a in args {
                    self.expr(a)?;
                }

                // 組み込み関数

                // 関数は呼び出し後は戻り値をstackに積まないといけないという呼び出し規約であるのでここでstack_countを増やしておく
                self.stack_count += 1;

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

                bail!("Ident {} not found in call", f);
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(ident) => {
                    self.load_address(ident.clone())?;

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
                self.current_loop_label = Some(label.clone());

                let p = self.stack_count;
                self.codes.push(OpCode::Label(label.clone()));
                self.statements(s, false)?;
                let q = self.stack_count;
                self.codes.push(OpCode::Pop(q - p));
                self.codes.push(OpCode::Jump(label));

                self.current_loop_label = None;

                self.push(DataType::Nil);
                Ok(())
            }
        }
    }

    fn statements(&mut self, stmts: Vec<Statement>, do_return: bool) -> Result<()> {
        for stmt in stmts {
            match stmt {
                // 関数宣言はstaticなものにコンパイルする必要があるのでここで特別扱いする
                Statement::Let(is_static, x, Expr::Fun(_id, args, body)) => {
                    if !is_static {
                        bail!("A function in a function is not supported");
                    }

                    let mut generator = CodeGenerator::new(self.ffi_table.clone());
                    generator.variables = self.variables.clone();
                    generator.statics = self.statics.clone();

                    for a in args {
                        generator.variables.insert(a, generator.stack_count);
                        generator.stack_count += 1;
                    }

                    // return addressの分だけずらす
                    generator.stack_count += 1;

                    generator.statements(body, true)?;

                    let addr = self.static_area.len();
                    self.static_area.push(HeapData::Closure(generator.codes));
                    self.statics.insert(x, addr);
                }
                Statement::Let(is_static, x, e) => {
                    self.expr(e.clone())?;

                    if is_static {
                        let addr = self.static_area.len();
                        self.static_area.push(HeapData::Nil);
                        self.set_static(addr);

                        self.statics.insert(x.clone(), self.static_area.len() - 1);
                    } else {
                        self.variables.insert(x.clone(), self.stack_count - 1);
                    }
                }
                Statement::Return(e) => {
                    self.expr(e.clone())?;
                    self.ret();
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                    self.pop();
                }
                Statement::ReturnIf(_, _) => todo!(),
                Statement::If(cond, s1, s2) => {
                    let p = self.stack_count;

                    let else_label = format!("else-{}", self.codes.len());
                    let end_if_label = format!("end-if-{}", self.codes.len());
                    self.expr(cond.as_ref().clone())?;
                    self.codes.push(OpCode::JumpIfNot(else_label.clone()));
                    self.stack_count -= 1;

                    let q = self.stack_count;

                    // then block
                    self.statements(s1, false)?;
                    self.codes.push(OpCode::Jump(end_if_label.clone()));

                    self.stack_count = q;

                    // else block
                    self.codes.push(OpCode::Label(else_label));
                    self.statements(s2, false)?;

                    // endif
                    self.codes.push(OpCode::Label(end_if_label));

                    self.stack_count = p;
                }
                Statement::Continue => {
                    let label = self.current_loop_label.clone().unwrap();
                    self.codes.push(OpCode::Pop(self.stack_count));
                    self.codes.push(OpCode::Jump(label));
                }
            }
        }

        if do_return {
            // returnがない場合はreturn nil;と同等
            self.push(DataType::Nil);
            self.ret();
        }
        Ok(())
    }

    fn module(&mut self, module: Module) -> Result<()> {
        self.statements(module.0, true)
    }

    pub fn gen_code(&mut self, module: Module) -> Result<()> {
        self.module(module)
    }
}

pub fn gen_code(
    module: Module,
    ffi_table: HashMap<String, usize>,
) -> Result<(Vec<OpCode>, Vec<HeapData>)> {
    let mut generator = CodeGenerator::new(ffi_table);
    generator.gen_code(module)?;

    Ok((generator.codes, generator.static_area))
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
                        Pop(1),
                        Push(StackData::Int(2)),
                        Pop(1),
                        Push(StackData::Int(3)),
                        Pop(1),
                        Push(StackData::Int(4)),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
                    ],
                    vec![],
                ),
            ),
            (
                r#"let f = fn(a) { return a; }; f(1);"#,
                (
                    vec![
                        Push(StackData::StaticAddr(0)),
                        Push(StackData::Int(1)),
                        Call(1),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
                    ],
                    vec![HeapData::Closure(vec![Copy(0), Return(3)])],
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
                        Pop(1),
                        CopyStatic(0),
                        Return(1),
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
                        Push(StackData::StaticAddr(1)),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        CopyStatic(0),
                        Call(5),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
                    ],
                    vec![HeapData::Nil, HeapData::Closure(vec![Copy(0), Return(7)])],
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
                        HeapData::Closure(vec![Copy(0), Deref, Return(3)]),
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
                        return 10;
                    };
                "#,
                (
                    vec![
                        Label("label-0".to_string()),
                        Push(StackData::Int(10)),
                        Return(1),
                        Pop(1),
                        Jump("label-0".to_string()),
                        Push(StackData::Nil),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(2),
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
                        Push(StackData::StaticAddr(1)),
                        Alloc(HeapData::String("loop".to_string())),
                        FFICall(1),
                        Pop(1),
                        Pop(0),
                        Jump("label-0".to_string()),
                        Push(StackData::Nil),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
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
                        Push(StackData::StaticAddr(2)),
                        Push(StackData::Int(0)),
                        Push(StackData::Int(0)),
                        Push(StackData::Int(0)),
                        Call(3),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
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
                            Return(7),
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
                        Push(StackData::StaticAddr(2)),
                        Call(0),
                        Pop(1),
                        Push(StackData::Nil),
                        Return(1),
                    ],
                    vec![
                        HeapData::Closure(vec![Push(StackData::Nil), Return(2)]),
                        HeapData::Closure(vec![
                            Push(StackData::StaticAddr(0)),
                            Call(0),
                            Pop(1),
                            Push(StackData::Nil),
                            Return(2),
                        ]),
                        HeapData::Closure(vec![
                            Push(StackData::StaticAddr(1)),
                            Call(0),
                            Pop(1),
                            Push(StackData::Nil),
                            Return(2),
                        ]),
                        HeapData::Nil,
                    ],
                ),
            ),
        ];

        for c in cases {
            let (ffi_table, _) = create_ffi_table();
            let m = run_parser(c.0).unwrap();

            let result = gen_code(m, ffi_table);
            assert!(result.is_ok(), "{:?} {}", result, c.0);
            assert_eq!(result.unwrap(), c.1, "{}", c.0);
        }
    }
}
