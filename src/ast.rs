#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    I32,
    F32,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Type::I32 => "i32",
            Type::F32 => "f32",
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lit {
    I32(i32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Lit(Lit),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Ident, Type, Expr),
    Return(Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub result: Type,
    pub body: Vec<Statement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Decl {
    Func(Func),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Decl>);
