use std::io::{BufWriter, Write};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Pattern, Statement, Type},
    util::{ident::Ident, source::Source},
};

#[derive(Clone)]
pub struct Formatter<'s> {
    source: &'s str,
    indent_size: usize,
    depth: usize,
    column: usize,
}

impl<'s> Formatter<'s> {
    pub fn new(source: &'s str) -> Formatter {
        Formatter {
            source,
            indent_size: 4,
            depth: 0,
            column: 0,
        }
    }

    pub fn format(&mut self, module: Module) -> String {
        let mut buf = BufWriter::new(Vec::new());
        self.module(&mut buf, module);
        self.write_newline(&mut buf);

        String::from_utf8(buf.into_inner().unwrap()).unwrap()
    }

    pub fn module(&mut self, writer: &mut impl Write, module: Module) {
        for decl in module.0 {
            self.decl(writer, decl);
        }
    }

    fn decl(&mut self, writer: &mut impl Write, decl: Decl) {
        match decl {
            Decl::Func(func) => self.func(writer, func),
            Decl::Let(_, _, _) => todo!(),
            Decl::Type(_, _) => todo!(),
            Decl::Module(_, _) => todo!(),
            Decl::Import(_) => todo!(),
        }
    }

    fn func(&mut self, writer: &mut impl Write, func: Func) {
        self.write(writer, "fun");
        self.write(writer, func.name.data.as_str());
        self.params(writer, func.params);
        if let Type::Nil = func.result {
        } else {
            self.write_no_space(writer, ":");
            self.write(writer, func.result.to_string().as_str());
        }
        self.statements(writer, func.body);
    }

    fn params(&mut self, writer: &mut impl Write, params: Vec<(Ident, Type)>) {
        self.write_no_space(writer, "(");

        let mut blocks = vec![];
        for (ident, type_) in params {
            blocks.push(format!("{}: {}", ident.as_str(), type_.to_string()));
        }
        self.write_block_oneline(writer, blocks, ",");
        self.write_no_space(writer, ")");
    }

    fn statements(&mut self, writer: &mut impl Write, stmts: Vec<Source<Statement>>) {
        let mut blocks = vec![];
        let mut prev_line = stmts.get(0).map(|t| t.start).flatten().unwrap_or(0);
        for stmt in stmts {
            let mut fwriter = Formatter::new(self.source);
            let mut buf = BufWriter::new(Vec::new());
            fwriter.statement(&mut buf, stmt.data);

            blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
        }

        self.write(writer, "{");
        self.write_block(writer, blocks, ";");
        self.write(writer, "}");
    }

    fn statement(&mut self, writer: &mut impl Write, stmt: Statement) {
        match stmt {
            Statement::Let(pattern, type_, expr) => {
                self.write(writer, "let");
                self.pattern(writer, pattern.data);
                if let Type::Omit(_) = type_ {
                } else {
                    self.write_no_space(writer, ":");
                    self.write(writer, type_.to_string().as_str());
                }
                self.write(writer, "=");
                self.expr(writer, expr.data);
            }
            Statement::Return(expr) => {
                self.write(writer, "return");
                self.expr(writer, expr.data);
            }
            Statement::Expr(_, _) => todo!(),
            Statement::Assign(_, _, _) => todo!(),
            Statement::If(_, _, _, _) => todo!(),
            Statement::While(_, _) => todo!(),
            Statement::For(_, _, _, _) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
        }
    }

    fn pattern(&mut self, writer: &mut impl Write, pattern: Pattern) {
        match pattern {
            Pattern::Ident(ident) => {
                self.write(writer, ident.as_str());
            }
            Pattern::Or(_, _) => todo!(),
            Pattern::Omit => todo!(),
        }
    }

    fn expr(&mut self, writer: &mut impl Write, expr: Expr) {
        match expr {
            Expr::Ident { ident, .. } => {
                self.write(writer, ident.as_str());
            }
            Expr::Self_ => {
                self.write(writer, "self");
            }
            Expr::Lit(lit) => match lit {
                Lit::Nil => {
                    self.write(writer, "nil");
                }
                Lit::Bool(b) => {
                    self.write(writer, if b { "true" } else { "false" });
                }
                Lit::I32(i) => {
                    self.write(writer, &i.to_string());
                }
                Lit::U32(u) => {
                    self.write(writer, &u.to_string());
                }
                Lit::I64(i) => {
                    self.write(writer, &i.to_string());
                }
                Lit::String(s) => {
                    self.write(writer, &s);
                }
            },
            Expr::Call(_, _, _, _) => todo!(),
            Expr::BinOp(op, _, lhs, rhs) => {
                self.expr(writer, lhs.data);
                self.write(writer, op.to_string());
                self.expr(writer, rhs.data);
            }
            Expr::Record(_, _, _) => todo!(),
            Expr::AnonymousRecord(_, _) => todo!(),
            Expr::Project(_, _, _) => todo!(),
            Expr::Make(_, _) => todo!(),
            Expr::SizeOf(_) => todo!(),
            Expr::Range(_, _) => todo!(),
            Expr::As(_, _, _) => todo!(),
            Expr::Path {
                path,
                resolved_path,
            } => todo!(),
            Expr::Wrap(_, _) => todo!(),
            Expr::Unwrap(_, _, _) => todo!(),
            Expr::Omit(_) => todo!(),
            Expr::EnumOr(_, _, _, _) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::Paren(expr) => {
                let mut fwriter = Formatter::new(self.source);
                let mut buf = BufWriter::new(Vec::new());
                fwriter.expr(&mut buf, expr.data);

                self.write(writer, "(");
                self.write_no_space(
                    writer,
                    String::from_utf8(buf.into_inner().unwrap()).unwrap(),
                );
                self.write_no_space(writer, ")");
            }
        }
    }

    fn write(&mut self, writer: &mut impl Write, s: impl AsRef<str>) {
        if self.column == 0 {
            write!(writer, "{}", " ".repeat(self.depth * self.indent_size)).unwrap();
        } else {
            write!(writer, " ").unwrap();
        }

        self.write_no_space(writer, s);
    }

    fn write_no_space(&mut self, writer: &mut impl Write, s: impl AsRef<str>) {
        write!(writer, "{}", s.as_ref()).unwrap();
        self.column += 1;
    }

    fn write_newline(&mut self, writer: &mut impl Write) {
        write!(writer, "\n").unwrap();
        self.column = 0;
    }

    fn write_block(&mut self, writer: &mut impl Write, blocks: Vec<String>, separator: &str) {
        self.write_newline(writer);
        self.depth += 1;
        for block in blocks {
            self.write(writer, block);
            self.write_no_space(writer, separator);
            self.write_newline(writer);
        }
        self.depth -= 1;
    }

    fn write_block_oneline(
        &mut self,
        writer: &mut impl Write,
        blocks: Vec<String>,
        separator: &str,
    ) {
        for block in blocks {
            self.write(writer, block);
            self.write_no_space(writer, separator);
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{compiler::Compiler, util::path::Path};

    use super::*;

    #[test]
    fn check_format() {
        let cases = vec![
            r#"
fun main() {
    return 10;
}
"#
            .trim_start(),
            r#"
fun main(): i32 {
    let a = 1;
    let b = (100 + 30) / 2;

    return a + b;
}
"#
            .trim_start(),
        ];

        for input in cases {
            let parsed = Compiler::run_parser(input, Path::empty(), true).unwrap();

            let mut fmt = Formatter::new(input);
            let formatted = fmt.format(parsed);
            assert_eq!(formatted, input);
        }
    }
}
