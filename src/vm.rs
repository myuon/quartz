use anyhow::{bail, Result};

use crate::ast::Statement;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum DataType {
    Nil,
    Bool(bool),
    Int(i32),
    String(String),
    Closure(
        usize, // UID for closure
        Vec<String>,
        Vec<Statement>,
    ),
}

impl DataType {
    pub fn as_closure(self) -> Result<(Vec<String>, Vec<Statement>)> {
        match self {
            DataType::Closure(uid, params, body) => Ok((params, body)),
            d => bail!("Expected a closure, but found {:?}", d),
        }
    }
}
