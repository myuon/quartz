use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Source, Statement, Struct, Type},
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
            end: Some(self.input[end].position),
        }
    }

    fn source_from<T>(&self, data: T, start: usize) -> Source<T> {
        self.source(data, start, self.position)
    }

    fn peek(&self) -> &Token {
        &self.input[self.position]
    }

    fn parse_error(&self, expected: &str, got: &str) -> anyhow::Error {
        anyhow::anyhow!(CompileError::ParseError {
            position: self.position,
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

    fn atype(&mut self) -> Result<Type> {
        if self.expect_lexeme(Lexeme::And).is_ok() {
            Ok(Type::Ref(Box::new(self.atype()?)))
        } else {
            let ident = self.ident()?;
            match ident.as_str() {
                "int" => Ok(Type::Int),
                "bool" => Ok(Type::Bool),
                "string" => Ok(Type::Struct("string".to_string())),
                "any" => Ok(Type::Any),
                "bytes" => Ok(Type::Array(Box::new(Type::Byte))),
                "byte" => Ok(Type::Byte),
                _ => todo!("{:?}", ident),
            }
        }
    }

    fn ident(&mut self) -> Result<String> {
        match &self.input[self.position].lexeme {
            Lexeme::Ident(t) => {
                self.position += 1;

                Ok(t.to_string())
            }
            _ => bail!(
                "Expected an ident but found {:?}",
                &self.input[self.position..]
            ),
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

                Ok(Literal::Array(values))
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
            let x = self.ident()?;
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
            let cond = self.short_expr()?;
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
        } else if self.expect_lexeme(Lexeme::Loop).is_ok() {
            self.expect_lexeme(Lexeme::LBrace)?;
            let statements = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(self.source(Statement::Loop(statements), start, self.position))
        } else if self.expect_lexeme(Lexeme::While).is_ok() {
            let e_start = self.position;
            let cond = self.short_expr()?;

            self.expect_lexeme(Lexeme::LBrace)?;
            let then = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(self.source(
                Statement::While(Box::new(self.source_from(cond, e_start)), then),
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
                        Box::new(self.source_from(e, e_start)),
                        self.source_from(rhs, rhs_start),
                    ),
                    start,
                    self.position,
                ))
            } else {
                // それ以外のケースは普通にexpr statement
                // FIXME: the outer Source should include semicolon at the end?
                Ok(self.source(
                    Statement::Expr(self.source(e, start, self.position)),
                    start,
                    self.position,
                ))
            }
        }
    }

    fn many_arguments(&mut self) -> Result<Vec<(String, Type)>> {
        let mut arguments = vec![];

        while self.peek().lexeme != Lexeme::RParen {
            let name = self.ident()?;
            let mut typ = Type::Infer(0);

            if self.expect_lexeme(Lexeme::Colon).is_ok() {
                typ = self.atype()?;
            }

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

            self.expect_lexeme(Lexeme::SemiColon)?;
            statements.push(st);
        }

        Ok(statements)
    }

    fn short_callee_expr(&mut self) -> Result<Expr> {
        self.ident()
            .map(|v| Expr::Var(v))
            .or_else(|_| -> Result<Expr> { self.literal().map(|lit| Expr::Lit(lit)) })
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
                    let i = self.ident()?;

                    result = Expr::Project(
                        false,
                        "<infer>".to_string(),
                        Box::new(self.source_from(result, result_start)),
                        i,
                    );
                } else if self.expect_lexeme(Lexeme::LParen).is_ok() {
                    let args = self.many_exprs()?;
                    self.expect_lexeme(Lexeme::RParen)?;

                    result = Expr::Call(Box::new(self.source_from(result, result_start)), args);
                } else {
                    break;
                }
            }

            Ok(result)
        }
    }

    fn expr(&mut self) -> Result<Expr> {
        let short_expr_start = self.position;
        let short_expr = self.short_expr()?;

        let mut result = match short_expr {
            Expr::Var(v) if self.expect_lexeme(Lexeme::LBrace).is_ok() => {
                // struct initialization
                let fields = self.many_fields_with_exprs()?;
                self.expect_lexeme(Lexeme::RBrace)?;

                Expr::Struct(v, fields)
            }
            expr if self.expect_lexeme(Lexeme::Dot).is_ok() => {
                // projection
                let i = self.ident()?;

                Expr::Project(
                    false,
                    "<infer>".to_string(),
                    Box::new(self.source_from(expr, short_expr_start)),
                    i,
                )
            }
            expr if self.expect_lexeme(Lexeme::LBracket).is_ok() => {
                // indexing
                let index_start = self.position;
                let index = self.expr()?;
                self.expect_lexeme(Lexeme::RBracket)?;

                Expr::Index(
                    Box::new(self.source_from(expr, short_expr_start)),
                    Box::new(self.source_from(index, index_start)),
                )
            }
            _ => short_expr,
        };

        // handling operators here
        let operators = vec![
            (Lexeme::Plus, "_add"),
            (Lexeme::Gt, "_gt"),
            (Lexeme::Lt, "_lt"),
            (Lexeme::DoubleEqual, "_eq"),
            (Lexeme::NotEqual, "_neq"),
            (Lexeme::Minus, "_sub"),
        ];
        for (lexeme, op) in operators {
            if self.expect_lexeme(lexeme).is_ok() {
                // This should be short_expr? idk
                let right_start = self.position;
                let right = self.expr()?;
                result = Expr::Call(
                    Box::new(Source::unknown(Expr::Var(op.to_string()))),
                    vec![
                        self.source_from(result, short_expr_start),
                        self.source_from(right, right_start),
                    ],
                );
            }
        }

        Ok(result)
    }

    fn declaration_function(&mut self) -> Result<Declaration> {
        let method_of = if self.expect_lexeme(Lexeme::LParen).is_ok() {
            let ident = self.ident()?;
            self.expect_lexeme(Lexeme::Colon)?;
            let pointer = self.expect_lexeme(Lexeme::And).is_ok();
            let type_name = self.ident()?;
            self.expect_lexeme(Lexeme::RParen)?;

            Some((ident, type_name, pointer))
        } else {
            None
        };

        let name = self.ident()?;
        self.expect_lexeme(Lexeme::LParen)?;
        let args = self.many_arguments()?;
        self.expect_lexeme(Lexeme::RParen)?;

        let return_type = if self.expect_lexeme(Lexeme::Colon).is_ok() {
            self.atype()?
        } else {
            Type::Infer(0)
        };

        self.expect_lexeme(Lexeme::LBrace)?;
        let statements = self.many_statements()?;
        self.expect_lexeme(Lexeme::RBrace)?;

        Ok(Declaration::Function(Function {
            name,
            args,
            return_type,
            body: statements,
            method_of,
            dead_code: false, // for now
        }))
    }

    fn many_fields_with_types(&mut self) -> Result<Vec<(String, Type)>> {
        let mut fields = vec![];

        while self.peek().lexeme != Lexeme::RBrace {
            let name = self.ident()?;
            self.expect_lexeme(Lexeme::Colon)?;
            let ty = self.atype()?;
            fields.push((name, ty));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(fields),
                r => r,
            }?;
        }

        Ok(fields)
    }

    fn many_fields_with_exprs(&mut self) -> Result<Vec<(String, Source<Expr>)>> {
        let mut fields = vec![];

        while self.peek().lexeme != Lexeme::RBrace {
            let name = self.ident()?;
            self.expect_lexeme(Lexeme::Colon)?;
            let e_start = self.position;
            let e = self.expr()?;
            fields.push((name, self.source_from(e, e_start)));

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(fields),
                r => r,
            }?;
        }

        Ok(fields)
    }

    fn declaration(&mut self) -> Result<Declaration> {
        if self.expect_lexeme(Lexeme::Func).is_ok() {
            self.declaration_function()
        } else if self.expect_lexeme(Lexeme::Let).is_ok() {
            let x = self.ident()?;
            self.expect_lexeme(Lexeme::Equal)?;
            let e_start = self.position;
            let e = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Declaration::Variable(x, self.source_from(e, e_start)))
        } else if self.expect_lexeme(Lexeme::Struct).is_ok() {
            let name = self.ident()?;
            self.expect_lexeme(Lexeme::LBrace)?;
            let fields = self.many_fields_with_types()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(Declaration::Struct(Struct { name, fields }))
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
        Ok(Module(self.many_declarations()?))
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
                    loop {
                        return 10;
                    };
                "#,
            r#"
                    obj.call(a1);
                "#,
            r#"if (s.data[i] != t.data[i]) {
                    return false;
                };"#,
        ];

        for c in cases {
            let result = run_parser_statements(c);
            assert!(matches!(result, Ok(_)), "{} {:?}", c, result);
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
}
