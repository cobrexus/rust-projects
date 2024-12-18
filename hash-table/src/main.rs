fn main() {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Value(u32),
    Deleted,
    Empty,
}

#[derive(Debug)]
pub struct HashTable<const SIZE: usize> {
    table: [Item; SIZE],
}

#[derive(Debug)]
pub enum Error {
    TableFull,
}

impl<const SIZE: usize> HashTable<SIZE> {
    pub fn new() -> Self {
        HashTable {
            table: [Item::Empty; SIZE],
        }
    }

    pub fn add(&mut self, value: u32) -> Result<usize, Error> {
        let key = Self::hash(value) as usize % self.table.len();

        let mut idx = key;

        loop {
            if idx >= self.table.len() {
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
        // Simple folding hashing algorithm

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

    pub fn search(&self, value: u32) -> Option<usize> {
        let key = Self::hash(value) as usize;

        let mut idx = key;

        loop {
            if idx >= self.table.len() {
                idx = 0;
            }

            if self.table[idx] == Item::Value(value) {
                break Some(idx);
            }

            if idx == key - 1 {
                break None;
            } else {
                idx += 1;
            }
        }
    }

    pub fn remove(&mut self, key: usize) {
        if let Item::Value(_) = self.table[key] {
            self.table[key] = Item::Deleted;
        }
    }
}
