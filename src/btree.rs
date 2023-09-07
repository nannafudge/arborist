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

use crate::bst::{BST, BSTWalker, BSTError};

use core::ops::{
    Deref, DerefMut
};

const DEFAULT_BALANCE_FACTOR: u8 = 8;

#[cfg(feature = "bumpalo_vec")]
pub mod bumpalo_vec {
    use bumpalo::collections::Vec;
    use super::{BTree, NodeKV};

    pub type BTreeSet<T> = BTree<Vec<T>>;
    pub type BTreeMap<'a, K, V> = BTree<Vec<NodeKV<'a, K, V>>>;
}

#[cfg(feature = "std_vec")]
pub mod std_vec {
    use std::vec::Vec;
    use super::{BTree, NodeKV};

    pub type BTreeSet<T> = BTree<Vec<T>>;
    pub type BTreeMap<'a, K, V> = BTree<Vec<NodeKV<'a, K, V>>>;
}

#[cfg(feature = "const_vec")]
pub mod const_vec {
    use tinyvec::ArrayVec;
    use super::{BTree, NodeKV, DEFAULT_BALANCE_FACTOR};

    const DEFAULT_BALANCE_FACTOR_L: usize = DEFAULT_BALANCE_FACTOR as usize;

    pub type BTreeSetConst<T, const B: usize = DEFAULT_BALANCE_FACTOR_L> = BTree<ArrayVec<[T; B]>>;
    pub type BTreeMapConst<'a, K, V, const B: usize = DEFAULT_BALANCE_FACTOR_L> = BTree<ArrayVec<[NodeKV<'a, K, V>; B]>>;
}

pub mod btreemap {
    pub use arborist_core::tree_kv::*;
    pub use arborist_core::fenwick::traits::*;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BTreeMap;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BTreeMap;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::BTreeMapConst;
}

pub mod btreeset {
    pub use arborist_core::tree::*;
    pub use arborist_core::fenwick::traits::*;

    #[cfg(all(feature = "bumpalo_vec", not(feature = "std_vec")))]
    pub use super::bumpalo_vec::BTreeSet;
    
    #[cfg(all(feature = "std_vec", not(feature = "bumpalo_vec")))]
    pub use super::std_vec::BTreeSet;
    
    #[cfg(feature = "const_vec")]
    pub use super::const_vec::BTreeSetConst;
}

#[derive(Length)]
#[length_method(self.length)]
pub struct BTree<C: Length> {
    pub(crate) inner: BST<BST<C>>,
    balance_factor: u8,
    length: usize
}

impl<C> BTree<C> where
    C: InsertableCollection
{
    pub fn new() -> Self {
        // BST Collections *always* allocate at least 1 slot
        Self {
            inner: BST::<BST<C>>::from(BST::<C>::new()),
            balance_factor: DEFAULT_BALANCE_FACTOR,
            length: 0
        }
    }
}

impl<C> TreeRead<C::Output, BTreeError> for BTree<C> where
    C: InsertableCollection,
    C::Output: PartialEq + PartialOrd + Sized
{
    fn get(&self, node: &C::Output) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn contains(&self, node: &C::Output) -> Result<bool, BTreeError> {
        todo!()
    }
}

impl<C> TreeReadMut<C::Output, BTreeError> for BTree<C> where
    C: InsertableCollection,
    C::Output: PartialEq + PartialOrd + Sized
{
    fn get_mut(&mut self, node: &C::Output) -> Result<&mut C::Output, BTreeError> {
        todo!()
    }
}

impl<C> TreeWrite<C::Output, BTreeError> for BTree<C> where
    C: InsertableCollection,
    C::Output: PartialEq + PartialOrd + Sized
{
    fn insert(&mut self, node: C::Output) -> Result<&C::Output, BTreeError> {
        if self.length == 0 {
            return self.push(node);
        }

        let index: usize = self.inner.allocate(&node)?;
        //self.inner[index]
        self.length += 1;

        todo!()
    }

    fn update(&mut self, node: C::Output) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn delete(&mut self, node: &C::Output) -> Result<C::Output, BTreeError> {
        todo!()
    }

    fn push(&mut self, node: C::Output) -> Result<&C::Output, BTreeError> {
        todo!()
    }

    fn pop(&mut self) -> Result<C::Output, BTreeError> {
        todo!()
    }
}

impl<C> PartialEq for BST<C> where
    C: InsertableCollection,
    C::Output: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        // length() should ALWAYS return 1 as all trees require at least 1 slot free
        self.inner[self.length() - 1] == other.inner[other.length() - 1]
    }
}

impl<C> PartialOrd for BST<C> where
    C: InsertableCollection,
    C::Output: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // length() should ALWAYS return 1 as all trees require at least 1 slot free
        self.inner[self.length() - 1].partial_cmp(&other.inner[other.length() - 1])
    }
}

impl<C> Deref for BST<C> where C: Length {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &(self.inner)
    }
}

impl<C> DerefMut for BST<C> where C: Length {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut(self.inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BTreeError {
    Inner(BSTError)
}

impl From<BSTError> for BTreeError {
    fn from(err: BSTError) -> Self {
        Self::Inner(err)
    }
}

impl From<FenwickTreeError> for BTreeError {
    fn from(err: FenwickTreeError) -> Self {
        Self::Inner(BSTError::Inner(err))
    }
}