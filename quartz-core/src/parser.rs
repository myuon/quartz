use crate::{
    ast::{
        CallMode, Declaration, Expr, Function, Literal, Module, Source, Statement, Struct, Type,
    },
    compiler::CompileError,
    lexer::{run_lexer, Lexeme, Token},
};
use anyhow::{bail, ensure, Result};

struct Parser {
    position: usize,
    input: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
        }
    }

    fn source<T>(&self, data: T, start: usize, end: usize) -> Source<T> {
        Source {
            data,
            start: Some(self.input[start].position),
            end: Some(self.input[end - 1].position),
        }
    }

    fn source_from<T>(&self, data: T, start: usize) -> Source<T> {
        self.source(data, start, self.position)
    }

    fn peek(&self) -> &Token {
        &self.input[self.position]
    }

    fn peek_prev(&self) -> &Token {
        &self.input[self.position - 1]
    }

    fn parse_error(&self, expected: &str, got: &str) -> anyhow::Error {
        anyhow::anyhow!(CompileError::ParseError {
            position: if self.position >= self.input.len() {
                self.input.len() - 1
            } else {
                self.input[self.position].position
            },
            source: anyhow::anyhow!("Expected {} but {}", expected, got),
        })
    }

    fn expect_lexeme(&mut self, lexeme: Lexeme) -> Result<Token> {
        if self.is_end() {
            return Err(self.parse_error(&format!("{:?}", lexeme), "EOS"));
        }

        let current = self.input[self.position].clone();
        ensure!(
            current.lexeme == lexeme,
            CompileError::ParseError {
                position: self.input[self.position].position,
                source: anyhow::anyhow!(
                    "Expected {:?} but {:?}",
                    lexeme,
                    self.input[self.position]
                ),
            }
        );
        self.position += 1;

        Ok(current)
    }

    fn next(&mut self) -> Result<Token> {
        if self.is_end() {
            return Err(self.parse_error("", "EOS"));
        }

        let current = self.input[self.position].clone();
        self.position += 1;

        Ok(current)
    }

    fn type_(&mut self, type_params: &Vec<String>) -> Result<Type> {
        let mut is_ref = false;
        if self.expect_lexeme(Lexeme::Ref).is_ok() {
            is_ref = true;
        }

        let ident = self.ident()?.data;
        let mut result = match ident.as_str() {
            "int" => Type::Int,
            "bool" => Type::Bool,
            "string" => Type::Struct("string".to_string()),
            "any" => Type::Any,
            "bytes" => Type::Array(Box::new(Type::Byte)),
            "byte" => Type::Byte,
            "array" => {
                self.expect_lexeme(Lexeme::LBracket)?;
                let type_ = self.type_(type_params)?;

                let t = if self.expect_lexeme(Lexeme::Comma).is_ok() {
                    let token = self.next()?;
                    if let Lexeme::Int(u) = token.lexeme {
                        Type::SizedArray(Box::new(type_), u as usize)
                    } else {
                        return Err(self.parse_error("int or 'sized'", &format!("{:?}", token)));
                    }
                } else {
                    Type::Array(Box::new(type_))
                };

                self.expect_lexeme(Lexeme::RBracket)?;

                t
            }
            t if type_params.contains(&t.to_string()) => Type::TypeVar(ident),
            _ => Type::Struct(ident),
        };
        let type_params = self.type_applications()?;
        if !type_params.is_empty() {
            result = Type::TypeApp(Box::new(result), type_params);
        }

        if self.expect_lexeme(Lexeme::Question).is_ok() {
            result = Type::Optional(Box::new(result));
        }

        if is_ref {
            result = Type::Ref(Box::new(result));
        }

        Ok(result)
    }

    fn ident(&mut self) -> Result<Source<String>> {
        let current = &self.input[self.position];

        match &current.lexeme {
            Lexeme::Ident(t) => {
                self.position += 1;

                Ok(Source::new(
                    t.to_string(),
                    current.position,
                    current.position + t.len(),
                ))
            }
            Lexeme::Self_ => {
                self.position += 1;

                Ok(Source::new(
                    "self".to_string(),
                    current.position,
                    current.position + 4,
                ))
            }
            t => Err(self.parse_error("ident", &format!("{:?}", t))),
        }
    }

    fn variable(&mut self) -> Result<Expr> {
        let subject = self.type_(&vec![])?;
        if self.expect_lexeme(Lexeme::DoubleColon).is_ok() {
            let label = self.ident()?.data;
            Ok(Expr::PathVar(subject, label))
        } else {
            Ok(Expr::Var(vec![subject.method_selector_name()?]))
        }
    }

    fn literal(&mut self) -> Result<Literal> {
        match &self.input[self.position].lexeme {
            Lexeme::Nil => {
                self.position += 1;

                Ok(Literal::Nil)
            }
            Lexeme::True => {
                self.position += 1;

                Ok(Literal::Bool(true))
            }
            Lexeme::False => {
                self.position += 1;

                Ok(Literal::Bool(false))
            }
            Lexeme::Int(n) => {
                self.position += 1;

                Ok(Literal::Int(*n))
            }
            Lexeme::String(s) => {
                self.position += 1;

                Ok(Literal::String(s.clone()))
            }
            Lexeme::LBracket => {
                self.position += 1;

                let mut values = vec![];
                while !self.is_end() && self.peek().lexeme != Lexeme::RBracket {
                    let e_start = self.position;
                    let e = self.expr()?;
                    values.push(self.source_from(e, e_start));
                    if self.peek().lexeme == Lexeme::Comma {
                        self.position += 1;
                    }
                }

                self.expect_lexeme(Lexeme::RBracket)?;

                Ok(Literal::Array(values, Type::Omit))
            }
            _ => {
                return Err(
                    self.parse_error("any literal", &format!("{:?}", &self.input[self.position]))
                );
            }
        }
    }

    fn statement(&mut self) -> Result<Source<Statement>> {
        let start = self.position;

        if self.expect_lexeme(Lexeme::Let).is_ok() {
            let x = self.ident()?.data;
            self.expect_lexeme(Lexeme::Equal)?;

            let e_start = self.position;
            let e = self.expr()?;

            Ok(self.source(
                Statement::Let(x, self.source_from(e, e_start)),
                start,
                self.position,
            ))
        } else if self.expect_lexeme(Lexeme::Return).is_ok() {
            let e_start = self.position;
            let e = self.expr()?;
            Ok(self.source(
                Statement::Return(self.source_from(e, e_start)),
                start,
                self.position,
            ))
        } else if self.expect_lexeme(Lexeme::If).is_ok() {
            let cond_start = self.position;
            let cond = self.condition_expr()?;
            self.expect_lexeme(Lexeme::LBrace)?;
            let then = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            // optional else block
            let else_statements = if self.expect_lexeme(Lexeme::Else).is_ok() {
                self.expect_lexeme(Lexeme::LBrace)?;
                let else_statements = self.many_statements()?;
                self.expect_lexeme(Lexeme::RBrace)?;

                else_statements
            } else {
                vec![]
            };

            Ok(self.source(
                Statement::If(
                    Box::new(self.source_from(cond, cond_start)),
                    then,
                    else_statements,
                ),
                start,
                self.position,
            ))
        } else if self.expect_lexeme(Lexeme::Continue).is_ok() {
            Ok(self.source(Statement::Continue, start, self.position))
        } else if self.expect_lexeme(Lexeme::While).is_ok() {
            let e_start = self.position;
            let cond = self.condition_expr()?;

            self.expect_lexeme(Lexeme::LBrace)?;
            let then = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(self.source(
                Statement::While(self.source_from(cond, e_start), then),
                start,
                self.position,
            ))
        } else {
            let e_start = self.position;
            let e = self.expr()?;
            if self.expect_lexeme(Lexeme::Equal).is_ok() {
                // =が続くのであればassingmentで確定
                let rhs_start = self.position;
                let rhs = self.expr()?;
                Ok(self.source(
                    Statement::Assignment(
                        self.source_from(e, e_start),
                        self.source_from(rhs, rhs_start),
                    ),
                    start,
                    self.position,
                ))
            } else {
                // それ以外のケースは普通にexpr statement
                // FIXME: the outer Source should include semicolon at the end?
                Ok(self.source(
                    Statement::Expr(self.source(e, start, self.position), Type::Omit),
                    start,
                    self.position,
                ))
            }
        }
    }

    fn many_arguments(&mut self) -> Result<Vec<(String, Type)>> {
        let mut arguments = vec![];

        while self.peek().lexeme != Lexeme::RParen {
            let name = self.ident()?.data;
            let typ = if self.expect_lexeme(Lexeme::Colon).is_ok() {
                self.type_(&vec![])?
            } else {
                Type::Omit
            };

            arguments.push((name, typ));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(arguments),
                r => r,
            }?;
        }

        Ok(arguments)
    }

    fn many_exprs(&mut self) -> Result<Vec<Source<Expr>>> {
        let mut exprs = vec![];

        while self.peek().lexeme != Lexeme::RParen {
            let e_start = self.position;
            let e = self.expr()?;
            exprs.push(self.source_from(e, e_start));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(exprs),
                r => r,
            }?;
        }

        Ok(exprs)
    }

    fn many_statements(&mut self) -> Result<Vec<Source<Statement>>> {
        let mut statements = vec![];

        while !self.is_end() && self.peek().lexeme != Lexeme::RBrace {
            let st = self.statement()?;

            if let Err(err) = self.expect_lexeme(Lexeme::SemiColon) {
                // if, while, etc. can omit semicolon
                if self.peek_prev().lexeme != Lexeme::RBrace {
                    return Err(err);
                }
            }
            statements.push(st);
        }

        Ok(statements)
    }

    fn make_expr(&mut self) -> Result<Expr> {
        self.expect_lexeme(Lexeme::Make)?;
        self.expect_lexeme(Lexeme::LBracket)?;
        let typ = self.type_(&vec![])?;
        self.expect_lexeme(Lexeme::RBracket)?;
        self.expect_lexeme(Lexeme::LParen)?;
        let args = self.many_exprs()?;
        self.expect_lexeme(Lexeme::RParen)?;

        Ok(Expr::Make(typ, args))
    }

    fn short_callee_expr(&mut self) -> Result<Expr> {
        self.variable()
            .or_else(|_| -> Result<Expr> { self.literal().map(|lit| Expr::Lit(lit)) })
            .or_else(|_| -> Result<Expr> { self.make_expr() })
    }

    fn short_expr(&mut self) -> Result<Expr> {
        if self.expect_lexeme(Lexeme::LParen).is_ok() {
            let expr = self.expr()?;
            self.expect_lexeme(Lexeme::RParen)?;

            Ok(expr)
        } else {
            let result_start = self.position;
            let mut result = self.short_callee_expr()?;
            loop {
                if self.expect_lexeme(Lexeme::Dot).is_ok() {
                    // projection
                    let label = self.ident()?.data;

                    // decide method call or field access
                    if self.expect_lexeme(Lexeme::LParen).is_ok() {
                        let args = self.many_exprs()?;
                        self.expect_lexeme(Lexeme::RParen)?;

                        result = Expr::MethodCall(
                            CallMode::Function,
                            Type::Omit,
                            label,
                            Box::new(self.source_from(result, result_start)),
                            args,
                        );
                    } else {
                        result = Expr::Project(
                            Type::Omit,
                            Box::new(self.source_from(result, result_start)),
                            label,
                        );
                    }
                } else if self.expect_lexeme(Lexeme::LParen).is_ok() {
                    let result_end = self.position;
                    let args = self.many_exprs()?;
                    self.expect_lexeme(Lexeme::RParen)?;

                    result = Expr::Call(
                        CallMode::Function,
                        Box::new(self.source(result, result_start, result_end)),
                        args,
                    );
                } else if self.expect_lexeme(Lexeme::Exclamation).is_ok() {
                    result = Expr::Unwrap(
                        Box::new(self.source(result, result_start, self.position)),
                        Type::Omit,
                    );
                } else {
                    break;
                }
            }

            Ok(result)
        }
    }

    fn expr_with_struct_option(&mut self, allow_struct: bool) -> Result<Expr> {
        let is_ref = self.expect_lexeme(Lexeme::Ref).is_ok();
        let short_expr_start = self.position;
        let short_expr = self.short_expr()?;

        let mut result = match short_expr {
            Expr::Var(v) if allow_struct && self.expect_lexeme(Lexeme::LBrace).is_ok() => {
                // struct initialization
                let fields = self.many_fields_with_exprs()?;
                self.expect_lexeme(Lexeme::RBrace)?;

                // FIXME: qualified struct
                assert_eq!(v.len(), 1);

                Expr::Struct(v[0].clone(), vec![], fields)
            }
            expr if self.expect_lexeme(Lexeme::Dot).is_ok() => {
                // projection
                let i = self.ident()?.data;

                Expr::Project(
                    Type::Omit,
                    Box::new(self.source_from(expr, short_expr_start)),
                    i,
                )
            }
            expr if self.expect_lexeme(Lexeme::As).is_ok() => {
                let typ = self.type_(&vec![])?;
                Expr::As(
                    Box::new(self.source_from(expr, short_expr_start)),
                    Type::Omit,
                    typ,
                )
            }
            _ => short_expr,
        };

        if is_ref {
            result = Expr::Ref(
                Box::new(self.source(result, short_expr_start, self.position)),
                Type::Omit,
            );
        }

        // handling operators here
        let operators = vec![
            (Lexeme::Plus, "_add"),
            (Lexeme::Gt, "_gt"),
            (Lexeme::Lt, "_lt"),
            (Lexeme::DoubleEqual, "_eq"),
            (Lexeme::NotEqual, "_neq"),
            (Lexeme::Minus, "_sub"),
            (Lexeme::Star, "_mult"),
        ];
        for (lexeme, op) in operators {
            if self.expect_lexeme(lexeme).is_ok() {
                // This should be short_expr? idk
                let right_start = self.position;
                let right = self.expr()?;
                result = Expr::Call(
                    CallMode::Function,
                    Box::new(Source::unknown(Expr::Var(vec![op.to_string()]))),
                    vec![
                        self.source_from(result, short_expr_start),
                        self.source_from(right, right_start),
                    ],
                );
            }
        }

        Ok(result)
    }

    fn condition_expr(&mut self) -> Result<Expr> {
        self.expr_with_struct_option(false)
    }

    fn expr(&mut self) -> Result<Expr> {
        self.expr_with_struct_option(true)
    }

    fn declaration_function(&mut self) -> Result<Declaration> {
        let name = self.ident()?;
        let type_params = self.type_parameters()?;
        self.expect_lexeme(Lexeme::LParen)?;
        let args = self.many_arguments()?;
        self.expect_lexeme(Lexeme::RParen)?;

        let return_type = if self.expect_lexeme(Lexeme::Colon).is_ok() {
            self.type_(&vec![])?
        } else {
            Type::Omit
        };

        self.expect_lexeme(Lexeme::LBrace)?;
        let statements = self.many_statements()?;
        self.expect_lexeme(Lexeme::RBrace)?;

        // checking self
        for (a, _) in &args {
            if a == "self" {
                bail!("`self` is not allowed in function arguments.");
            }
        }

        Ok(Declaration::Function(Function {
            name,
            type_params,
            args,
            return_type,
            body: statements,
            dead_code: false, // for now
        }))
    }

    fn declaration_method(&mut self) -> Result<Declaration> {
        let struct_name = self.ident()?;
        let struct_type_params = self.type_parameters()?;

        let name = self.ident()?;
        let type_params = self.type_parameters()?;
        self.expect_lexeme(Lexeme::LParen)?;
        let mut args = self.many_arguments()?;
        self.expect_lexeme(Lexeme::RParen)?;

        let return_type = if self.expect_lexeme(Lexeme::Colon).is_ok() {
            self.type_(&vec![])?
        } else {
            Type::Omit
        };

        self.expect_lexeme(Lexeme::LBrace)?;
        let statements = self.many_statements()?;
        self.expect_lexeme(Lexeme::RBrace)?;

        // checking self conditions
        // NOTE: type for self should be treated & inferred in typecheck phase
        if !args.is_empty() {
            if args[0].0 == "self" {
                args[0].1 = Type::Self_;
            }

            for (s, _) in &args[1..] {
                if s == "self" {
                    bail!("`self` must be placed first in method arguments.");
                }
            }
        }

        Ok(Declaration::Method(
            struct_name,
            struct_type_params,
            Function {
                name,
                type_params,
                args,
                return_type,
                body: statements,
                dead_code: false,
            },
        ))
    }

    fn many_fields_with_types(&mut self, type_params: &Vec<String>) -> Result<Vec<(String, Type)>> {
        let mut fields = vec![];

        while self.peek().lexeme != Lexeme::RBrace {
            let name = self.ident()?.data;
            self.expect_lexeme(Lexeme::Colon)?;
            let ty = self.type_(type_params)?;
            fields.push((name, ty));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(fields),
                r => r,
            }?;
        }

        Ok(fields)
    }

    fn many_fields_with_exprs(&mut self) -> Result<Vec<(String, Source<Expr>, Type)>> {
        let mut fields = vec![];

        while self.peek().lexeme != Lexeme::RBrace {
            let name = self.ident()?.data;
            self.expect_lexeme(Lexeme::Colon)?;
            let e_start = self.position;
            let e = self.expr()?;
            fields.push((name, self.source_from(e, e_start), Type::Omit));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(fields),
                r => r,
            }?;
        }

        Ok(fields)
    }

    fn type_applications(&mut self) -> Result<Vec<Type>> {
        let mut fields = vec![];

        if !self.expect_lexeme(Lexeme::LBracket).is_ok() {
            return Ok(fields);
        }
        while self.peek().lexeme != Lexeme::RBracket {
            let name = self.type_(&vec![])?;
            fields.push(name);

            // allow trailing comma
            if self.expect_lexeme(Lexeme::Comma).is_err() {
                break;
            }
        }
        self.expect_lexeme(Lexeme::RBracket)?;

        Ok(fields)
    }

    fn type_parameters(&mut self) -> Result<Vec<String>> {
        let mut fields = vec![];

        if !self.expect_lexeme(Lexeme::LBracket).is_ok() {
            return Ok(fields);
        }
        while self.peek().lexeme != Lexeme::RBracket {
            let name = self.ident()?.data;
            fields.push(name);

            // allow trailing comma
            if self.expect_lexeme(Lexeme::Comma).is_err() {
                break;
            }
        }
        self.expect_lexeme(Lexeme::RBracket)?;

        Ok(fields)
    }

    fn declaration(&mut self) -> Result<Declaration> {
        if self.expect_lexeme(Lexeme::Func).is_ok() {
            self.declaration_function()
        } else if self.expect_lexeme(Lexeme::Method).is_ok() {
            self.declaration_method()
        } else if self.expect_lexeme(Lexeme::Let).is_ok() {
            let x = self.ident()?.data;
            self.expect_lexeme(Lexeme::Equal)?;
            let e_start = self.position;
            let e = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Declaration::Variable(
                x,
                self.source_from(e, e_start),
                Type::Omit,
            ))
        } else if self.expect_lexeme(Lexeme::Struct).is_ok() {
            let name = self.ident()?.data;
            let type_params = self.type_parameters()?;
            self.expect_lexeme(Lexeme::LBrace)?;
            let fields = self.many_fields_with_types(&type_params)?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(Declaration::Struct(Struct {
                name,
                type_params,
                fields,
                dead_code: false,
            }))
        } else if self.expect_lexeme(Lexeme::Import).is_ok() {
            let path = self.ident()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Declaration::Import(path))
        } else {
            bail!("Expected a declaration, but found {:?}", self.peek())
        }
    }

    fn many_declarations(&mut self) -> Result<Vec<Declaration>> {
        let mut decls = vec![];

        while !self.is_end() {
            decls.push(self.declaration()?);
        }

        Ok(decls)
    }

    fn parse_module(&mut self) -> Result<Module> {
        let decls = self.many_declarations()?;
        let mut paths = vec![];
        for d in &decls {
            match d {
                Declaration::Import(path) => paths.push(path.data.clone()),
                _ => (),
            }
        }

        Ok(Module {
            module_path: String::new(),
            decls,
            imports: paths,
        })
    }

    pub fn run_parser(&mut self, tokens: Vec<Token>) -> Result<Module> {
        self.position = 0;
        self.input = tokens;

        self.parse_module()
    }

    pub fn run_parser_statements(&mut self, tokens: Vec<Token>) -> Result<Vec<Source<Statement>>> {
        self.position = 0;
        self.input = tokens;

        self.many_statements()
    }

    pub fn run_parser_expr(&mut self, tokens: Vec<Token>) -> Result<Expr> {
        self.position = 0;
        self.input = tokens;

        self.expr()
    }

    pub fn is_end(&self) -> bool {
        self.input.len() == self.position
    }
}

