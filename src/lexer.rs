use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum Lexeme {
    Func,
    Return,
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
    Ident(String),
    IntLiteral(i32),
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static INT_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());

#[derive(PartialEq, Debug)]
pub struct Token {
    pub lexeme: Lexeme,
    pub position: usize,
}

struct TokenReader {
    position: usize,
    tokens: Vec<Token>,
}

impl TokenReader {
    fn new() -> TokenReader {
        TokenReader {
            position: 0,
            tokens: vec![],
        }
    }

    fn matches(&mut self, input: &str, keyword: &str, lexeme: Lexeme) -> bool {
        if &input[self.position..(self.position + keyword.len()).min(input.len())] == keyword {
            self.tokens.push(Token {
                lexeme,
                position: self.position,
            });
            self.position += keyword.len();

            true
        } else {
            false
        }
    }

    fn matches_any(&mut self, input: &str, patterns: Vec<(&str, Lexeme)>) -> bool {
        for (keyword, lexeme) in patterns {
            if self.matches(input, keyword, lexeme) {
                return true;
            }
        }

        false
    }

    pub fn run_lexer(&mut self, input: &str) {
        while input.len() > self.position {
            match SPACE_PATTERN.find(&input[self.position..]) {
                Some(m) => {
                    self.position += m.end();

                    continue;
                }
                None => (),
            }

            if self.matches_any(
                input,
                vec![
                    ("func", Lexeme::Func),
                    ("return", Lexeme::Return),
                    ("(", Lexeme::LParen),
                    (")", Lexeme::RParen),
                    ("{", Lexeme::LBrace),
                    ("}", Lexeme::RBrace),
                    (";", Lexeme::SemiColon),
                ],
            ) {
                continue;
            }

            match IDENT_PATTERN.find(&input[self.position..]) {
                Some(m) => {
                    self.tokens.push(Token {
                        lexeme: Lexeme::Ident(m.as_str().to_string()),
                        position: self.position,
                    });

                    self.position += m.end();
                    continue;
                }
                None => (),
            }

            match INT_LITERAL.find(&input[self.position..]) {
                Some(m) => {
                    self.tokens.push(Token {
                        lexeme: Lexeme::IntLiteral(m.as_str().parse::<i32>().unwrap()),
                        position: self.position,
                    });

                    self.position += m.end();
                    continue;
                }
                None => (),
            }

            panic!("{}", &input[self.position..]);
        }
    }
}

pub fn run_lexer(input: &str) -> Vec<Token> {
    let mut reader = TokenReader::new();
    reader.run_lexer(input);

    reader.tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_lexer() {
        use Lexeme::*;
        let cases = vec![(
            r#"func main() {
                    return 10;
                }"#,
            vec![
                Func,
                Ident("main".to_string()),
                LParen,
                RParen,
                LBrace,
                Return,
                IntLiteral(10),
                SemiColon,
                RBrace,
            ],
        )];

        for c in cases {
            assert_eq!(
                run_lexer(c.0)
                    .into_iter()
                    .map(|t| t.lexeme)
                    .collect::<Vec<_>>(),
                c.1,
                "{:?}",
                c.0
            );
        }
    }
}
