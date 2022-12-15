use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    Module(Path),
    Fun,
    Let,
    Type,
    Return,
    If,
    Else,
    While,
    For,
    In,
    As,
    Self_,
    True,
    False,
    Continue,
    Import,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    DoubleColon,
    Colon,
    Semicolon,
    NotEqual,
    DoubleEqual,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Comma,
    DoubleDot,
    Dot,
    Lt,
    Gt,
    Underscore,
    Question,
    Bang,
    DoublePipe,
    Ident(String),
    Int(i32),
    String(String),
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static INT_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());
static STRING_LITERAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"((([^"]|\\")*[^\\])?)""#).unwrap());
static RAW_STRING_LITERAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^`((([^`]|\\`)*[^\\])?)`"#).unwrap());
static COMMENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^//[^\n]*\n"#).unwrap());

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub lexeme: Lexeme,
    pub position: usize,
}

fn is_term_boundary(s: &str) -> bool {
    let next = s.chars().next();
    if let Some(ch) = next {
        !ch.is_ascii_alphanumeric() && ch != '_'
    } else {
        true
    }
}

pub struct Lexer {
    current_path: Path,
    position: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            current_path: Path::new(vec![]),
            position: 0,
            tokens: vec![],
        }
    }

    fn matches(&mut self, input: &str, keyword: &str, lexeme: Lexeme) -> bool {
        if input[self.position..].starts_with(keyword) {
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

    fn matches_term(&mut self, input: &str, keyword: &str, lexeme: Lexeme) -> bool {
        if input[self.position..].starts_with(keyword)
            && is_term_boundary(&input[self.position + keyword.len()..])
        {
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

    fn matches_any_term(&mut self, input: &str, patterns: Vec<(&str, Lexeme)>) -> bool {
        for (keyword, lexeme) in patterns {
            if self.matches_term(input, keyword, lexeme) {
                return true;
            }
        }

        false
    }

    pub fn run(&mut self, input: &str, path: Path) -> Result<()> {
        self.current_path = path;

        while input.len() > self.position {
            match SPACE_PATTERN.find(&input[self.position..]) {
                Some(m) => {
                    self.position += m.end();

                    continue;
                }
                None => (),
            }

            match COMMENT_PATTERN.find(&input[self.position..]) {
                Some(m) => {
                    self.position += m.end();

                    continue;
                }
                None => (),
            }

            if self.matches_any_term(
                input,
                vec![
                    ("module", Lexeme::Module(self.current_path.clone())),
                    ("fun", Lexeme::Fun),
                    ("let", Lexeme::Let),
                    ("type", Lexeme::Type),
                    ("return", Lexeme::Return),
                    ("if", Lexeme::If),
                    ("else", Lexeme::Else),
                    ("while", Lexeme::While),
                    ("for", Lexeme::For),
                    ("in", Lexeme::In),
                    ("as", Lexeme::As),
                    ("self", Lexeme::Self_),
                    ("true", Lexeme::True),
                    ("false", Lexeme::False),
                    ("continue", Lexeme::Continue),
                    ("import", Lexeme::Import),
                ],
            ) {
                continue;
            }

            if self.matches_any(
                input,
                vec![
                    ("(", Lexeme::LParen),
                    (")", Lexeme::RParen),
                    ("{", Lexeme::LBrace),
                    ("}", Lexeme::RBrace),
                    ("[", Lexeme::LBracket),
                    ("]", Lexeme::RBracket),
                    ("::", Lexeme::DoubleColon),
                    (":", Lexeme::Colon),
                    (";", Lexeme::Semicolon),
                    ("!=", Lexeme::NotEqual),
                    ("==", Lexeme::DoubleEqual),
                    ("=", Lexeme::Equal),
                    ("+", Lexeme::Plus),
                    ("-", Lexeme::Minus),
                    ("*", Lexeme::Star),
                    ("/", Lexeme::Slash),
                    ("%", Lexeme::Percent),
                    (",", Lexeme::Comma),
                    ("..", Lexeme::DoubleDot),
                    (".", Lexeme::Dot),
                    ("<", Lexeme::Lt),
                    (">", Lexeme::Gt),
                    ("_", Lexeme::Underscore),
                    ("?", Lexeme::Question),
                    ("!", Lexeme::Bang),
                    ("||", Lexeme::DoublePipe),
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
                        lexeme: Lexeme::Int(m.as_str().parse::<i32>().unwrap()),
                        position: self.position,
                    });

                    self.position += m.end();
                    continue;
                }
                None => (),
            }

            match STRING_LITERAL.captures(&input[self.position..]) {
                Some(m) => {
                    self.tokens.push(Token {
                        lexeme: Lexeme::String(m.get(1).unwrap().as_str().to_string()),
                        position: self.position,
                    });

                    self.position += m.get(0).unwrap().end();
                    continue;
                }
                None => (),
            }
            match RAW_STRING_LITERAL.captures(&input[self.position..]) {
                Some(m) => {
                    self.tokens.push(Token {
                        lexeme: Lexeme::String(m.get(1).unwrap().as_str().to_string()),
                        position: self.position,
                    });

                    self.position += m.get(0).unwrap().end();
                    continue;
                }
                None => (),
            }

            panic!("{}", &input[self.position..]);
        }

        Ok(())
    }
}
