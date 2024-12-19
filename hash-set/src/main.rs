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
        let key = Self::hash(&value) as usize % self.array.len();

        if let Some(pos) = self.array[key..]
            .iter()
            .chain(self.array[0..key].iter())
            .position(|x| *x == Item::Empty || *x == Item::Deleted)
        {
            self.array[pos] = Item::Value(value);

            Ok(pos)
        } else {
            Err(Error::Full)
        }
    }

    pub fn search(&self, value: &str) -> Option<usize> {
        let key = Self::hash(&value) as usize;

        self.array[key..]
            .iter()
            .chain(self.array[0..key].iter())
            .position(|x| *x == Item::Value(value))
    }

    pub fn remove(&mut self, key: usize) {
        if let Item::Value(_) = self.array[key] {
            self.array[key] = Item::Deleted;
        }
    }
}
