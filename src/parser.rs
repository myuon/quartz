use crate::{
    ast::{Decl, Expr, Literal},
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

    fn expect_lexeme(&mut self, lexeme: Lexeme) -> Result<()> {
        ensure!(
            self.input[self.position].lexeme == lexeme,
            "Expected {:?} but found {:?}",
            lexeme,
            &self.input[self.position..]
        );
        self.position += 1;

        Ok(())
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
        match self.input[self.position].lexeme {
            Lexeme::IntLiteral(n) => {
                self.position += 1;

                Ok(Literal::IntLiteral(n))
            }
            _ => bail!(
                "Expected a literal but found {:?}",
                &self.input[self.position..]
            ),
        }
    }

    fn expr_short(&mut self) -> Result<Expr> {
        // var
        (self.ident().map(|v| Expr::Var(v))).or_else(
            // literal
            |_| self.literal().map(|v| Expr::Lit(v)),
        )
    }

    fn expr(&mut self) -> Result<Expr> {
        self.expr_short()
    }

    fn statement(&mut self) -> Result<Expr> {
        (|| -> Result<Expr> {
            // return expr;
            self.expect_lexeme(Lexeme::Return)?;
            let expr = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Expr::Return(Box::new(expr)))
        }())
        .or_else(|_| -> Result<Expr> {
            // let v = e;
            self.expect_lexeme(Lexeme::Let)?;
            let var = self.ident()?;
            self.expect_lexeme(Lexeme::Equal)?;
            let expr = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Expr::Let(var, Box::new(expr)))
        })
        .or_else(|_| -> Result<Expr> {
            // const v = e;
            self.expect_lexeme(Lexeme::Const)?;
            let var = self.ident()?;
            self.expect_lexeme(Lexeme::Equal)?;
            let expr = self.expr()?;
            self.expect_lexeme(Lexeme::SemiColon)?;

            Ok(Expr::Const(var, Box::new(expr)))
        })
        .or_else(|_| -> Result<Expr> {
            // exprStatement (s) OR s = e;
            let s = self.expr_short()?;
            if self.expect_lexeme(Lexeme::Equal).is_err() {
                Ok(s)
            } else {
                let e = self.expr()?;
                self.expect_lexeme(Lexeme::SemiColon)?;

                Ok(Expr::Assign(Box::new(s), Box::new(e)))
            }
        })
    }

    fn statments_block(&mut self) -> Result<Vec<Expr>> {
        self.expect_lexeme(Lexeme::LBrace)?;

        let mut stmts = vec![];
        loop {
            match self.statement() {
                Ok(r) => {
                    stmts.push(r);
                    continue;
                }
                Err(_) => {
                    // need recover
                    break;
                }
            }
        }

        self.expect_lexeme(Lexeme::RBrace)?;

        Ok(stmts)
    }

    fn parse_decl(&mut self) -> Result<Decl> {
        // func
        self.expect_lexeme(Lexeme::Func)?;
        let name = self.ident()?;

        self.expect_lexeme(Lexeme::LParen)?;
        self.expect_lexeme(Lexeme::RParen)?;

        let body = self.statments_block()?;

        Ok(Decl::Func(name, body))
    }

    pub fn run_parser(&mut self, tokens: Vec<Token>) -> Result<Decl> {
        self.position = 0;
        self.input = tokens;

        self.parse_decl()
    }
}

pub fn run_parser_from_tokens(tokens: Vec<Token>) -> Result<Decl> {
    let mut parser = Parser::new();
    let result = parser.run_parser(tokens)?;

    if parser.input.len() != parser.position {
        bail!("Unexpected token: {:?}", &parser.input[parser.position..]);
    }

    Ok(result)
}

pub fn run_parser(input: &str) -> Result<Decl> {
    run_parser_from_tokens(run_lexer(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_parser() {
        let cases = vec![
            (
                r#"func main() {
                    const x = 10;
                    let y = 10;
                    y = 20;

                    return 10;
                }"#,
                Decl::Func(
                    "main".to_string(),
                    vec![
                        Expr::Const(
                            "x".to_string(),
                            Box::new(Expr::Lit(Literal::IntLiteral(10))),
                        ),
                        Expr::Let(
                            "y".to_string(),
                            Box::new(Expr::Lit(Literal::IntLiteral(10))),
                        ),
                        Expr::Assign(
                            Box::new(Expr::Var("y".to_string())),
                            Box::new(Expr::Lit(Literal::IntLiteral(20))),
                        ),
                        Expr::Return(Box::new(Expr::Lit(Literal::IntLiteral(10)))),
                    ],
                ),
            ),
            (
                // 最後にexprを返す形
                r#"func main() {
                    x = 10;

                    200
                }"#,
                Decl::Func(
                    "main".to_string(),
                    vec![
                        Expr::Assign(
                            Box::new(Expr::Var("x".to_string())),
                            Box::new(Expr::Lit(Literal::IntLiteral(10))),
                        ),
                        Expr::Lit(Literal::IntLiteral(200)),
                    ],
                ),
            ),
        ];

        for c in cases {
            let result = run_parser(c.0);
            assert!(matches!(result, Ok(_)), "{:?} {:?}", c.0, result);
            assert_eq!(result.unwrap(), c.1, "{:?}", c.0);
        }
    }
}
