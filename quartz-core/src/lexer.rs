use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    Nil,
    True,
    False,
    Func,
    Let,
    Return,
    If,
    Else,
    Loop,
    While,
    Continue,
    Struct,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    SemiColon,
    Colon,
    Comma,
    Dot,
    DoubleEqual,
    NotEqual,
    Equal,
    And,
    Star,
    Lt,
    LEq,
    Gt,
    GEq,
    Plus,
    Ident(String),
    Int(i32),
    String(String),
}

static SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+").unwrap());
static IDENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
static INT_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+").unwrap());
static STRING_LITERAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"((([^"]|\\")*[^\\])?)""#).unwrap());
static COMMENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^//[^\n]*\n"#).unwrap());

#[derive(PartialEq, Debug, Clone)]
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

            match COMMENT_PATTERN.find(&input[self.position..]) {
                Some(m) => {
                    self.position += m.end();

                    continue;
                }
                None => (),
            }

            if self.matches_any(
                input,
                vec![
                    ("nil", Lexeme::Nil),
                    ("true", Lexeme::True),
                    ("false", Lexeme::False),
                    ("func", Lexeme::Func),
                    ("let", Lexeme::Let),
                    ("return", Lexeme::Return),
                    ("if", Lexeme::If),
                    ("else", Lexeme::Else),
                    ("loop", Lexeme::Loop),
                    ("while", Lexeme::While),
                    ("continue", Lexeme::Continue),
                    ("struct", Lexeme::Struct),
                    ("(", Lexeme::LParen),
                    (")", Lexeme::RParen),
                    ("{", Lexeme::LBrace),
                    ("}", Lexeme::RBrace),
                    ("[", Lexeme::LBracket),
                    ("]", Lexeme::RBracket),
                    (";", Lexeme::SemiColon),
                    (":", Lexeme::Colon),
                    (",", Lexeme::Comma),
                    (".", Lexeme::Dot),
                    ("==", Lexeme::DoubleEqual),
                    ("!=", Lexeme::NotEqual),
                    ("=", Lexeme::Equal),
                    ("&", Lexeme::And),
                    ("*", Lexeme::Star),
                    ("<", Lexeme::Lt),
                    ("<=", Lexeme::LEq),
                    (">", Lexeme::Gt),
                    (">=", Lexeme::GEq),
                    ("+", Lexeme::Plus),
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_run_lexer() {
        use Lexeme::*;
        let cases = vec![
            (
                r#"
                    // this is a comment
                    let main = func () {
                    f(10, 20, 40);
                    100;
                    "foo"; // and comment
                    let u = func () { 20 };
            };
            main();"#,
                vec![
                    Let,
                    Ident("main".to_string()),
                    Equal,
                    Func,
                    LParen,
                    RParen,
                    LBrace,
                    Ident("f".to_string()),
                    LParen,
                    Int(10),
                    Comma,
                    Int(20),
                    Comma,
                    Int(40),
                    RParen,
                    SemiColon,
                    Int(100),
                    SemiColon,
                    String("foo".to_string()),
                    SemiColon,
                    Let,
                    Ident("u".to_string()),
                    Equal,
                    Func,
                    LParen,
                    RParen,
                    LBrace,
                    Int(20),
                    RBrace,
                    SemiColon,
                    RBrace,
                    SemiColon,
                    Ident("main".to_string()),
                    LParen,
                    RParen,
                    SemiColon,
                ],
            ),
            (
                r#"f("日本語")"#,
                vec![
                    Ident("f".to_string()),
                    LParen,
                    String("日本語".to_string()),
                    RParen,
                ],
            ),
            (
                r#"func () { return 10; }"#,
                vec![
                    Func,
                    LParen,
                    RParen,
                    LBrace,
                    Return,
                    Int(10),
                    SemiColon,
                    RBrace,
                ],
            ),
            (
                r#"&10; *v"#,
                vec![And, Int(10), SemiColon, Star, Ident("v".to_string())],
            ),
            (
                // empty string
                r#"return "";"#,
                vec![Return, String("".to_string()), SemiColon],
            ),
            (
                // escaped double quote
                r#"return "ab\"c";"#,
                vec![Return, String(r#"ab\"c"#.to_string()), SemiColon],
            ),
            (
                // escaped escape
                r#"return "ab\\c";"#,
                vec![Return, String(r#"ab\\c"#.to_string()), SemiColon],
            ),
        ];

        for c in cases {
            assert_eq!(
                run_lexer(c.0)
                    .into_iter()
                    .map(|t| t.lexeme)
                    .collect::<Vec<_>>(),
                c.1,
                "{}",
                c.0
            );
        }
    }
}
