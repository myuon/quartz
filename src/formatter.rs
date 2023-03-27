use std::io::{BufWriter, Write};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Pattern, Statement, Type},
    util::{ident::Ident, source::Source},
};

#[derive(Clone)]
pub struct Formatter<'s> {
    source: &'s str,
    max_width: usize,
    indent_size: usize,
    depth: usize,
    column: usize,
}

impl<'s> Formatter<'s> {
    pub fn new(source: &'s str) -> Formatter {
        Formatter {
            source,
            max_width: 110,
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
            Decl::Let(ident, type_, expr) => {
                self.write(writer, "let");
                self.write(writer, ident.as_str());
                if let Type::Omit(_) = type_ {
                } else {
                    self.write_no_space(writer, ":");
                    self.write(writer, type_.to_string().as_str());
                }
                self.write(writer, "=");
                self.expr(writer, expr.data);
                self.write(writer, ";");
            }
            Decl::Type(ident, type_) => {
                self.write(writer, "type");
                self.write(writer, ident.as_str());
                self.write(writer, "=");
                self.write(writer, type_.to_string().as_str());
                self.write(writer, ";");
            }
            Decl::Module(path, module) => {
                self.write(writer, "module");
                self.write(writer, path.as_str());
                self.write(writer, "{");
                self.module(writer, module);
                self.write(writer, "}");
            }
            Decl::Import(path) => {
                self.write(writer, "import");
                self.write(writer, path.as_str());
                self.write(writer, ";");
            }
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
        let mut prev_position = stmts.get(0).map(|t| t.start).flatten().unwrap_or(0);
        let mut need_empty_lines = vec![];
        for (index, stmt) in stmts.into_iter().enumerate() {
            let mut fwriter = Formatter::new(self.source);
            let mut buf = BufWriter::new(Vec::new());
            let current_position = stmt.start.unwrap_or(0);
            let need_empty_line = self.source[prev_position..current_position]
                .chars()
                .filter(|p| p == &'\n')
                .count()
                >= 2;
            if need_empty_line {
                need_empty_lines.push(index);
            }
            prev_position = current_position;

            fwriter.statement(&mut buf, stmt.data);
            blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
        }

        self.write(writer, "{");
        self.write_block(writer, blocks, ";", need_empty_lines);
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
            Statement::Expr(expr, _) => {
                self.expr(writer, expr.data);
                self.write(writer, ";");
            }
            Statement::Assign(lhs, _, rhs) => {
                self.expr(writer, lhs.data);
                self.write(writer, "=");
                self.expr(writer, rhs.data);
            }
            Statement::If(cond, _, then_block, else_block) => {
                self.write(writer, "if");
                self.expr(writer, cond.data);
                self.statements(writer, then_block);
                if let Some(else_block) = else_block {
                    self.write(writer, "else");
                    self.statements(writer, else_block);
                }
            }
            Statement::While(cond, body) => {
                self.write(writer, "while");
                self.expr(writer, cond.data);
                self.statements(writer, body);
            }
            Statement::For(_, ident, range, body) => {
                self.write(writer, "for");
                self.write(writer, ident.as_str());
                self.write(writer, "in");
                self.expr(writer, range.data);
                self.statements(writer, body);
            }
            Statement::Continue => {
                self.write(writer, "continue");
            }
            Statement::Break => {
                self.write(writer, "break");
            }
        }
    }

