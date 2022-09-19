use std::collections::HashMap;

use anyhow::{bail, Result};

pub struct PrettyPrinter {
    buffer: String,
    indent: usize,
    depth: usize,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent: 4,
            depth: 0,
        }
    }

    pub fn writeln(&mut self, s: &str) {
        for line in s.lines() {
            self.buffer.push_str(&" ".repeat(self.depth * self.indent));
            self.buffer.push_str(line);
            self.buffer.push_str("\n");
        }
    }

    pub fn indent(&mut self) {
        self.depth += 1;
    }

    pub fn dedent(&mut self) {
        self.depth -= 1;
    }

    pub fn item(&mut self, key: &str, value: &str) {
        self.writeln(&format!("{}: {}", key, value));
    }

    pub fn item_verbose(&mut self, key: &str, value: &str) {
        self.item(key, "");
        self.indent();
        self.writeln(value);
        self.dedent();
    }

    pub fn finalize(self) -> String {
        self.buffer
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Source<T> {
    pub data: T,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl<T> Source<T> {
    pub fn new(data: T, start: usize, end: usize) -> Source<T> {
        Source {
            data,
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn unknown(data: T) -> Source<T> {
        Source {
            data,
            start: None,
            end: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Array(Vec<Source<Expr>>, Type),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(String, Source<Expr>),
    Expr(Source<Expr>, Type),
    Return(Source<Expr>),
    If(
        Box<Source<Expr>>,
        Vec<Source<Statement>>,
        Vec<Source<Statement>>,
    ),
    Continue,
    Assignment(Source<Expr>, Source<Expr>),
    While(Source<Expr>, Vec<Source<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum CallMode {
    Function,
    SizedArray,
    Array,
}

#[derive(PartialEq, Debug, Clone)]
pub enum OptionalMode {
    Nil,
    Some,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Var(Vec<String>), // qualifier in vector
    PathVar(Box<Source<Expr>>, String, Vec<Type>),
    Make(Type, Vec<Source<Expr>>),
    Lit(Literal),
    Call(CallMode, Box<Source<Expr>>, Vec<Type>, Vec<Source<Expr>>),
    MethodCall(CallMode, Type, String, Box<Source<Expr>>, Vec<Source<Expr>>),
    AssociatedCall(CallMode, Type, String, Vec<Source<Expr>>),
    Struct(String, Vec<Type>, Vec<(String, Source<Expr>, Type)>),
    Project(Type, Box<Source<Expr>>, String),
    Deref(Box<Source<Expr>>, Type),
    As(Box<Source<Expr>>, Type, Type),
    Ref(Box<Source<Expr>>, Type),
    Address(Box<Source<Expr>>), // [compiler only] take the address of expr
    Optional(OptionalMode, Type, Box<Source<Expr>>),
    Unwrap(Box<Source<Expr>>, Type),
    TypeApp(Box<Source<Expr>>, Vec<Type>),
}

impl Expr {
    pub fn function_call(callee: Source<Expr>, args: Vec<Source<Expr>>) -> Expr {
        Expr::Call(CallMode::Function, Box::new(callee), vec![], args)
    }

    pub fn member(proj: Source<Expr>, field: impl Into<String>) -> Expr {
        Expr::Project(Type::Omit, Box::new(proj), field.into())
    }

    pub fn method_call(
        type_: Type,
        var: impl Into<String>,
        callee: Source<Expr>,
        args: Vec<Source<Expr>>,
    ) -> Expr {
        Expr::MethodCall(
            CallMode::Function,
            type_,
            var.into(),
            Box::new(callee),
            args,
        )
    }

    pub fn unwrap(expr: Source<Expr>) -> Expr {
        Expr::Unwrap(Box::new(expr), Type::Omit)
    }

    fn show_inner(&self, printer: &mut PrettyPrinter) {
        match self {
            Expr::Var(v) => {
                printer.writeln("<<Var>>");
                printer.item("name", &v.join("."));
            }
            Expr::PathVar(t, v, vs) => {
                printer.writeln("<<PathVar>>");
                printer.item_verbose("expr", &t.data.show());
                printer.item("name", v);
                printer.item_verbose("type_args", &format!("{:?}", vs));
            }
            Expr::Make(t, v) => {
                printer.writeln("<<Make>>");
                printer.item_verbose("type", &t.show());
                for (i, e) in v.iter().enumerate() {
                    printer.item(&format!("{}", i), &e.data.show());
                }
            }
            Expr::Lit(l) => {
                printer.writeln("<<Lit>>");
                match l {
                    Literal::Nil => {
                        printer.item("value", "nil");
                    }
                    Literal::Bool(b) => {
                        printer.item("value", &format!("{}", b));
                    }
                    Literal::Int(i) => {
                        printer.item("value", &format!("{}", i));
                    }
                    Literal::String(s) => {
                        printer.item("value", &format!("{}", s));
                    }
                    Literal::Array(v, t) => {
                        printer.item_verbose("type", &t.show());
                        for (i, e) in v.iter().enumerate() {
                            printer.item(&format!("{}", i), &e.data.show());
                        }
                    }
                }
            }
            Expr::Call(m, c, t, a) => {
                printer.writeln("<<Call>>");
                printer.item("mode", &format!("{:?}", m));
                printer.item_verbose("callee", &c.data.show());
                printer.item("params", &format!("{:?}", t));

                printer.item("args", "");
                printer.indent();
                for (i, e) in a.iter().enumerate() {
                    printer.item(&format!("{}", i), &e.data.show());
                }
                printer.dedent();
            }
            Expr::MethodCall(m, t, v, c, a) => {
                printer.writeln("<<MethodCall>>");
                printer.item("mode", &format!("{:?}", m));
                printer.item_verbose("type", &t.show());
                printer.item("name", v);
                printer.item_verbose("callee", &c.data.show());

                printer.item("args", "");
                printer.indent();
                for (i, e) in a.iter().enumerate() {
                    printer.item_verbose(&format!("{}", i), &e.data.show());
                }
                printer.dedent();
            }
            Expr::AssociatedCall(m, t, c, a) => {
                printer.writeln("<<AssociatedCall>>");
                printer.item("mode", &format!("{:?}", m));
                printer.item_verbose("type", &t.show());
                printer.item_verbose("callee", c);

                printer.item("args", "");
                printer.indent();
                for (i, e) in a.iter().enumerate() {
                    printer.item_verbose(&format!("{}", i), &e.data.show());
                }
                printer.dedent();
            }
            Expr::Struct(n, t, f) => {
                printer.writeln("<<Struct>>");
                printer.item("name", n);
                printer.item_verbose("params", &format!("{:?}", t));

                printer.item("fields", "");
                printer.indent();
                for (n, e, _) in f {
                    printer.item_verbose(&n, &e.data.show());
                }
                printer.dedent();
            }
            Expr::Project(t, e, f) => {
                printer.writeln("<<Project>>");
                printer.item_verbose("type", &t.show());
                printer.item_verbose("expr", &e.data.show());
                printer.item("field", f);
            }
            Expr::Deref(e, t) => {
                printer.writeln("<<Deref>>");
                printer.item_verbose("expr", &e.data.show());
                printer.item_verbose("type", &t.show());
            }
            Expr::As(e, t, r) => {
                printer.writeln("<<As>>");
                printer.item_verbose("expr", &e.data.show());
                printer.item_verbose("type", &t.show());
                printer.item_verbose("result", &r.show());
            }
            Expr::Ref(e, t) => {
                printer.writeln("<<Ref>>");
                printer.item_verbose("expr", &e.data.show());
                printer.item_verbose("type", &t.show());
            }
            Expr::Address(e) => {
                printer.writeln("<<Address>>");
                printer.item_verbose("expr", &e.data.show());
            }
            Expr::Optional(m, t, e) => {
                printer.writeln("<<Optional>>");
                printer.item("mode", &format!("{:?}", m));
                printer.item_verbose("type", &t.show());
                printer.item_verbose("expr", &e.data.show());
            }
            Expr::Unwrap(e, t) => {
                printer.writeln("<<Unwrap>>");
                printer.item_verbose("expr", &e.data.show());
                printer.item_verbose("type", &t.show());
            }
            Expr::TypeApp(e, t) => {
                printer.writeln("<<TypeApp>>");
                printer.item_verbose("expr", &e.data.show());

                printer.item("type", "");
                printer.indent();
                for (i, t) in t.iter().enumerate() {
                    printer.item(&format!("{}", i), &t.show());
                }
                printer.dedent();
            }
        }
    }

    pub fn show(&self) -> String {
        let mut printer = PrettyPrinter::new();

        self.show_inner(&mut printer);

        printer.finalize()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<(String, Type)>,
    pub dead_code: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: Source<String>,
    pub type_params: Vec<String>,
    pub args: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Source<Statement>>,
    pub dead_code: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Method(Source<String>, Vec<String>, Function),
    Function(Function),
    Variable(String, Source<Expr>, Type),
    Struct(Struct),
    Import(Source<String>),
}

impl Declaration {
    pub fn into_function(self) -> Result<Function> {
        match self {
            Declaration::Function(f) => Ok(f),
            _ => bail!("Expected function declaration, but found {:?}", self),
        }
    }

    // if the function is a method, [STRUCT_NAME]::[METHOD_NAME]
    pub fn function_path(&self) -> Option<String> {
        match self {
            Declaration::Method(typ, _, func) => Some(format!("{}::{}", typ.data, func.name.data)),
            Declaration::Function(func) => Some(func.name.data.clone()),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module {
    pub module_path: String,
    pub imports: Vec<String>,
    pub decls: Vec<Declaration>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    // for parse
    Self_,
    Omit,
    TypeVar(String),
    // for typecheck
    Infer(usize),
    Any,
    Nil, // literal "nil"
    Bool,
    Int,
    Byte,
    Fn(Vec<String>, Vec<Type>, Box<Type>),
    Method(Box<Type>, Vec<Type>, Box<Type>),
    Struct(String),
    Ref(Box<Type>),
    Array(Box<Type>),
    SizedArray(Box<Type>, usize),
    Optional(Box<Type>),
    TypeApp(Box<Type>, Vec<Type>),
}

impl Type {
    pub fn unwrap_type(&self) -> Result<&Type> {
        match self {
            Type::Optional(t) => Ok(t.as_ref()),
            Type::Ref(t) => Ok(t.as_ref()),
            _ => bail!("[type] Expected optional type, but found {:?}", self),
        }
    }

    pub fn as_fn_type(&self) -> Option<(&Vec<Type>, &Box<Type>)> {
        match self {
            Type::Fn(_, args, ret) => Some((args, ret)),
            Type::Method(_, args, ret) => Some((args, ret)),
            _ => None,
        }
    }

    pub fn is_method_type(&self) -> bool {
        match self {
            Type::Method(_, _, _) => true,
            _ => false,
        }
    }

    pub fn as_ref_type(&self) -> Option<&Box<Type>> {
        match self {
            Type::Ref(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_struct_type(&self) -> Option<&String> {
        match self {
            Type::Struct(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_sized_array(&self) -> Option<(&Box<Type>, &usize)> {
        match self {
            Type::SizedArray(t, size) => Some((t, size)),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Box<Type>> {
        match self {
            Type::Array(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_optional(&self) -> Option<&Box<Type>> {
        match self {
            Type::Optional(t) => Some(t),
            _ => None,
        }
    }

    pub fn type_app_or(typ: Type, params: Vec<Type>) -> Type {
        if params.is_empty() {
            typ
        } else {
            Type::TypeApp(Box::new(typ), params)
        }
    }

    pub fn has_infer(&self, index: usize) -> bool {
        match self {
            Type::Infer(t) => *t == index,
            Type::Any => false,
            Type::Bool => false,
            Type::Int => false,
            Type::Fn(_, args, ret) => {
                args.iter().find(move |t| t.has_infer(index)).is_some() || ret.has_infer(index)
            }
            Type::Method(self_, args, ret) => {
                self_.has_infer(index)
                    || args.iter().find(move |t| t.has_infer(index)).is_some()
                    || ret.has_infer(index)
            }
            Type::Struct(_) => false,
            Type::Ref(_) => todo!(),
            Type::Byte => false,
            Type::Array(t) => t.has_infer(index),
            Type::SizedArray(t, _n) => t.has_infer(index),
            Type::Optional(t) => t.has_infer(index),
            Type::Nil => false,
            Type::Self_ => false,
            Type::TypeApp(_, _) => todo!(),
            Type::TypeVar(_) => todo!(),
            Type::Omit => todo!(),
        }
    }

    pub fn subst(&mut self, index: usize, typ: &Type) {
        match self {
            Type::Infer(t) => {
                if *t == index {
                    *self = typ.clone();
                }
            }
            Type::Any => {}
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(_, args, ret) => {
                for arg in args {
                    arg.subst(index, typ);
                }

                ret.subst(index, typ);
            }
            Type::Method(self_, args, ret) => {
                self_.subst(index, typ);
                for arg in args {
                    arg.subst(index, typ);
                }

                ret.subst(index, typ);
            }
            Type::Struct(_) => {}
            Type::Ref(_) => todo!(),
            Type::Byte => {}
            Type::Array(t) => t.subst(index, typ),
            Type::SizedArray(t, _n) => t.subst(index, typ),
            Type::Optional(t) => t.subst(index, typ),
            Type::Nil => {}
            Type::Self_ => {}
            Type::TypeApp(_, _) => todo!(),
            Type::TypeVar(_) => todo!(),
            Type::Omit => todo!(),
        }
    }

    pub fn subst_typevar(&mut self, var_name: &String, typ: &Type) {
        match self {
            Type::Infer(_) => {}
            Type::Any => {}
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(_, args, ret) => {
                for arg in args {
                    arg.subst_typevar(var_name, typ);
                }

                ret.subst_typevar(var_name, typ);
            }
            Type::Method(self_, args, ret) => {
                self_.subst_typevar(var_name, typ);
                for arg in args {
                    arg.subst_typevar(var_name, typ);
                }

                ret.subst_typevar(var_name, typ);
            }
            Type::Struct(_) => {}
            Type::Ref(r) => {
                r.subst_typevar(var_name, typ);
            }
            Type::Byte => {}
            Type::Array(t) => t.subst_typevar(var_name, typ),
            Type::SizedArray(t, _n) => t.subst_typevar(var_name, typ),
            Type::Optional(t) => t.subst_typevar(var_name, typ),
            Type::Nil => {}
            Type::Self_ => {}
            Type::TypeApp(t, vs) => {
                t.subst_typevar(var_name, typ);
                for v in vs {
                    v.subst_typevar(var_name, typ);
                }
            }
            Type::TypeVar(t) => {
                if t == var_name {
                    *self = typ.clone();
                }
            }
            Type::Omit => todo!(),
        }
    }

    pub fn subst_struct_name(&mut self, var_name: &String, typ: &Type) {
        match self {
            Type::Infer(_) => {}
            Type::Any => {}
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(_, args, ret) => {
                for arg in args {
                    arg.subst_struct_name(var_name, typ);
                }

                ret.subst_struct_name(var_name, typ);
            }
            Type::Method(self_, args, ret) => {
                self_.subst_struct_name(var_name, typ);
                for arg in args {
                    arg.subst_struct_name(var_name, typ);
                }

                ret.subst_struct_name(var_name, typ);
            }
            Type::Struct(t) => {
                if t == var_name {
                    *self = typ.clone();
                }
            }
            Type::Ref(t) => {
                t.subst_struct_name(var_name, typ);
            }
            Type::Byte => {}
            Type::Array(t) => t.subst_struct_name(var_name, typ),
            Type::SizedArray(t, _n) => t.subst_struct_name(var_name, typ),
            Type::Optional(t) => t.subst_struct_name(var_name, typ),
            Type::Nil => {}
            Type::Self_ => {}
            Type::TypeApp(t, vs) => {
                t.subst_struct_name(var_name, typ);
                for v in vs {
                    v.subst_struct_name(var_name, typ);
                }
            }
            Type::TypeVar(_) => {}
            Type::Omit => todo!(),
        }
    }

    // Whether the representation is an adress or not
    pub fn is_struct(&self) -> bool {
        match self {
            Type::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Type::Optional(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Type::Nil => true,
            _ => false,
        }
    }

    pub fn method_selector_name(&self) -> Result<String> {
        Ok(match self {
            Type::Any => "any".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Int => "int".to_string(),
            Type::Byte => "byte".to_string(),
            Type::Struct(s) => s.to_string(),
            Type::Ref(r) => r.method_selector_name()?,
            Type::Array(_) => "array".to_string(),
            Type::SizedArray(_, _) => "sized_array".to_string(),
            Type::Optional(n) => n.method_selector_name()?,
            Type::TypeApp(t, _) => t.method_selector_name()?,
            s => bail!("{:?} is not a method selector", s),
        })
    }

    pub fn get_projection_type(&mut self, label: &String, structs: &Structs) -> Result<Type> {
        match self {
            Type::Struct(s) => structs.get_projection_type(s, &label),
            Type::TypeApp(t, ps) => match t.as_ref() {
                Type::Struct(s) => {
                    let struct_ = structs.0[s].clone();
                    let mut typ = structs.get_projection_type(s, &label)?;
                    for (pn, pt) in struct_.type_params.into_iter().zip(ps) {
                        typ.subst_typevar(&pn, &pt);
                    }
                    Ok(typ)
                }
                _ => bail!("Cannot project on {:?}", t),
            },
            Type::Ref(r) => r.get_projection_type(label, structs),
            t => bail!("[get_projection_type] {:?}", t),
        }
    }

    pub fn type_applications(&self) -> Result<Vec<Type>> {
        match self {
            Type::Struct(_) => Ok(vec![]),
            Type::TypeApp(t, ps) => {
                let mut ts = t.type_applications()?;
                ts.extend(ps.clone());
                Ok(ts)
            }
            Type::Ref(r) => r.type_applications(),
            _ => Ok(vec![]),
        }
    }

    fn show_inner(&self, printer: &mut PrettyPrinter) {
        match self {
            Type::Self_ => {
                printer.writeln("Self");
            }
            Type::Omit => {
                printer.writeln("Omit");
            }
            Type::TypeVar(v) => {
                printer.writeln("<<TypeVar>>");
                printer.item("value", v);
            }
            Type::Infer(t) => {
                printer.writeln("<<Infer>>");
                printer.item("value", &format!("{:?}", t));
            }
            Type::Any => {
                printer.writeln("Any");
            }
            Type::Nil => {
                printer.writeln("Nil");
            }
            Type::Bool => {
                printer.writeln("Bool");
            }
            Type::Int => {
                printer.writeln("Int");
            }
            Type::Byte => {
                printer.writeln("Byte");
            }
            Type::Fn(vs, args, ret) => {
                printer.writeln("<<Fn>>");
                printer.item("params", &format!("{:?}", vs));

                printer.item("args", "");
                printer.indent();
                for arg in args {
                    arg.show_inner(printer);
                }
                printer.dedent();

                printer.item("ret", "");
                printer.indent();
                ret.show_inner(printer);
                printer.dedent();
            }
            Type::Method(self_, args, ret) => {
                printer.writeln("<<Method>>");
                printer.item("self", "");
                printer.indent();
                self_.show_inner(printer);
                printer.dedent();

                printer.item("args", "");
                printer.indent();
                for arg in args {
                    arg.show_inner(printer);
                }
                printer.dedent();

                printer.item("ret", "");
                printer.indent();
                ret.show_inner(printer);
                printer.dedent();
            }
            Type::Struct(s) => {
                printer.writeln("<<Struct>>");
                printer.item("name", s);
            }
            Type::Ref(t) => {
                printer.writeln("<<Ref>>");
                printer.item("type", "");
                printer.indent();
                t.show_inner(printer);
                printer.dedent();
            }
            Type::Array(t) => {
                printer.writeln("<<Array>>");
                printer.item("type", "");
                printer.indent();
                t.show_inner(printer);
                printer.dedent();
            }
            Type::SizedArray(t, n) => {
                printer.writeln("<<SizedArray>>");
                printer.item("type", "");
                printer.indent();
                t.show_inner(printer);
                printer.dedent();
                printer.item("size", &format!("{}", n));
            }
            Type::Optional(t) => {
                printer.writeln("<<Optional>>");
                printer.item("type", "");
                printer.indent();
                t.show_inner(printer);
                printer.dedent();
            }
            Type::TypeApp(t, ps) => {
                printer.writeln("<<TypeApp>>");
                printer.item("type", "");
                printer.indent();
                t.show_inner(printer);
                printer.dedent();
                printer.item("params", "");
                printer.indent();
                for p in ps {
                    p.show_inner(printer);
                }
                printer.dedent();
            }
        }
    }

    pub fn show(&self) -> String {
        let mut printer = PrettyPrinter::new();

        self.show_inner(&mut printer);

        printer.finalize()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructTypeInfo {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<(String, Type)>,
}

impl StructTypeInfo {
    pub fn replace_params_in_fields(&mut self, type_params: &Vec<(String, Type)>) {
        for (p, pt) in type_params {
            for (_, t) in &mut self.fields {
                t.subst_typevar(p, pt);
            }
        }
    }

    pub fn normalize_type_params(&mut self) {
        for p in &mut self.type_params {
            let new_p = format!("?{}", p);
            for (_, f) in &mut self.fields {
                f.subst_typevar(p, &Type::TypeVar(new_p.clone()));
            }

            *p = new_p;
        }
    }

    pub fn field_labels(&self) -> Vec<&String> {
        self.fields.iter().map(|(l, _)| l).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Structs(pub HashMap<String, StructTypeInfo>);

impl Structs {
    pub fn get_projection_type(&self, val: &str, label: &str) -> Result<Type> {
        let struct_fields = self.0.get(val).ok_or(anyhow::anyhow!(
            "project type: {} not found in {}",
            label,
            val
        ))?;

        let (_, t) = struct_fields
            .fields
            .iter()
            .find(|(name, _)| name == label)
            .ok_or(anyhow::anyhow!(
                "project type: {} not found in {}",
                label,
                val
            ))?;

        Ok(t.clone())
    }

    pub fn get_projection_offset(&self, val: &str, label: &str) -> Result<usize> {
        let struct_fields =
            self.0
                .get(val)
                .ok_or(anyhow::anyhow!("project: {} not found in {}", label, val))?;

        let mut index = 0;
        for (l, _) in &struct_fields.fields {
            if l == label {
                break;
            }

            index += 1;
        }

        Ok(index)
    }
}

#[derive(Debug, Clone)]
pub struct Functions(
    pub  HashMap<
        String,
        (
            Vec<(String, Type)>,    // argument types
            Box<Type>,              // return type
            Vec<Source<Statement>>, // body
        ),
    >,
);

#[derive(Debug, Clone)]
pub struct MethodTypeInfo {
    pub name: String,
    pub type_params: Vec<String>,
    pub args: Vec<Type>,
    pub ret: Type,
}

impl MethodTypeInfo {
    pub fn apply(&mut self, type_params: &Vec<Type>) {
        assert_eq!(self.type_params.len(), type_params.len());

        for (p, pt) in self.type_params.iter().zip(type_params) {
            for a in &mut self.args {
                a.subst_typevar(p, pt);
            }
            self.ret.subst_typevar(p, pt);
        }

        self.type_params = vec![];
    }

    pub fn as_fn_type(&self) -> Type {
        Type::Fn(
            self.type_params.clone(),
            self.args.iter().map(|t| t.clone()).collect::<Vec<Type>>(),
            Box::new(self.ret.clone()),
        )
    }
}
