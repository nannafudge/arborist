use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    Length, InsertableCollection,
    IndexedCollection, IndexedCollectionMut,
    StatefulTreeView, FenwickTreeError,
    root
};
use arborist_core::{
    TreeRead, TreeReadMut, TreeWrite,
    TreeWalker, Direction, NodeSide,
    Height, require, unwrap_enum
};
use arborist_core::tree_kv::NodeKV;

use crate::bst::{BST, BSTWalker, BSTError, BSTWalkerResult};

use core::ops::{
    Deref, DerefMut
};

const DEFAULT_BALANCE_FACTOR: u8 = 8;

#[cfg(feature = "bumpalo_vec")]
pub mod bumpalo_vec {
    use bumpalo::collections::Vec;
    use super::{BTree, NodeKV};

    pub type BTreeSet<T> = BTree<Vec<T>>;
    pub type BTreeMap<'t, K, V> = BTree<Vec<NodeKV<'t, K, V>>>;
}

#[cfg(feature = "std_vec")]
pub mod std_vec {
    use std::vec::Vec;
    use super::{BTree, NodeKV};

    pub type BTreeSet<T> = BTree<Vec<T>>;
    pub type BTreeMap<'t, K, V> = BTree<Vec<NodeKV<'t, K, V>>>;
}

#[cfg(feature = "const_vec")]
pub mod const_vec {
    use tinyvec::ArrayVec;
    use super::{BTree, NodeKV, DEFAULT_BALANCE_FACTOR};

    pub type BTreeSetConst<T, const B: usize = {DEFAULT_BALANCE_FACTOR as usize}> = BTree<ArrayVec<[T; B]>>;
    pub type BTreeMapConst<'t, K, V, const B: usize = {DEFAULT_BALANCE_FACTOR as usize}> = BTree<ArrayVec<[NodeKV<'t, K, V>; B]>>;
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
pub struct BTree<C: InsertableCollection> {
    pub(crate) inner: C,
    balance_factor: u8,
    length: usize
}

impl<C, I> BTree<C> where
    C: InsertableCollection<Output = BST<I>>,
    C::Output: PartialOrd + Sized,
    I::Output: PartialOrd<C::Output> + Sized,
    BST<C>: From<C>
{
    pub fn new() -> Self {
        // BST Collections *always* allocate at least 1 slot
        Self {
            inner: BST::from(C::new()),
            balance_factor: DEFAULT_BALANCE_FACTOR,
            length: 0
        }
    }
}

impl<C, I> TreeRead for BTree<C> where
    C: InsertableCollection<Output = BST<I>>,
    C::Output: PartialOrd + Sized,
    I::Output: PartialOrd<C::Output> + Sized,
{
    type Node = I::Output;
    type Error = BTreeError;

    fn get(&self, node: &Self::Node) -> Result<&Self::Node, Self::Error> {
        todo!()
    }

    fn contains(&self, node: &Self::Node) -> Result<bool, Self::Error> {
        todo!()
    }

    fn first(&self) -> Result<&Self::Node, Self::Error> {
        todo!()
    }

    fn last(&self) -> Result<&Self::Node, Self::Error> {
        todo!()
    }

    fn root(&self) -> Result<&Self::Node, Self::Error> {
        todo!()
    }
}

impl<C, I> TreeReadMut for BTree<C> where
    C: InsertableCollection<Output = BST<I>>,
    C::Output: PartialOrd + Sized,
    I::Output: PartialOrd<C::Output> + Sized,
{
    fn get_mut(&mut self, node: &Self::Node) -> Result<&mut Self::Node, Self::Error> {
        todo!()
    }

    fn first_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        todo!()
    }

    fn last_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        todo!()
    }

    fn root_mut(&mut self) -> Result<&mut Self::Node, Self::Error> {
        todo!()
    }
}

impl<C, I> TreeWrite for BTree<C> where
    C: InsertableCollection<Output = BST<I>>,
    C::Output: PartialOrd + Sized,
    I::Output: PartialOrd<C::Output> + Sized,
{
    fn insert(&mut self, node: Self::Node) -> Result<Option<Self::Node>, Self::Error> {
        let subtree_index: usize = unwrap_enum!(
            self.inner.allocate(&node)?,
            BSTWalkerResult::Existing(index) => index,
            BSTWalkerResult::New(index) => {
                
                index
            }
        );

        let subtree: C = self.inner.inner_mut()

        self.inner[subtree_index].insert(&node);
        self.length += 1;

        todo!()
    }

    fn delete(&mut self, node: &Self::Node) -> Result<Self::Node, Self::Error> {
        todo!()
    }

    fn pop(&mut self) -> Result<Self::Node, Self::Error> {
        todo!()
    }
}

/*impl<C> PartialEq for BST<C> where
    C: InsertableCollection,
    C::Output: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        // length() should ALWAYS return 1 as all trees require at least 1 slot free
        self.inner[self.length() - 1] == other.inner[other.length() - 1]
    }
}*/

impl<I> PartialEq<I::Output> for BST<I> where
    I: InsertableCollection,
    I::Output: PartialEq
{
    fn eq(&self, other: &I::Output) -> bool {
        // length() should ALWAYS return 1 as all trees require at least 1 slot free
        other.eq(&self.inner[self.inner.length() - 1])
    }
}

impl<I> PartialOrd<I::Output> for BST<I> where
    I: InsertableCollection,
    I::Output: PartialOrd
{
    fn partial_cmp(&self, other: &I::Output) -> Option<std::cmp::Ordering> {
        // length() should ALWAYS return 1 as all trees require at least 1 slot free
        other.partial_cmp(&self.inner[self.inner.length() - 1])
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
                Ok(node) => {
                    match key.partial_cmp(node) {
                        Some(Ordering::Greater) => {
                            self.view.traverse(Direction::Down(NodeSide::Right));
                        },
                        Some(Ordering::Less) => {
                            self.view.traverse(Direction::Down(NodeSide::Left));
                        },
                        Some(Ordering::Equal) => {
                            return BSTWalkerResult::Existing(self.view.index());
                        },
                        None => panic!("Invariant: PartialCmp failed to return a value")
                    }
                }
            );
        }
        
        unwrap_enum!(
            self.view.current(),
            BSTWalkerResult::New(self.view.index()),
            Ok(node) => {
                match key.partial_cmp(node) {
                    Some(Ordering::Greater) => {
                        BSTWalkerResult::New(self.view.index() + 1)
                    },
                    Some(Ordering::Less) => {
                        BSTWalkerResult::New(self.view.index())
                    },
                    Some(Ordering::Equal) => {
                        BSTWalkerResult::Existing(self.view.index())
                    },
                    _ => panic!("Invariant: PartialCmp failed to return a value")
                }
            }
        )
    }

    pub fn find(&mut self, key: &impl PartialOrd<C::Output>) -> Result<usize, BSTError> {
        match self.allocate(key) {
            BSTWalkerResult::Existing(index) => Ok(index),
            BSTWalkerResult::New(_) => Err(BSTError::KeyNotFound)
        }
    }

    pub fn reset(&mut self) {
        self.view.seek(root(&self.view.height()))
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