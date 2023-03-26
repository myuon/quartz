use anyhow::{anyhow, bail, Context, Result};
use pretty_assertions::assert_eq;

use crate::{
    ast::{BinOp, Decl, Expr, Func, Lit, Module, Pattern, Statement, Type},
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
    skip_errors: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            position: 0,
            input: vec![],
            omit_index: 0,
            imports: vec![],
            current_path: Path::empty(),
            skip_errors: false,
        }
    }

    pub fn run(&mut self, input: Vec<Token>, path: Path, skip_errors: bool) -> Result<Module> {
        self.input = input;
        self.current_path = path;
        self.skip_errors = skip_errors;
        self.module()
    }

    pub fn module(&mut self) -> Result<Module> {
        let mut decls = vec![];

        while !self.is_end() && self.peek()?.lexeme != Lexeme::RBrace {
            let result = self.decl();
            if result.is_ok() || !self.skip_errors {
                decls.push(result?);
            }
        }

        Ok(Module(decls))
    }

    pub fn decl(&mut self) -> Result<Decl> {
        let consume = self.peek()?;
        match consume.lexeme {
            Lexeme::Fun => Ok(Decl::Func(self.func()?)),
            Lexeme::Let => {
                let (ident, type_, value) = self.let_()?;
                Ok(Decl::Let(
                    ident.data.as_ident().unwrap().clone(),
                    type_,
                    value,
                ))
            }
            Lexeme::Type => {
                let (ident, type_) = self.type_decl()?;
                Ok(Decl::Type(ident, type_))
            }
            Lexeme::Struct => {
                let (ident, type_) = self.struct_decl()?;
                Ok(Decl::Type(ident, type_))
            }
            Lexeme::Module(_path) => {
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

    fn struct_decl(&mut self) -> Result<(Ident, Type)> {
        self.expect(Lexeme::Struct)?;
        let ident = self.ident()?;

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

        Ok((ident, Type::Record(record_type)))
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
        let start = self.position;
        let ident = self.ident()?;
        let name = self.source_from(ident, start);
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
            let result = self.statement();
            if result.is_ok() || !self.skip_errors {
                let statement = result?;
                statements.push(self.source_from(statement, position));
            }
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
                let position = self.position;
                self.consume()?;

                if self.peek()?.lexeme == Lexeme::Semicolon {
                    self.consume()?;

                    Ok(Statement::Return(
                        self.source_from(Expr::Lit(Lit::Nil), position),
                    ))
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

                    let else_body = if self.peek()?.lexeme == Lexeme::If {
                        let position = self.position;
                        let statement = self.statement()?;

                        vec![self.source_from(statement, position)]
                    } else if self.peek()?.lexeme == Lexeme::LBrace {
                        self.expect(Lexeme::LBrace)?;
                        let else_body = self.block()?;
                        self.expect(Lexeme::RBrace)?;

                        else_body
                    } else {
                        return Err(anyhow!("expected else {{ or else if {{"));
                    };

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

                Ok(Statement::For(None, ident, range, body))
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

                        Ok(Statement::Assign(
                            Box::new(expr),
                            self.gen_omit()?,
                            Box::new(value),
                        ))
                    }
                    _ => {
                        // Mainly for dot completion
                        if self.skip_errors {
                            Ok(Statement::Expr(expr, Type::Omit(0)))
                        } else {
                            Err(
                                anyhow!("Unexpected token {:?}", self.peek()?.lexeme).context(
                                    ErrorInSource {
                                        path: Some(self.current_path.clone()),
                                        start: self.input[self.position].position,
                                        end: self.input[self.position].position,
                                    },
                                ),
                            )
                        }
                    }
                }
            }
        }
    }

    fn let_(&mut self) -> Result<(Source<Pattern>, Type, Source<Expr>)> {
        self.expect(Lexeme::Let)?;
        let pattern = self.pattern()?;

        let mut type_ = self.gen_omit()?;
        if self.peek()?.lexeme == Lexeme::Colon {
            self.consume()?;
            type_ = self.type_()?;
        }

        self.expect(Lexeme::Equal)?;
        let value = self.expr()?;
        self.expect(Lexeme::Semicolon).context("let:end")?;

        Ok((pattern, type_, value))
    }

    fn pattern(&mut self) -> Result<Source<Pattern>> {
        let position = self.position;
        let current = if self.peek()?.lexeme == Lexeme::Underscore {
            self.source_from(Pattern::Omit, position)
        } else {
            let ident = self.ident()?;
            self.source_from(Pattern::Ident(ident), position)
        };

        if self.peek()?.lexeme == Lexeme::Or {
            self.consume()?;
            let pat = self.pattern()?;
            Ok(self.source_from(Pattern::Or(Box::new(current), Box::new(pat)), position))
        } else {
            Ok(current)
        }
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
                Lexeme::Or => {
                    self.consume()?;
                    let rhs = self.term_4(with_struct)?;

                    let lhs = if let Expr::Omit(_) = current.data {
                        None
                    } else {
                        Some(Box::new(current))
                    };
                    let rhs = if let Expr::Omit(_) = rhs.data {
                        None
                    } else {
                        Some(Box::new(rhs))
                    };

                    let lhs_type = self.gen_omit()?;
                    let rhs_type = self.gen_omit()?;

                    current =
                        self.source_from(Expr::EnumOr(lhs_type, rhs_type, lhs, rhs), position);
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
                let t = self.gen_omit()?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Equal, t, Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            Lexeme::NotEqual => {
                self.consume()?;
                let rhs = self.term_3(with_struct)?;
                let t = self.gen_omit()?;

                current = self.source_from(
                    Expr::BinOp(BinOp::NotEqual, t, Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            _ => (),
        }

        Ok(current)
    }

    fn term_3(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_2(with_struct)?;

        let token = self.peek()?;
        match token.lexeme {
            Lexeme::Lt => {
                self.consume()?;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Lt, Type::Omit(0), Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            Lexeme::Gt => {
                self.consume()?;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Gt, Type::Omit(0), Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            Lexeme::Gte => {
                self.consume()?;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Gte, Type::Omit(0), Box::new(current), Box::new(rhs)),
                    position,
                );
            }
            Lexeme::Lte => {
                self.consume()?;
                let rhs = self.expr_(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Lte, Type::Omit(0), Box::new(current), Box::new(rhs)),
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

        loop {
            match self.peek()?.lexeme {
                Lexeme::Plus => {
                    self.consume()?;
                    let rhs = self.term_1(with_struct)?;

                    current = self.source_from(
                        Expr::BinOp(BinOp::Add, Type::Omit(0), Box::new(current), Box::new(rhs)),
                        position,
                    );
                }
                Lexeme::Minus => {
                    self.consume()?;
                    let rhs = self.term_1(with_struct)?;

                    current = self.source_from(
                        Expr::BinOp(BinOp::Sub, Type::Omit(0), Box::new(current), Box::new(rhs)),
                        position,
                    );
                }
                Lexeme::As => {
                    self.consume()?;
                    let type_ = self.type_()?;

                    current = self
                        .source_from(Expr::As(Box::new(current), Type::Omit(0), type_), position);
                }
                _ => break,
            }
        }

        Ok(current)
    }

    fn term_1(&mut self, with_struct: bool) -> Result<Source<Expr>> {
        let position = self.position;
        let mut current = self.term_0(with_struct)?;

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
                let rhs = self.term_1(with_struct)?;

                current = self.source_from(
                    Expr::BinOp(BinOp::Div, Type::Omit(0), Box::new(current), Box::new(rhs)),
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
            Lexeme::Underscore => {
                self.consume()?;

                let t = self.gen_omit()?;
                self.source_from(Expr::Omit(t), position)
            }
            _ => {
                let position = self.position;
                let lit = self.lit()?;

                self.source_from(Expr::Lit(lit), position)
            }
        };

        loop {
            match self.peek()?.lexeme {
                Lexeme::LParen => {
                    self.consume()?;
                    let mut args = vec![];
                    let mut expansion = None;
                    while self.peek()?.lexeme != Lexeme::RParen {
                        // expansion
                        if self.peek()?.lexeme == Lexeme::DoubleDot {
                            self.consume()?;
                            expansion = Some(Box::new(self.expr()?));
                            break;
                        }

                        args.push(self.expr()?);

                        if self.peek()?.lexeme == Lexeme::Comma {
                            self.consume()?;
                        } else {
                            break;
                        }
                    }
                    self.consume()?;

                    current = self.source_from(
                        Expr::Call(Box::new(current), args, None, expansion),
                        position,
                    );
                }
                Lexeme::Dot => {
                    self.consume()?;

                    let result = self.dot_after(current.clone(), position);
                    if result.is_ok() || !self.skip_errors {
                        current = result?;
                    }
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

                    let type_ = self.gen_omit()?;
                    current = self.source_from(Expr::Wrap(type_, Box::new(current)), position);
                }
                Lexeme::Bang => {
                    self.consume()?;

                    let type_ = self.gen_omit()?;
                    current =
                        self.source_from(Expr::Unwrap(type_, None, Box::new(current)), position);
                }
                _ => break,
            }
        }

        Ok(current)
    }

    fn dot_after(&mut self, current: Source<Expr>, position: usize) -> Result<Source<Expr>> {
        if self.peek()?.lexeme == Lexeme::Try {
            self.consume()?;

            Ok(self.source_from(Expr::Try(Box::new(current)), position))
        } else {
            let position = self.position;
            let ident = self.ident()?;
            let field = self.source_from(Path::ident(ident), position);
            let omit = self.gen_omit()?;

            Ok(self.source_from(Expr::Project(Box::new(current), omit, field), position))
        }
    }

    fn ident(&mut self) -> Result<Ident> {
        let current = self.peek()?;
        if let Lexeme::Ident(ident) = &current.lexeme {
            self.consume()?;

            Ok(Ident(ident.clone()))
        } else {
            Err(
                anyhow!("Expected identifier, got {:?}", current.lexeme).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: self
                        .input
                        .get(self.position)
                        .map(|p| p.position)
                        .unwrap_or(0),
                    end: self
                        .input
                        .get(self.position + 1)
                        .map(|p| p.position)
                        .unwrap_or(0),
                }),
            )
        }
    }

    fn lit(&mut self) -> Result<Lit> {
        let current = self.consume()?;
        match current.lexeme {
            Lexeme::Int(int) if int <= i32::MAX as i64 => Ok(Lit::I32(int as i32)),
            Lexeme::Int(int) if int <= u32::MAX as i64 => Ok(Lit::U32(int as u32)),
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
                "u32" => Type::U32,
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
                "any" => Type::Any,
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

        if self.peek()?.lexeme == Lexeme::Or {
            self.consume()?;
            let type_2 = self.type_()?;
            type_ = Type::Or(Box::new(type_), Box::new(type_2));
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
                raw: "a".to_string(),
            },
            Token {
                lexeme: Lexeme::Minus,
                position: 0,
                raw: "-".to_string(),
            },
            Token {
                lexeme: Lexeme::Ident("b".to_string()),
                position: 0,
                raw: "b".to_string(),
            },
            Token {
                lexeme: Lexeme::Minus,
                position: 0,
                raw: "-".to_string(),
            },
            Token {
                lexeme: Lexeme::Ident("c".to_string()),
                position: 0,
                raw: "c".to_string(),
            },
            Token {
                lexeme: Lexeme::Semicolon,
                position: 0,
                raw: ";".to_string(),
            },
        ],
        source(Expr::BinOp(
            BinOp::Sub,
            Type::Omit(0),
            Box::new(source(Expr::BinOp(
                BinOp::Sub,
                Type::Omit(0),
                Box::new(source(Expr::ident(Ident("a".to_string())))),
                Box::new(source(Expr::ident(Ident("b".to_string())))),
            ))),
            Box::new(source(Expr::ident(Ident("c".to_string())))),
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
