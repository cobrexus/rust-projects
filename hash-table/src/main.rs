fn main() {}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Value(u32),
    Deleted,
    Empty,
}

#[derive(Debug)]
pub enum Error {
    TableFull,
    ValueNotFound,
}

#[derive(Debug)]
pub struct HashTable {
    size: usize,
    table: Vec<Item>,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        HashTable {
            size,
            table: vec![Item::Empty; size],
        }
    }

    pub fn add(&mut self, value: u32) -> Result<usize, Error> {
        // Simple folding hashing algorithm

        let key = Self::hash(value) as usize % self.size;

        let mut idx = key;

        loop {
            if idx >= self.size {
                idx = 0;
            }

            if self.table[idx] == Item::Empty || self.table[idx] == Item::Deleted {
                self.table[idx] = Item::Value(value);

                break Ok(idx);
            }

            if idx + 1 >= key {
                break Err(Error::TableFull);
            } else {
                idx += 1;
            }
        }
    }

    fn hash(value: u32) -> u32 {
        let piece_size = 2;
        let mut hash = 0;
        let mut temp = value;

        while temp != 0 {
            hash *= 10_u32.pow(piece_size);
            hash += temp % 10_u32.pow(piece_size);
            temp /= 10_u32.pow(piece_size);
        }

        hash
    }

    pub fn search(&self, value: u32) -> Result<usize, Error> {
        let key = Self::hash(value) as usize;

        let mut idx = key;

        loop {
            if idx >= self.size {
                idx = 0;
            }

            if self.table[idx] == Item::Value(value) {
                break Ok(idx);
            }

            if idx == key - 1 {
                break Err(Error::ValueNotFound);
            } else {
                idx += 1;
            }
        }
    }

    pub fn remove(&mut self, key: usize) -> Result<(), Error> {
        if let Item::Value(_) = self.table[key] {
            self.table[key] = Item::Deleted;
            Ok(())
        } else {
            Err(Error::ValueNotFound)
        }
    }
}
