use std::io::Read;

use anyhow::{Context, Result};
use thiserror::Error;

use crate::{
    generator::Generator, ir::IrTerm, ir_code_gen::IrCodeGenerator, lexer::Lexer, parser::Parser,
    typecheck::TypeChecker,
};

#[derive(Debug, Error)]
#[error("Found error in span ({start:?},{end:?})")]
pub struct ErrorInSource {
    pub start: usize,
    pub end: usize,
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    fn compile_(&mut self, input: &str) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();
        let mut generator = Generator::new();
        let mut ir_code_generator = IrCodeGenerator::new();

        let mut loaded_modules = vec![];

        lexer.run(input).context("lexer phase")?;
        loaded_modules.push(parser.run(lexer.tokens).context("parser phase")?);

        while let Some(path) = parser.imports.pop() {
            let mut lexer = Lexer::new();
            let mut parser = Parser::new();

            assert_eq!(path.0.len(), 1);

            let mut file = std::fs::File::open(format!("{}.rs", path.0[0].clone().as_str()))
                .context(format!("opening file {}.rs", path.0[0].clone().as_str()))?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).context("reading file")?;

            lexer.run(input).context("lexer phase")?;
            loaded_modules.push(parser.run(lexer.tokens).context("parser phase")?);
        }

        loaded_modules.reverse();

        typechecker
            .run(&mut loaded_modules)
            .context("typechecker phase")?;

        ir_code_generator.set_types(typechecker.types.clone());
        let ir = ir_code_generator
            .run(&mut loaded_modules)
            .context("ir code generator phase")?;

        let mut decls = vec![];
        for term in ir {
            match term {
                IrTerm::Module { elements } => {
                    decls.extend(elements);
                }
                _ => todo!(),
            }
        }

        let mut ir = IrTerm::Module { elements: decls };

        generator.set_globals(typechecker.globals.keys().into_iter().cloned().collect());
        generator.set_types(typechecker.types);
        generator.run(&mut ir).context("generator phase")?;

        Ok(generator.writer.buffer)
    }

    pub fn compile(&mut self, input: &str) -> Result<String> {
        let mut input = input.to_string();
        input.push_str(
            r#"
fun println(s: string) {
    let l = s.length;
    let d = s.data;
    let n = 0;

    // >>
    write_stdout(62 as byte);
    write_stdout(62 as byte);
    write_stdout(32 as byte);

    while n < l {
        write_stdout(d.at(n));
        n = n + 1;
    }

    // $\newline
    write_stdout(36 as byte);
    write_stdout(10 as byte);
}

fun vec_make(init_capacity: i32, size: i32): vec[i32] {
    let capacity = init_capacity;
    let length = 0;
    let data = alloc(capacity * size);
    return vec {
        data: data,
        length: length,
        capacity: capacity,
    };
}

fun vec_push(v: vec[i32], e: i32) {
    if (v.length + 1) == v.capacity {
        let new_capacity = v.capacity * 2;
        let new_data = alloc(new_capacity);
        mem_copy(v.data, new_data, v.length * sizeof[i32]());
        mem_free(v.data);
        v.data = new_data;
        v.capacity = new_capacity;
    }

    v.data.at(v.length) = e;
    v.length = v.length + 1;
}

fun digit_to_string(digit: i32): string {
    return char_to_string(digit as byte);
}

fun char_to_string(char: byte): string {
    let s = make[array[byte,1]]();
    s.at(0) = char;

    return string {
        data: s.data,
        length: s.length,
    };
}

fun new_empty_string(length: i32): string {
    return string {
        data: alloc(length) as ptr[byte],
        length: length,
    };
}
"#,
        );

        self.compile_(&input).map_err(|error| {
            if let Some(source) = error.downcast_ref::<ErrorInSource>() {
                let start = source.start;
                let end = source.end;

                let (start_line_number, start_column_index) = find_position(&input, start);
                let start_line = input.lines().nth(start_line_number).unwrap();

                let (end_line_number, end_column_index) = find_position(&input, end);

                let line_number_gutter = format!("{}: ", start_line_number);

                error.context(format!(
                    "Error at (line.{}:{}) to (line.{}:{})\n{}{}\n{}{}",
                    start_line_number,
                    start_column_index,
                    end_line_number,
                    end_column_index,
                    line_number_gutter,
                    start_line,
                    " ".repeat(line_number_gutter.len() + start_column_index),
                    "^".repeat(end - start)
                ))
            } else {
                error
            }
        })
    }
}

fn find_position(input: &str, position: usize) -> (usize, usize) {
    let mut line_number = 0;
    let mut count = 0;
    for line in input.lines() {
        if count + line.len() > position {
            break;
        }

        line_number += 1;
        count += line.len() + 1;
    }

    (
        line_number,
        if position > count {
            position - count
        } else {
            0
        },
    )
}
