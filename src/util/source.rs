#[derive(PartialEq, Debug, Clone)]
pub struct Source<T> {
    pub data: T,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl<T> Source<T> {
    pub fn new(data: T, start: usize, end: usize) -> Source<T> {
        assert!(start <= end);

        Source {
            data,
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn unknown(data: T) -> Source<T> {
        Source {
            data,
            start: None,
            end: None,
        }
    }

    pub fn transfer<S>(data: T, other: &Source<S>) -> Source<T> {
        Source {
            data,
            start: other.start,
            end: other.end,
        }
    }
}
