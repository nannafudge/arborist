use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    Length,
    InsertableCollection, IndexedCollection, IndexedCollectionMut,
    VirtualTreeView, FenwickTreeError
};
use arborist_core::{
    TreeRead, TreeReadMut,
    TreeWrite, TreeWriteMut,
    TreeWalker, Direction, NodeSide,
    Height, require
};
use arborist_core::tree_kv::NodeKV;

use core::cmp::Ordering;

#[cfg(feature = "bumpalo_vec")]
pub mod bumpalo_vec {
    use bumpalo::collections::Vec;
    use super::{BST, NodeKV};

    pub type BSTSet<T> = BST<Vec<T>>;
    pub type BSTMap<'a, K, V> = BST<Vec<NodeKV<'a, K, V>>>;
}

#[cfg(feature = "std_vec")]
pub mod std_vec {
    use std::vec::Vec;
    use super::{BST, NodeKV};

    pub type BSTSet<T> = BST<Vec<T>>;
    pub type BSTMap<'a, K, V> = BST<Vec<NodeKV<'a, K, V>>>;
}

#[cfg(feature = "const_vec")]
pub mod const_vec {
    use tinyvec::ArrayVec;
    use super::{BST, NodeKV};

    pub type BSTSetConst<T, const N: usize> = BST<ArrayVec<[T; N]>>;
    pub type BSTMapConst<'a, K, V, const N: usize> = BST<ArrayVec<[NodeKV<'a, K, V>; N]>>;
}

pub mod bstmap {
    pub use arborist_core::tree_kv::*;
    pub use arborist_core::fenwick::traits::*;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BSTMap;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BSTMap;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::BSTMapConst;
}

pub mod bstset {
    pub use arborist_core::tree::*;
    pub use arborist_core::fenwick::traits::*;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BSTSet;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BSTSet;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::BSTSetConst;
}

#[derive(Length)]
#[length_method(self.inner.length() - 1)]
pub struct BST<C: Length>  {
    pub(crate) inner: C
}

impl<C> BST<C> where C: InsertableCollection {
    pub fn new() -> Self {
        // InsertableCollection::new() ensures collections
        // always have at least 1 cap at compile time
        let mut inner: C = C::new();

        // Force collection ptr one forward
        inner.set_length(1);

        Self {
            inner: inner
        }
    }
}

impl<C> BST<C> where
    C: InsertableCollection,
    C::Output: PartialEq + PartialOrd + Sized
{
    pub(crate) fn allocate(&self, node: &C::Output) -> Result<usize, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        Ok(walker.allocate(node)?)
    }
}

impl<C> From<C> for BST<C> where C: Length {
    fn from(inner: C) -> Self {
        Self {
            inner: inner
        }
    }
}

impl<C> TreeRead<C::Output, BSTError> for BST<C> where
    C: IndexedCollection,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn get(&self, node: &C::Output) -> Result<&C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(node).map_err(|_| BSTError::KeyNotFound)?;
        Ok(&self.inner[index])
    }

    fn contains(&self, node: &C::Output) -> Result<bool, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        Ok(walker.find(node).is_ok())
    }
}

impl<C> TreeReadMut<C::Output, BSTError> for BST<C> where
    C: IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn get_mut(&mut self, node: &C::Output) -> Result<&mut C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(node).map_err(|_| BSTError::KeyNotFound)?;
        Ok(&mut self.inner[index])
    }
}

impl<C> TreeWrite<C::Output, BSTError> for BST<C> where
    C: InsertableCollection + IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn insert(&mut self, node: C::Output) -> Result<&C::Output, BSTError> {
        if self.length() == 0 {
            return self.push(node);
        }

        let index: usize = self.allocate(&node)?;

        self.inner.insert(index, node);
        Ok(&self.inner[index])
    }

    fn update(&mut self, node: C::Output) -> Result<&C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(&node)?;
        let current: &mut C::Output = &mut self.inner[index];
        *current = node;

        Ok(current)
    }

    fn delete(&mut self, node: &C::Output) -> Result<C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(&node)?;

        Ok(self.inner.remove(index))
    }

    fn push(&mut self, node: C::Output) -> Result<&C::Output, BSTError> {
        require!(self.inner.has_capacity(), BSTError::Inner(FenwickTreeError::Full));

        self.inner.insert(1, node);
        Ok(&self.inner[1])
    }

    fn pop(&mut self) -> Result<C::Output, BSTError> {
        require!(self.inner.length() > 1, BSTError::Inner(FenwickTreeError::Empty));

        Ok(self.inner.remove(self.inner.length() - 1))
    }
}

impl<C> TreeWriteMut<C::Output, BSTError> for BST<C> where
    C: InsertableCollection + IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn insert_mut(&mut self, node: C::Output) -> Result<&mut C::Output, BSTError> {
        if self.length() == 1 {
            return self.push_mut(node);
        }

        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.allocate(&node)?;

        self.inner.insert(index, node);
        Ok(&mut self.inner[index])
    }

    fn update_mut(&mut self, node: C::Output) -> Result<&mut C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(&node)?;
        let current: &mut C::Output = &mut self.inner[index];
        *current = node;

        Ok(current)
    }

    fn push_mut(&mut self, node: C::Output) -> Result<&mut C::Output, BSTError> {
        require!(self.inner.has_capacity(), BSTError::Inner(FenwickTreeError::Full));

        self.inner.insert(1, node);
        Ok(&mut self.inner[1])
    }
}

pub struct BSTWalker<'w, C: Length> {
    pub view: VirtualTreeView,
    inner: &'w C
}

impl<'w, C> BSTWalker<'w, C> where
    C: IndexedCollection,
    C::Output: Sized + PartialOrd
{
    pub fn new(inner: &'w C) -> Result<Self, BSTError> {
        Ok(Self {
            view: VirtualTreeView::new(inner, inner.height())?,
            inner: inner
        })
    }

    pub fn find(&mut self, key: &C::Output) -> Result<usize, BSTError> {
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
                _ => panic!("PartialCmp failed for BST node in get()")
            }
        }
    }

    // Finds the appropriate/corresponding array index for `key`
    pub fn allocate(&mut self, key: &C::Output) -> Result<usize, BSTError> {
        let mut node: usize = self.view.current()?;
        loop {
            match &self.inner[node].partial_cmp(key) {
                Some(Ordering::Less) => {
                    match self.view.traverse(Direction::Down(NodeSide::Left)) {
                        Ok(next) => node = next,
                        _ => break
                    }
                },
                Some(Ordering::Greater) => {
                    match self.view.traverse(Direction::Down(NodeSide::Left)) {
                        Ok(next) => node = next,
                        _ => break
                    }
                },
                Some(Ordering::Equal) => {
                    return Err(BSTError::DuplicateKey);
                },
                _ => panic!("PartialCmp failed for BST node in get()")
            }
        }

        Ok(node)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BSTError {
    KeyNotFound,
    DuplicateKey,
    Inner(FenwickTreeError)
}

impl From<FenwickTreeError> for BSTError {
    fn from(err: FenwickTreeError) -> Self {
        Self::Inner(err)
    }
}