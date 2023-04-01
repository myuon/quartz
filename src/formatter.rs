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
        let (imports, decls) = module
            .0
            .into_iter()
            .partition::<Vec<_>, _>(|d| matches!(d.data, Decl::Import(_)));

        let imports_is_empty = imports.is_empty();
        for decl in imports {
            self.restore_comments(decl.start.unwrap_or(0), writer);
            self.decl(writer, decl.data);
            self.write_newline(writer);
        }

        if !imports_is_empty {
            self.write_newline(writer);
        }

        self.decls(writer, decls);
    }

    pub fn decls(&mut self, writer: &mut impl Write, decls: Vec<Source<Decl>>) {
        let decls_len = decls.len();
        for (i, decl) in decls.into_iter().enumerate() {
            self.restore_comments(decl.start.unwrap_or(0), writer);
            self.decl(writer, decl.data);
            self.write_newline(writer);
            if i != decls_len - 1 {
                self.write_newline(writer);
            }
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
                self.expr(writer, expr.data, false);
                self.write_no_space(writer, ";");
            }
            Decl::Type(ident, rs) => {
                self.write(writer, "struct");
                self.write(writer, ident.as_str());
                self.record_fields(writer, rs);
            }
            Decl::Module(path, module) => {
                self.write(writer, "module");
                self.write(writer, path.as_str());
                self.write(writer, "{");
                self.write_newline(writer);

                self.depth += 1;
                self.decls(writer, module.0);
                self.depth -= 1;

                self.write(writer, "}");
            }
            Decl::Import(path) => {
                self.write(writer, "import");
                self.write(writer, path.as_str());
                self.write_no_space(writer, ";");
            }
        }
    }

    fn func(&mut self, writer: &mut impl Write, func: Func) {
        let mut fwriter = self.clone();
        let mut buf = BufWriter::new(Vec::new());

        fwriter.write(&mut buf, "fun");
        fwriter.write(&mut buf, func.name.data.as_str());
        fwriter.params(&mut buf, func.params.clone(), func.variadic.clone());
        if let Type::Nil = func.result.clone() {
        } else {
            fwriter.write_no_space(&mut buf, ":");
            fwriter.type_(&mut buf, func.result.clone(), false);
        }

        let buf_string = String::from_utf8(buf.into_inner().unwrap()).unwrap();
        if buf_string.lines().count() == 1
            && self.indent_size * self.depth + buf_string.len() < self.max_width
        {
            self.write_no_space(writer, buf_string.as_str());
            *self = fwriter;
        } else {
            self.write(writer, "fun");
            self.write(writer, func.name.data.as_str());
            self.params_multilines(writer, func.params, func.variadic);
            if let Type::Nil = func.result {
            } else {
                fwriter.write_no_space(writer, ":");
                fwriter.type_(writer, func.result, false);
            }
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

        for (index, (ident, type_)) in params.clone().into_iter().enumerate() {
            if ident.0.as_str() == "self" {
                self.write_no_space(writer, "self");
            } else {
                self.write_no_space(writer, ident.as_str());
                self.write_no_space(writer, ":");
                self.type_(writer, type_, false);
            }

            if index != params.len() - 1 {
                self.write_no_space(writer, ", ");
            }
        }
        if let Some((ident, type_)) = variadic {
            if !params.is_empty() {
                self.write_no_space(writer, ", ");
            }

            self.write_no_space(writer, "..");
            self.write_if(writer, ident.as_str(), true);
            self.write_no_space(writer, ":");
            self.type_(writer, type_, false);
        }

        self.write_no_space(writer, ")");
    }

    fn params_multilines(
        &mut self,
        writer: &mut impl Write,
        params: Vec<(Ident, Type)>,
        variadic: Option<(Ident, Type)>,
    ) {
        self.write_no_space(writer, "(");
        self.write_newline(writer);
        self.depth += 1;

        for (ident, type_) in params {
            if ident.0.as_str() == "self" {
                self.write(writer, "self");
            } else {
                self.write(writer, ident.as_str());
                self.write_no_space(writer, ":");
                self.type_(writer, type_, false);
            }

            self.write_no_space(writer, ",");
            self.write_newline(writer);
        }
        if let Some((ident, type_)) = variadic {
            self.write_no_space(writer, "..");
            self.write_if(writer, ident.as_str(), true);
            self.write_no_space(writer, ":");
            self.type_(writer, type_, false);
            self.write_no_space(writer, ",");
            self.write_newline(writer);
        }

        self.depth -= 1;
        self.write(writer, ")");
    }

    fn statements(&mut self, writer: &mut impl Write, stmts: Source<Vec<Source<Statement>>>) {
        self.write(writer, "{");
        self.depth += 1;

        let mut current_pos = 0;

        macro_rules! update {
            ($start:expr, $end:expr, $write:expr) => {{
                if current_pos > 0 && self.need_empty_lines(current_pos, $start) {
                    self.write_newline(writer);
                }

                if self.need_line_follow(current_pos, $start) {
                    $write;
                } else {
                    self.write_newline(writer);
                    $write;
                }

                current_pos = $end;
            }};
        }

        for stmt in stmts.data {
            let comments = self.consume_comments(stmt.start.unwrap_or(0));
            for comment in comments {
                update!(
                    comment.start,
                    comment.start + comment.raw.len(),
                    self.write(writer, comment.raw.to_string())
                );
            }

            update!(
                stmt.start.unwrap_or(0),
                stmt.end.unwrap_or(0),
                self.statement(writer, stmt.data)
            );

            let comments = self.consume_comments(stmt.end.unwrap_or(0));
            for comment in comments {
                update!(
                    comment.start,
                    comment.start + comment.raw.len(),
                    self.write(writer, comment.raw.to_string())
                );
            }
        }

        let comments = self.consume_comments(stmts.end.unwrap_or(0));
        for comment in comments {
            update!(
                comment.start,
                comment.start + comment.raw.len(),
                self.write(writer, comment.raw.to_string())
            );
        }

        self.depth -= 1;
        self.write_newline(writer);
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
                if self.has_comments(expr.start.unwrap_or(0)) {
                    self.write_newline(writer);
                    self.depth += 1;
                    self.restore_comments(expr.start.unwrap_or(0), writer);
                    self.expr(writer, expr.data, false);
                    self.write_no_space(writer, ";");
                    self.depth -= 1;
                } else {
                    self.expr(writer, expr.data, false);
                    self.write_no_space(writer, ";");
                }
            }
            Statement::Return(expr) => {
                self.write(writer, "return");
                self.expr(writer, expr.data, false);
                self.write_no_space(writer, ";");
            }
            Statement::Expr(expr, _) => {
                self.expr(writer, expr.data, false);
                self.write_no_space(writer, ";");
            }
            Statement::Assign(lhs, _, rhs) => {
                self.expr(writer, lhs.data, false);
                self.write(writer, "=");
                self.restore_comments(rhs.start.unwrap_or(0), writer);
                self.expr(writer, rhs.data, false);
                self.write_no_space(writer, ";");
            }
            Statement::If(cond, _, then_block, else_block) => {
                self.write(writer, "if");
                self.expr(writer, cond.data, false);
                self.statements(writer, then_block);
                if let Some(else_block) = else_block {
                    self.write(writer, "else");
                    if else_block.data.len() == 1
                        && matches!(else_block.data[0].data, Statement::If(_, _, _, _))
                    {
                        self.statement(writer, else_block.data[0].data.clone());
                    } else {
                        self.statements(writer, else_block);
                    }
                }
            }
            Statement::While(cond, body) => {
                self.write(writer, "while");
                self.expr(writer, cond.data, false);
                self.statements(writer, body);
            }
            Statement::For(_, ident, range, body) => {
                self.write(writer, "for");
                self.write(writer, ident.as_str());
                self.write(writer, "in");
                self.expr(writer, range.data, false);
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

    fn expr(&mut self, writer: &mut impl Write, expr: Expr, skip_space: bool) {
        match expr {
            Expr::Ident { ident, .. } => {
                self.write_if(writer, ident.as_str(), skip_space);
            }
            Expr::Self_ => {
                self.write_if(writer, "self", skip_space);
            }
            Expr::Lit(lit) => match lit {
                Lit::Nil(raw) => {
                    if raw {
                        self.write_if(writer, "nil", skip_space);
                    }
                }
                Lit::Bool(b) => {
                    self.write_if(writer, if b { "true" } else { "false" }, skip_space);
                }
                Lit::I32(i) => {
                    self.write_if(writer, &i.to_string(), skip_space);
                }
                Lit::U32(u) => {
                    self.write_if(writer, &u.to_string(), skip_space);
                }
                Lit::I64(i) => {
                    self.write_if(writer, &i.to_string(), skip_space);
                }
                Lit::String(s, literal_type) => match literal_type {
                    StringLiteralType::String => {
                        self.write_if(writer, format!("{:?}", s), skip_space);
                    }
                    StringLiteralType::Raw => {
                        self.write_if(writer, format!("`{}`", s), skip_space);
                    }
                },
            },
            Expr::Call(callee, args, _, expansion) => {
                self.expr(writer, callee.data, skip_space);
                self.arguments(writer, args, expansion);
            }
            Expr::Not(expr) => {
                self.write_if(writer, "!", skip_space);
                self.expr(writer, expr.data, true);
            }
            Expr::BinOp(op, _, lhs, rhs) => {
                self.expr(writer, lhs.data, skip_space);
                self.write(writer, op.to_string());
                self.expr(writer, rhs.data, false);
            }
            Expr::Record(name, fields, expansion) => {
                self.write_if(writer, name.data.as_str(), skip_space);
                self.write(writer, "{");
                self.write_newline(writer);
                self.depth += 1;
                for (name, expr) in fields {
                    self.write(writer, name.as_str());
                    self.write_no_space(writer, ":");
                    self.expr(writer, expr.data, false);
                    self.write_no_space(writer, ",");
                    self.write_newline(writer);
                }
                if let Some(expansion) = expansion {
                    self.write(writer, "..");
                    self.expr(writer, expansion.data, true);
                    self.write_no_space(writer, ",");
                    self.write_newline(writer);
                }
                self.depth -= 1;
                self.write(writer, "}");
            }
            Expr::AnonymousRecord(fields, _) => {
                self.write_if(writer, "struct", skip_space);
                self.write(writer, "{");
                self.depth += 1;
                self.write_newline(writer);
                for (name, expr) in fields {
                    self.write(writer, name.as_str());
                    self.write_no_space(writer, ":");
                    self.expr(writer, expr.data, false);
                    self.write_no_space(writer, ",");
                    self.write_newline(writer);
                }
                self.depth -= 1;
                self.write(writer, "}");
            }
            Expr::Project(expr, _, field) => {
                self.expr(writer, expr.data, skip_space);
                self.write_no_space(writer, ".");
                self.write_no_space(writer, field.data.as_str());
            }
            Expr::Make(type_, args) => {
                self.write_if(writer, "make", skip_space);
                self.write_no_space(writer, "[");
                self.type_(writer, type_, true);
                self.write_no_space(writer, "]");
                self.arguments(writer, args, None);
            }
            Expr::SizeOf(type_) => {
                self.write_if(writer, "sizeof", skip_space);
                self.write_no_space(writer, "[");
                self.type_(writer, type_, true);
                self.write_no_space(writer, "]");
                self.write_no_space(writer, "()");
            }
            Expr::Range(start, end) => {
                self.expr(writer, start.data, skip_space);
                self.write_no_space(writer, "..");

                let mut fwriter = Formatter::new(self.source, self.comments, self.comment_position);
                let mut buf = BufWriter::new(Vec::new());
                fwriter.expr(&mut buf, end.data, false);
                self.write_no_space(
                    writer,
                    String::from_utf8(buf.into_inner().unwrap()).unwrap(),
                );
                self.comment_position = fwriter.comment_position;
            }
            Expr::As(expr, _, target) => {
                self.expr(writer, expr.data, skip_space);
                self.write(writer, "as");
                self.type_(writer, target, false);
            }
            Expr::Path { path, .. } => {
                self.write_if(writer, path.data.as_str(), skip_space);
            }
            Expr::Wrap(_, expr) => {
                self.expr(writer, expr.data, skip_space);
                self.write_no_space(writer, "?");
            }
            Expr::Unwrap(_, _, expr) => {
                self.expr(writer, expr.data, skip_space);
                self.write_no_space(writer, "!");
            }
            Expr::Omit(_) => {
                self.write(writer, "_");
            }
            Expr::EnumOr(_, _, lhs, rhs) => {
                if let Some(lhs) = lhs {
                    self.expr(writer, lhs.data, skip_space);
                } else {
                    self.write(writer, "_");
                }
                self.write(writer, "or");
                if let Some(rhs) = rhs {
                    self.expr(writer, rhs.data, false);
                } else {
                    self.write(writer, "_");
                }
            }
            Expr::Try(expr) => {
                self.expr(writer, expr.data, skip_space);
                self.write_no_space(writer, ".try");
            }
            Expr::Paren(expr) => {
                self.write_if(writer, "(", skip_space);
                self.expr(writer, expr.data, true);
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
        let mut fwriter = self.clone();
        let mut buf = BufWriter::new(Vec::new());
        fwriter.write_no_space(&mut buf, "(");
        for (i, arg) in args.clone().into_iter().enumerate() {
            fwriter.expr(&mut buf, arg.data, true);
            if i < args.len() - 1 {
                fwriter.write_no_space(&mut buf, ", ");
            }
        }
        if let Some(expansion) = expansion.clone() {
            if !args.is_empty() {
                fwriter.write_no_space(&mut buf, ", ");
            }
            fwriter.write_no_space(&mut buf, "..");
            fwriter.expr(&mut buf, expansion.data, true);
        }
        fwriter.write_no_space(&mut buf, ")");
        let buf_string = String::from_utf8(buf.into_inner().unwrap()).unwrap();

        if (buf_string.lines().count() == 1 || args.len() == 1) && fwriter.column < self.max_width {
            self.write_no_space(writer, buf_string.as_str());
            *self = fwriter;
        } else {
            self.write_no_space(writer, "(");
            self.write_newline(writer);
            self.depth += 1;

            for arg in args {
                self.expr(writer, arg.data, false);
                self.write_no_space(writer, ",");
                self.write_newline(writer);
            }
            if let Some(expansion) = expansion {
                self.write_no_space(writer, "..");
                self.expr(writer, expansion.data, true);
                self.write_newline(writer);
            }

            self.depth -= 1;
            self.write(writer, ")");
        }
    }

    fn type_(&mut self, writer: &mut impl Write, type_: Type, skip_space: bool) {
        match type_ {
            Type::Record(rs) => {
                self.write_if(writer, "struct", skip_space);
                self.record_fields(writer, rs.into_iter().map(Source::unknown).collect());
            }
            Type::Vec(v) => {
                self.write_if(writer, "vec", skip_space);
                self.write_no_space(writer, "[");
                self.type_(writer, *v, true);
                self.write_no_space(writer, "]");
            }
            Type::Ident(i) => {
                self.write_if(writer, i.as_str(), skip_space);
            }
            Type::Ptr(p) => {
                self.write_if(writer, "ptr", skip_space);
                self.write_no_space(writer, "[");
                self.type_(writer, *p, true);
                self.write_no_space(writer, "]");
            }
            Type::Optional(t) => {
                self.type_(writer, *t, skip_space);
                self.write_no_space(writer, "?");
            }
            Type::Map(k, v) => {
                self.write_if(writer, "map", skip_space);
                self.write_no_space(writer, "[");
                self.type_(writer, *k, true);
                self.write_no_space(writer, ",");
                self.type_(writer, *v, false);
                self.write_no_space(writer, "]");
            }
            Type::Or(lhs, rhs) => {
                self.type_(writer, *lhs, skip_space);
                self.write(writer, "or");
                self.type_(writer, *rhs, false);
            }
            _ => {
                self.write_if(writer, type_.to_string(), skip_space);
            }
        }
    }

    fn record_fields(&mut self, writer: &mut impl Write, fields: Vec<Source<(Ident, Type)>>) {
        self.write(writer, "{");
        self.write_newline(writer);
        self.depth += 1;
        for item in fields {
            let (name, type_) = item.data;
            self.write(writer, name.as_str());
            self.write_no_space(writer, ":");
            self.type_(writer, type_, false);
            self.write_no_space(writer, ",");

            self.restore_comments(item.end.unwrap_or(0), writer);

            self.write_newline(writer);
        }
        self.depth -= 1;
        self.write(writer, "}");
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
            && self.comments[self.comment_position].start <= start
        {
            comments.push(self.comments[self.comment_position].clone());
            self.comment_position += 1;
        }

        comments
    }

    fn has_comments(&self, start: usize) -> bool {
        self.comment_position < self.comments.len()
            && self.comments[self.comment_position].start <= start
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
            self.write_no_space(writer, " ".repeat(self.depth * self.indent_size));
        } else {
            self.write_no_space(writer, " ");
        }

        self.write_no_space(writer, s);
    }

    fn write_no_space(&mut self, writer: &mut impl Write, s: impl AsRef<str>) {
        let s = s.as_ref();
        let s_len = s.len();
        write!(writer, "{}", s).unwrap();
        self.column += s_len;
    }

    fn write_if(&mut self, writer: &mut impl Write, s: impl AsRef<str>, skip_space: bool) {
        if skip_space {
            self.write_no_space(writer, s);
        } else {
            self.write(writer, s);
        }
    }

    fn write_space(&mut self, writer: &mut impl Write) {
        self.write_no_space(writer, " ");
    }

    fn write_newline(&mut self, writer: &mut impl Write) {
        write!(writer, "\n").unwrap();
        self.column = 0;
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
            r#"
fun main(): i32 {
    if a == 1 {
        return 1;
    }
    if a == 2 {
        return 2;
    }
    if a == 3 {
        return 3;
    }

    return 0;
}
"#
            .trim_start(),
            r#"
fun main(
    loooooooooooong: string,
    loooooooooooong: string,
    loooooooooooong: string,
    arguments: string,
): string {
    return 0;
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
            .trim_start()),
            (
                r#"
module M {
    fun main() {
        if self.t_if != nil {
            let result = format("if ({}) {\n{}}", self.t_if!.condition.to_string(), self.t_if!.then_block.to_string());
        }
    }
}"#
                .trim_start(),
                r#"
module M {
    fun main() {
        if self.t_if != nil {
            let result = format(
                "if ({}) {\n{}}",
                self.t_if!.condition.to_string(),
                self.t_if!.then_block.to_string(),
            );
        }
    }
}
"#.trim_start()
            ),
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
