use anyhow::{anyhow, bail, Context, Result};
use pretty_assertions::assert_eq;

use crate::{
    ast::{BinOp, Decl, Expr, Func, Lit, Module, Statement, Type},
    compiler::ErrorInSource,
    lexer::{Lexeme, Token},
    util::{ident::Ident, path::Path, source::Source},
};

pub struct Parser {
    position: usize,
    input: Vec<Token>,
    omit_index: usize,
    pub imports: Vec<Path>,
    current_path: Path,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
            omit_index: 0,
            imports: vec![],
            current_path: Path::empty(),
        }
    }

    pub fn run(&mut self, input: Vec<Token>, path: Path) -> Result<Module> {
        self.input = input;
        self.current_path = path;
        self.module()
    }

    pub fn module(&mut self) -> Result<Module> {
        let mut decls = vec![];

        while !self.is_end() && self.peek()?.lexeme != Lexeme::RBrace {
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
            Lexeme::Module(path) => {
                self.consume()?;
                let ident = self.ident()?;
                self.expect(Lexeme::LBrace)?;
                let module = self.module()?;
                self.expect(Lexeme::RBrace)?;

                Ok(Decl::Module(Path::ident(ident), module))
            }
            Lexeme::Import => {
                self.expect(Lexeme::Import)?;
                let path = self.path()?;
                self.expect(Lexeme::Semicolon)?;

                self.imports.push(path.clone());

                Ok(Decl::Import(path))
            }
            _ => Err(anyhow!("Unexpected token {:?}", self.peek()?.lexeme)),
        }
    }

    fn path(&mut self) -> Result<Path> {
        let mut path = vec![self.ident()?];
        loop {
            if self.peek()?.lexeme == Lexeme::DoubleColon {
                self.consume()?;
            } else {
                break;
            }

            path.push(self.ident()?);
        }

        Ok(Path(path))
    }

    fn func(&mut self) -> Result<Func> {
        self.expect(Lexeme::Fun)?;
        let name = self.ident()?;
        self.expect(Lexeme::LParen)?;
        let (params, variadic) = self.params()?;
        self.expect(Lexeme::RParen)?;

        let mut result = Type::Nil;
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
            variadic,
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

    fn params(&mut self) -> Result<(Vec<(Ident, Type)>, Option<(Ident, Type)>)> {
        let mut variadic = None;
        let mut params = vec![];
        while self.peek()?.lexeme != Lexeme::RParen {
            if self.peek()?.lexeme == Lexeme::Self_ {
                self.consume()?;
                params.push((Ident("self".to_string()), self.gen_omit()?));
            } else if self.peek()?.lexeme == Lexeme::DoubleDot {
                self.consume()?;
                let name = self.ident()?;
                self.expect(Lexeme::Colon)?;
                let type_ = self.type_()?;
                variadic = Some((name, type_));
            } else {
                let name = self.ident()?;
                self.expect(Lexeme::Colon)?;
                let type_ = self.type_()?;
                params.push((name, type_));
            }

            if self.peek()?.lexeme == Lexeme::RParen {
                break;
            }

            self.expect(Lexeme::Comma)?;
        }

        Ok((params, variadic))
    }

    fn block(&mut self) -> Result<Vec<Source<Statement>>> {
        let mut statements = vec![];
        while !self.is_end() && self.peek()?.lexeme != Lexeme::RBrace {
            let position = self.position;
            let statement = self.statement()?;
            statements.push(self.source_from(statement, position));
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

                if self.peek()?.lexeme == Lexeme::Semicolon {
                    self.consume()?;

                    Ok(Statement::Return(Source::unknown(Expr::Lit(Lit::Nil))))
                } else {
                    let value = self.expr()?;
                    self.expect(Lexeme::Semicolon)?;

                    Ok(Statement::Return(value))
                }
            }
            Lexeme::If => {
                self.consume()?;
                let condition = self.expr_conditional()?;
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
                let condition = self.expr_conditional()?;
                self.expect(Lexeme::LBrace)?;
                let body = self.block()?;
                self.expect(Lexeme::RBrace)?;

                Ok(Statement::While(condition, body))
            }
            Lexeme::For => {
                self.consume()?;
                let ident = self.ident()?;
                self.expect(Lexeme::In)?;
                let range = self.expr_conditional()?;
                self.expect(Lexeme::LBrace)?;
                let body = self.block()?;
                self.expect(Lexeme::RBrace)?;

                Ok(Statement::For(ident, range, body))
            }
            Lexeme::Continue => {
                self.consume()?;
                self.expect(Lexeme::Semicolon)?;
                Ok(Statement::Continue)
            }
            Lexeme::Break => {
                self.consume()?;
                self.expect(Lexeme::Semicolon)?;
                Ok(Statement::Break)
            }
            _ => {
                let expr = self.expr()?;

                match self.peek()?.lexeme {
                    Lexeme::Semicolon => {
                        self.consume()?;
                        Ok(Statement::Expr(expr, Type::Omit(0)))
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

    fn let_(&mut self) -> Result<(Ident, Type, Source<Expr>)> {
        self.expect(Lexeme::Let)?;
        let ident = self.ident()?;

        let mut type_ = self.gen_omit()?;
        if self.peek()?.lexeme == Lexeme::Colon {
            self.consume()?;
            type_ = self.type_()?;
        }

        self.expect(Lexeme::Equal)?;
        let value = self.expr()?;
        self.expect(Lexeme::Semicolon).context("let:end")?;

        Ok((ident, type_, value))
    }

    fn expr_conditional(&mut self) -> Result<Source<Expr>> {
        self.expr_(false)
    }

    fn expr(&mut self) -> Result<Source<Expr>> {
        self.expr_(true)
    }

    fn expr_(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        self.term_5(with_struct)
    }

    fn term_5(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_4(with_struct)?;

        loop {
            let token = self.peek()?;
            match token.lexeme {
                Lexeme::DoublePipe => {
                    self.consume()?;
                    let rhs = self.term_4(with_struct)?;

                    current = self.source_from(
                        Expr::BinOp(BinOp::Or, Type::Bool, Box::new(current), Box::new(rhs)),
                        position,
                    );
                }
                Lexeme::DoubleAmp => {
                    self.consume()?;
                    let rhs = self.term_4(with_struct)?;

                    current = self.source_from(
                        Expr::BinOp(BinOp::And, Type::Bool, Box::new(current), Box::new(rhs)),
                        position,
                    );
                }
                _ => break,
            }
        }

        Ok(current)
    }

    fn term_4(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_3(with_struct)?;

        let token = self.peek()?;
        match token.lexeme {
            Lexeme::DoubleEqual => {
                self.consume()?;
                let rhs = self.term_3(with_struct)?;

                current = self.source_from(Expr::Equal(Box::new(current), Box::new(rhs)), position);
            }
            Lexeme::NotEqual => {
                self.consume()?;
                let rhs = self.term_3(with_struct)?;

                current =
                    self.source_from(Expr::NotEqual(Box::new(current), Box::new(rhs)), position);
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_3(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_2(with_struct)?;

        let token_position = self.position;
        let token = self.peek()?;
        match token.lexeme {
            Lexeme::Lt => {
                self.consume()?;
                let token_position_end = self.position;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("lt".to_string())),
                            token_position,
                            token_position_end,
                        )),
                        vec![current, rhs],
                        None,
                    ),
                    position,
                );
            }
            Lexeme::Gt => {
                self.consume()?;
                let token_position_end = self.position;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("gt".to_string())),
                            token_position,
                            token_position_end,
                        )),
                        vec![current, rhs],
                        None,
                    ),
                    position,
                );
            }
            Lexeme::Gte => {
                self.consume()?;
                let token_position_end = self.position;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("gte".to_string())),
                            token_position,
                            token_position_end,
                        )),
                        vec![current, rhs],
                        None,
                    ),
                    position,
                );
            }
            Lexeme::Lte => {
                self.consume()?;
                let token_position_end = self.position;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("lte".to_string())),
                            token_position,
                            token_position_end,
                        )),
                        vec![current, rhs],
                        None,
                    ),
                    position,
                );
            }
            Lexeme::DoubleDot => {
                self.consume()?;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(Expr::Range(Box::new(current), Box::new(rhs)), position);
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_2(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_1(with_struct)?;

        let token_position = self.position;
        loop {
            match self.peek()?.lexeme {
                Lexeme::Plus => {
                    self.consume()?;
                    let token_position_end = self.position;
                    let rhs = self.term_1(with_struct)?;

                    current = self.source_from(
                        Expr::Call(
                            Box::new(self.source(
                                Expr::ident(Ident("add".to_string())),
                                token_position,
                                token_position_end,
                            )),
                            vec![current, rhs],
                            None,
                        ),
                        position,
                    );
                }
                Lexeme::Minus => {
                    self.consume()?;
                    let token_position_end = self.position;
                    let rhs = self.term_1(with_struct)?;

                    current = self.source_from(
                        Expr::Call(
                            Box::new(self.source(
                                Expr::ident(Ident("sub".to_string())),
                                token_position,
                                token_position_end,
                            )),
                            vec![current, rhs],
                            None,
                        ),
                        position,
                    );
                }
                Lexeme::As => {
                    self.consume()?;
                    let type_ = self.type_()?;

                    current = self.source_from(Expr::As(Box::new(current), type_), position);
                }
                _ => break,
            }
        }

        Ok(current)
    }

    fn term_1(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_0(with_struct)?;

        let token_position = self.position;
        let token = self.peek()?;
        match token.lexeme {
            Lexeme::Star => {
                self.consume()?;
                let rhs = self.term_1(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Mul, Type::Omit(0), Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            Lexeme::Slash => {
                self.consume()?;
                let token_position_end = self.position;
                let rhs = self.term_1(with_struct)?;

                current = self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("div".to_string())),
                            token_position,
                            token_position_end,
                        )),
                        vec![current, rhs],
                        None,
                    ),
                    position,
                );
            }
            Lexeme::Percent => {
                self.consume()?;
                let rhs = self.term_1(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Mod, Type::Omit(0), Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_0(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = match self.peek()?.lexeme {
            Lexeme::LParen => {
                self.consume()?;
                let expr = self.expr()?;
                self.expect(Lexeme::RParen)?;

                expr
            }
            Lexeme::Ident(ident) => {
                let expr = if ident == "nil" {
                    self.consume()?;

                    Expr::Lit(Lit::Nil)
                } else {
                    let ident = self.ident()?;

                    Expr::ident(ident.clone())
                };

                self.source_from(expr, position)
            }
            Lexeme::Self_ => {
                let position = self.position;
                self.consume()?;

                self.source_from(Expr::Self_, position)
            }
            Lexeme::Bang => {
                self.consume()?;
                let expr = self.term_0(with_struct)?;

                self.source_from(
                    Expr::Call(
                        Box::new(self.source(
                            Expr::ident(Ident("not".to_string())),
                            position,
                            position,
                        )),
                        vec![expr],
                        None,
                    ),
                    position,
                )
            }
            Lexeme::Struct => {
                self.consume()?;
                self.expect(Lexeme::LBrace)?;

                let mut fields = vec![];
                while self.peek()?.lexeme != Lexeme::RBrace {
                    let ident = self.ident()?;
                    self.expect(Lexeme::Colon)?;
                    let expr = self.expr()?;

                    fields.push((ident, expr));

                    if self.peek()?.lexeme == Lexeme::Comma {
                        self.consume()?;
                    } else {
                        break;
                    }
                }
                self.expect(Lexeme::RBrace)?;

                self.source_from(Expr::AnonymousRecord(fields, Type::Omit(0)), position)
            }
            _ => {
                let lit = self.lit()?;

                Source::unknown(Expr::Lit(lit))
            }
        };

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

                    current = self.source_from(Expr::Call(Box::new(current), args, None), position);
                }
                Lexeme::Dot => {
                    self.consume()?;

                    let field = self.ident()?;
                    let omit = self.gen_omit()?;
                    current = self.source_from(
                        Expr::Project(Box::new(current), omit, Path::ident(field)),
                        position,
                    );
                }
                Lexeme::LBrace if with_struct => {
                    self.consume()?;

                    let ident = match current.data {
                        Expr::Ident { ident, .. } => ident,
                        _ => {
                            return Err(anyhow!(
                                "Expected identifier for record name, found {:?}",
                                current.data
                            )
                            .context(ErrorInSource {
                                path: None,
                                start: current.start.unwrap_or(0),
                                end: current.end.unwrap_or(0),
                            }));
                        }
                    };

                    let mut fields = vec![];
                    let mut expansion = None;
                    while self.peek()?.lexeme != Lexeme::RBrace {
                        let field = self.ident()?;
                        self.expect(Lexeme::Colon)
                            .context("record:initialization")?;
                        let value = self.expr()?;
                        fields.push((field, value));

                        if self.peek()?.lexeme == Lexeme::Comma {
                            self.consume()?;

                            if self.peek()?.lexeme == Lexeme::DoubleDot {
                                self.consume()?;
                                expansion = Some(Box::new(self.expr()?));

                                if self.peek()?.lexeme == Lexeme::Comma {
                                    self.consume()?;
                                }

                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.expect(Lexeme::RBrace)?;

                    current = self.source_from(
                        Expr::Record(
                            Source::new(
                                ident,
                                current.start.unwrap_or(0),
                                current.end.unwrap_or(0),
                            ),
                            fields,
                            expansion,
                        ),
                        position,
                    );
                }
                Lexeme::LBracket => {
                    self.consume()?;

                    let ident = match current.data {
                        Expr::Ident { ident, .. } => ident,
                        _ => {
                            return Err(anyhow!(
                                "Expected identifier for record name, found {:?}",
                                current.data
                            )
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: current.start.unwrap_or(0),
                                end: current.end.unwrap_or(0),
                            }));
                        }
                    };

                    let type_ = self.type_()?;
                    self.expect(Lexeme::RBracket)?;

                    self.expect(Lexeme::LParen)?;
                    let mut args = vec![];
                    while self.peek()?.lexeme != Lexeme::RParen {
                        args.push(self.expr()?);

                        if self.peek()?.lexeme == Lexeme::Comma {
                            self.consume()?;
                        } else {
                            break;
                        }
                    }
                    self.expect(Lexeme::RParen)?;

                    match ident.as_str() {
                        "make" => {
                            current = self.source_from(Expr::Make(type_, args), position);
                        }
                        "sizeof" => {
                            assert_eq!(args.len(), 0);

                            current = self.source_from(Expr::SizeOf(type_), position);
                        }
                        _ => bail!("Unknown type constructor: {}", ident.as_str()),
                    }
                }
                Lexeme::DoubleColon => {
                    self.consume()?;

                    let ident = match current.data {
                        Expr::Ident { ident, .. } => ident,
                        _ => {
                            bail!(
                                "Expected identifier for record name, found {:?}",
                                current.data
                            )
                        }
                    };

                    let ident_position = self.position;

                    let name = self.ident()?;
                    current = self.source_from(
                        Expr::Path {
                            path: self.source_from(Path::new(vec![ident, name]), ident_position),
                            resolved_path: None,
                        },
                        position,
                    );
                }
                Lexeme::Question => {
                    self.consume()?;

                    current = self.source_from(Expr::Wrap(Box::new(current)), position);
                }
                Lexeme::Bang => {
                    self.consume()?;

                    current = self.source_from(Expr::Unwrap(Box::new(current)), position);
                }
                _ => break,
            }
        }

        Ok(current)
    }

    fn ident(&mut self) -> Result<Ident> {
        let current = self.consume()?;
        if let Lexeme::Ident(ident) = &current.lexeme {
            Ok(Ident(ident.clone()))
        } else {
            Err(
                anyhow!("Expected identifier, got {:?}", current.lexeme).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: self.input[self.position].position,
                    end: self.input[self.position + 1].position,
                }),
            )
        }
    }

    fn lit(&mut self) -> Result<Lit> {
        let current = self.consume()?;
        match current.lexeme {
            Lexeme::Int(int) if int <= i32::MAX as i64 => Ok(Lit::I32(int as i32)),
            Lexeme::Int(int) => Ok(Lit::I64(int)),
            Lexeme::String(string) => Ok(Lit::String(string)),
            Lexeme::True => Ok(Lit::Bool(true)),
            Lexeme::False => Ok(Lit::Bool(false)),
            _ => {
                return Err(
                    anyhow!("Expected literal, got {:?}", current.lexeme).context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: self.input[self.position].position,
                        end: self.input[self.position].position,
                    }),
                )
            }
        }
    }

    fn type_(&mut self) -> Result<Type> {
        let current = self.consume()?;
        let mut type_ = match current.lexeme {
            Lexeme::Ident(ident) => match ident.as_str() {
                "nil" => Type::Nil,
                "i32" => Type::I32,
                "i64" => Type::I64,
                "bool" => Type::Bool,
                "byte" => Type::Byte,
                "array" => {
                    self.expect(Lexeme::LBracket)?;
                    let type_ = self.type_()?;
                    self.expect(Lexeme::Comma)?;

                    let token = self.consume()?;
                    let Lexeme::Int(size) = token.lexeme else {
                        bail!("Expected integer, got {:?}", ident);
                    };

                    self.expect(Lexeme::RBracket)?;

                    Type::Array(Box::new(type_), size as usize)
                }
                "vec" => {
                    self.expect(Lexeme::LBracket)?;
                    let type_ = self.type_()?;
                    self.expect(Lexeme::RBracket)?;

                    Type::Vec(Box::new(type_))
                }
                "ptr" => {
                    self.expect(Lexeme::LBracket)?;
                    let type_ = self.type_()?;
                    self.expect(Lexeme::RBracket)?;

                    Type::Ptr(Box::new(type_))
                }
                "map" => {
                    self.expect(Lexeme::LBracket)?;
                    let key = self.type_()?;
                    self.expect(Lexeme::Comma)?;
                    let value = self.type_()?;
                    self.expect(Lexeme::RBracket)?;

                    Type::Map(Box::new(key), Box::new(value))
                }
                ident => Type::Ident(Ident(ident.to_string())),
            },
            Lexeme::LBrace => {
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

                Type::Record(fields)
            }
            Lexeme::Underscore => self.gen_omit()?,
            Lexeme::Struct => {
                self.expect(Lexeme::LBrace)?;
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

                Type::Record(fields)
            }
            _ => bail!("Expected type, got {:?}", current.lexeme),
        };

        if self.peek()?.lexeme == Lexeme::Question {
            self.consume()?;
            type_ = Type::Optional(Box::new(type_));
        }

        Ok(type_)
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
                    path: Some(self.current_path.clone()),
                    start: self.input[self.position].position,
                    end: self.input[self.position].position,
                }),
            );
        }
    }

    fn gen_omit(&mut self) -> Result<Type> {
        self.omit_index += 1;

        Ok(Type::Omit(self.omit_index))
    }

    fn source<T>(&self, data: T, start: usize, end: usize) -> Source<T> {
        Source {
            data,
            start: Some(self.input.get(start).map(|p| p.position).unwrap_or(0)),
            end: Some(self.input.get(end).map(|p| p.position).unwrap_or(0)),
        }
    }

    fn source_from<T>(&self, data: T, start: usize) -> Source<T> {
        self.source(data, start, self.position)
    }
}

