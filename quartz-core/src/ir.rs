use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::ast::{size_of, Structs, Type};

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    Bool(bool),
    Int(i32),
    Ident(String, usize),   // name, size
    Argument(usize, usize), // index, size
    Info(usize),
}

impl IrTerm {
    pub fn into_ident(self) -> Result<String> {
        match self {
            IrTerm::Ident(s, _) => Ok(s),
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
    pub fn ident(name: impl Into<String>) -> IrElement {
        IrElement::Term(IrTerm::Ident(name.into(), 1))
    }

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
                IrTerm::Ident(i, t) => {
                    if *t > 1 {
                        format!("${}({})", i, t)
                    } else {
                        format!("${}", i)
                    }
                }
                IrTerm::Argument(a, t) => {
                    if *t > 1 {
                        format!("${}({})", a, t)
                    } else {
                        format!("${}", a)
                    }
                }
                IrTerm::Info(i) => format!("{}", i),
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

    // = Types

    pub fn ir_type(typ: &Type, structs: &Structs) -> Result<IrElement> {
        Ok(match typ {
            Type::Bool => IrElement::ident("bool"),
            Type::Int => IrElement::ident("int"),
            Type::Byte => IrElement::ident("byte"),
            Type::Nil => IrElement::ident("nil"),
            Type::Fn(_, _) => IrElement::ident("addr"),
            Type::Struct(t) => IrElement::block(
                "struct",
                vec![IrElement::int(
                    size_of(&Type::Struct(t.clone()), structs) as i32
                )],
            ),
            Type::Array(_) => IrElement::ident("array"),
            Type::SizedArray(t, n) => IrElement::block(
                "sizedarray",
                vec![IrElement::ir_type(t, structs)?, IrElement::int(*n as i32)],
            ),
            Type::Ref(_) => IrElement::ident("ref"),
            Type::Optional(t) => {
                IrElement::block("optional", vec![IrElement::ir_type(t, structs)?])
            }
            t => bail!("Unsupported type: {:?}", t),
        })
    }

    pub fn size_of_as_ir_type(typ: &IrElement) -> Result<usize> {
        match typ {
            IrElement::Term(IrTerm::Ident(_, _)) => Ok(1),
            IrElement::Block(block) => {
                if &block.name == "struct" {
                    Ok(block.elements[0].clone().into_term()?.into_int()? as usize)
                } else if &block.name == "optional" {
                    IrElement::size_of_as_ir_type(&block.elements[0])
                } else if &block.name == "sizedarray" {
                    Ok(block.elements[1].clone().into_term()?.into_int()? as usize)
                } else {
                    unreachable!("{:?}", block)
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn is_ir_type_addr(typ: &IrElement) -> bool {
        match typ {
            IrElement::Term(term) => match term {
                IrTerm::Ident(ident, _) => {
                    if ident == "addr" {
                        return true;
                    }

                    return false;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    // = IR instructions

    pub fn nil() -> IrElement {
        IrElement::Term(IrTerm::Nil)
    }

    pub fn int(num: i32) -> IrElement {
        IrElement::Term(IrTerm::Int(num))
    }

    pub fn i_let(typ: IrElement, ident: String, element: IrElement) -> IrElement {
        IrElement::block(
            "let",
            vec![typ, IrElement::Term(IrTerm::Ident(ident, 1)), element],
        )
    }

    pub fn i_assign(size: usize, lhs: IrElement, rhs: IrElement) -> IrElement {
        IrElement::block("assign", vec![IrElement::int(size as i32), lhs, rhs])
    }

    pub fn i_unload(element: IrElement) -> IrElement {
        IrElement::block("unload", vec![element])
    }

    pub fn i_load(size: usize, element: IrElement) -> IrElement {
        IrElement::block("load", vec![IrElement::int(size as i32), element])
    }

    pub fn i_copy(size: usize, source: IrElement) -> IrElement {
        IrElement::block("copy", vec![IrElement::int(size as i32), source])
    }

    pub fn i_call(name: impl Into<String>, mut args: Vec<IrElement>) -> IrElement {
        args.insert(0, IrElement::Term(IrTerm::Ident(name.into(), 1)));

        IrElement::i_call_raw(args)
    }

    pub fn i_call_raw(args: Vec<IrElement>) -> IrElement {
        IrElement::block("call", args)
    }

    pub fn i_coerce(actual_size: usize, expected_size: usize, element: IrElement) -> IrElement {
        IrElement::block(
            "coerce",
            vec![
                IrElement::int(actual_size as i32),
                IrElement::int(expected_size as i32),
                element,
            ],
        )
    }

    pub fn i_deref(size: usize, element: IrElement) -> IrElement {
        IrElement::block("deref", vec![IrElement::int(size as i32), element])
    }

    pub fn i_address(element: IrElement) -> IrElement {
        IrElement::block("address", vec![element])
    }

    pub fn i_offset(size: usize, element: IrElement, offset: IrElement) -> IrElement {
        IrElement::block("offset", vec![IrElement::int(size as i32), element, offset])
    }

    pub fn i_offset_im(size: usize, element: IrElement, offset: usize) -> IrElement {
        IrElement::i_offset(size, element, IrElement::int(offset as i32))
    }
}

#[derive(Debug, Clone)]
pub enum IrSingleType {
    Nil,
    Bool,
    Int,
    Address,
    Fn(Vec<IrType>, Box<IrType>),
}

impl IrSingleType {
    pub fn to_element(&self) -> IrElement {
        match self {
            IrSingleType::Nil => IrElement::ident("nil"),
            IrSingleType::Bool => IrElement::ident("bool"),
            IrSingleType::Int => IrElement::ident("int"),
            IrSingleType::Address => IrElement::ident("address"),
            IrSingleType::Fn(args, ret) => IrElement::block(
                "fn",
                vec![
                    IrElement::block("args", args.iter().map(|t| t.to_element()).collect()),
                    ret.to_element(),
                ],
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IrType {
    Unknown,
    Single(IrSingleType),
    Tuple(usize, Vec<IrType>),
}

impl IrType {
    pub fn unknown() -> IrType {
        IrType::Unknown
    }

    pub fn nil() -> IrType {
        IrType::Single(IrSingleType::Nil)
    }

    pub fn bool() -> IrType {
        IrType::Single(IrSingleType::Bool)
    }

    pub fn int() -> IrType {
        IrType::Single(IrSingleType::Int)
    }

    pub fn addr() -> IrType {
        IrType::Single(IrSingleType::Address)
    }

    pub fn func(args: Vec<IrType>, ret: Box<IrType>) -> IrType {
        IrType::Single(IrSingleType::Fn(args, ret))
    }

    pub fn tuple(size: usize, args: Vec<IrType>) -> IrType {
        IrType::Tuple(size, args)
    }

    pub fn from_element(element: &IrElement) -> Result<IrType> {
        Ok(match element {
            IrElement::Term(t) => match t {
                IrTerm::Ident(ident, _) => match ident.as_str() {
                    "nil" => IrType::nil(),
                    "bool" => IrType::bool(),
                    "int" => IrType::int(),
                    "address" => IrType::addr(),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            IrElement::Block(block) => match block.name.as_str() {
                "tuple" => {
                    let mut types = Vec::new();
                    let size = block.elements[0].clone().into_term()?.into_int()? as usize;

                    for element in block.elements.iter().skip(1) {
                        types.push(IrType::from_element(element)?);
                    }
                    IrType::tuple(size, types)
                }
                _ => unreachable!(),
            },
        })
    }

    pub fn to_element(&self) -> IrElement {
        match self {
            IrType::Unknown => todo!(),
            IrType::Single(s) => s.to_element(),
            IrType::Tuple(u, ts) => {
                let mut elements = vec![];
                elements.push(IrElement::int(*u as i32));
                for t in ts {
                    elements.push(t.to_element());
                }

                IrElement::block("tuple", elements)
            }
        }
    }

    pub fn size_of(&self) -> usize {
        match self {
            IrType::Unknown => todo!(),
            IrType::Single(_) => 1,
            IrType::Tuple(_t, vs) => vs.into_iter().map(|v| v.size_of()).sum(),
        }
    }
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());

#[derive(PartialEq, Debug, Clone)]
enum IrLexeme {
    Ident(String, usize), // $ident
    Keyword(String),
    Argument(usize, usize),
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
                let mut token = IrLexeme::Argument(index, 1);
                position += m.end() + 1;

                if &input[position..position + 1] == "(" {
                    position += 1;

                    if let Some(m) = NUMBER_PATTERN.find(&input[position..]) {
                        let size = m.as_str().parse::<usize>().unwrap();
                        token = IrLexeme::Argument(index, size);
                        position += m.end();
                    }
                    assert_eq!(&input[position..position + 1], ")");
                    position += 1;
                }

                tokens.push(token);

                continue;
            }

            if let Some(m) = IDENT_PATTERN.find(&input[position + 1..]) {
                let ident = m.as_str().to_string();
                let mut token = IrLexeme::Ident(ident.clone(), 1);
                position += m.end() + 1;

                if &input[position..position + 1] == "(" {
                    position += 1;

                    if let Some(m) = NUMBER_PATTERN.find(&input[position..]) {
                        let size = m.as_str().parse::<usize>().unwrap();
                        token = IrLexeme::Ident(ident, size);
                        position += m.end();
                    }
                    assert_eq!(&input[position..position + 1], ")");
                    position += 1;
                }

                tokens.push(token);

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
            IrLexeme::Ident(ident, i) => IrTerm::Ident(ident.to_string(), *i), // FIXME: support multiple words
            IrLexeme::Argument(arg, i) => IrTerm::Argument(*arg, *i), // FIXME: support multiple words
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
                IrLexeme::Ident("main".to_string(), 1),
                IrLexeme::LParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("let".to_string()),
                IrLexeme::Ident("x".to_string(), 1),
                IrLexeme::Number("10".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("assign".to_string()),
                IrLexeme::Ident("x".to_string(), 1),
                IrLexeme::Number("20".to_string()),
                IrLexeme::RParen,
                IrLexeme::LParen,
                IrLexeme::Keyword("return".to_string()),
                IrLexeme::Ident("x".to_string(), 1),
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
                        IrElement::Term(IrTerm::Ident("main".to_string(), 1)),
                        IrElement::Block(IrBlock {
                            name: "let".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string(), 1)),
                                IrElement::Term(IrTerm::Int(10)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "assign".to_string(),
                            elements: vec![
                                IrElement::Term(IrTerm::Ident("x".to_string(), 1)),
                                IrElement::Term(IrTerm::Int(20)),
                            ],
                        }),
                        IrElement::Block(IrBlock {
                            name: "return".to_string(),
                            elements: vec![IrElement::Term(IrTerm::Ident("x".to_string(), 1))],
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
