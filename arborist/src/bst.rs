use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    InsertableCollection, IndexedCollection, IndexedCollectionMut,
    StatefulTreeView, FenwickTreeError, Length, root
};
use arborist_core::{
    TreeRead, TreeReadMut, TreeWrite,
    TreeWalker, Height, Direction, NodeSide,
    require, unwrap_enum
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
    use arborist_core::fenwick::IndexedCollection;

    use super::{BST, NodeKV};

    pub type BSTSet<T> = BST<Vec<T>>;
    pub type BSTMap<'t, K, V> = BST<Vec<NodeKV<'t, K, V>>>;

    impl<C> From<C> for BST<Vec<C::Output>> where
        C: IndexedCollection + Into<Vec<C::Output>> + ?Sized,
        C::Output: Sized
    {
        fn from(inner: C) -> Self {
            Self {
                inner: inner.into()
            }
        }
    }
}

#[cfg(feature = "const_vec")]
pub mod const_vec {
    use tinyvec::{Array, ArrayVec};
    use super::{
        BST, NodeKV,
        IndexedCollection
    };

    pub type BSTSetConst<T, const N: usize> = BST<ArrayVec<[T; N]>>;
    pub type BSTMapConst<'t, K, V, const N: usize> = BST<ArrayVec<[NodeKV<'t, K, V>; N]>>;

    impl<I: Array + IndexedCollection> From<I> for BST<ArrayVec<I>> {
        fn from(value: I) -> Self {
            Self { inner: ArrayVec::from(value) }
        }
    }
}

pub mod bstmap {
    pub use arborist_core::tree_kv::*;
    pub use arborist_core::fenwick::traits::*;
    pub use arborist_core::Height;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BSTMap;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BSTMap;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::*;
}

pub mod bstset {
    pub use arborist_core::tree::*;
    pub use arborist_core::fenwick::traits::*;
    pub use arborist_core::Height;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BSTSet;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BSTSet;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::*;
}

#[derive(Length, Clone, Copy)]
#[length_method(self.inner.length() - 1)]
pub struct BST<C: Length> {
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

    pub fn inner(&self) -> &C {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

impl<C> BST<C> where
    C: InsertableCollection,
    C::Output: PartialEq + PartialOrd + Sized
{
    pub(crate) fn allocate(&self, node: &impl PartialOrd<C::Output>) -> Result<BSTWalkerResult, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        Ok(walker.allocate(node))
    }
}

impl<C> TreeRead for BST<C> where
    C: IndexedCollection,
    C::Output: Sized + PartialEq + PartialOrd
{
    type Node = C::Output;
    type Error = BSTError;

    fn get(&self, node: &C::Output) -> Result<&C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(node)?;
        Ok(&self.inner[index])
    }

    fn first(&self) -> Result<&C::Output, BSTError> {
        require!(self.length() != 0, BSTError::EMPTY);

        Ok(&self.inner[1])
    }

    fn last(&self) -> Result<&C::Output, BSTError> {
        require!(self.length() != 0, BSTError::EMPTY);

        Ok(&self.inner[self.length()])
    }

    fn root(&self) -> Result<&C::Output, BSTError> {
        require!(self.length() != 0, BSTError::EMPTY);

        Ok(&self.inner[root(&self.inner.height())])
    }

    fn contains(&self, node: &C::Output) -> Result<bool, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        Ok(walker.find(node).is_ok())
    }
}

impl<C> TreeReadMut for BST<C> where
    C: IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn get_mut(&mut self, node: &C::Output) -> Result<&mut C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(node)?;
        Ok(&mut self.inner[index])
    }

    fn first_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        require!(self.length() != 0, BSTError::EMPTY);

        Ok(&mut self.inner[1])
    }

    fn last_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        require!(self.length() != 0, BSTError::EMPTY);
        let length: usize = self.length();

        Ok(&mut self.inner[length])
    }

    fn root_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        require!(self.length() != 0, BSTError::EMPTY);
        let height: usize = self.inner.height();

        Ok(&mut self.inner[root(&height)])
    }
}

