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

    fn expr(&mut self) -> Result<Expr> {
        // var
        (self.ident().map(|v| Expr::Var(v))).or_else(
            // literal
            |_| self.literal().map(|v| Expr::Lit(v)),
        )
    }

    fn statement(&mut self) -> Result<Expr> {
        // return expr;
        self.expect_lexeme(Lexeme::Return)?;
        let expr = self.expr()?;

        Ok(Expr::Return(Box::new(expr)))
    }

    fn statments_block(&mut self) -> Result<Vec<Expr>> {
        self.expect_lexeme(Lexeme::LBrace)?;

        // single statement
        let s = self.statement()?;

        self.expect_lexeme(Lexeme::RBrace)?;

        Ok(vec![s])
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
    parser.run_parser(tokens)
}

pub fn run_parser(input: &str) -> Result<Decl> {
    run_parser_from_tokens(run_lexer(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_parser() {
        let cases = vec![(
            "func main() { return 10; }",
            Decl::Func(
                "main".to_string(),
                vec![Expr::Return(Box::new(Expr::Lit(Literal::IntLiteral(10))))],
            ),
        )];

        for c in cases {
            assert_eq!(run_parser(c.0).unwrap(), c.1, "{:?}", c.0);
        }
    }
}
