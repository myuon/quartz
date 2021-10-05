use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ast::{Module, Statement, Type},
    runtime::FFIFunction,
    typechecker::{typecheck_statements_with, typecheck_with},
    vm::{HeapData, StackData},
};

pub fn stdlib() -> HashMap<String, Type> {
    vec![
        ("_object", Type::Any),
        ("_vec", Type::Any),
        (
            "_vpush",
            Type::Fn(vec![Type::Any, Type::Any], Box::new(Type::Unit)),
        ),
        ("_passign", Type::Any),
        ("_tuple", Type::Any),
        ("_get", Type::Any),
        ("_set", Type::Any),
        (
            "_regex",
            Type::Fn(vec![Type::String, Type::String], Box::new(Type::Bool)),
        ),
        ("_print", Type::Any),
        ("_panic", Type::Any),
        (
            "_add",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_eq",
            Type::Fn(vec![Type::Any, Type::Any], Box::new(Type::Bool)),
        ),
        ("_free", Type::Fn(vec![Type::Any], Box::new(Type::Unit))),
        (
            "_slice",
            Type::Fn(vec![Type::Any, Type::Int, Type::Int], Box::new(Type::Any)),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

pub fn typecheck_with_stdlib(m: &mut Module) -> Result<()> {
    return typecheck_with(m, stdlib());
}

pub fn typecheck_statements_with_stdlib(m: &mut Vec<Statement>) -> Result<()> {
    return typecheck_statements_with(m, stdlib());
}

pub fn create_ffi_table() -> (HashMap<String, usize>, Vec<FFIFunction>) {
    let mut ffi_table: Vec<(String, FFIFunction)> = vec![];
    ffi_table.push((
        "_add".to_string(),
        Box::new(|mut stack: Vec<StackData>, heap: Vec<HeapData>| {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();

            match (x, y) {
                (StackData::Int(x), StackData::Int(y)) => {
                    stack.push(StackData::Int(x + y));
                }
                (x, y) => panic!("{:?} {:?}", x, y),
            }

            (stack, heap)
        }),
    ));
    ffi_table.push((
        "_print".to_string(),
        Box::new(|mut stack: Vec<StackData>, heap: Vec<HeapData>| {
            let x = stack.pop().unwrap();
            match x {
                StackData::HeapAddr(p) => {
                    println!("{:?}", heap[p]);
                }
                StackData::StackAddr(p) => {
                    println!("{:?}", stack[p]);
                }
                _ => println!("{:?}", x),
            }
            stack.push(StackData::Nil);

            (stack, heap)
        }),
    ));
    ffi_table.push((
        "_eq".to_string(),
        Box::new(|mut stack: Vec<StackData>, heap: Vec<HeapData>| {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();

            match (x, y) {
                (StackData::Bool(x), StackData::Bool(y)) => {
                    stack.push(StackData::Bool(x == y));
                }
                (StackData::Int(x), StackData::Int(y)) => {
                    stack.push(StackData::Bool(x == y));
                }
                (StackData::HeapAddr(x), StackData::HeapAddr(y)) => {
                    match (heap[x].clone(), heap[y].clone()) {
                        (HeapData::String(a), HeapData::String(b)) => {
                            stack.push(StackData::Bool(a == b));
                        }
                        (x, y) => panic!("{:?} {:?}", x, y),
                    }
                }
                (x, y) => panic!("{:?} {:?}", x, y),
            }

            (stack, heap)
        }),
    ));
    ffi_table.push((
        "_not".to_string(),
        Box::new(|mut stack: Vec<StackData>, heap: Vec<HeapData>| {
            let x = stack.pop().unwrap();

            match x {
                StackData::Bool(x) => {
                    stack.push(StackData::Bool(!x));
                }
                x => panic!("{:?}", x),
            }

            (stack, heap)
        }),
    ));

    let enumerated = ffi_table.into_iter().enumerate().collect::<Vec<_>>();

    let variables = enumerated
        .iter()
        .map(|(i, (k, _))| (k.clone(), *i))
        .collect::<HashMap<_, _>>();
    let table = enumerated.into_iter().map(|(_, (_, v))| v).collect();

    (variables, table)
}
