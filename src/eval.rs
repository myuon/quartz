use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::ast::{DataValue, Declaration, Expr, Module, Statement, Type};

type NativeFunction = Box<dyn Fn(Vec<DataValue>) -> Result<DataValue>>;

fn new_native_functions() -> HashMap<String, NativeFunction> {
    let mut natives = HashMap::<String, NativeFunction>::new();
    natives.insert(
        "_add".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? + args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_sub".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? - args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_mult".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? * args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_leq".to_string(),
        Box::new(|args| {
            Ok(DataValue::Bool(
                args[0].clone().as_int()? <= args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_eq".to_string(),
        Box::new(|args| {
            Ok(DataValue::Bool(
                args[0].clone().as_int()? == args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_tuple".to_string(),
        Box::new(|args| {
            Ok(DataValue::Tuple(
                args.into_iter().map(|arg| arg.clone()).collect(),
            ))
        }),
    );
    natives.insert(
        "_len_string".to_string(),
        Box::new(|args| Ok(DataValue::Int(args[0].clone().as_string()?.len() as i32))),
    );
    natives.insert(
        "_slice_string".to_string(),
        Box::new(|args| {
            Ok(DataValue::String(
                args[0].clone().as_string()?
                    [args[1].clone().as_int()? as usize..args[2].clone().as_int()? as usize]
                    .to_string(),
            ))
        }),
    );
    natives.insert(
        "_concat_string".to_string(),
        Box::new(|args| {
            Ok(DataValue::String(
                args[0].clone().as_string()? + &args[1].clone().as_string()?,
            ))
        }),
    );
    natives.insert(
        "_vec".to_string(),
        Box::new(|args| {
            Ok(DataValue::Tuple(
                args.into_iter().map(|arg| arg.clone()).collect(),
            ))
        }),
    );
    natives.insert(
        "_print".to_string(),
        Box::new(|args| {
            println!("{:?}", args);

            Ok(DataValue::Nil)
        }),
    );

    natives
}

pub struct Evaluator {
    variables: HashMap<String, DataValue>,
    natives: HashMap<String, NativeFunction>,
    escape_return: Option<DataValue>,
    struct_types: HashMap<String, Vec<(String, Type)>>,
    function_types: HashMap<
        String,
        (
            Vec<(String, Type)>, // argument types
            Box<Type>,           // return type
            Vec<Statement>,      // body
        ),
    >,
    method_types: HashMap<
        (String, String), // receiver type, method name
        (
            String,              // receiver name
            Vec<(String, Type)>, // argument types
            Box<Type>,           // return type
            Vec<Statement>,      // body
        ),
    >,
}

impl Evaluator {
    pub fn new(
        struct_types: HashMap<String, Vec<(String, Type)>>,
        functions: HashMap<
            String,
            (
                Vec<(String, Type)>, // argument types
                Box<Type>,           // return type
                Vec<Statement>,      // body
            ),
        >,
        methods: HashMap<
            (String, String), // receiver type, method name
            (
                String,              // receiver name
                Vec<(String, Type)>, // argument types
                Box<Type>,           // return type
                Vec<Statement>,      // body
            ),
        >,
    ) -> Self {
        Evaluator {
            variables: HashMap::new(),
            natives: new_native_functions(),
            escape_return: None,
            struct_types,
            function_types: functions,
            method_types: methods,
        }
    }

    fn load(&self, name: &String) -> Result<DataValue> {
        if self.natives.contains_key(name) {
            Ok(DataValue::NativeFunction(name.to_string()))
        } else if self.function_types.contains_key(name) {
            Ok(DataValue::Function(name.clone()))
        } else {
            self.variables
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Variable {} not found", name))
        }
    }

    pub fn eval_statement(&mut self, stmt: Statement) -> Result<DataValue> {
        match stmt {
            Statement::Let(x, e) => {
                let value = self.eval_expr(e)?;
                self.variables.insert(x.to_string(), value);
            }
            Statement::Expr(e) => {
                self.eval_expr(e)?;
            }
            Statement::Return(e) => {
                self.escape_return = Some(self.eval_expr(e)?);
            }
            Statement::If(cond, body1, body2) => {
                let result = if self.eval_expr(cond.as_ref().clone())?.as_bool()? {
                    self.eval_statements(body1)?
                } else {
                    self.eval_statements(body2)?
                };

                return Ok(result);
            }
            Statement::Continue => todo!(),
            Statement::Assignment(lhs, rhs) => {
                let value = self.eval_expr(rhs)?;

                // TODO: ちゃんと左辺値計算をしましょう
                match lhs.as_ref() {
                    Expr::Var(v) => {
                        self.variables.insert(v.to_string(), value);
                    }
                    Expr::Project(false, name, e, field) => match e.as_ref() {
                        Expr::Var(v) => {
                            let index = self
                                .struct_types
                                .get(name)
                                .ok_or_else(|| anyhow::anyhow!("Variable {} not found", name))?
                                .iter()
                                .position(|(n, _)| n == field)
                                .unwrap();
                            let st = self.variables[v].clone();
                            let mut tuple = st.as_tuple()?;
                            tuple[index] = value;

                            self.variables.insert(v.clone(), DataValue::Tuple(tuple));
                        }
                        _ => bail!("Unsupported LHS: {:?}", lhs),
                    },
                    _ => bail!("Unsupported LHS: {:?}", lhs),
                }
            }
        }

        Ok(DataValue::Nil)
    }

    pub fn eval_statements(&mut self, statements: Vec<Statement>) -> Result<DataValue> {
        let mut result = DataValue::Nil;
        for stmt in statements.clone() {
            result = self.eval_statement(stmt)?;
            if self.escape_return.is_some() {
                return Ok(DataValue::Nil);
            }
        }

        Ok(result)
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<DataValue> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => Ok(lit.into_datatype()),
            Expr::Call(caller, args) => {
                let f = self.eval_expr(caller.as_ref().clone())?;
                let args = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;

                match f {
                    DataValue::NativeFunction(n) => self.natives[&n](args),
                    DataValue::Function(name) => {
                        let func = self.function_types[&name].clone();

                        let variables_snapshot = self.variables.clone();

                        // ここは本来extendしてはならない(元の環境を引き継いでしまうから)
                        // ただしそれで問題になるようなケースはtypechecker時に弾かれるので今のところは問題になっていない？(要調査)
                        self.variables.extend(
                            func.0
                                .into_iter()
                                .map(|(k, _)| k)
                                .zip(args)
                                .map(|(name, value)| (name.clone(), value)),
                        );

                        let result = self.eval_statements(func.2)?;
                        self.variables = variables_snapshot;

                        if let Some(ret) = self.escape_return.clone() {
                            assert_eq!(result, DataValue::Nil);
                            self.escape_return = None;

                            return Ok(ret);
                        }

                        Ok(result)
                    }
                    DataValue::Method(typ, name, receiver) => {
                        let method = self.method_types[&(typ.clone(), name.clone())].clone();

                        let variables_snapshot = self.variables.clone();

                        self.variables.extend(
                            method
                                .1
                                .into_iter()
                                .map(|(k, _)| k)
                                .zip(args)
                                .map(|(name, value)| (name.clone(), value)),
                        );
                        self.variables
                            .insert(method.0.clone(), receiver.as_ref().clone());

                        let result = self.eval_statements(method.3)?;
                        self.variables = variables_snapshot;

                        if let Some(ret) = self.escape_return.clone() {
                            assert_eq!(result, DataValue::Nil);
                            self.escape_return = None;

                            return Ok(ret);
                        }

                        Ok(result)
                    }
                    _ => {
                        bail!("{:?} is not a function", f);
                    }
                }
            }
            Expr::Loop(body) => loop {
                self.eval_statements(body.clone())?;
                if self.escape_return.is_some() {
                    return Ok(DataValue::Nil);
                }
            },
            Expr::Struct(_, fields) => {
                let mut values = Vec::new();
                for (_, e) in fields {
                    values.push(self.eval_expr(e)?);
                }

                Ok(DataValue::Tuple(values))
            }
            Expr::Project(is_method, name, e, field) if is_method => {
                let value = self.eval_expr(e.as_ref().clone())?;

                Ok(DataValue::Method(name, field, Box::new(value)))
            }
            Expr::Project(_, name, e, field) => {
                let index = self
                    .struct_types
                    .get(&name)
                    .ok_or_else(|| anyhow::anyhow!("Variable {} not found", name))?
                    .iter()
                    .position(|(n, _)| *n == field)
                    .unwrap();
                let value = self.eval_expr(e.as_ref().clone())?;

                Ok(value.as_tuple()?.remove(index))
            }
        }
    }

    pub fn eval_decl(&mut self, decl: Declaration) -> Result<DataValue> {
        match decl {
            Declaration::Function(_) => {}
            Declaration::Variable(x, expr) => {
                let val = self.eval_expr(expr)?;
                self.variables.insert(x, val);
            }
            Declaration::Struct(_) => {}
        }

        Ok(DataValue::Nil)
    }

    pub fn eval_module(&mut self, m: Module) -> Result<DataValue> {
        for decl in m.0 {
            self.eval_decl(decl)?;
        }

        self.eval_expr(Expr::Call(
            Box::new(Expr::Var(String::from("main"))),
            vec![],
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn test_eval() -> Result<()> {
        let cases = vec![
            (
                // main
                r#"
                    fn main() {
                        return 10;
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // function call
                r#"
                    fn f() {
                        return 10;
                    }

                    fn main() {
                        return f();
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // add
                r#"
                    fn main() {
                        return _add(1, 2);
                    }
                "#,
                DataValue::Int(3),
            ),
            (
                // if
                r#"
                    fn check_bool(b) {
                        if b {
                            return 10;
                        } else {
                            return 20;
                        };
                    }

                    fn main() {
                        return check_bool(false);
                    }
                "#,
                DataValue::Int(20),
            ),
            (
                // recursion
                r#"
                    fn count_up(n) {
                        if _eq(n, 5) {
                            return true;
                        } else {
                            return count_up(_add(n, 1));
                        };
                    }

                    fn main() {
                        return count_up(1);
                    }
                "#,
                DataValue::Bool(true),
            ),
            (
                // factorial
                r#"
                    fn factorial(n) {
                        if _eq(n, 0) {
                            return 1;
                        } else {
                            return _mult(n, factorial(_sub(n, 1)));
                        };
                    }

                    fn main() {
                        return factorial(5);
                    }
                "#,
                DataValue::Int(120),
            ),
            (
                // global variables
                r#"
                    let x = 10;

                    fn main() {
                        return x;
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // reassignment
                r#"
                    fn f(b) {
                        let x = 0;

                        if b {
                            x = 10;
                        } else {
                        };

                        return x;
                    }

                    fn main() {
                        return f(true);
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // loop
                r#"
                    fn fib(n) {
                        let a = 1;
                        let b = 1;
                        let counter = 0;

                        loop {
                            if _eq(counter, n) {
                                return b;
                            };

                            let c = _add(a, b);
                            a = b;
                            b = c;

                            counter = _add(counter, 1);
                        };
                    }

                    fn main() {
                        return fib(10);
                    }
                "#,
                DataValue::Int(144),
            ),
            (
                // struct
                r#"
                    struct Foo {
                        x: int,
                        y: int,
                    }

                    fn main() {
                        let foo = Foo { x: 10, y: 20 };

                        return Foo {
                            x: foo.y,
                            y: _add(foo.x, foo.y),
                        };
                    }
                "#,
                DataValue::Tuple(vec![DataValue::Int(20), DataValue::Int(30)]),
            ),
            (
                // method calling
                r#"
                    struct Foo {
                        x: int,
                        y: int,
                    }

                    fn (foo: Foo) sum() {
                        return _add(foo.x, foo.y);
                    }

                    fn main() {
                        let foobar = Foo { x: 10, y: 20 };

                        return foobar.sum();
                    }
                "#,
                DataValue::Int(30),
            ),
            (
                // method for basic types
                r#"
                    fn main() {
                        return _tuple(100.add(200));
                    }
                "#,
                DataValue::Tuple(vec![DataValue::Int(300)]),
            ),
            (
                // reverse a string
                r#"
                    fn main() {
                        let s = "hello";
                        let r = "";
                        let i = 0;

                        loop {
                            if i.eq(s.len()) {
                                return r;
                            };

                            r = r.concat(s.slice(s.len().sub(i).sub(1), s.len().sub(i)));
                            i = i.add(1);
                        };
                    }
                "#,
                DataValue::String("olleh".to_string()),
            ),
            (
                // modify via method
                r#"
                    struct Foo {
                        x: int,
                        y: int,
                    }

                    fn (foo: Foo) add(n) {
                        foo.x = foo.x.add(n);
                    }

                    fn main() {
                        let foobar = Foo { x: 10, y: 20 };
                        foobar.add(10);

                        return foobar.x;
                    }
                "#,
                DataValue::Int(10),
            ),
        ];

        for (input, want) in cases {
            let compiler = Compiler::new();
            let result = compiler
                .exec(input)
                .map_err(|err| err.context(format!("{}", input)))?;
            assert_eq!(want, result, "{}", input);
        }

        Ok(())
    }

    #[test]
    fn test_eval_fail() -> Result<()> {
        let cases = vec![
            (
                // variable pollution for a function
                r#"
                    fn foo() {
                        return a;
                    }

                    fn main() {
                        let a = 10;

                        return foo();
                    }
                "#,
                "Variable a not found",
            ),
            (
                // variable pollution for a method
                r#"
                    struct Foo {
                        a: int,
                    }

                    fn (foo: Foo) get_a() {
                        return bar.a;
                    }

                    fn main() {
                        let bar = Foo { a: 100 };

                        return bar.get_a();
                    }
                "#,
                "Variable bar not found",
            ),
        ];

        for (input, want) in cases {
            let compiler = Compiler::new();
            let result = compiler.exec(input).unwrap_err();
            assert!(
                result.root_cause().to_string().contains(want),
                "\nWant: {}\nGot: {}\n{}",
                want,
                result,
                input
            );
        }

        Ok(())
    }
}
