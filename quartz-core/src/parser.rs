use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Statement, Struct, Type},
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

    fn peek(&self) -> &Token {
        &self.input[self.position]
    }

    fn expect_lexeme(&mut self, lexeme: Lexeme) -> Result<Token> {
        if self.is_end() {
            bail!(CompileError::ParseError {
                position: self.position,
                source: anyhow::anyhow!("Expected {:?} but EOS", lexeme),
            });
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
                "string" => Ok(Type::String),
                "any" => Ok(Type::Any),
                _ => todo!(),
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
            _ => bail!(
                "Expected a literal but found {:?}",
                &self.input[self.position..]
            ),
        }
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.expect_lexeme(Lexeme::Let).is_ok() {
            let x = self.ident()?;
            self.expect_lexeme(Lexeme::Equal)?;
            let e = self.expr()?;

            Ok(Statement::Let(x, e))
        } else if self.expect_lexeme(Lexeme::Return).is_ok() {
            let e = self.expr()?;
            Ok(Statement::Return(e))
        } else if self.expect_lexeme(Lexeme::If).is_ok() {
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

            Ok(Statement::If(Box::new(cond), then, else_statements))
        } else if self.expect_lexeme(Lexeme::Continue).is_ok() {
            Ok(Statement::Continue)
        } else if self.expect_lexeme(Lexeme::Loop).is_ok() {
            self.expect_lexeme(Lexeme::LBrace)?;
            let statements = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(Statement::Loop(statements))
        } else if self.expect_lexeme(Lexeme::While).is_ok() {
            let cond = self.short_expr()?;

            self.expect_lexeme(Lexeme::LBrace)?;
            let then = self.many_statements()?;
            self.expect_lexeme(Lexeme::RBrace)?;

            Ok(Statement::While(Box::new(cond), then))
        } else {
            let e = self.expr()?;
            if self.expect_lexeme(Lexeme::Equal).is_ok() {
                // =が続くのであればassingmentで確定
                let rhs = self.expr()?;
                Ok(Statement::Assignment(Box::new(e), rhs))
            } else {
                // それ以外のケースは普通にexpr statement
                Ok(Statement::Expr(e))
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

    fn many_exprs(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = vec![];

        while self.peek().lexeme != Lexeme::RParen {
            let e = self.expr()?;
            exprs.push(e);

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(exprs),
                r => r,
            }?;
        }

        Ok(exprs)
    }

    fn many_statements(&mut self) -> Result<Vec<Statement>> {
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
            let mut result = self.short_callee_expr()?;

            loop {
                if self.expect_lexeme(Lexeme::Dot).is_ok() {
                    // projection
                    let i = self.ident()?;

                    result = Expr::Project(false, "<infer>".to_string(), Box::new(result), i);
                } else if self.expect_lexeme(Lexeme::LParen).is_ok() {
                    let args = self.many_exprs()?;
                    self.expect_lexeme(Lexeme::RParen)?;

                    result = Expr::Call(Box::new(result), args);
                } else {
                    break;
                }
            }

            Ok(result)
        }
    }

    fn expr(&mut self) -> Result<Expr> {
        if self.expect_lexeme(Lexeme::Star).is_ok() {
            Ok(Expr::Deref(Box::new(self.expr()?)))
        } else if self.expect_lexeme(Lexeme::And).is_ok() {
            Ok(Expr::Ref(Box::new(self.expr()?)))
        } else {
            let short_expr = self.short_expr()?;

            match short_expr {
                Expr::Var(v) if self.expect_lexeme(Lexeme::LBrace).is_ok() => {
                    // struct initialization
                    let fields = self.many_fields_with_exprs()?;
                    self.expect_lexeme(Lexeme::RBrace)?;

                    Ok(Expr::Struct(v, fields))
                }
                expr if self.expect_lexeme(Lexeme::Dot).is_ok() => {
                    // projection
                    let i = self.ident()?;

                    Ok(Expr::Project(
                        false,
                        "<infer>".to_string(),
                        Box::new(expr),
                        i,
                    ))
                }
                expr if self.expect_lexeme(Lexeme::LBracket).is_ok() => {
                    // indexing
                    let index = self.expr()?;
                    self.expect_lexeme(Lexeme::RBracket)?;

                    Ok(Expr::Index(Box::new(expr), Box::new(index)))
                }
                _ => Ok(short_expr),
            }
        }
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

    fn many_fields_with_exprs(&mut self) -> Result<Vec<(String, Expr)>> {
        let mut fields = vec![];

        while self.peek().lexeme != Lexeme::RBrace {
            let name = self.ident()?;
            self.expect_lexeme(Lexeme::Colon)?;
            let e = self.expr()?;
            fields.push((name, e));

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
            let e = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Declaration::Variable(x, e))
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

    pub fn run_parser_statements(&mut self, tokens: Vec<Token>) -> Result<Vec<Statement>> {
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

fn run_parser_statements_from_tokens(tokens: Vec<Token>) -> Result<Vec<Statement>> {
    let mut parser = Parser::new();
    let result = parser.run_parser_statements(tokens)?;

    if !parser.is_end() {
        bail!("Unexpected token: {:?}", &parser.input[parser.position..]);
    }

    Ok(result)
}

pub fn run_parser_statements(input: &str) -> Result<Vec<Statement>> {
    run_parser_statements_from_tokens(run_lexer(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_parser_statements() {
        let cases = vec![
            (
                r#"
                    let y = 10;
                    _assign(y, 20);
                    return y;
                "#,
                vec![
                    Statement::Let("y".to_string(), Expr::Lit(Literal::Int(10))),
                    Statement::Expr(Expr::Call(
                        Box::new(Expr::Var("_assign".to_string())),
                        vec![Expr::Var("y".to_string()), Expr::Lit(Literal::Int(20))],
                    )),
                    Statement::Return(Expr::Var("y".to_string())),
                ],
            ),
            (
                r#"
                    f(10, 20, 40);
                    100;
                    "foo";
                "#,
                vec![
                    (Statement::Expr(Expr::Call(
                        Box::new(Expr::Var("f".to_string())),
                        vec![
                            Expr::Lit(Literal::Int(10)),
                            Expr::Lit(Literal::Int(20)),
                            Expr::Lit(Literal::Int(40)),
                        ],
                    ))),
                    (Statement::Expr(Expr::Lit(Literal::Int(100)))),
                    (Statement::Expr(Expr::Lit(Literal::String("foo".to_string())))),
                ],
            ),
            (
                r#"let x = 10; return x;"#,
                vec![
                    (Statement::Let("x".to_string(), Expr::Lit(Literal::Int(10)))),
                    (Statement::Return(Expr::Var("x".to_string()))),
                ],
            ),
            (
                r#"1; _panic(); 10;"#,
                vec![
                    Statement::Expr(Expr::Lit(Literal::Int(1))),
                    Statement::Expr(Expr::Call(
                        Box::new(Expr::Var("_panic".to_string())),
                        vec![],
                    )),
                    Statement::Expr(Expr::Lit(Literal::Int(10))),
                ],
            ),
            (
                r#"
                    loop {
                        return 10;
                    };
                "#,
                vec![Statement::Loop(vec![Statement::Return(Expr::Lit(
                    Literal::Int(10),
                ))])],
            ),
            (
                r#"
                    obj.call(a1);
                "#,
                vec![Statement::Expr(Expr::Call(
                    Box::new(Expr::Project(
                        false,
                        "<infer>".to_string(),
                        Box::new(Expr::Var("obj".to_string())),
                        "call".to_string(),
                    )),
                    vec![Expr::Var("a1".to_string())],
                ))],
            ),
        ];

        for c in cases {
            let result = run_parser_statements(c.0);
            assert!(matches!(result, Ok(_)), "{} {:?}", c.0, result);
            assert_eq!(result.unwrap(), c.1, "{}", c.0);
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
