use std::io::{BufWriter, Write};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Pattern, Statement, StringLiteralType, Type},
    lexer::Token,
    util::{ident::Ident, source::Source},
};

#[derive(Clone)]
pub struct Formatter<'s> {
    source: &'s str,
    max_width: usize,
    indent_size: usize,
    depth: usize,
    column: usize,
    comments: &'s Vec<Token>,
    comment_position: usize,
}

impl<'s> Formatter<'s> {
    pub fn new(
        source: &'s str,
        comments: &'s Vec<Token>,
        comment_position: usize,
    ) -> Formatter<'s> {
        Formatter {
            source,
            max_width: 110,
            indent_size: 4,
            depth: 0,
            column: 0,
            comments,
            comment_position,
        }
    }

    pub fn format(&mut self, module: Module) -> String {
        let mut buf = BufWriter::new(Vec::new());
        self.module(&mut buf, module);

        String::from_utf8(buf.into_inner().unwrap()).unwrap()
    }

    pub fn module(&mut self, writer: &mut impl Write, module: Module) {
        let mut blocks = vec![];
        for decl in module.0 {
            let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
            let mut buf = BufWriter::new(Vec::new());

            fwriter.restore_comments(decl.start.unwrap_or(0), &mut buf);
            fwriter.decl(&mut buf, decl.data);
            self.comment_position = fwriter.comment_position;

            blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
        }

        self.write_block(writer, blocks, "\n", false, true);
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
                self.write_no_space(writer, ";");
            }
            Decl::Type(ident, type_) => {
                self.write(writer, "type");
                self.write(writer, ident.as_str());
                self.write(writer, "=");
                self.write(writer, type_.to_string().as_str());
                self.write_no_space(writer, ";");
            }
            Decl::Module(path, module) => {
                let mut blocks = vec![];
                for decl in module.0 {
                    let mut fwriter =
                        Formatter::new(self.source, self.comments, self.comment_position);
                    let mut buf = BufWriter::new(Vec::new());
                    fwriter.decl(&mut buf, decl.data);

                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                    self.comment_position = fwriter.comment_position;
                }

                self.write(writer, "module");
                self.write(writer, path.as_str());
                self.write(writer, "{");
                self.write_block(writer, blocks, "\n", false, false);
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
        self.params(writer, func.params, func.variadic);
        if let Type::Nil = func.result {
        } else {
            self.write_no_space(writer, ":");
            self.write(writer, func.result.to_string().as_str());
        }
        self.statements(writer, func.body);
    }

    fn params(
        &mut self,
        writer: &mut impl Write,
        params: Vec<(Ident, Type)>,
        variadic: Option<(Ident, Type)>,
    ) {
        self.write_no_space(writer, "(");

        let mut blocks = vec![];
        for (ident, type_) in params {
            if ident.0.as_str() == "self" {
                blocks.push("self".to_string());
            } else {
                blocks.push(format!("{}: {}", ident.as_str(), type_.to_string()));
            }
        }
        if let Some((ident, type_)) = variadic {
            blocks.push(format!("..{}: {}", ident.as_str(), type_.to_string()));
        }
        self.write_block_oneline(writer, blocks, ",");
        self.write_no_space(writer, ")");
    }

    fn statements(&mut self, writer: &mut impl Write, stmts: Vec<Source<Statement>>) {
        let mut lines = vec![];
        for stmt in &stmts {
            let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
            let mut buf = BufWriter::new(Vec::new());

            let comments = fwriter.consume_comments(stmt.start.unwrap_or(0));
            for comment in comments {
                lines.push((comment.position, comment.raw.to_string()));
            }

            fwriter.statement(&mut buf, stmt.data.clone());
            let block_string = String::from_utf8(buf.into_inner().unwrap()).unwrap();
            lines.push((stmt.start.unwrap_or(0), block_string));

            let comments = fwriter.consume_comments(stmt.end.unwrap_or(0));
            for comment in comments {
                lines.push((comment.position, comment.raw.to_string()));
            }

            self.comment_position = fwriter.comment_position;
        }

        let mut blocks = vec![];
        if !lines.is_empty() {
            let mut current_pos = lines[0].0;
            for (pos, line) in lines {
                let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
                let mut buf = BufWriter::new(Vec::new());

                if self.need_empty_lines(current_pos, pos) {
                    fwriter.write_newline(&mut buf);
                }

                fwriter.write(&mut buf, line.as_str());

                if !blocks.is_empty() && self.need_line_follow(current_pos, pos) {
                    let last_line = blocks.pop().unwrap();
                    blocks.push(format!("{} {}", last_line, line));
                } else {
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                }

                current_pos = pos;
            }
        }

        self.write(writer, "{");
        self.write_newline(writer);
        self.write_block(writer, blocks, "", true, false);
        self.write(writer, "}");
        self.write_newline(writer);
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
                if self.has_comments(expr.start.unwrap_or(0)) {
                    self.write_newline(writer);
                    self.depth += 1;
                    self.restore_comments(expr.start.unwrap_or(0), writer);
                    self.expr(writer, expr.data);
                    self.write_no_space(writer, ";");
                    self.depth -= 1;
                } else {
                    self.expr(writer, expr.data);
                    self.write_no_space(writer, ";");
                }
            }
            Statement::Return(expr) => {
                self.write(writer, "return");
                self.expr(writer, expr.data);
                self.write_no_space(writer, ";");
            }
            Statement::Expr(expr, _) => {
                self.expr(writer, expr.data);
                self.write_no_space(writer, ";");
            }
            Statement::Assign(lhs, _, rhs) => {
                self.expr(writer, lhs.data);
                self.write(writer, "=");
                self.restore_comments(rhs.start.unwrap_or(0), writer);
                self.expr(writer, rhs.data);
                self.write_no_space(writer, ";");
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
                self.write_no_space(writer, ";");
            }
            Statement::Break => {
                self.write(writer, "break");
                self.write_no_space(writer, ";");
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
                Lit::String(s, literal_type) => match literal_type {
                    StringLiteralType::String => {
                        self.write(writer, format!("{:?}", s));
                    }
                    StringLiteralType::Raw => {
                        self.write(writer, format!("`{}`", s));
                    }
                },
            },
            Expr::Call(callee, args, _, expansion) => {
                self.expr(writer, callee.data);
                self.arguments(writer, args, expansion);
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
                    let mut fwriter =
                        Formatter::new(self.source, self.comments, self.comment_position);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.write_no_space(&mut buf, name.as_str());
                    fwriter.write_no_space(&mut buf, ":");
                    fwriter.expr(&mut buf, expr.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                    self.comment_position = fwriter.comment_position;
                }
                self.write_block(writer, blocks, ",", true, false);
                self.write(writer, "}");
            }
            Expr::AnonymousRecord(fields, _) => {
                self.write(writer, "struct");
                self.write(writer, "{");
                let mut blocks = vec![];
                for (name, expr) in fields {
                    let mut fwriter =
                        Formatter::new(self.source, self.comments, self.comment_position);
                    let mut buf = BufWriter::new(Vec::new());

                    fwriter.write_no_space(&mut buf, name.as_str());
                    fwriter.write_no_space(&mut buf, ":");
                    fwriter.expr(&mut buf, expr.data);
                    blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
                    self.comment_position = fwriter.comment_position;
                }
                self.write_block(writer, blocks, ",", true, false);
                self.write(writer, "}");
            }
            Expr::Project(expr, _, field) => {
                self.expr(writer, expr.data);
                self.write_no_space(writer, ".");
                self.write_no_space(writer, field.data.as_str());
            }
            Expr::Make(type_, args) => {
                self.write(writer, "make");
                self.write_no_space(writer, "[");
                self.type_no_space(writer, type_);
                self.write_no_space(writer, "]");
                self.arguments(writer, args, None);
            }
            Expr::SizeOf(type_) => {
                self.write(writer, "sizeof");
                self.write_no_space(writer, "[");
                self.type_no_space(writer, type_);
                self.write_no_space(writer, "]");
            }
            Expr::Range(start, end) => {
                self.expr(writer, start.data);
                self.write_no_space(writer, "..");

                let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
                let mut buf = BufWriter::new(Vec::new());
                fwriter.expr(&mut buf, end.data);
                self.write_no_space(
                    writer,
                    String::from_utf8(buf.into_inner().unwrap()).unwrap(),
                );
                self.comment_position = fwriter.comment_position;
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
                self.write_no_space(writer, "?");
            }
            Expr::Unwrap(_, _, expr) => {
                self.expr(writer, expr.data);
                self.write_no_space(writer, "!");
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
                let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
                let mut buf = BufWriter::new(Vec::new());
                fwriter.expr(&mut buf, expr.data);
                self.comment_position = fwriter.comment_position;

                self.write(writer, "(");
                self.write_no_space(
                    writer,
                    String::from_utf8(buf.into_inner().unwrap()).unwrap(),
                );
                self.write_no_space(writer, ")");
            }
        }
    }

    fn arguments(
        &mut self,
        writer: &mut impl Write,
        args: Vec<Source<Expr>>,
        expansion: Option<Box<Source<Expr>>>,
    ) {
        let mut blocks = vec![];
        for arg in args {
            let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
            let mut buf = BufWriter::new(Vec::new());

            fwriter.expr(&mut buf, arg.data);
            self.comment_position = fwriter.comment_position;
            blocks.push(String::from_utf8(buf.into_inner().unwrap()).unwrap());
        }
        if let Some(expansion) = expansion {
            let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
            let mut buf = BufWriter::new(Vec::new());

            fwriter.expr(&mut buf, expansion.data);
            self.comment_position = fwriter.comment_position;
            blocks.push(format!(
                "..{}",
                String::from_utf8(buf.into_inner().unwrap()).unwrap()
            ));
        }

        let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
        let mut buf = BufWriter::new(Vec::new());
        fwriter.write_no_space(&mut buf, "(");
        fwriter.write_block_oneline(&mut buf, blocks.clone(), ",");
        fwriter.write_no_space(&mut buf, ")");
        self.comment_position = fwriter.comment_position;
        let buf_string = String::from_utf8(buf.into_inner().unwrap()).unwrap();
        if self.depth * self.indent_size + buf_string.len() < self.max_width {
            self.write_no_space(writer, buf_string.as_str());
        } else {
            self.write_no_space(writer, "(");
            self.write_newline(writer);
            self.write_block(writer, blocks, ",", true, false);
            self.write(writer, ")");
        }
    }

    fn type_(&mut self, writer: &mut impl Write, type_: Type) {
        self.write(writer, type_.to_string());
    }

    fn type_no_space(&mut self, writer: &mut impl Write, type_: Type) {
        self.write_no_space(writer, type_.to_string());
    }

    fn need_empty_lines(&self, start: usize, end: usize) -> bool {
        self.source[start..end]
            .chars()
            .filter(|p| p == &'\n')
            .count()
            >= 2
    }

    fn need_line_follow(&self, start: usize, end: usize) -> bool {
        self.source[start..end]
            .chars()
            .filter(|p| p == &'\n')
            .count()
            == 0
    }

    fn consume_comments(&mut self, start: usize) -> Vec<Token> {
        let mut comments = vec![];
        while self.comment_position < self.comments.len()
            && self.comments[self.comment_position].position <= start
        {
            comments.push(self.comments[self.comment_position].clone());
            self.comment_position += 1;
        }

        comments
    }

    fn has_comments(&self, start: usize) -> bool {
        self.comment_position < self.comments.len()
            && self.comments[self.comment_position].position <= start
    }

    fn restore_comments(&mut self, start: usize, writer: &mut impl Write) {
        let comments = self.consume_comments(start);
        for comment in comments {
            self.write(writer, comment.raw);
            self.write_newline(writer);
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
        trailing_separator: bool,
        no_depth: bool,
    ) {
        if !no_depth {
            self.depth += 1;
        }
        let blocks_len = blocks.len();
        for (index, block) in blocks.into_iter().enumerate() {
            // each block may have multiple lines
            let mut first = true;
            for line in block.lines() {
                if !first {
                    self.write_newline(writer);
                }

                if line.trim().is_empty() {
                    self.write_no_space(writer, line);
                } else {
                    self.write(writer, line);
                }
                first = false;
            }

            if index < blocks_len - 1 || trailing_separator {
                self.write_no_space(writer, separator);
            }
            self.write_newline(writer);
        }
        if !no_depth {
            self.depth -= 1;
        }
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
            r#"
// function
fun main(): i32 {
    // let foo
    let b =
        // definition of b
        (100 + 30) / 2;

    // return
    return a + b; // a + b

    // last line
}
"#
            .trim_start(),
        ];

        for input in cases {
            let (parsed, comments) =
                Compiler::run_parser_with_comments(input, Path::empty(), true).unwrap();

            let mut fmt = Formatter::new(input, &comments, 0);
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
            let (parsed, comments) =
                Compiler::run_parser_with_comments(input, Path::empty(), true).unwrap();

            let mut fmt = Formatter::new(input, &comments, 0);
            let formatted = fmt.format(parsed);
            assert_eq!(formatted, output);
        }
    }
}
