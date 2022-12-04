use anyhow::{anyhow, Result};

use crate::{
    ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type},
    compiler::ErrorInSource,
    lexer::{Lexeme, Token},
};

pub struct Parser {
    position: usize,
    input: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
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
            _ => Err(anyhow!("Unexpected token {:?}", self.peek()?.lexeme)),
        }
    }

    fn func(&mut self) -> Result<Func> {
        self.expect(Lexeme::Fun)?;
        let name = self.ident()?;
        self.expect(Lexeme::LParen)?;
        self.expect(Lexeme::RParen)?;
        self.expect(Lexeme::Colon)?;
        let result = self.type_()?;

        self.expect(Lexeme::LBrace)?;
        let body = self.block()?;
        self.expect(Lexeme::RBrace)?;

        Ok(Func {
            name,
            params: vec![],
            result,
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = vec![];
        while !self.is_end() && self.peek()?.lexeme != Lexeme::RBrace {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement> {
        let current = self.consume()?;
        if current.lexeme == Lexeme::Let {
            let ident = self.ident()?;
            self.expect(Lexeme::Colon)?;
            let type_ = self.type_()?;
            self.expect(Lexeme::Equal)?;
            let value = self.expr()?;
            self.expect(Lexeme::Semicolon)?;
            Ok(Statement::Let(ident, type_, value))
        } else if current.lexeme == Lexeme::Return {
            let value = self.expr()?;
            self.expect(Lexeme::Semicolon)?;
            Ok(Statement::Return(value))
        } else {
            Err(anyhow!("Unexpected token {:?}", current.lexeme))
        }
    }

    fn expr(&mut self) -> Result<Expr> {
        let mut current = self.term()?;

        if let Lexeme::Plus = self.peek()?.lexeme {
            self.consume()?;
            let rhs = self.expr()?;

            current = Expr::Call(
                Box::new(Expr::Ident(Ident("add".to_string()))),
                vec![current, rhs],
            );
        }

        Ok(current)
    }

    fn term(&mut self) -> Result<Expr> {
        let current = self.peek()?;
        if let Lexeme::LParen = current.lexeme {
            self.consume()?;
            let expr = self.expr()?;
            self.expect(Lexeme::RParen)?;
            Ok(expr)
        } else if let Lexeme::Ident(_) = current.lexeme {
            self.ident().map(Expr::Ident)
        } else {
            self.lit().map(Expr::Lit)
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
}
