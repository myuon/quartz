use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::runtime::{AddrPlace, Value, ValueAddrFlag, ValueIntFlag};

#[derive(Debug, Clone)]
pub struct LinkObjectHeader {
    pointer: usize,
    len: usize,  // at 0
    prev: usize, // at 1
    next: usize, // at 2
    info: Value, // at 3
}

impl LinkObjectHeader {
    pub fn size_of() -> usize {
        4
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_collectable(&self) -> bool {
        self.pointer != 0 && self.next != 0
    }

    pub fn get_end_pointer(&self) -> usize {
        self.pointer + LinkObjectHeader::size_of() + self.len
    }

    pub fn get_data_pointer(&self) -> usize {
        self.pointer + LinkObjectHeader::size_of()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Freelist {
    pub data: Vec<Value>,
    alloc_search_pointer: usize,
}

impl Freelist {
    pub fn new(len: usize) -> Freelist {
        let mut list = Freelist {
            data: vec![Value::nil(); len],
            alloc_search_pointer: 0,
        };

        let root = 0;
        let last = len - LinkObjectHeader::size_of();
        list.push(LinkObjectHeader {
            pointer: root,
            len: 0,
            prev: last,
            next: last,
            info: Value::Addr(0, AddrPlace::InfoTable, ValueAddrFlag::Nodata),
        });
        list.push(LinkObjectHeader {
            pointer: last,
            len: 0,
            prev: root,
            next: root,
            info: Value::Addr(0, AddrPlace::InfoTable, ValueAddrFlag::Nodata),
        });

        list
    }

    fn push(&mut self, obj: LinkObjectHeader) {
        let prev = obj.prev;
        let next = obj.next;

        self.data[obj.pointer] = Value::Int(obj.len as i32, ValueIntFlag::Len);
        self.data[obj.pointer + 1] = Value::Addr(prev, AddrPlace::Heap, ValueAddrFlag::Prev);
        self.data[obj.pointer + 2] = Value::Addr(next, AddrPlace::Heap, ValueAddrFlag::Next);
        self.data[obj.pointer + 3] = obj.info;
    }

    pub fn parse(&self, index: usize) -> Result<LinkObjectHeader> {
        let len = self.data[index].clone();
        let prev = self.data[index + 1].clone();
        let next = self.data[index + 2].clone();
        let info = self.data[index + 3].clone();

        let len = len.as_named_int(ValueIntFlag::Len).unwrap() as usize;
        let prev = prev.as_named_addr(ValueAddrFlag::Prev).unwrap();
        let next = next.as_named_addr(ValueAddrFlag::Next).unwrap();

        Ok(LinkObjectHeader {
            pointer: index,
            len,
            prev,
            next,
            info,
        })
    }

    pub fn parse_from_data_pointer(&self, index: usize) -> Result<LinkObjectHeader> {
        if index == 0 {
            return self.root();
        }

        match self.data[index - 1] {
            Value::Addr(_, AddrPlace::InfoTable, _) => {}
            _ => anyhow::bail!(
                "not a valid data pointer, {} {:?}",
                index,
                self.data[index - 1]
            ),
        };

        self.parse(index - LinkObjectHeader::size_of())
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
        let mut current = self.parse(self.alloc_search_pointer)?;

        while current.next != 0 {
            let prev = current.clone();
            current = self.find_next(&current)?;

            if (current.pointer - prev.get_end_pointer()) > size + 3 {
                let new_object = LinkObjectHeader {
                    pointer: prev.get_end_pointer(),
                    len: size,
                    prev: prev.pointer,
                    next: current.pointer,
                    info: Value::Addr(0, AddrPlace::InfoTable, ValueAddrFlag::Nodata),
                };
                let pointer = new_object.get_data_pointer();

                self.insert(new_object)?;

                return Ok(pointer);
            }
        }

        // SUGGESTION: run GC here

        for obj in self.debug_objects() {
            debug!("{:?}", obj);
        }
        anyhow::bail!("no space left: {}", size);
    }

    pub fn debug_objects(&self) -> Vec<(LinkObjectHeader, Vec<Value>)> {
        let mut result = vec![];

        let mut current = self.root().unwrap();
        while let Ok(next) = self.find_next(&current) {
            if !next.is_collectable() {
                break;
            }

            result.push((
                next.clone(),
                self.data[next.get_data_pointer()..next.get_end_pointer()].to_vec(),
            ));
            current = next;
        }

        result
    }
}

#[test]
fn test_alloc_many() -> Result<()> {
    use log::info;

    let mut freelist = Freelist::new(100);
    let space = 100 - LinkObjectHeader::size_of() * 2;
    for _ in 0..(space / (10 + LinkObjectHeader::size_of())) {
        freelist.alloc(10)?;
    }

    let mut current = freelist.root()?;
    for _ in 0..=(space / (10 + LinkObjectHeader::size_of())) {
        info!("{:?}", current);
        current = freelist.find_next(&current)?;
    }

    assert!(freelist.alloc(10).is_err());

    Ok(())
}
