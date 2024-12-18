fn main() {}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Value(String),
    Deleted,
    Empty,
}

#[derive(Debug)]
pub struct HashSet<const SIZE: usize> {
    array: [Item; SIZE],
}

#[derive(Debug)]
pub enum Error {
    ArrayFull,
}

impl<const SIZE: usize> HashSet<SIZE> {
    pub fn new() -> Self {
        Self {
            array: [const { Item::Empty }; SIZE],
        }
    }

    pub fn add(&mut self, value: String) -> Result<usize, Error> {
        let key = Self::hash(&value) as usize % self.array.len();

        let mut idx = key;

        loop {
            if idx >= self.array.len() {
                idx = 0;
            }

            if self.array[idx] == Item::Empty || self.array[idx] == Item::Deleted {
                self.array[idx] = Item::Value(value);

                break Ok(idx);
            }

            if idx + 1 >= key {
                break Err(Error::ArrayFull);
            } else {
                idx += 1;
            }
        }
    }

    fn hash(value: &str) -> u32 {
        value.bytes().map(|x| x as u32).sum::<u32>() % value.len() as u32
    }

    pub fn search(&self, value: String) -> Option<usize> {
        let key = Self::hash(&value) as usize;

        let mut idx = key;

        let item = Item::Value(value);

        loop {
            if idx >= self.array.len() {
                idx = 0;
            }

            if self.array[idx] == item {
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
        if let Item::Value(_) = self.array[key] {
            self.array[key] = Item::Deleted;
        }
    }
}
