use std::io::{BufWriter, Write};

use anyhow::Result;

use crate::lexer::{Lexeme, Token};

enum BlockType {
    Paren,   // ()
    Brace,   // {}
    Bracket, // []
}

enum Fragment {
    Token(Token),
    Block(BlockType, Vec<Sequence>),
}

struct Sequence(Vec<Fragment>);

struct FWriter {}

impl FWriter {
    pub fn sequence(&mut self, writer: &mut impl Write, sequence: Sequence) {
        for fragment in sequence.0 {
            self.fragment(writer, fragment);
        }
    }

    pub fn fragment(&mut self, writer: &mut impl Write, fragment: Fragment) {
        match fragment {
            Fragment::Token(token) => {
                write!(writer, "{}", token.raw).unwrap();
            }
            Fragment::Block(block_type, items) => {
                self.block(writer, block_type, items);
            }
        }
    }

    pub fn block(&mut self, writer: &mut impl Write, block_type: BlockType, items: Vec<Sequence>) {
        match block_type {
            BlockType::Paren => {
                write!(writer, "{}", "(").unwrap();
            }
            BlockType::Brace => {
                write!(writer, "{}", "{").unwrap();
            }
            BlockType::Bracket => {
                write!(writer, "{}", "[").unwrap();
            }
        }

        for item in items {
            self.sequence(writer, item);
        }

        match block_type {
            BlockType::Paren => {
                write!(writer, ")").unwrap();
            }
            BlockType::Brace => {
                write!(writer, "}}").unwrap();
            }
            BlockType::Bracket => {
                write!(writer, "]").unwrap();
            }
        }
    }
}

struct Formatter {
    input: Vec<Token>,
    position: usize,
}

impl Formatter {
    pub fn new(input: Vec<Token>) -> Formatter {
        Formatter { input, position: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.input.get(self.position).cloned()
    }

    fn consume(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Sequence {
        self.sequence()
    }

    pub fn sequence(&mut self) -> Sequence {
        let mut fragments = Vec::new();

        while let Some(token) = self.peek() {
            match token.lexeme {
                Lexeme::LParen => {
                    fragments.push(self.block(BlockType::Paren));
                }
                Lexeme::Comma => {
                    fragments.push(Fragment::Token(token.clone()));
                    break;
                }
                _ => {
                    fragments.push(Fragment::Token(token.clone()));
                }
            }
        }

        Sequence(fragments)
    }

    fn block(&mut self, block_type: BlockType) -> Fragment {
        let mut items = Vec::new();

        while let Some(token) = self.peek() {
            self.consume();

            match token.lexeme {
                Lexeme::RParen if matches!(block_type, BlockType::Paren) => {
                    break;
                }
                Lexeme::RBrace if matches!(block_type, BlockType::Brace) => {
                    break;
                }
                Lexeme::RBracket if matches!(block_type, BlockType::Bracket) => {
                    break;
                }
                _ => {
                    items.push(self.sequence());
                }
            }
        }

        Fragment::Block(block_type, items)
    }

    pub fn write_to(&mut self, writer: &mut impl Write) {
        let sequence = self.parse();

        let mut fwriter = FWriter {};
        fwriter.sequence(writer, sequence);
    }
}

pub fn run_formatter(tokens: Vec<Token>) -> Result<String> {
    let mut formatter = Formatter::new(tokens);
    let mut buffer = BufWriter::new(Vec::new());

    formatter.write_to(&mut buffer);

    Ok(String::from_utf8(buffer.into_inner()?)?)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{compiler::Compiler, util::path::Path};

    use super::*;

    #[test]
    fn test_formatter() {
        let cases = vec!["(a, b, c)"];
        for input in cases {
            let tokens = Compiler::run_lexer(input, Path::empty()).unwrap();
            let result = run_formatter(tokens).unwrap();

            assert_eq!(result, input.to_string());
        }
    }
}
