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

    pub fn extend(&mut self, other: &Path) {
        self.0.extend(other.0.clone());
    }

    pub fn starts_with(&self, other: &Path) -> bool {
        self.0.starts_with(&other.0)
    }

    pub fn as_str(&self) -> String {
        self.0
            .iter()
            .map(|ident| ident.as_str())
            .collect::<Vec<&str>>()
            .join("::")
    }

    pub fn remove_prefix(&self, prefix: &Path) -> Path {
        if !self.starts_with(prefix) {
            panic!("Tried to remove prefix from path that doesn't start with it");
        }

        Path(self.0[prefix.0.len()..].to_vec())
    }

    pub fn as_joined_str(&self, delimiter: &str) -> String {
        self.0
            .iter()
            .map(|ident| ident.as_str())
            .collect::<Vec<&str>>()
            .join(delimiter)
    }
}
