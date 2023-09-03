use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    Length,
    IndexedCollection, IndexedCollectionMut,
    VirtualTreeView, FenwickTreeError, StatefulTreeView
};
use arborist_core::{
    Tree, Height,
    TreeWalker,
    Direction, NodeSide, TreeMut
};

use core::cmp::Ordering;

#[cfg(feature = "no_std")]
use bumpalo::Vec;
#[cfg(not(feature = "no_std"))]
use std::vec::Vec;

pub struct BTreeSet<C: Height>  {
    inner: C
}

impl<C: Length> Length for BTreeSet<C> {
    fn length(&self) -> usize {
        self.inner.length()
    }
}

impl<C> Tree for BTreeSet<C> where
    C: IndexedCollection,
    C::Output: Sized + PartialEq + PartialOrd
{
    type Key = C::Output;
    type Value = C::Output;
    type Error = BTreeError;

    fn get(&self, key: &Self::Key) -> Result<&C::Output, BTreeError> {
        let mut walker: BTreeWalker<C> = BTreeWalker::new(&self.inner)?;
        let index: usize = walker.find(key).map_err(|_| BTreeError::KeyNotFound)?;
        Ok(&self.inner[index])
    }

    fn contains(&self, key: &Self::Key) -> Result<bool, BTreeError> {
        let mut walker: BTreeWalker<C> = BTreeWalker::new(&self.inner)?;
        Ok(walker.find(key).is_ok())
    }

    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> Result<&C::Output, BTreeError> {
        let mut walker: BTreeWalker<C> = BTreeWalker::new(&self.inner)?;
        let _ = walker.find(key);

        todo!()
    }

    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn push(&mut self, value: Self::Value) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn pop(&mut self, key: &Self::Key) -> Result<&C::Output, BTreeError> {
        todo!()
    }
}

impl<C> TreeMut for BTreeSet<C> where
    C: IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn get_mut(&mut self, key: &Self::Key) -> Result<&mut Self::Value, Self::Error> {
        let mut walker: BTreeWalker<C> = BTreeWalker::new(&self.inner)?;
        let index: usize = walker.find(key).map_err(|_| BTreeError::KeyNotFound)?;
        Ok(&mut self.inner[index])
    }
}

struct BTreeWalker<'w, C: Length> {
    pub view: VirtualTreeView,
    inner: &'w C
}

impl<'w, C> BTreeWalker<'w, C> where
    C: IndexedCollection,
    C::Output: Sized + PartialOrd
{
    pub fn new(inner: &'w C) -> Result<Self, BTreeError> {
        Ok(Self {
            view: VirtualTreeView::new(inner, inner.height())?,
            inner: inner
        })
    }

    pub fn find(&mut self, key: &C::Output) -> Result<usize, FenwickTreeError> {
        let mut node: usize = self.view.current()?;
        loop {
            match &self.inner[node].partial_cmp(key) {
                Some(Ordering::Less) => {
                    node = self.view.traverse(Direction::Down(NodeSide::Left))?;
                },
                Some(Ordering::Greater) => {
                    node = self.view.traverse(Direction::Down(NodeSide::Right))?;
                },
                Some(Ordering::Equal) => {
                    return Ok(node)
                },
                _ => panic!("PartialCmp failed for BTree node in get()")
            }
        }
    }
}

pub enum BTreeError {
    KeyNotFound,
    Inner(FenwickTreeError)
}

impl From<FenwickTreeError> for BTreeError {
    fn from(err: FenwickTreeError) -> Self {
        Self::Inner(err)
    }
}