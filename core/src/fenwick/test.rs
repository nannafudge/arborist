use crate::{Tree, fenwick::FenwickTreeError};

impl Tree for [usize] {
    type Key = usize;
    type Value = usize;
    type Error = FenwickTreeError;

    fn size(&self) -> usize {
        todo!()
    }

    fn get(&self, _key: &Self::Key) -> Result<&Self::Value, Self::Error> {
        todo!()
    }

    fn contains(&self, _key: &Self::Key) -> Result<&Self::Value, Self::Error> {
        todo!()
    }
}