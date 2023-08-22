#[cfg(test)]
use crate::{Tree, TreeWalker};
use crate::fenwick::traits::*;
use crate::fenwick::{FenwickTreeError, FenwickTreeView, FenwickIndexView};

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

fn gen_collection() -> [usize; 16] {
    let mut out: [usize; 16] = [0; 16];
    for i in 0..16 {
        out[i] = i;
    }
    out
}

#[test]
fn test_construction() {
    let collection: &[usize] = &gen_collection();

    let walker: FenwickTreeView::<[usize]> = FenwickTreeView::<[usize]>::new(collection, 1).unwrap();
    assert_eq!(walker.view, FenwickIndexView { index: 1, lsb: 1 });
}

#[test]
fn test_construction_invalid_index() {
    let collection: &[usize] = &gen_collection();

    // Zero'd index
    assert_eq!(
        FenwickTreeView::new(collection, 0),
        Err(FenwickTreeError::OutOfBounds { index: 0, tree_len: collection.len() })
    );

    // Too large an index
    assert_eq!(
        FenwickTreeView::new(collection, collection.len() + 1),
        Err(FenwickTreeError::OutOfBounds { index: collection.len() + 1, tree_len: collection.len() })
    );
}