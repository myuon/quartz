use super::ident::Ident;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Path(pub Vec<Ident>);

impl Path {
    pub fn empty() -> Path {
        Path(vec![])
    }

    pub fn new(segments: Vec<Ident>) -> Path {
        Path(segments)
    }

    pub fn ident(ident: Ident) -> Path {
        Path(vec![ident])
    }

    pub fn push(&mut self, ident: Ident) {
        self.0.push(ident);
    }

    pub fn as_str(&mut self) -> String {
        self.0
            .iter()
            .map(|ident| ident.as_str())
            .collect::<Vec<&str>>()
            .join("::")
    }
}