pub fn run_parser_from_tokens(tokens: Vec<Token>) -> Result<Module> {
    let mut parser = Parser::new();
    let result = parser.run_parser(tokens)?;

    if !parser.is_end() {
        bail!("Unexpected token: {:?}", &parser.input[parser.position..]);
    }

    Ok(result)
}

pub fn run_parser(input: &str) -> Result<Module> {
    run_parser_from_tokens(run_lexer(input))
}

fn run_parser_statements_from_tokens(tokens: Vec<Token>) -> Result<Vec<Source<Statement>>> {
    let mut parser = Parser::new();
    let result = parser.run_parser_statements(tokens)?;

    if !parser.is_end() {
        bail!("Unexpected token: {:?}", &parser.input[parser.position..]);
    }

    Ok(result)
}

pub fn run_parser_expr(input: &str) -> Result<Expr> {
    let mut parser = Parser::new();
    let result = parser.run_parser_expr(run_lexer(input))?;

    if !parser.is_end() {
        bail!("Unexpected token: {:?}", &parser.input[parser.position..]);
    }

    Ok(result)
}

pub fn run_parser_statements(input: &str) -> Result<Vec<Source<Statement>>> {
    run_parser_statements_from_tokens(run_lexer(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_parser_statements() {
        let cases = vec![
            r#"
                    let y = 10;
                    _assign(y, 20);
                    return y;
                "#,
            r#"
                    f(10, 20, 40);
                    100;
                    "foo";
                "#,
            r#"let x = 10; return x;"#,
            r#"1; _panic(); 10;"#,
            r#"
                    obj.call(a1);
                "#,
            r#"if s.data(i) != t.data(i) {
                    return false;
                };"#,
            r#"while true {
                c = 1;
            }"#,
        ];

        for c in cases {
            let result = run_parser_statements(c);
            assert!(matches!(result, Ok(_)), "{} {:#?}", c, result);
        }
    }

    #[test]
    fn test_run_parser_expr() {
        let cases = vec![
            (
                "self.field!.f()",
                Expr::method_call(
                    Type::Omit,
                    "f",
                    Source::unknown(Expr::unwrap(Source::unknown(Expr::member(
                        Source::unknown(Expr::Var(vec!["self".to_string()])),
                        "field",
                    )))),
                    vec![],
                ),
            ),
            (
                r#"a.f(self.k!.name).g(b)"#,
                Expr::method_call(
                    Type::Omit,
                    "g",
                    Source::unknown(Expr::method_call(
                        Type::Omit,
                        "f",
                        Source::unknown(Expr::Var(vec!["a".to_string()])),
                        vec![Source::unknown(Expr::member(
                            Source::unknown(Expr::unwrap(Source::unknown(Expr::member(
                                Source::unknown(Expr::Var(vec!["self".to_string()])),
                                "k",
                            )))),
                            "name",
                        ))],
                    )),
                    vec![Source::unknown(Expr::Var(vec!["b".to_string()]))],
                ),
            ),
        ];

        for (c, expr) in cases {
            let result = run_parser_expr(c).unwrap();
            expr.require_same_structure(&result).unwrap();
        }
    }

    #[test]
    fn test_run_parser_fail() {
        let cases = vec![(r#"fn () { let u = 10 }"#, "Expected")];

        for c in cases {
            let result = run_parser(c.0);
            assert!(matches!(result, Err(_)), "{:?} {:?}", c.0, result);

            let err = result.unwrap_err();
            assert!(err.to_string().contains(c.1), "{:?} {}", err, c.0);
        }
    }

    #[test]
    fn test_run_parser_decl() {
        let cases = vec![
            r#"
                struct General[X,Y] {
                    x: X,
                    y: Y,
                }
            "#,
            r#"
                func id[T](x: T): T {
                    return x;
                }

                method Container[T] concat(other: Container[T]): Container[T] {
                    return nil;
                }

                method Container[T] id[Y](x: Y): Y {
                    return x;
                }
            "#,
        ];

        for c in cases {
            let result = run_parser(c);
            assert!(matches!(result, Ok(_)), "{} {:?}", c, result);
        }
    }
}
