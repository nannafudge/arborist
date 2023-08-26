use arborist_core::{Tree, TreeMut, TreeWalker, Height};
use arborist_core::fenwick::{
    IndexedCollection, IndexedCollectionMut,
    FenwickTreeWalker, Length
};
use core::ops::{Index, IndexMut};

#[cfg(feature = "no_std")]
use bumpalo::Vec;
use std::process::Output;
#[cfg(not(feature = "no_std"))]
use std::vec::Vec;

pub struct BTreeSet<T> {
    inner: dyn IndexedCollectionMut<Output = T>
}

impl<T> Length for BTreeSet<T> {
    fn length(&self) -> usize {
        self.inner.length()
    }
}

impl<T> Index<usize> for BTreeSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for BTreeSet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T> Tree for BTreeSet<T> where
    T: PartialEq + PartialOrd
{
    type Key = T;
    type Value = T;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }

    fn contains(&self, key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }
}
