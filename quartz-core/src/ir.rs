use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::ast::{Structs, Type};

#[derive(PartialEq, Debug, Clone)]
pub enum IrTerm {
    Nil,
    Bool(bool),
    Int(i32),
    Ident(String),
    Argument(usize),
    Info(usize),
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
    pub fn ident(name: impl Into<String>) -> IrElement {
        IrElement::Term(IrTerm::Ident(name.into()))
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
                IrTerm::Ident(i) => format!("${}", i),
                IrTerm::Argument(a) => format!("${}", a),
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

    // = IR instructions

    pub fn nil() -> IrElement {
        IrElement::Term(IrTerm::Nil)
    }

    pub fn bool(b: bool) -> IrElement {
        IrElement::Term(IrTerm::Bool(b))
    }

    pub fn int(num: i32) -> IrElement {
        IrElement::Term(IrTerm::Int(num))
    }

    pub fn i_let(ident: String, element: IrElement) -> IrElement {
        IrElement::block("let", vec![IrElement::Term(IrTerm::Ident(ident)), element])
    }

    pub fn i_assign(lhs: IrElement, rhs: IrElement) -> IrElement {
        IrElement::block("assign", vec![lhs, rhs])
    }

    pub fn i_copy(size: usize, source: IrElement) -> IrElement {
        IrElement::block("copy", vec![IrElement::int(size as i32), source])
    }

    pub fn i_call(name: impl Into<String>, mut args: Vec<IrElement>) -> IrElement {
        args.insert(0, IrElement::Term(IrTerm::Ident(name.into())));

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

    pub fn i_deref(element: IrElement) -> IrElement {
        IrElement::block("deref", vec![element])
    }

    pub fn i_address(element: IrElement) -> IrElement {
        IrElement::block("address", vec![element])
    }

    pub fn i_index(element: IrElement, offset: IrElement) -> IrElement {
        IrElement::block("index", vec![element, offset])
    }

    pub fn i_addr_index(element: IrElement, offset: IrElement) -> IrElement {
        IrElement::block("addr_index", vec![element, offset])
    }

    pub fn i_offset(element: IrElement, offset: usize) -> IrElement {
        IrElement::block("offset", vec![element, IrElement::int(offset as i32)])
    }

    pub fn i_addr_offset(element: IrElement, offset: usize) -> IrElement {
        IrElement::block("addr_offset", vec![element, IrElement::int(offset as i32)])
    }

    pub fn i_tuple(typ: IrType, mut element: Vec<IrElement>) -> IrElement {
        element.insert(0, typ.to_element());

        IrElement::block("tuple", element)
    }

    pub fn i_slice(len: usize, typ: IrType, element: IrElement) -> IrElement {
        IrElement::block(
            "slice",
            vec![IrElement::int(len as i32), typ.to_element(), element],
        )
    }

    pub fn i_slice_raw(len: IrElement, typ: IrType, element: IrElement) -> IrElement {
        IrElement::block("slice", vec![len, typ.to_element(), element])
    }

    pub fn i_size_of(typ: IrType) -> IrElement {
        IrElement::block("size_of", vec![typ.to_element()])
    }

    pub fn i_pop(typ: IrType) -> IrElement {
        IrElement::block("pop", vec![typ.to_element()])
    }

    pub fn i_return(element: IrElement) -> IrElement {
        IrElement::block("return", vec![element])
    }

    pub fn d_var(name: impl Into<String>, typ: IrType, expr: IrElement) -> IrElement {
        IrElement::block(
            "var",
            vec![
                IrElement::Term(IrTerm::Ident(name.into())),
                typ.to_element(),
                expr,
            ],
        )
    }

    pub fn d_func(
        name: impl Into<String>,
        args: Vec<IrType>,
        ret: Box<IrType>,
        body: Vec<IrElement>,
    ) -> IrElement {
        let mut elements = vec![
            IrElement::Term(IrTerm::Ident(name.into())),
            IrElement::block(
                "args",
                args.into_iter().rev().map(|t| t.to_element()).collect(),
            ),
            IrElement::block("return", vec![ret.to_element()]),
        ];
        elements.extend(body);

        IrElement::block("func", elements)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrSingleType {
    Nil,
    Bool,
    Int,
    Address(Box<IrType>),
    Fn(Vec<IrType>, Box<IrType>),
    Byte,
}

impl IrSingleType {
    pub fn to_element(&self) -> IrElement {
        match self {
            IrSingleType::Nil => IrElement::ident("nil"),
            IrSingleType::Bool => IrElement::ident("bool"),
            IrSingleType::Int => IrElement::ident("int"),
            IrSingleType::Address(t) => IrElement::block("address", vec![t.to_element()]),
            IrSingleType::Fn(args, ret) => IrElement::block(
                "fn",
                vec![
                    IrElement::block("args", args.iter().map(|t| t.to_element()).collect()),
                    ret.to_element(),
                ],
            ),
            IrSingleType::Byte => IrElement::ident("byte"),
        }
    }

    pub fn unify(self, to: IrSingleType) -> Result<IrSingleType> {
        match (self, to) {
            (IrSingleType::Nil, IrSingleType::Nil) => Ok(IrSingleType::Nil),
            (IrSingleType::Bool, IrSingleType::Bool) => Ok(IrSingleType::Bool),
            (IrSingleType::Int, IrSingleType::Int) => Ok(IrSingleType::Int),
            (IrSingleType::Byte, IrSingleType::Byte) => Ok(IrSingleType::Byte),
            (IrSingleType::Address(t), IrSingleType::Address(u)) => {
                let unified = t.unify(u.as_ref().clone())?;

                Ok(IrSingleType::Address(Box::new(unified)))
            }
            (IrSingleType::Fn(args1, ret1), IrSingleType::Fn(args2, ret2)) => {
                if args1.len() != args2.len() {
                    bail!(
                        "function arity mismatch, {} vs {}",
                        args1.len(),
                        args2.len()
                    );
                }

                let mut args = Vec::new();
                for (t, u) in args1.iter().zip(args2.iter()) {
                    let unified = t.clone().unify(u.clone())?;

                    args.push(unified);
                }

                let unified = ret1.unify(ret2.as_ref().clone())?;

                Ok(IrSingleType::Fn(args, Box::new(unified)))
            }
            // nil can be an address
            (IrSingleType::Nil, IrSingleType::Address(t)) => Ok(IrSingleType::Address(t)),
            // byte can be an address
            (IrSingleType::Byte, IrSingleType::Address(t)) => Ok(IrSingleType::Address(t)),
            (s, t) => {
                bail!(
                    "Type want {} but got {}",
                    s.to_element().show_compact(),
                    t.to_element().show_compact()
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Unknown,
    Single(IrSingleType),
    Tuple(Vec<IrType>),
    Slice(usize, Box<IrType>),
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

    pub fn byte() -> IrType {
        IrType::Single(IrSingleType::Byte)
    }

    pub fn addr_of(t: IrType) -> IrType {
        IrType::Single(IrSingleType::Address(Box::new(t)))
    }

    pub fn addr_unknown() -> IrType {
        IrType::Single(IrSingleType::Address(Box::new(IrType::unknown())))
    }

    pub fn func(args: Vec<IrType>, ret: IrType) -> IrType {
        IrType::Single(IrSingleType::Fn(args, Box::new(ret)))
    }

    pub fn tuple(args: Vec<IrType>) -> IrType {
        IrType::Tuple(args)
    }

    pub fn slice(size: usize, typ: Box<IrType>) -> IrType {
        IrType::Slice(size, typ)
    }

    pub fn from_element(element: &IrElement) -> Result<IrType> {
        Ok(match element {
            IrElement::Term(t) => match t {
                IrTerm::Ident(ident) => match ident.as_str() {
                    "nil" => IrType::nil(),
                    "bool" => IrType::bool(),
                    "int" => IrType::int(),
                    "byte" => IrType::byte(),
                    "unknown" => IrType::unknown(),
                    _ => unreachable!("{:?}", t),
                },
                t => unreachable!("{:?}", t),
            },
            IrElement::Block(block) => match block.name.as_str() {
                "tuple" => {
                    let mut types = Vec::new();
                    for element in block.elements.iter() {
                        types.push(IrType::from_element(element)?);
                    }
                    IrType::tuple(types)
                }
                "slice" => IrType::slice(
                    block.elements[0].clone().into_term()?.into_int()? as usize,
                    Box::new(IrType::from_element(&block.elements[1])?),
                ),
                "address" => IrType::addr_of(IrType::from_element(&block.elements[0])?),
                t => unreachable!("{:?}", t),
            },
        })
    }

    pub fn from_type_ast(typ: &Type, structs: &Structs) -> Result<IrType> {
        IrType::from_type_ast_traced(typ, structs, vec![])
    }

    fn from_type_ast_traced(typ: &Type, structs: &Structs, trace: Vec<String>) -> Result<IrType> {
        Ok(match typ {
            Type::Nil => IrType::nil(),
            Type::Bool => IrType::bool(),
            Type::Int => IrType::int(),
            Type::Byte => IrType::byte(),
            Type::Fn(_, _) => todo!(),
            Type::Method(_, _, _) => todo!(),
            Type::Struct(s) if s == "string" => {
                // string = array[byte]
                IrType::from_type_ast_traced(&Type::Array(Box::new(Type::Byte)), structs, trace)?
            }
            Type::Struct(t) => {
                if trace.contains(t) {
                    return Ok(IrType::unknown());
                }

                let fields = structs.0.get(t).ok_or(anyhow::anyhow!(
                    "struct {} not found, {:?}",
                    t,
                    structs
                ))?;
                let mut types = Vec::new();
                for (_label, typ) in fields {
                    let mut current_trace = trace.clone();
                    current_trace.push(t.to_string());

                    types.push(IrType::from_type_ast_traced(typ, structs, current_trace)?);
                }
                IrType::tuple(types)
            }
            Type::Ref(t) => IrType::addr_of(IrType::from_type_ast_traced(t, structs, trace)?),
            Type::Array(t) => {
                // array[T] = (tuple (array[T]) (address (slice _ T)))
                // but slice is unsized, so we use *T instead
                IrType::tuple(vec![IrType::addr_of(IrType::addr_of(
                    IrType::from_type_ast_traced(t, structs, trace)?,
                ))])
            }
            Type::SizedArray(t, u) => IrType::slice(
                *u,
                Box::new(IrType::from_type_ast_traced(t.as_ref(), structs, trace)?),
            ),
            Type::Optional(t) => IrType::addr_of(IrType::from_type_ast_traced(t, structs, trace)?),
            Type::Self_ => todo!(),
            Type::Any => IrType::byte(),
            t => bail!("Unsupported type: {:?}", t),
        })
    }

    pub fn to_element(&self) -> IrElement {
        match self {
            IrType::Unknown => IrElement::ident("unknown"),
            IrType::Single(s) => s.to_element(),
            IrType::Tuple(ts) => {
                let mut elements = vec![];
                for t in ts {
                    elements.push(t.to_element());
                }

                IrElement::block("tuple", elements)
            }
            IrType::Slice(u, t) => {
                let mut elements = vec![];
                elements.push(IrElement::int(*u as i32));
                elements.push(t.to_element());
                IrElement::block("slice", elements)
            }
        }
    }

    pub fn size_of(&self) -> usize {
        match self {
            IrType::Unknown => todo!(),
            IrType::Single(_) => 1,
            IrType::Tuple(vs) => vs.into_iter().map(|v| v.size_of()).sum::<usize>() + 1, // +1 for a pointer to info table
            IrType::Slice(len, t) => len * t.size_of() + 1,
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            IrType::Unknown => true,
            _ => false,
        }
    }

    pub fn as_addr(&self) -> Result<Box<IrType>> {
        match self {
            IrType::Single(IrSingleType::Address(t)) => Ok(t.clone()),
            _ => bail!("{:?} is not address", self),
        }
    }

    pub fn as_func(&self) -> Option<(Vec<IrType>, Box<IrType>)> {
        match self {
            IrType::Single(IrSingleType::Fn(args, ret)) => Some((args.clone(), ret.clone())),
            _ => None,
        }
    }

    pub fn as_slice(&self) -> Option<(usize, Box<IrType>)> {
        match self {
            IrType::Slice(len, t) => Some((*len, t.clone())),
            _ => None,
        }
    }

    pub fn as_element(&self) -> Option<IrType> {
        match self {
            IrType::Tuple(ts) if ts.len() == 1 => Some(ts[0].as_addr().unwrap().as_ref().clone()),
            _ => None,
        }
    }

    pub fn as_element_sized(&self) -> Option<IrType> {
        match self {
            IrType::Slice(_, t) => Some(t.as_ref().clone()),
            // *T can be used as a slice
            IrType::Single(IrSingleType::Address(t)) => Some(t.as_ref().clone()),
            _ => None,
        }
    }

    pub fn unify(self, from: IrType) -> Result<IrType> {
        match (self, from) {
            (s, t) if s == t => Ok(s),
            (IrType::Unknown, t) => Ok(t),
            (s, IrType::Unknown) => Ok(s),
            (IrType::Single(s), IrType::Single(t)) => Ok(IrType::Single(s.unify(t)?)),
            (IrType::Tuple(ts), Self::Tuple(vs)) => {
                if ts.len() != vs.len() {
                    bail!("{:?} and {:?} are not unifiable", ts, vs);
                }

                let mut result = vec![];
                for (t, s) in ts.into_iter().zip(vs) {
                    result.push(t.unify(s)?);
                }

                Ok(IrType::Tuple(result))
            }
            // slice as an address
            (IrType::Slice(_, _), IrType::Single(IrSingleType::Address(s))) => {
                Ok(IrType::Single(IrSingleType::Address(s)))
            }
            (s, t) => {
                bail!(
                    "Type want {} but got {}",
                    t.to_element().show_compact(),
                    s.to_element().show_compact(),
                )
            }
        }
    }

    pub fn offset(self, index: usize) -> Result<IrType> {
        match self {
            IrType::Single(IrSingleType::Address(t)) => Ok(t.as_ref().clone()),
            IrType::Slice(r, t) => {
                if index < r {
                    Ok(t.as_ref().clone())
                } else {
                    bail!("Out of offset, {} in {:?}", index, IrType::Slice(r, t))
                }
            }
            IrType::Tuple(ts) => {
                if index < ts.len() {
                    Ok(ts[index].clone())
                } else {
                    bail!("Out of offset, {} in {:?}", index, IrType::Tuple(ts))
                }
            }
            _ => bail!("Type is not address"),
        }
    }

    pub fn offset_in_words(self, index: usize) -> Result<usize> {
        let mut result = 1;
        for i in 0..index {
            result += self.clone().offset(i)?.size_of();
        }

        Ok(result)
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
                let token = IrLexeme::Argument(index);
                position += m.end() + 1;

                tokens.push(token);

                continue;
            }

            if let Some(m) = IDENT_PATTERN.find(&input[position + 1..]) {
                let ident = m.as_str().to_string();
                let token = IrLexeme::Ident(ident.clone());
                position += m.end() + 1;

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
