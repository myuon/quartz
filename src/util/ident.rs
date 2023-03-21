#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Ident(pub String);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
