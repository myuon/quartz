use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct SerialIdMap<T> {
    pub keys: HashMap<T, usize>,
    pub next: usize,
}

impl<T: Hash + Eq> SerialIdMap<T> {
    pub fn new() -> SerialIdMap<T> {
        SerialIdMap {
            keys: HashMap::new(),
            next: 0,
        }
    }

    pub fn get_or_insert(&mut self, key: T) -> usize {
        match self.keys.get(&key) {
            Some(&id) => id,
            None => {
                let id = self.next;
                self.next += 1;
                self.keys.insert(key, id);
                id
            }
        }
    }
}
