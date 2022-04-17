use anyhow::Result;

#[derive(Debug, Clone)]
pub struct LinkObjectHeader {
    pointer: usize,
    len: usize,  // at 0
    prev: usize, // at 1
    next: usize, // at 2
}

impl LinkObjectHeader {
    pub fn get_end_pointer(&self) -> usize {
        self.pointer + 3 + self.len
    }
}

#[derive(Debug)]
pub struct Freelist {
    data: Vec<usize>,
}

impl Freelist {
    pub fn new(len: usize) -> Freelist {
        let mut list = Freelist { data: vec![0; len] };

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

        self.data[obj.pointer] = obj.len;
        self.data[obj.pointer + 1] = prev;
        self.data[obj.pointer + 2] = next;
    }

    pub fn parse(&self, index: usize) -> Result<LinkObjectHeader> {
        let len = self.data[index];
        let prev = self.data[index + 1];
        let next = self.data[index + 2];

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

        while current.next == 0 {
            let prev = current.clone();
            current = self.find_next(&current)?;

            if (current.pointer - prev.get_end_pointer()) > size {
                let new_object = LinkObjectHeader {
                    pointer: prev.get_end_pointer(),
                    len: size,
                    prev: prev.pointer,
                    next: current.pointer,
                };
                let pointer = new_object.pointer;

                self.insert(new_object)?;

                return Ok(pointer);
            }
        }

        anyhow::bail!("no space left: {}", size);
    }
}
