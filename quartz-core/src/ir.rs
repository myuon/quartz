use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct IrIdent(String);

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    Bool(bool),
    Int(i32),
    Address(usize),
    Ident(IrIdent),
    Argument(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub struct IrInstruction {
    code: IrIdent,
    arguments: Vec<IrTerm>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrBlock {
    Function {
        name: IrIdent,
        body: Vec<IrInstruction>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct IrModule(Vec<IrBlock>);

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\$)?[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());
static STRING_LITERAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"((([^"]|\\")*[^\\])?)""#).unwrap());
static COMMENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^//[^\n]*\n"#).unwrap());

#[derive(PartialEq, Debug, Clone)]
enum IrLexeme {
    Ident(String),
    String(String),
    Number(String),
    LParen,
    RParen,
}

fn run_lexer(input: &str) -> Vec<IrLexeme> {
    let mut tokens = vec![];
    let mut position = 0;

    while position < input.len() {
        if let Some(m) = SPACE_PATTERN.find(&input[position..]) {
            position += m.end();
            continue;
        }

        if &input[position..position + 1] == "(" {
            tokens.push(IrLexeme::LParen);
            position += 1;
            continue;
        }

        if &input[position..position + 1] == ")" {
            tokens.push(IrLexeme::RParen);
            position += 1;
            continue;
        }

        if &input[position..position + 1] == "\"" {
            if let Some(m) = IDENT_PATTERN.find(&input[position..]) {
                tokens.push(IrLexeme::String(m.as_str().to_string()));

                position += m.end();
                continue;
            }
        }

        if let Some(m) = NUMBER_PATTERN.find(&input[position..]) {
            tokens.push(IrLexeme::Number(m.as_str().to_string()));

            position += m.end();
            continue;
        }

        if let Some(m) = IDENT_PATTERN.find(&input[position..]) {
            tokens.push(IrLexeme::Ident(m.as_str().to_string()));

            position += m.end();
            continue;
        }

        break;
    }

    tokens
}

struct IrParser<'s> {
    position: usize,
    tokens: &'s [IrLexeme],
}

impl IrParser<'_> {
    fn next(&mut self) -> &IrLexeme {
        let token = &self.tokens[self.position];
        self.position += 1;

        token
    }

    fn expect_ident(&mut self, ident: &str) -> Result<()> {
        match &self.tokens[self.position] {
            IrLexeme::Ident(i) if i == ident => {
                self.position += 1;
                return Ok(());
            }
            lexeme => {
                bail!("Expected {:?} but got {:?}", ident, lexeme);
            }
        }
    }

    fn expect(&mut self, lexeme: IrLexeme) -> Result<()> {
        if self.tokens[self.position] == lexeme {
            self.position += 1;
            return Ok(());
        } else {
            bail!(
                "Expected {:?} but got {:?}",
                lexeme,
                self.tokens[self.position]
            );
        }
    }

    fn ident(&mut self) -> Result<IrIdent> {
        match &self.tokens[self.position] {
            IrLexeme::Ident(i) if i.starts_with("$") => {
                self.position += 1;
                return Ok(IrIdent(i[1..].to_string()));
            }
            lexeme => {
                bail!("Expected keyword but got {:?}", lexeme);
            }
        }
    }

    fn instruction(&mut self) -> Result<IrInstruction> {
        let mut terms = vec![];

        self.expect(IrLexeme::LParen)?;
        let code = match self.next() {
            IrLexeme::Ident(i) => IrIdent(i.to_string()),
            _ => unreachable!(),
        };
        while self.tokens[self.position] != IrLexeme::RParen {
            let term = self.term()?;
            terms.push(term);
        }
        self.expect(IrLexeme::RParen)?;

        Ok(IrInstruction {
            code,
            arguments: terms,
        })
    }

    fn instructions(&mut self) -> Result<Vec<IrInstruction>> {
        let mut instructions = vec![];

        self.expect(IrLexeme::LParen)?;
        instructions.push(self.instruction()?);
        self.expect(IrLexeme::RParen)?;

        Ok(instructions)
    }

    fn term(&mut self) -> Result<IrTerm> {
        let token = self.next();

        Ok(match token {
            IrLexeme::Ident(ident) if ident.starts_with("$") => IrTerm::Ident(self.ident()?),
            IrLexeme::Ident(ident) => {
                if ident == "nil" {
                    IrTerm::Nil
                } else if ident == "true" {
                    IrTerm::Bool(true)
                } else if ident == "false" {
                    IrTerm::Bool(false)
                } else {
                    bail!("Unknown identifier {:?}", ident);
                }
            }
            IrLexeme::Number(n) => {
                if let Ok(d) = n.parse::<i32>() {
                    IrTerm::Int(d)
                } else {
                    bail!("Invalid number {:?}", n);
                }
            }
            _ => unreachable!(),
        })
    }

    fn block(&mut self) -> Result<IrBlock> {
        self.expect(IrLexeme::LParen)?;
        self.expect_ident("func")?;
        let name = self.ident()?;
        let body = self.instructions()?;
        self.expect(IrLexeme::RParen)?;

        Ok(IrBlock::Function { name, body })
    }

    fn module(&mut self) -> Result<IrModule> {
        let mut blocks = vec![];

        self.expect(IrLexeme::LParen)?;
        self.expect_ident("module")?;
        while self.tokens[self.position] != IrLexeme::RParen {
            blocks.push(self.block()?);
        }
        self.expect(IrLexeme::RParen)?;

        Ok(IrModule(blocks))
    }
}

pub fn parse_ir(input: &str) -> Result<IrModule> {
    let tokens = run_lexer(input);
    let mut parser = IrParser {
        position: 0,
        tokens: &tokens,
    };

    parser.module()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_run_lexer() {
        let cases = vec![(
            r#"
(module
    (func $main (
        (let $x 10)
        (assign $x 20)
        (return $x)
    ))
)
"#,
            vec![
                IrLexeme::LParen,
                IrLexeme::Ident("module".to_string()),
                IrLexeme::LParen,
                IrLexeme::Ident("func".to_string()),
                IrLexeme::Ident("$main".to_string()),
                IrLexeme::LParen,
                IrLexeme::LParen,
                IrLexeme::Ident("let".to_string()),
                IrLexeme::Ident("$x".to_string()),
                IrLexeme::Number("10".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Ident("assign".to_string()),
                IrLexeme::Ident("$x".to_string()),
                IrLexeme::Number("20".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Ident("return".to_string()),
                IrLexeme::Ident("$x".to_string()),
                IrLexeme::RParen,
                IrLexeme::RParen,
                IrLexeme::RParen,
                IrLexeme::RParen,
            ],
        )];

        for (input, result) in cases {
            assert_eq!(result, run_lexer(input));
        }
    }

    #[test]
    fn test_parse_ir() {
        let cases = vec![(
            r#"
(module
    (func $main (
        (let $x 10)
        (assign $x 20)
        (return $x)
    ))
)
"#,
            vec![],
        )];

        for (input, result) in cases {
            let ast = parse_ir(input);

            assert!(ast.is_ok(), "Error:{:?}\n{}", ast, input);
            assert_eq!(IrModule(result), ast.unwrap(), "{}", input);
        }
    }
}
