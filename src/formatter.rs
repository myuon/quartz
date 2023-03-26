use std::io::{BufWriter, Write};

use anyhow::Result;

use crate::lexer::{Lexeme, Token};

#[derive(Clone)]
enum BlockType {
    Paren,   // ()
    Brace,   // {}
    Bracket, // []
}

#[derive(Clone)]
enum Fragment {
    Token(Token),
    Block(BlockType, String, Vec<Sequence>),
}

#[derive(Clone)]
struct Sequence(Vec<Fragment>, Option<Token>);

struct FWriter {
    max_width: usize,
    indent_size: usize,
    column: usize,
    context: Vec<String>,
}

impl FWriter {
    pub fn new(max_width: usize, indent_size: usize) -> FWriter {
        FWriter {
            column: 0,
            max_width,
            indent_size,
            context: vec![],
        }
    }

    fn is_module_context(&self) -> bool {
        self.context.last() == Some(&"module".to_string())
    }

    fn is_function_context(&self) -> bool {
        self.context.last() == Some(&"fun".to_string())
    }

    fn write(&mut self, writer: &mut impl Write, raw: String) {
        write!(writer, "{}", raw).unwrap();
    }

    fn write_to(&mut self, writer: &mut impl Write, sequence: Sequence) {
        self.sequence(writer, sequence)
    }

    fn sequence(&mut self, writer: &mut impl Write, sequence: Sequence) {
        let mut first = true;
        for fragment in sequence.0 {
            if !first {
                write!(writer, " ").unwrap();
            }

            self.fragment(writer, fragment);
            first = false;
        }
    }

    fn fragment(&mut self, writer: &mut impl Write, fragment: Fragment) {
        match fragment {
            Fragment::Token(token) => {
                write!(writer, "{}", token.raw).unwrap();

                if matches!(token.lexeme, Lexeme::Module(_)) {
                    self.context.push("module".to_string());
                }
                if matches!(token.lexeme, Lexeme::Fun) {
                    self.context.push("fun".to_string());
                }
                if matches!(token.lexeme, Lexeme::RBrace) {
                    self.context.pop();
                }
            }
            Fragment::Block(block_type, token, items) => {
                if !self.is_function_context() {
                    self.block_in_block(writer, block_type.clone(), token.clone(), items.clone());
                    return;
                }

                let mut new_writer = BufWriter::new(Vec::new());
                self.block_in_line(
                    &mut new_writer,
                    block_type.clone(),
                    token.clone(),
                    items.clone(),
                );
                let new_writer_string =
                    String::from_utf8(new_writer.into_inner().unwrap()).unwrap();

                if self.column + new_writer_string.len() > self.max_width {
                    self.block_in_block(writer, block_type.clone(), token.clone(), items.clone());
                } else {
                    write!(writer, "{}", new_writer_string).unwrap();
                }
            }
        }
    }

    fn block_in_line(
        &mut self,
        writer: &mut impl Write,
        block_type: BlockType,
        separator: String,
        items: Vec<Sequence>,
    ) {
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
                write!(writer, "{} ", separator).unwrap();
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

    fn block_in_block(
        &mut self,
        writer: &mut impl Write,
        block_type: BlockType,
        separator: String,
        items: Vec<Sequence>,
    ) {
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

        self.column += 1;

        for item in items {
            write!(writer, "\n").unwrap();
            write!(writer, "{}", " ".repeat(self.indent_size)).unwrap();
            self.sequence(writer, item);
            write!(writer, "{}", separator).unwrap();
        }

        write!(writer, "\n").unwrap();

        self.column -= 1;

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
    indent_size: usize,
}

impl Formatter {
    pub fn new(input: Vec<Token>) -> Formatter {
        Formatter {
            input,
            position: 0,
            max_width: 110,
            indent_size: 4,
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
        let mut last_token = None;

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
                    last_token = Some(token.clone());

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

        Sequence(fragments, last_token)
    }

    fn block(&mut self, block_type: BlockType) -> Fragment {
        let mut items = Vec::new();
        let mut sep_token: Option<Token> = None;

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
                    let item = self.sequence(
                        match block_type {
                            BlockType::Paren => vec![Lexeme::Comma],
                            BlockType::Brace => {
                                if let Some(token) = sep_token {
                                    vec![token.lexeme]
                                } else {
                                    vec![Lexeme::Comma, Lexeme::Semicolon]
                                }
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
                    );
                    sep_token = item.1.clone();

                    items.push(item);
                }
            }
        }

        Fragment::Block(
            block_type,
            sep_token.map(|t| t.raw).unwrap_or(",".to_string()),
            items,
        )
    }

    pub fn write_to(&mut self, writer: &mut impl Write) {
        let sequence = self.parse();

        let mut fwriter = FWriter::new(self.max_width, self.indent_size);
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
    fn test_formatter_success() {
        let cases = vec![
            r#"fun main() {
return (a, b, c);
}"#,
            "[a, b, c]",
            "{a, b, c}",
            r#"
(
    sooooooooooooooooo,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    text,
)"#
            .trim_start(),
            r#"
module main {
    fun main() {
        let a = 1;
        let b = 2;
        let c = 3;

        return Ok(a + b + c);
    }
}
"#
            .trim_start(),
        ];
        for input in cases {
            let tokens = Compiler::run_lexer(input, Path::empty()).unwrap();
            let result = run_formatter(tokens).unwrap();

            assert_eq!(result, input.to_string());
        }
    }

    #[test]
    fn test_formatter_forced() {
        let cases = vec![(
            r#"
(
    sooooooooooooooooo,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    text
)"#
            .trim_start(),
            r#"
(
    sooooooooooooooooo,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    loooooooooooooooong,
    text,
)"#
            .trim_start(),
        )];
        for (input, expected) in cases {
            let tokens = Compiler::run_lexer(input, Path::empty()).unwrap();
            let result = run_formatter(tokens).unwrap();

            assert_eq!(result, expected.to_string());
        }
    }
}
