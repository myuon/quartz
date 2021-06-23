use std::{collections::HashMap, io::Read};

use anyhow::Result;

use crate::{
    code_gen::gen_code,
    parser::run_parser,
    runtime::{execute, FFIFunction},
    vm::{HeapData, StackData},
};

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;
mod vm;

pub fn create_ffi_table() -> (HashMap<String, usize>, Vec<FFIFunction>) {
    let mut ffi_table: Vec<(String, FFIFunction)> = vec![];
    ffi_table.push((
        "_add".to_string(),
        Box::new(|mut vs: Vec<StackData>, _| {
            let x = vs.pop().unwrap();
            let y = vs.pop().unwrap();

            match (x, y) {
                (StackData::Int(x), StackData::Int(y)) => vs.push(StackData::Int(x + y)),
                _ => todo!(),
            }

            vs
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
                StackData::StackRevAddr(p) => {
                    println!("{:?}", stack[p]);
                }
                _ => println!("{:?}", x),
            }
            stack.push(StackData::Nil);

            stack
        }),
    ));
    ffi_table.push((
        "_eq".to_string(),
        Box::new(|mut vs: Vec<StackData>, heap: Vec<HeapData>| {
            let x = vs.pop().unwrap();
            let y = vs.pop().unwrap();

            match (x, y) {
                (StackData::Int(x), StackData::Int(y)) => {
                    vs.push(StackData::Int(if x == y { 0 } else { 1 }))
                }
                (StackData::HeapAddr(s), StackData::HeapAddr(t)) => match (&heap[s], &heap[t]) {
                    (HeapData::String(a), HeapData::String(b)) => {
                        vs.push(StackData::Int(if a == b { 0 } else { 1 }))
                    }
                    (s, t) => panic!("{:?} {:?}", s, t),
                },
                (x, y) => panic!("{:?} {:?}", x, y),
            }

            vs
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

    println!(
        "{:?}",
        execute(gen_code(run_parser(&buffer)?, ffi_table)?, ffi_functions)
    );

    Ok(())
}