impl<C> TreeWrite for BST<C> where
    C: InsertableCollection + IndexedCollectionMut,
    C::Output: Sized + PartialEq + PartialOrd
{
    fn insert(&mut self, node: C::Output) -> Result<Option<C::Output>, BSTError> {
        Ok(match self.allocate(&node)? {
            BSTWalkerResult::Existing(index) => {
                Some(core::mem::replace(&mut self.inner[index], node))
            },
            BSTWalkerResult::New(index, side) => {
                // Inserting always requires capacity - it always expands the array one to the right
                require!(self.inner.has_capacity(), BSTError::FULL);

                self.inner.insert(index + side as usize, node);
                None
            }
        })
    }

    fn delete(&mut self, node: &C::Output) -> Result<C::Output, BSTError> {
        let mut walker: BSTWalker<C> = BSTWalker::new(&self.inner)?;
        let index: usize = walker.find(node)?;

        Ok(self.inner.remove(index))
    }

    fn pop(&mut self) -> Result<C::Output, BSTError> {
        require!(self.length() != 0, BSTError::EMPTY);

        Ok(self.inner.remove(self.length()))
    }
}

#[derive(Debug, Length)]
#[length_method(self.view.length())]
pub struct BSTWalker<'w, C: IndexedCollection> {
    pub view: StatefulTreeView<'w, C>
}

impl<'w, C> BSTWalker<'w, C> where
    C: IndexedCollection,
    C::Output: PartialOrd + Sized
{
    pub fn new(inner: &'w C) -> Result<Self, BSTError> {
        // Start at centermost point of tree
        let start_index: usize = root(&inner.height());

        Ok(Self {
            view: StatefulTreeView::new(inner, start_index)?
        })
    }

    pub fn allocate(&mut self, key: &impl PartialOrd<C::Output>) -> BSTWalkerResult {
        while self.view.lsb() > 1 {
            unwrap_enum!(
                self.view.current(),
                self.view.traverse(Direction::Down(NodeSide::Left)),
                Ok(node) => unwrap_enum!(
                    key.partial_cmp(node),
                    panic!("Invariant: PartialCmp failed to return a value"),
                    Some(Ordering::Greater) => self.view.traverse(Direction::Down(NodeSide::Right)),
                    Some(Ordering::Less) => self.view.traverse(Direction::Down(NodeSide::Left)),
                    Some(Ordering::Equal) => return BSTWalkerResult::Existing(self.view.index())
                )
            );
        }

        unwrap_enum!(
            self.view.current(),
            BSTWalkerResult::New(self.view.index(), NodeSide::Left),
            Ok(node) => {
                unwrap_enum!(
                    key.partial_cmp(node),
                    panic!("Invariant: PartialCmp failed to return a value"),
                    Some(Ordering::Greater) => BSTWalkerResult::New(self.view.index(), NodeSide::Right),
                    Some(Ordering::Less) => BSTWalkerResult::New(self.view.index(), NodeSide::Left),
                    Some(Ordering::Equal) => BSTWalkerResult::Existing(self.view.index())
                )
            }
        )
    }

    pub fn find(&mut self, key: &impl PartialOrd<C::Output>) -> Result<usize, BSTError> {
        match self.allocate(key) {
            BSTWalkerResult::Existing(index) => Ok(index),
            _ => Err(BSTError::KeyNotFound)
        }
    }

    pub fn reset(&mut self) {
        self.view.seek(root(&self.view.height()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BSTWalkerResult {
    New(usize, NodeSide),
    Existing(usize)
}

impl From<BSTWalkerResult> for usize {
    fn from(value: BSTWalkerResult) -> usize {
        // (Discriminant(0|1), NodeSide(0|1), index)
        let raw: &(u8, u8, usize) = unsafe {
            core::mem::transmute::<&BSTWalkerResult, &(u8, u8, usize)>(&value)
        };
        
        // Round the index up/down appropriately, according to the side the match
        // fell on (Right/Left). Select for such using bitmask based on the Discriminant:
        // New = 0, Existing = 1. Existing = ignore value as it'll be garbage
        // (whatever's read from memory at that location)
        (raw.1 & (1 ^ raw.0)) as usize + raw.2
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BSTError {
    KeyNotFound,
    Inner(FenwickTreeError)
}

impl BSTError {
    pub const EMPTY: Self = BSTError::Inner(FenwickTreeError::Empty);
    pub const FULL: Self = BSTError::Inner(FenwickTreeError::Full);
}

impl From<FenwickTreeError> for BSTError {
    fn from(err: FenwickTreeError) -> Self {
        Self::Inner(err)
    }
}