    fn pattern(&mut self, writer: &mut impl Write, pattern: Pattern) {
        match pattern {
            Pattern::Ident(ident) => {
                self.write(writer, ident.as_str());
            }
            Pattern::Or(lhs, rhs) => {
                self.pattern(writer, lhs.data);
                self.write(writer, "or");
                self.pattern(writer, rhs.data);
            }
            Pattern::Omit => {
                self.write(writer, "_");
            }
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
                    self.write(writer, format!("{:?}", s));
                }
            },
            Expr::Call(callee, args, varadic, expansion) => {
                self.expr(writer, callee.data);

                let mut blocks = vec![];
                for arg in args {
                    let mut fwriter = Formatter::new(self.source);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.expr(&mut buf, arg.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                }

                let mut fwriter = Formatter::new(self.source);
                let mut buf = BufWriter::new(Vec::new());
                fwriter.write_no_space(&mut buf, "(");
                fwriter.write_block_oneline(&mut buf, blocks.clone(), ",");
                fwriter.write_no_space(&mut buf, ")");
                let buf_string = String::from_utf8(buf.into_inner().unwrap()).unwrap();
                if self.depth * self.indent_size + buf_string.len() < self.max_width {
                    self.write_no_space(writer, buf_string.as_str());
                } else {
                    self.write_no_space(writer, "(");
                    self.write_block(writer, blocks, ",", vec![]);
                    self.write(writer, ")");
                }
            }
            Expr::BinOp(op, _, lhs, rhs) => {
                self.expr(writer, lhs.data);
                self.write(writer, op.to_string());
                self.expr(writer, rhs.data);
            }
            Expr::Record(name, fields, expansion) => {
                self.write(writer, name.data.as_str());
                self.write(writer, "{");
                let mut blocks = vec![];
                for (name, expr) in fields {
                    let mut fwriter = Formatter::new(self.source);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.write_no_space(&mut buf, name.as_str());
                    fwriter.write_no_space(&mut buf, ":");
                    fwriter.expr(&mut buf, expr.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                }
                self.write_block(writer, blocks, ",", vec![]);
                self.write(writer, "}");
            }
            Expr::AnonymousRecord(fields, _) => {
                self.write(writer, "struct");
                self.write(writer, "{");
                let mut blocks = vec![];
                for (name, expr) in fields {
                    let mut fwriter = Formatter::new(self.source);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.write_no_space(&mut buf, name.as_str());
                    fwriter.write_no_space(&mut buf, ":");
                    fwriter.expr(&mut buf, expr.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                }
                self.write_block(writer, blocks, ",", vec![]);
                self.write(writer, "}");
            }
            Expr::Project(expr, _, field) => {
                self.expr(writer, expr.data);
                self.write(writer, ".");
                self.write(writer, field.data.as_str());
            }
            Expr::Make(type_, args) => {
                self.write(writer, "make");
                self.write(writer, "[");
                self.type_(writer, type_);
                self.write(writer, "]");
                self.write(writer, "(");
                let mut blocks = vec![];
                for arg in args {
                    let mut fwriter = Formatter::new(self.source);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.expr(&mut buf, arg.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                }
                self.write_block(writer, blocks, ",", vec![]);
                self.write(writer, ")");
            }
            Expr::SizeOf(type_) => {
                self.write(writer, "sizeof");
                self.write(writer, "[");
                self.type_(writer, type_);
                self.write(writer, "]");
            }
            Expr::Range(start, end) => {
                self.expr(writer, start.data);
                self.write(writer, "..");
                self.expr(writer, end.data);
            }
            Expr::As(expr, _, target) => {
                self.expr(writer, expr.data);
                self.write(writer, "as");
                self.type_(writer, target);
            }
            Expr::Path { path, .. } => {
                self.write(writer, path.data.as_str());
            }
            Expr::Wrap(_, expr) => {
                self.expr(writer, expr.data);
            }
            Expr::Unwrap(_, _, expr) => {
                self.expr(writer, expr.data);
            }
            Expr::Omit(_) => {
                self.write(writer, "_");
            }
            Expr::EnumOr(_, _, lhs, rhs) => {
                if let Some(lhs) = lhs {
                    self.expr(writer, lhs.data);
                } else {
                    self.write(writer, "_");
                }
                self.write(writer, "or");
                if let Some(rhs) = rhs {
                    self.expr(writer, rhs.data);
                } else {
                    self.write(writer, "_");
                }
            }
            Expr::Try(expr) => {
                self.expr(writer, expr.data);
                self.write_no_space(writer, ".try");
            }
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

    fn type_(&mut self, writer: &mut impl Write, type_: Type) {
        self.write(writer, type_.to_string());
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

    fn write_space(&mut self, writer: &mut impl Write) {
        self.write_no_space(writer, " ");
    }

    fn write_newline(&mut self, writer: &mut impl Write) {
        write!(writer, "\n").unwrap();
        self.column = 0;
    }

    fn write_block(
        &mut self,
        writer: &mut impl Write,
        blocks: Vec<String>,
        separator: &str,
        empty_lines: Vec<usize>,
    ) {
        self.write_newline(writer);
        self.depth += 1;
        for (index, block) in blocks.into_iter().enumerate() {
            if empty_lines.contains(&index) {
                self.write_newline(writer);
            }

            // each block may have multiple lines
            let mut first = true;
            for line in block.lines() {
                if !first {
                    self.write_newline(writer);
                }

                self.write(writer, line);
                first = false;
            }

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
        let mut first = true;
        for block in blocks {
            if !first {
                self.write_no_space(writer, separator);
                self.write_space(writer);
            }
            self.write_no_space(writer, block);

            first = false;
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

    #[test]
    fn format_forced() {
        let cases = vec![
            (r#"
fun main() {
    let a = f("short", "text");

    return f("looooooooooooooooooooooong", "looooooooooooooooooooooong", "looooooooooooooooooooooong", "looooooooooooooooooooooong", "text");
}
"#
            .trim_start(),
            r#"
fun main() {
    let a = f("short", "text");

    return f(
        "looooooooooooooooooooooong",
        "looooooooooooooooooooooong",
        "looooooooooooooooooooooong",
        "looooooooooooooooooooooong",
        "text",
    );
}
"#
            .trim_start())
        ];

        for (input, output) in cases {
            let parsed = Compiler::run_parser(input, Path::empty(), true).unwrap();

            let mut fmt = Formatter::new(input);
            let formatted = fmt.format(parsed);
            assert_eq!(formatted, output);
        }
    }
}
