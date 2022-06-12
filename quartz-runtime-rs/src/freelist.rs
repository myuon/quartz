use anyhow::Result;

use crate::runtime::{AddrPlace, Value};

#[derive(Debug, Clone)]
pub struct LinkObjectHeader {
    pointer: usize,
    len: usize,  // at 0
    prev: usize, // at 1
    next: usize, // at 2
}

impl LinkObjectHeader {
    pub fn is_collectable(&self) -> bool {
        self.len > 0
    }

    pub fn get_end_pointer(&self) -> usize {
        self.pointer + 3 + self.len
    }

    pub fn get_data_pointer(&self) -> usize {
        self.pointer + 3
    }
}

#[derive(Debug)]
pub struct Freelist {
    pub data: Vec<Value>,
}

impl Freelist {
    pub fn new(len: usize) -> Freelist {
        let mut list = Freelist {
            data: vec![Value::nil(); len],
        };

        let root = 0;
        let last = len - 3;
        list.push(LinkObjectHeader {
            pointer: root,
            len: 0,
            prev: last,
            next: last,
        });
        list.push(LinkObjectHeader {
            pointer: last,
            len: 0,
            prev: root,
            next: root,
        });

        list
    }

    fn push(&mut self, obj: LinkObjectHeader) {
        let prev = obj.prev;
        let next = obj.next;

        self.data[obj.pointer] = Value::Int(obj.len as i32, "len");
        self.data[obj.pointer + 1] = Value::Addr(prev, AddrPlace::Heap, "prev");
        self.data[obj.pointer + 2] = Value::Addr(next, AddrPlace::Heap, "next");
    }

    pub fn parse(&self, index: usize) -> Result<LinkObjectHeader> {
        let len = self.data[index].as_named_int("len").unwrap() as usize;
        let prev = self.data[index + 1].as_named_addr("prev").unwrap();
        let next = self.data[index + 2].as_named_addr("next").unwrap();

        Ok(LinkObjectHeader {
            pointer: index,
            len,
            prev,
            next,
        })
    }

    pub fn root(&self) -> Result<LinkObjectHeader> {
        self.parse(0)
    }

    pub fn find_prev(&self, object: &LinkObjectHeader) -> Result<LinkObjectHeader> {
        let obj = self.parse(object.prev)?;

        Ok(obj)
    }

    pub fn find_next(&self, object: &LinkObjectHeader) -> Result<LinkObjectHeader> {
        let obj = self.parse(object.next)?;

        Ok(obj)
    }

    pub fn insert(&mut self, object: LinkObjectHeader) -> Result<()> {
        let mut next = self.find_next(&object)?;
        next.prev = object.pointer;
        self.push(next);

        let mut prev = self.find_prev(&object)?;
        prev.next = object.pointer;
        self.push(prev);

        self.push(object);

        Ok(())
    }

    pub fn free(&mut self, object: LinkObjectHeader) -> Result<()> {
        let mut prev = self.find_prev(&object)?;
        let mut next = self.find_next(&object)?;
        prev.next = next.pointer;
        next.prev = prev.pointer;

        self.push(prev);
        self.push(next);

        Ok(())
    }

    pub fn alloc(&mut self, size: usize) -> Result<usize> {
        let mut current = self.root()?;

        while current.next != 0 {
            let prev = current.clone();
            current = self.find_next(&current)?;

            if (current.pointer - prev.get_end_pointer()) > size + 3 {
                let new_object = LinkObjectHeader {
                    pointer: prev.get_end_pointer(),
                    len: size,
                    prev: prev.pointer,
                    next: current.pointer,
                };
                let pointer = new_object.get_data_pointer();

                self.insert(new_object)?;

                return Ok(pointer);
            }
        }

        anyhow::bail!("no space left: {}", size);
    }
}

#[test]
fn test_alloc_many() -> Result<()> {
    let mut freelist = Freelist::new(100);
    for _ in 0..(100 / 13) {
        freelist.alloc(10)?;
    }

    let mut current = freelist.root()?;
    for _ in 0..=(100 / 13) {
        println!("{:?}", current);
        current = freelist.find_next(&current)?;
    }

    assert!(freelist.alloc(10).is_err());

    Ok(())
}
