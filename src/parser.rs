use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Statement},
    lexer::{run_lexer, Lexeme, Token},
};
use anyhow::{bail, ensure, Result};

struct Parser {
    position: usize,
    input: Vec<Token>,
    is_toplevel: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
            is_toplevel: true,
        }
    }

    fn peek(&self) -> &Token {
        &self.input[self.position]
    }

    fn expect_lexeme(&mut self, lexeme: Lexeme) -> Result<Token> {
        if self.is_end() {
            bail!("Expected {:?} but EOS", lexeme);
        }

        let current = self.input[self.position].clone();
        ensure!(
            current.lexeme == lexeme,
            "Expected {:?} but found {:?}",
            lexeme,
            &self.input[self.position..]
        );
        self.position += 1;

        Ok(current)
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

    fn statement_let(&mut self) -> Result<Statement> {
        self.expect_lexeme(Lexeme::Let)?;
        let x = self.ident()?;
        self.expect_lexeme(Lexeme::Equal)?;
        let e = self.expr()?;

        Ok(Statement::Let(self.is_toplevel, x, e))
    }

    fn statement_if(&mut self) -> Result<Statement> {
        self.expect_lexeme(Lexeme::If)?;
        let cond = self.expr()?;
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
    }

    fn statement(&mut self) -> Result<Statement> {
        self.statement_let()
            .or_else(|_| -> Result<Statement> {
                self.expect_lexeme(Lexeme::Return)?;
                let e = self.expr()?;

                if self.expect_lexeme(Lexeme::If).is_ok() {
                    let cond = self.expr()?;
                    Ok(Statement::ReturnIf(e, cond))
                } else {
                    Ok(Statement::Return(e))
                }
            })
            .or_else(|_| -> Result<Statement> { self.statement_if() })
            .or_else(|_| -> Result<Statement> {
                self.expect_lexeme(Lexeme::Continue)?;

                Ok(Statement::Continue)
            })
            .or_else(|_| self.expr().map(|e| Statement::Expr(e)))
    }

    fn many_idents(&mut self) -> Result<Vec<String>> {
        let mut idents = vec![];

        while self.peek().lexeme != Lexeme::RParen {
            let e = self.ident()?;
            idents.push(e);

            // allow trailing comma
            match self.expect_lexeme(Lexeme::Comma) {
                Err(_) => return Ok(idents),
                r => r,
            }?;
        }

        Ok(idents)
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

    fn expr(&mut self) -> Result<Expr> {
        (self.literal().map(|lit| Expr::Lit(lit)))
            .or_else(|_| -> Result<Expr> {
                self.expect_lexeme(Lexeme::And)?;
                let e = self.expr()?;

                Ok(Expr::Ref(Box::new(e)))
            })
            .or_else(|_| -> Result<Expr> {
                self.expect_lexeme(Lexeme::Star)?;
                let e = self.expr()?;

                Ok(Expr::Deref(Box::new(e)))
            })
            .or_else(|_| -> Result<Expr> {
                // var or fun call
                let v = self.ident()?;

                match self.expect_lexeme(Lexeme::LParen) {
                    Ok(_) => {
                        // function call
                        let args = self.many_exprs()?;
                        self.expect_lexeme(Lexeme::RParen)?;

                        Ok(Expr::Call(v, args))
                    }
                    Err(_) => {
                        // var
                        Ok(Expr::Var(v))
                    }
                }
            })
            .or_else(|_| -> Result<Expr> {
                self.expect_lexeme(Lexeme::Loop)?;

                let is_toplevel = self.is_toplevel;
                self.is_toplevel = false;
                self.expect_lexeme(Lexeme::LBrace)?;
                let statements = self.many_statements()?;
                self.expect_lexeme(Lexeme::RBrace)?;
                self.is_toplevel = is_toplevel;

                Ok(Expr::Loop(statements))
            })
            .or_else(|_| -> Result<Expr> {
                let token = self.expect_lexeme(Lexeme::Fn)?;

                self.expect_lexeme(Lexeme::LParen)?;
                let args = self.many_idents()?;
                self.expect_lexeme(Lexeme::RParen)?;

                self.expect_lexeme(Lexeme::LBrace)?;
                let is_toplevel = self.is_toplevel;
                self.is_toplevel = false;
                let statements = self.many_statements()?;
                self.is_toplevel = is_toplevel;
                self.expect_lexeme(Lexeme::RBrace)?;

                Ok(Expr::Fun(token.position, args, statements))
            })
    }

    fn declaration_function(&mut self) -> Result<Declaration> {
        self.expect_lexeme(Lexeme::Fn)?;

        let name = self.ident()?;
        self.expect_lexeme(Lexeme::LParen)?;
        let args = self.many_idents()?;
        self.expect_lexeme(Lexeme::RParen)?;

        self.expect_lexeme(Lexeme::LBrace)?;
        let is_toplevel = self.is_toplevel;
        self.is_toplevel = false;
        let statements = self.many_statements()?;
        self.is_toplevel = is_toplevel;
        self.expect_lexeme(Lexeme::RBrace)?;

        Ok(Declaration::Function(Function {
            name,
            args,
            body: statements,
        }))
    }

    fn many_declarations(&mut self) -> Result<Vec<Declaration>> {
        let mut decls = vec![];

        while !self.is_end() {
            let d = self.declaration_function()?;

            decls.push(d);
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
                r#"let main = fn () {
                    let y = 10;
                    _assign(y, 20);
                    return y;
                };"#,
                vec![Statement::Let(
                    true,
                    "main".to_string(),
                    Expr::Fun(
                        11,
                        vec![],
                        vec![
                            Statement::Let(false, "y".to_string(), Expr::Lit(Literal::Int(10))),
                            Statement::Expr(Expr::Call(
                                "_assign".to_string(),
                                vec![Expr::Var("y".to_string()), Expr::Lit(Literal::Int(20))],
                            )),
                            Statement::Return(Expr::Var("y".to_string())),
                        ],
                    ),
                )],
            ),
            (
                r#"let main = fn () {
                    f(10, 20, 40);
                    100;
                    "foo";
                    let u = fn () { return 20; };
                };
                main();"#,
                vec![
                    (Statement::Let(
                        true,
                        "main".to_string(),
                        Expr::Fun(
                            11,
                            vec![],
                            vec![
                                (Statement::Expr(Expr::Call(
                                    "f".to_string(),
                                    vec![
                                        Expr::Lit(Literal::Int(10)),
                                        Expr::Lit(Literal::Int(20)),
                                        Expr::Lit(Literal::Int(40)),
                                    ],
                                ))),
                                (Statement::Expr(Expr::Lit(Literal::Int(100)))),
                                (Statement::Expr(Expr::Lit(Literal::String("foo".to_string())))),
                                (Statement::Let(
                                    false,
                                    "u".to_string(),
                                    Expr::Fun(
                                        134,
                                        vec![],
                                        vec![(Statement::Return(Expr::Lit(Literal::Int(20))))],
                                    ),
                                )),
                            ],
                        ),
                    )),
                    (Statement::Expr(Expr::Call("main".to_string(), vec![]))),
                ],
            ),
            (
                r#"let x = 10; return x;"#,
                vec![
                    (Statement::Let(true, "x".to_string(), Expr::Lit(Literal::Int(10)))),
                    (Statement::Return(Expr::Var("x".to_string()))),
                ],
            ),
            (
                r#"1; _panic(); 10;"#,
                vec![
                    Statement::Expr(Expr::Lit(Literal::Int(1))),
                    Statement::Expr(Expr::Call("_panic".to_string(), vec![])),
                    Statement::Expr(Expr::Lit(Literal::Int(10))),
                ],
            ),
            (
                r#"let x = 10; return &x;"#,
                vec![
                    Statement::Let(true, "x".to_string(), Expr::Lit(Literal::Int(10))),
                    Statement::Return(Expr::Ref(Box::new(Expr::Var("x".to_string())))),
                ],
            ),
            (
                r#"return 10 if _eq(x, y);"#,
                vec![Statement::ReturnIf(
                    Expr::Lit(Literal::Int(10)),
                    Expr::Call(
                        "_eq".to_string(),
                        vec![Expr::Var("x".to_string()), Expr::Var("y".to_string())],
                    ),
                )],
            ),
            (
                r#"
                    let x = loop {
                        return 10;
                    };

                    return x;
                "#,
                vec![
                    Statement::Let(
                        true,
                        "x".to_string(),
                        Expr::Loop(vec![Statement::Return(Expr::Lit(Literal::Int(10)))]),
                    ),
                    Statement::Return(Expr::Var("x".to_string())),
                ],
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
        let cases = vec![(r#"fn () { let u = 10 }"#, "Expected an ident")];

        for c in cases {
            let result = run_parser(c.0);
            assert!(matches!(result, Err(_)), "{:?} {:?}", c.0, result);

            let err = result.unwrap_err();
            assert!(err.to_string().contains(c.1), "{:?} {}", err, c.0);
        }
    }
}
