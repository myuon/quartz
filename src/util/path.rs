use super::ident::Ident;

pub struct Path(pub Vec<Ident>);

impl Path {
    pub fn new(segments: Vec<Ident>) -> Path {
        Path(segments)
    }

    pub fn ident(ident: Ident) -> Path {
        Path(vec![ident])
    }
}
