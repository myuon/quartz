use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    Bool(bool),
    Int(i32),
    Ident(String),
    Argument(usize),
    Keyword(String),
}

impl IrTerm {
    pub fn into_ident(self) -> Result<String> {
        match self {
            IrTerm::Ident(s) => Ok(s),
            _ => bail!("expected ident"),
        }
    }

    pub fn into_int(self) -> Result<i32> {
        match self {
            IrTerm::Int(i) => Ok(i),
            _ => bail!("expected int"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IrBlock {
    pub name: String,
    pub elements: Vec<IrElement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IrElement {
    Term(IrTerm),
    Block(IrBlock),
}

impl IrElement {
    pub fn block(name: &str, elements: Vec<IrElement>) -> IrElement {
        IrElement::Block(IrBlock {
            name: name.to_string(),
            elements,
        })
    }

    pub fn instruction(name: &str, elements: Vec<IrTerm>) -> IrElement {
        IrElement::Block(IrBlock {
            name: name.to_string(),
            elements: elements.into_iter().map(|e| IrElement::Term(e)).collect(),
        })
    }

    pub fn into_term(self) -> Result<IrTerm> {
        match self {
            IrElement::Term(t) => Ok(t),
            _ => bail!("Expected a term, but found {:?}", self),
        }
    }

    pub fn into_block(self) -> Result<IrBlock> {
        match self {
            IrElement::Block(b) => Ok(b),
            _ => bail!("Expected a block, but found {:?}", self),
        }
    }

    fn show_recur(&self, depth: i32, compact: bool) -> String {
        match self {
            IrElement::Term(t) => match t {
                IrTerm::Nil => "nil".to_string(),
                IrTerm::Bool(b) => format!("{}", b),
                IrTerm::Int(n) => format!("{}", n),
                IrTerm::Ident(i) => format!("${}", i),
                IrTerm::Argument(a) => format!("${}", a),
                IrTerm::Keyword(k) => format!("{}", k),
            },
            IrElement::Block(b) => {
                let mut buffer = String::new();
                let indent = if compact {
                    if depth > 0 {
                        " ".to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "  ".repeat(depth as usize)
                };

                buffer.push_str(&format!("{}({}", indent, b.name));
                for e in &b.elements {
                    match e {
                        IrElement::Term(_) => {
                            buffer.push_str(&format!(" {}", e.show_recur(depth, compact)));
                        }
                        IrElement::Block(_) => {
                            buffer.push_str(&format!(
                                "{}{}",
                                if compact { "" } else { "\n" },
                                e.show_recur(depth + 1, compact)
                            ));
                        }
                    }
                }
                buffer.push_str(")");

                buffer
            }
        }
    }

    pub fn show(&self) -> String {
        self.show_recur(0, false)
    }

    pub fn show_compact(&self) -> String {
        self.show_recur(0, true)
    }
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());

#[derive(PartialEq, Debug, Clone)]
enum IrLexeme {
    Ident(String), // $ident
    Keyword(String),
    Argument(usize),
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

        if let Some(m) = NUMBER_PATTERN.find(&input[position..]) {
            tokens.push(IrLexeme::Number(m.as_str().to_string()));

            position += m.end();
            continue;
        }

        if &input[position..position + 1] == "$" {
            if let Some(m) = NUMBER_PATTERN.find(&input[position + 1..]) {
                let index = m.as_str().parse::<usize>().unwrap();
                tokens.push(IrLexeme::Argument(index));

                position += m.end() + 1;
                continue;
            }

            if let Some(m) = IDENT_PATTERN.find(&input[position + 1..]) {
                tokens.push(IrLexeme::Ident(m.as_str().to_string()));

                position += m.end() + 1;
                continue;
            }

            unreachable!("{:?}", &input[position..position + 20]);
        }

        if let Some(m) = IDENT_PATTERN.find(&input[position..]) {
            let name = m.as_str();
            tokens.push(IrLexeme::Keyword(name.to_string()));

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

    fn term(&mut self) -> Result<IrTerm> {
        let token = self.next();

        Ok(match token {
            IrLexeme::Ident(ident) => IrTerm::Ident(ident.to_string()),
            IrLexeme::Argument(arg) => IrTerm::Argument(*arg),
            IrLexeme::Keyword(ident) => {
                if ident == "nil" {
                    IrTerm::Nil
                } else if ident == "true" {
                    IrTerm::Bool(true)
                } else if ident == "false" {
                    IrTerm::Bool(false)
                } else {
                    bail!("Unknown keyword {:?}", ident);
                }
            }
            IrLexeme::Number(n) => {
                if let Ok(d) = n.parse::<i32>() {
                    IrTerm::Int(d)
                } else {
                    bail!("Invalid number {:?}", n);
                }
            }
            token => unreachable!("{:?}", token),
        })
    }

    fn element(&mut self) -> Result<IrElement> {
        if self.expect(IrLexeme::LParen).is_ok() {
            let name = match self.next() {
                IrLexeme::Keyword(i) => i.to_string(),
                _ => unreachable!(),
            };
            let mut elements = vec![];

            while self.tokens[self.position] != IrLexeme::RParen {
                elements.push(self.element()?);
            }

            self.expect(IrLexeme::RParen)?;

            Ok(IrElement::Block(IrBlock { name, elements }))
        } else {
            let term = self.term()?;

            Ok(IrElement::Term(term))
        }
    }
}

pub fn parse_ir(input: &str) -> Result<IrElement> {
    let tokens = run_lexer(input);
    let mut parser = IrParser {
        position: 0,
        tokens: &tokens,
    };

    parser.element()
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
                IrLexeme::Keyword("module".to_string()),
                IrLexeme::LParen,
                IrLexeme::Keyword("func".to_string()),
                IrLexeme::Ident("main".to_string()),
                IrLexeme::LParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("let".to_string()),
                IrLexeme::Ident("x".to_string()),
                IrLexeme::Number("10".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("assign".to_string()),
                IrLexeme::Ident("x".to_string()),
                IrLexeme::Number("20".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("return".to_string()),
                IrLexeme::Ident("x".to_string()),
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
    (func $main
        (let $x 10)
        (assign $x 20)
        (return $x)
    )
)
"#,
            IrElement::Block(IrBlock {
                name: "module".to_string(),
                elements: vec![IrElement::Block(IrBlock {
                    name: "func".to_string(),
                    elements: vec![
                        IrElement::Term(IrTerm::Ident("main".to_string())),
                        IrElement::Block(IrBlock {
                            name: "let".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string())),
                                IrElement::Term(IrTerm::Int(10)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "assign".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string())),
                                IrElement::Term(IrTerm::Int(20)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "return".to_string(),
                            elements: vec![IrElement::Term(IrTerm::Ident("x".to_string()))],
                        }),
                    ],
                })],
            }),
        )];

        for (input, result) in cases {
            let ast = parse_ir(input);

            assert!(ast.is_ok(), "Error:{:?}\n{}", ast, input);
            assert_eq!(result, ast.unwrap(), "{}", input);
        }
    }
}
