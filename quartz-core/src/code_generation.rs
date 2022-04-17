use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Statement, Structs},
    vm::QVMInstruction,
};

#[derive(Debug)]
struct Generator<'a> {
    code: Vec<QVMInstruction>,
    local_pointer: usize,
    locals: HashMap<String, usize>,
    args: HashMap<String, usize>,
    globals: &'a HashMap<String, usize>,
    labels: &'a mut HashMap<String, usize>,
    offset: usize,
    structs: &'a Structs,
}

impl<'a> Generator<'a> {
    fn new(
        args: HashMap<String, usize>,
        globals: &'a HashMap<String, usize>,
        labels: &'a mut HashMap<String, usize>,
        offset: usize,
        structs: &'a Structs,
    ) -> Generator<'a> {
        Generator {
            code: vec![],
            local_pointer: 0,
            locals: HashMap::new(),
            args,
            globals,
            labels,
            offset,
            structs,
        }
    }

    fn push_local(&mut self, name: String) {
        self.locals.insert(name, self.local_pointer);
        self.local_pointer += 1;
    }

    fn register_label(&mut self, name: String) {
        self.labels.insert(name, self.offset + self.code.len());
    }

    fn expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => {
                if let Some(u) = self.locals.get(v) {
                    self.code.push(QVMInstruction::AddrConst(*u));
                    self.code.push(QVMInstruction::Load("local"));
                } else if let Some(u) = self.args.get(v) {
                    self.code.push(QVMInstruction::LoadArg(*u));
                } else if let Some(u) = self.globals.get(v) {
                    self.code.push(QVMInstruction::AddrConst(*u));
                    self.code.push(QVMInstruction::Load("global"));
                } else {
                    self.code.push(match v.as_str() {
                        "_add" => QVMInstruction::Add,
                        "_sub" => QVMInstruction::Sub,
                        "_mult" => QVMInstruction::Mult,
                        "_eq" => QVMInstruction::Eq,
                        "_new" => QVMInstruction::Alloc,
                        _ => QVMInstruction::LabelAddrConst(v.clone()),
                    });
                }
            }
            Expr::Lit(lit) => {
                use crate::ast::Literal::*;

                match lit {
                    Nil => {
                        self.code.push(QVMInstruction::I32Const(9999));
                    }
                    Bool(_) => todo!(),
                    Int(n) => {
                        self.code.push(QVMInstruction::I32Const(*n));
                    }
                    String(_) => todo!(),
                }
            }
            Expr::Call(f, es) => {
                for e in es {
                    self.expr(e)?;
                }
                self.expr(f)?;

                // add Call instruction only if the previous instruction is a label address
                if matches!(self.code.last().unwrap(), QVMInstruction::LabelAddrConst(_)) {
                    self.code.push(QVMInstruction::Call);
                }
            }
            Expr::Struct(st, items) => {
                /* Example:
                 *   struct Foo {
                 *      a: i32,
                 *      b: i32,
                 *   }
                 *   let x = Foo { a: 10, b: 20 };
                 *
                 * desugars into:
                 *   let x = _new(2);
                 *   x[0] = 10;
                 *   x[1] = 20;
                 */

                let var_name = "_struct_".to_string();
                let size = self.structs.size_of_struct(st);

                let mut desugared = vec![Statement::Let(
                    var_name.clone(),
                    Expr::Call(
                        Box::new(Expr::Var("_new".to_string())),
                        vec![Expr::Lit(Literal::Int(size as i32))],
                    ),
                )];
                for (label, value) in items {
                    let index = self.structs.get_projection_offset(st, label)?;

                    desugared.push(Statement::Assignment(
                        Box::new(Expr::Index(
                            Box::new(Expr::Var(var_name.clone())),
                            Box::new(Expr::Lit(Literal::Int(index as i32))),
                        )),
                        value.clone(),
                    ));
                }

                self.statements(&desugared)?;
            }
            Expr::Project(true, st, proj, label) => {
                self.code
                    .push(QVMInstruction::LabelAddrConst(label.clone()));
                todo!()
            }
            Expr::Project(false, st, proj, label) => {
                // [proj].label
                let index = self.structs.get_projection_offset(st, label)?;

                self.expr(proj)?;
                self.code.extend(vec![
                    QVMInstruction::AddrConst(index),
                    QVMInstruction::Add,
                    QVMInstruction::Load("local"),
                ]);
            }
            Expr::Deref(_) => todo!(),
            Expr::Ref(_) => todo!(),
            Expr::Index(e, i) => {
                self.expr(e)?;
                self.expr(i)?;
                self.code.push(QVMInstruction::Add);
                self.code.push(QVMInstruction::Load("heap"));
            }
        }

        Ok(())
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(v, expr) => {
                self.expr(expr)?;
                self.push_local(v.clone());
            }
            Statement::Expr(expr) => {
                self.expr(expr)?;
            }
            Statement::Return(e) => {
                self.expr(e)?;
                self.code.push(QVMInstruction::Return(self.args.len()));
            }
            Statement::If(b, e1, e2) => {
                let label = format!("if-{}", self.globals.len());
                let label_then = format!("then-{}", self.globals.len());
                let label_else = format!("else-{}", self.globals.len());
                let label_end = format!("end-{}", self.globals.len());

                self.register_label(label.clone());
                self.expr(b)?;
                self.code
                    .push(QVMInstruction::LabelJumpIfFalse(label_else.clone()));
                self.register_label(label_then.clone());
                self.statements(e1)?;
                self.code
                    .push(QVMInstruction::LabelJumpIfFalse(label_end.clone()));
                self.register_label(label_else.clone());
                self.statements(e2)?;
                self.register_label(label_end.clone());
            }
            Statement::Continue => todo!(),
            Statement::Assignment(v, e) => {
                self.expr(e)?;

                match v.as_ref() {
                    Expr::Var(v) => {
                        if let Some(u) = self.locals.get(v).cloned() {
                            self.code.push(QVMInstruction::AddrConst(u));
                            self.code.push(QVMInstruction::Store("local"));
                        } else if let Some(u) = self.globals.get(v).cloned() {
                            self.code.push(QVMInstruction::AddrConst(u));
                            self.code.push(QVMInstruction::Store("global"));
                        } else {
                            anyhow::bail!("{} not found", v);
                        }
                    }
                    Expr::Index(arr, i) => {
                        self.expr(arr)?;
                        self.expr(i)?;
                        self.code.push(QVMInstruction::Add);
                        self.code.push(QVMInstruction::Store("heap"));
                    }
                    _ => todo!(),
                }
            }
            Statement::Loop(_) => todo!(),
            Statement::While(_, _) => todo!(),
        }

        Ok(())
    }

    fn statements(&mut self, statements: &Vec<Statement>) -> Result<()> {
        for statement in statements {
            self.statement(statement)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CodeGeneration {
    global_pointer: usize,
    globals: HashMap<String, usize>,
    structs: Structs,
}

impl CodeGeneration {
    pub fn new() -> CodeGeneration {
        CodeGeneration {
            global_pointer: 0,
            globals: HashMap::new(),
            structs: Structs(HashMap::new()),
        }
    }

    pub fn context(&mut self, structs: Structs) {
        self.structs = structs;
    }

    fn push_global(&mut self, name: String) {
        self.globals.insert(name, self.global_pointer);
        self.global_pointer += 1;
    }

    pub fn globals(&self) -> usize {
        self.globals.len()
    }

    fn variable(
        &mut self,
        name: &String,
        expr: &Expr,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator =
            Generator::new(HashMap::new(), &self.globals, labels, offset, &self.structs);
        generator.expr(expr)?;
        let mut code = generator.code;

        self.push_global(name.clone());
        code.push(QVMInstruction::AddrConst(self.globals[name]));
        code.push(QVMInstruction::Store("global"));

        Ok(code)
    }

    fn function(
        &mut self,
        function: &Function,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut args = HashMap::new();
        for (index, (name, _)) in function.args.iter().enumerate() {
            args.insert(name.clone(), index);
        }

        let mut generator = Generator::new(args, &self.globals, labels, offset, &self.structs);
        for b in &function.body {
            generator.statement(b)?;
        }
        let code = generator.code;

        Ok(code)
    }

    fn call_main(
        &mut self,
        labels: &mut HashMap<String, usize>,
        offset: usize,
    ) -> Result<Vec<QVMInstruction>> {
        let mut generator =
            Generator::new(HashMap::new(), &self.globals, labels, offset, &self.structs);
        generator.statement(&Statement::Return(Expr::Call(
            Box::new(Expr::Var("main".to_string())),
            vec![],
        )))?;

        Ok(generator.code)
    }

    pub fn generate(&mut self, module: &Module) -> Result<Vec<QVMInstruction>> {
        let mut code = vec![];

        let mut var_decls = vec![];
        let mut function_decls = vec![];
        for m in &module.0 {
            match m {
                Declaration::Function(f) => {
                    function_decls.push(f);
                }
                Declaration::Variable(n, e) => {
                    var_decls.push((n, e));
                }
                Declaration::Struct(_) => {}
            }
        }

        let mut labels = HashMap::new();

        // first path
        for (name, expr) in var_decls {
            code.extend(self.variable(name, expr, &mut labels, code.len())?);
        }

        // call main
        labels.insert("main".to_string(), code.len());
        code.extend(self.call_main(&mut labels, code.len())?);

        for function in function_decls {
            labels.insert(function.name.to_string(), code.len());
            code.extend(self.function(function, &mut labels, code.len())?);
        }

        // resolve labels
        for i in 0..code.len() {
            if let QVMInstruction::LabelAddrConst(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::AddrConst(*pc);
                } else {
                    anyhow::bail!("label {} not found", label);
                }
            } else if let QVMInstruction::LabelJumpIfFalse(label) = &code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::JumpIfFalse(*pc);
                } else {
                    anyhow::bail!("label {} not found", label);
                }
            }
        }

        Ok(code)
    }
}

pub fn generate(module: &Module) -> Result<Vec<QVMInstruction>> {
    let mut g = CodeGeneration::new();
    let code = g.generate(module)?;

    Ok(code)
}
