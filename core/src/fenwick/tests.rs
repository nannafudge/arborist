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
        Err(FenwickTreeError::OutOfBounds { index: 0 })
    );

    // Too large an index
    assert_eq!(
        FenwickTreeView::new(collection, collection.len() + 1),
        Err(FenwickTreeError::OutOfBounds { index: collection.len() + 1 })
    );
}

#[test]
fn test_construction_invalid_collection() {
    let collection: &[usize] = &[];

    assert_eq!(
        FenwickTreeView::new(collection, collection.len()),
        Err(FenwickTreeError::OutOfBounds { index: collection.len() })
    );
}

#[test]
fn test_traverse_up() {
    // [usize; 16] where each index corresponds to itself
    // i.e. c[0] = 0; c[1] = 1; c[2] = 2, etc...
    let collection: &[usize] = &gen_collection();

    // Start from index 1
    let mut walker: FenwickTreeView<[usize]> = FenwickTreeView::new(collection, 1).unwrap();
    // Fenwick: parent of 1 is 2
    assert_eq!(walker.up(), Ok(&collection[2]));
    assert_eq!(walker.view, FenwickIndexView { index: 2, lsb: 2 });
    // Fenwick: parent of 2 is 4
    assert_eq!(walker.up(), Ok(&collection[4]));
    assert_eq!(walker.view, FenwickIndexView { index: 4, lsb: 4 });
    // Fenwick: parent of 4 is 8
    assert_eq!(walker.up(), Ok(&collection[8]));
    assert_eq!(walker.view, FenwickIndexView { index: 8, lsb: 8 });
    // Fenwick: parent of 8 is 32, Out of bounds
    assert_eq!(walker.up(), Err(FenwickTreeError::OutOfBounds { index: 16 }));
    assert_eq!(walker.view, FenwickIndexView { index: 16, lsb: 16 });
}