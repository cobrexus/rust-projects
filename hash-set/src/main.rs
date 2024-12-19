fn main() {}

#[derive(Debug)]
pub struct HashSet<'a, const SIZE: usize> {
    array: [Item<'a>; SIZE],
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Value(&'a str),
    Deleted,
    Empty,
}

#[derive(Debug)]
pub enum Error {
    Full,
}

impl<'a, const SIZE: usize> HashSet<'a, SIZE> {
    pub fn new() -> Self {
        Self {
            array: [const { Item::Empty }; SIZE],
        }
    }

    fn hash(value: &str) -> u32 {
        value.bytes().map(|x| x as u32).sum::<u32>() % value.len() as u32
    }

    pub fn add(&mut self, value: &'a str) -> Result<usize, Error> {
        let key = Self::hash(value) as usize % self.array.len();

        if let Some(x) = self
            .array
            .iter()
            .enumerate()
            .cycle()
            .skip(key)
            .take(self.array.len())
            .find(|x| *x.1 == Item::Empty || *x.1 == Item::Deleted)
        {
            let pos = x.0;
            self.array[pos] = Item::Value(value);

            Ok(pos)
        } else {
            Err(Error::Full)
        }
    }

    pub fn search(&self, value: &str) -> Option<usize> {
        let key = Self::hash(value) as usize;

        if let Some(x) = self
            .array
            .iter()
            .enumerate()
            .cycle()
            .skip(key)
            .take(self.array.len())
            .find(|x| *x.1 == Item::Value(value))
        {
            Some(x.0)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: usize) {
        if let Item::Value(_) = self.array[key] {
            self.array[key] = Item::Deleted;
        }
    }
}
