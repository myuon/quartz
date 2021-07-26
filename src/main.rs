use std::{collections::HashMap, io::Read};

use anyhow::Result;

use crate::{
    code_gen::gen_code,
    parser::run_parser,
    runtime::{execute, FFIFunction},
    typechecker::typecheck,
    vm::{HeapData, StackData},
};

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;
mod typechecker;
mod vm;

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

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let (ffi_table, ffi_functions) = create_ffi_table();

    let mut module = run_parser(&buffer)?;
    typecheck(&mut module)?;

    let (program, static_area) = gen_code(module, ffi_table)?;
    println!("{:?}", execute(program, static_area, ffi_functions));

    Ok(())
}
