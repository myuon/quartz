use anyhow::{anyhow, Result};

use crate::{
    ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type},
    compiler::ErrorInSource,
    lexer::{Lexeme, Token},
};

pub struct Parser {
    position: usize,
    input: Vec<Token>,
    omit_index: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
            omit_index: 0,
        }
    }

    pub fn run(&mut self, input: Vec<Token>) -> Result<Module> {
        self.input = input;
        self.module()
    }

    pub fn module(&mut self) -> Result<Module> {
        let mut decls = vec![];

        while !self.is_end() {
            decls.push(self.decl()?);
        }

        Ok(Module(decls))
    }

    pub fn decl(&mut self) -> Result<Decl> {
        let consume = self.peek()?;
        match consume.lexeme {
            Lexeme::Fun => Ok(Decl::Func(self.func()?)),
            Lexeme::Let => {
                let (ident, type_, value) = self.let_()?;
                Ok(Decl::Let(ident, type_, value))
            }
            Lexeme::Type => {
                let (ident, type_) = self.type_decl()?;
                Ok(Decl::Type(ident, type_))
            }
            _ => Err(anyhow!("Unexpected token {:?}", self.peek()?.lexeme)),
        }
    }

    fn func(&mut self) -> Result<Func> {
        self.expect(Lexeme::Fun)?;
        let name = self.ident()?;
        self.expect(Lexeme::LParen)?;
        let params = self.params()?;
        self.expect(Lexeme::RParen)?;

        let mut result = self.gen_omit()?;
        if self.peek()?.lexeme == Lexeme::Colon {
            self.consume()?;
            result = self.type_()?;
        }

        self.expect(Lexeme::LBrace)?;
        let body = self.block()?;
        self.expect(Lexeme::RBrace)?;

        Ok(Func {
            name,
            params,
            result,
            body,
        })
    }

    fn type_decl(&mut self) -> Result<(Ident, Type)> {
        self.expect(Lexeme::Type)?;
        let ident = self.ident()?;
        self.expect(Lexeme::Equal)?;

        self.expect(Lexeme::LBrace)?;

        let mut record_type = vec![];
        while self.peek()?.lexeme != Lexeme::RBrace {
            let field = self.ident()?;
            self.expect(Lexeme::Colon)?;
            let type_ = self.type_()?;

            record_type.push((field, type_));

            if self.peek()?.lexeme == Lexeme::Comma {
                self.consume()?;
            } else {
                break;
            }
        }

        self.expect(Lexeme::RBrace)?;
        self.expect(Lexeme::Semicolon)?;

        Ok((ident, Type::Record(record_type)))
    }

    fn params(&mut self) -> Result<Vec<(Ident, Type)>> {
        let mut params = vec![];
        while self.peek()?.lexeme != Lexeme::RParen {
            let name = self.ident()?;
            self.expect(Lexeme::Colon)?;
            let type_ = self.type_()?;
            params.push((name, type_));
        }

        Ok(params)
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = vec![];
        while !self.is_end() && self.peek()?.lexeme != Lexeme::RBrace {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement> {
        match self.peek()?.lexeme {
            Lexeme::Let => {
                let (ident, type_, value) = self.let_()?;
                Ok(Statement::Let(ident, type_, value))
            }
            Lexeme::Return => {
                self.consume()?;
                let value = self.expr()?;
                self.expect(Lexeme::Semicolon)?;
                Ok(Statement::Return(value))
            }
            Lexeme::If => {
                self.consume()?;
                let condition = self.expr()?;
                self.expect(Lexeme::LBrace)?;
                let then_block = self.block()?;
                self.expect(Lexeme::RBrace)?;

                let else_block = if self.peek()?.lexeme == Lexeme::Else {
                    self.consume()?;
                    self.expect(Lexeme::LBrace)?;
                    let else_body = self.block()?;
                    self.expect(Lexeme::RBrace)?;

                    Some(else_body)
                } else {
                    None
                };

                Ok(Statement::If(
                    condition,
                    self.gen_omit()?,
                    then_block,
                    else_block,
                ))
            }
            Lexeme::While => {
                self.consume()?;
                let condition = self.expr()?;
                self.expect(Lexeme::LBrace)?;
                let body = self.block()?;
                self.expect(Lexeme::RBrace)?;

                Ok(Statement::While(condition, body))
            }
            _ => {
                let expr = self.expr()?;

                match self.peek()?.lexeme {
                    Lexeme::Semicolon => {
                        self.consume()?;
                        Ok(Statement::Expr(expr))
                    }
                    Lexeme::Equal => {
                        self.consume()?;
                        let value = self.expr()?;
                        self.expect(Lexeme::Semicolon)?;

                        Ok(Statement::Assign(Box::new(expr), Box::new(value)))
                    }
                    _ => Err(anyhow!("Unexpected token {:?}", self.peek()?.lexeme)),
                }
            }
        }
    }

    fn let_(&mut self) -> Result<(Ident, Type, Expr)> {
        self.expect(Lexeme::Let)?;
        let ident = self.ident()?;

        let mut type_ = self.gen_omit()?;
        if self.peek()?.lexeme == Lexeme::Colon {
            self.consume()?;
            type_ = self.type_()?;
        }

        self.expect(Lexeme::Equal)?;
        let value = self.expr()?;
        self.expect(Lexeme::Semicolon)?;

        Ok((ident, type_, value))
    }

    fn expr(&mut self) -> Result<Expr> {
        self.term_4()
    }

    fn term_4(&mut self) -> Result<Expr> {
        let mut current = self.term_3()?;

        match self.peek()?.lexeme {
            Lexeme::DoubleEqual => {
                self.consume()?;
                let rhs = self.expr()?;

                current = Expr::Call(
                    Box::new(Expr::Ident(Ident("equal".to_string()))),
                    vec![current, rhs],
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_3(&mut self) -> Result<Expr> {
        let mut current = self.term_2()?;

        match self.peek()?.lexeme {
            Lexeme::Lt => {
                self.consume()?;
                let rhs = self.expr()?;

                current = Expr::Call(
                    Box::new(Expr::Ident(Ident("lt".to_string()))),
                    vec![current, rhs],
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_2(&mut self) -> Result<Expr> {
        let mut current = self.term_1()?;

        match self.peek()?.lexeme {
            Lexeme::Plus => {
                self.consume()?;
                let rhs = self.expr()?;

                current = Expr::Call(
                    Box::new(Expr::Ident(Ident("add".to_string()))),
                    vec![current, rhs],
                );
            }
            Lexeme::Minus => {
                self.consume()?;
                let rhs = self.expr()?;

                current = Expr::Call(
                    Box::new(Expr::Ident(Ident("sub".to_string()))),
                    vec![current, rhs],
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_1(&mut self) -> Result<Expr> {
        let mut current = self.term_0()?;

        match self.peek()?.lexeme {
            Lexeme::Star => {
                self.consume()?;
                let rhs = self.term_1()?;

                current = Expr::Call(
                    Box::new(Expr::Ident(Ident("mult".to_string()))),
                    vec![current, rhs],
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_0(&mut self) -> Result<Expr> {
        match self.peek()?.lexeme {
            Lexeme::LParen => {
                self.consume()?;
                let expr = self.expr()?;
                self.expect(Lexeme::RParen)?;

                Ok(expr)
            }
            Lexeme::Ident(_) => {
                let ident = self.ident()?;
                let mut current = Expr::Ident(ident.clone());

                loop {
                    match self.peek()?.lexeme {
                        Lexeme::LParen => {
                            self.consume()?;
                            let mut args = vec![];
                            while self.peek()?.lexeme != Lexeme::RParen {
                                args.push(self.expr()?);

                                if self.peek()?.lexeme == Lexeme::Comma {
                                    self.consume()?;
                                } else {
                                    break;
                                }
                            }
                            self.consume()?;

                            current = Expr::Call(Box::new(current), args);
                        }
                        Lexeme::Dot => {
                            self.consume()?;

                            let field = self.ident()?;
                            current = Expr::Project(Box::new(current), self.gen_omit()?, field);
                        }
                        Lexeme::LBrace => {
                            self.consume()?;

                            let mut fields = vec![];
                            while self.peek()?.lexeme != Lexeme::RBrace {
                                let field = self.ident()?;
                                self.expect(Lexeme::Colon)?;
                                let value = self.expr()?;
                                fields.push((field, value));

                                if self.peek()?.lexeme == Lexeme::Comma {
                                    self.consume()?;
                                } else {
                                    break;
                                }
                            }
                            self.expect(Lexeme::RBrace)?;

                            current = Expr::Record(ident.clone(), fields);
                        }
                        _ => break,
                    }
                }

                Ok(current)
            }
            _ => self.lit().map(Expr::Lit),
        }
    }

    fn ident(&mut self) -> Result<Ident> {
        let current = self.consume()?;
        if let Lexeme::Ident(ident) = &current.lexeme {
            Ok(Ident(ident.clone()))
        } else {
            Err(
                anyhow!("Expected identifier, got {:?}", current.lexeme).context(ErrorInSource {
                    start: self.position,
                    end: current.position,
                }),
            )
        }
    }

    fn lit(&mut self) -> Result<Lit> {
        let current = self.consume()?;
        if let Lexeme::Int(int) = current.lexeme {
            Ok(Lit::I32(int))
        } else {
            Err(anyhow!("Expected literal, got {:?}", current.lexeme))
        }
    }

    fn type_(&mut self) -> Result<Type> {
        let current = self.consume()?;
        if current.lexeme == Lexeme::Ident("i32".to_string()) {
            Ok(Type::I32)
        } else if current.lexeme == Lexeme::LBrace {
            let mut fields = vec![];
            while self.peek()?.lexeme != Lexeme::RBrace {
                let ident = self.ident()?;
                self.expect(Lexeme::Colon)?;
                let type_ = self.type_()?;
                fields.push((ident, type_));

                if self.peek()?.lexeme == Lexeme::Comma {
                    self.consume()?;
                } else {
                    break;
                }
            }
            self.expect(Lexeme::RBrace)?;

            Ok(Type::Record(fields))
        } else {
            Err(anyhow!("Expected type, got {:?}", current.lexeme))
        }
    }

    fn consume(&mut self) -> Result<Token> {
        let token = self.peek()?;
        self.position += 1;

        Ok(token)
    }

    fn is_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> Result<Token> {
        self.input
            .get(self.position)
            .cloned()
            .ok_or(anyhow!("Unexpected end of input"))
    }

    fn expect(&mut self, lexeme: Lexeme) -> Result<Token> {
        let token = self.peek()?;
        if token.lexeme == lexeme {
            self.position += 1;
            Ok(token.clone())
        } else {
            return Err(
                anyhow!("Expected {:?}, got {:?}", lexeme, token.lexeme).context(ErrorInSource {
                    start: self.position,
                    end: token.position,
                }),
            );
        }
    }

    fn gen_omit(&mut self) -> Result<Type> {
        self.omit_index += 1;

        Ok(Type::Omit(self.omit_index))
    }
}
