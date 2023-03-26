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

struct FWriter {
    column: usize,
}

impl FWriter {
    pub fn new() -> FWriter {
        FWriter { column: 0 }
    }

    fn write(&mut self, writer: &mut impl Write, raw: String) {
        write!(writer, "{}", raw).unwrap();
    }

    fn write_to(&mut self, writer: &mut impl Write, sequence: Sequence) {
        self.sequence(writer, sequence)
    }

    fn sequence(&mut self, writer: &mut impl Write, sequence: Sequence) {
        for fragment in sequence.0 {
            self.fragment(writer, fragment);
        }
    }

    fn fragment(&mut self, writer: &mut impl Write, fragment: Fragment) {
        match fragment {
            Fragment::Token(token) => {
                write!(writer, "{}", token.raw).unwrap();
            }
            Fragment::Block(block_type, items) => {
                self.block(writer, block_type, items);
            }
        }
    }

    fn block(&mut self, writer: &mut impl Write, block_type: BlockType, items: Vec<Sequence>) {
        match block_type {
            BlockType::Paren => {
                write!(writer, "(").unwrap();
            }
            BlockType::Brace => {
                write!(writer, "{{").unwrap();
            }
            BlockType::Bracket => {
                write!(writer, "[").unwrap();
            }
        }

        for (index, item) in items.into_iter().enumerate() {
            if index > 0 {
                write!(writer, " ").unwrap();
            }

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
    max_width: usize,
}

impl Formatter {
    pub fn new(input: Vec<Token>) -> Formatter {
        Formatter {
            input,
            position: 0,
            max_width: 110,
        }
    }

    fn peek(&self) -> Option<Token> {
        self.input.get(self.position).cloned()
    }

    fn consume(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Sequence {
        self.sequence(vec![], vec![])
    }

    pub fn sequence(&mut self, sep_tokens: Vec<Lexeme>, end_tokens: Vec<Lexeme>) -> Sequence {
        let mut fragments = Vec::new();

        while let Some(token) = self.peek() {
            match &token.lexeme {
                Lexeme::LParen => {
                    self.consume();

                    fragments.push(self.block(BlockType::Paren));
                }
                Lexeme::LBrace => {
                    self.consume();

                    fragments.push(self.block(BlockType::Brace));
                }
                Lexeme::LBracket => {
                    self.consume();

                    fragments.push(self.block(BlockType::Bracket));
                }
                lexeme if sep_tokens.contains(lexeme) => {
                    self.consume();
                    fragments.push(Fragment::Token(token));

                    break;
                }
                lexeme if end_tokens.contains(lexeme) => {
                    self.consume();

                    break;
                }
                _ => {
                    self.consume();

                    fragments.push(Fragment::Token(token.clone()));
                }
            }
        }

        Sequence(fragments)
    }

    fn block(&mut self, block_type: BlockType) -> Fragment {
        let mut items = Vec::new();

        while let Some(token) = self.peek() {
            match token.lexeme {
                Lexeme::RParen if matches!(block_type, BlockType::Paren) => {
                    self.consume();

                    break;
                }
                Lexeme::RBrace if matches!(block_type, BlockType::Brace) => {
                    self.consume();

                    break;
                }
                Lexeme::RBracket if matches!(block_type, BlockType::Bracket) => {
                    self.consume();

                    break;
                }
                _ => {
                    items.push(self.sequence(
                        match block_type {
                            BlockType::Paren => vec![Lexeme::Comma],
                            BlockType::Brace => {
                                vec![Lexeme::Comma, Lexeme::Semicolon]
                            }
                            BlockType::Bracket => vec![Lexeme::Comma],
                        },
                        match block_type {
                            BlockType::Paren => vec![Lexeme::RParen],
                            BlockType::Brace => {
                                vec![Lexeme::RBrace]
                            }
                            BlockType::Bracket => vec![Lexeme::RBracket],
                        },
                    ));
                }
            }
        }

        Fragment::Block(block_type, items)
    }

    pub fn write_to(&mut self, writer: &mut impl Write) {
        let sequence = self.parse();

        let mut fwriter = FWriter::new();
        fwriter.write_to(writer, sequence);
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
        let cases = vec![
            "(a, b, c)",
            "[a, b, c]",
            "{a, b, c}",
            "(sooooooooooooooooo, loooooooooooooooong, loooooooooooooooong, loooooooooooooooong, loooooooooooooooong, loooooooooooooooong, text)",
        ];
        for input in cases {
            let tokens = Compiler::run_lexer(input, Path::empty()).unwrap();
            let result = run_formatter(tokens).unwrap();

            assert_eq!(result, input.to_string());
        }
    }
}
