use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    Length,
    InsertableCollection, IndexedCollectionMut,
    VirtualTreeView, FenwickTreeError
};
use arborist_core::{
    Tree, Height,
    TreeWalker,
    Direction, NodeSide, TreeMut, require
};

use core::cmp::Ordering;

#[cfg(feature = "no_std")]
use bumpalo::Vec;
#[cfg(not(feature = "no_std"))]
use std::vec::Vec;

#[derive(Length)]
#[length_method("self.inner.length()")]
pub struct BTreeSet<C: Height>  {
    inner: C
}

impl<'t, C> Tree<'t> for BTreeSet<C> where
    C: InsertableCollection,
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
        let index: usize = walker.find(key)?;

        self.inner.insert(index, value);
        Ok(&self.inner[index])
    }

    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Result<&C::Output, BTreeError> {
        let current = self.get_mut(key)?;
        *current = value;

        Ok(current)
    }

    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Result<C::Output, BTreeError> {
        let mut walker: BTreeWalker<C> = BTreeWalker::new(&self.inner)?;
        let index: usize = walker.find(key)?;

        Ok(self.inner.remove(index))
    }

    fn push(&mut self, value: Self::Value) -> Result<&C::Output, BTreeError> {
        self.inner.insert(1, value);
        Ok(&self.inner[1])
    }

    fn pop(&mut self, key: &Self::Key) -> Result<&C::Output, BTreeError> {
        require!(self.inner.length() > 1, BTreeError::Inner(FenwickTreeError::Empty));

        Ok(&self.inner.remove(self.inner.length() - 1))
    }
}

impl<'t, C> TreeMut<'t> for BTreeSet<C> where
    C: InsertableCollection,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn get_mut(&'t mut self, key: &Self::Key) -> Result<&'t mut Self::Value, Self::Error> {
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
    C: InsertableCollection,
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