#[test]
fn test_expr() -> Result<()> {
    let source = |data: Expr| Source {
        data,
        start: Some(0),
        end: Some(0),
    };

    let cases = vec![(
        "a - b - c",
        vec![
            Token {
                lexeme: Lexeme::Ident("a".to_string()),
                position: 0,
            },
            Token {
                lexeme: Lexeme::Minus,
                position: 0,
            },
            Token {
                lexeme: Lexeme::Ident("b".to_string()),
                position: 0,
            },
            Token {
                lexeme: Lexeme::Minus,
                position: 0,
            },
            Token {
                lexeme: Lexeme::Ident("c".to_string()),
                position: 0,
            },
            Token {
                lexeme: Lexeme::Semicolon,
                position: 0,
            },
        ],
        source(Expr::Call(
            Box::new(source(Expr::ident(Ident("sub".to_string())))),
            vec![
                source(Expr::Call(
                    Box::new(source(Expr::ident(Ident("sub".to_string())))),
                    vec![
                        source(Expr::ident(Ident("a".to_string()))),
                        source(Expr::ident(Ident("b".to_string()))),
                    ],
                    None,
                )),
                source(Expr::ident(Ident("c".to_string()))),
            ],
            None,
        )),
    )];

    for (input, tokens, expected) in cases {
        let mut parser = Parser::new();
        parser.input = tokens;

        let actual = parser.expr()?;
        assert_eq!(expected, actual, "{}", input);
    }

    Ok(())
}
