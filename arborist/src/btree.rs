use arborist_proc::{Length, length_method};
use arborist_core::fenwick::{
    Length, InsertableCollection,
    IndexedCollection, IndexedCollectionMut,
    StatefulTreeView, FenwickTreeError,
    root, StatefulTreeViewMut
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

const MAX_ELEMENTS: usize = 16;

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
    use super::{BTree, NodeKV, MAX_ELEMENTS};

    pub type BTreeSetConst<T> = BTree<ArrayVec<[T; MAX_ELEMENTS]>>;
    pub type BTreeMapConst<'t, K, V> = BTree<ArrayVec<[NodeKV<'t, K, V>; MAX_ELEMENTS]>>;
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

pub enum BTreeNode<'t, T> {
    Layer(&'t BTreeNode<'t, T>),
    Node(T)
}

#[derive(Length)]
#[length_method(self.length)]
pub struct BTree<C: InsertableCollection> {
    pub(crate) root_tree: BST<C>,
    length: usize
}

impl<'t, C, I> BTree<C> where
    C: InsertableCollection<Output = BTreeNode<'t, BST<I>>>,
    C::Output: PartialOrd + Sized,
    I: InsertableCollection + 't,
    I::Output: Sized,
    BST<C>: From<C>
{
    pub fn new() -> Self {
        // BST Collections *always* allocate at least 1 slot
        Self {
            root_tree: BST::from(C::new()),
            length: 0
        }
    }
}

impl<'t, C, I> TreeRead for BTree<C> where
    C: InsertableCollection<Output = BTreeNode<'t, BST<I>>>,
    C::Output: PartialOrd + Sized,
    I: InsertableCollection + 't,
    I::Output: PartialOrd<C::Output> + Sized
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

impl<'t, C, I> TreeReadMut for BTree<C> where
    C: InsertableCollection<Output = BTreeNode<'t, BST<I>>>,
    C::Output: PartialOrd + Sized,
    I: InsertableCollection + 't,
    I::Output: PartialOrd<C::Output> + Sized
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

impl<'t, C, I> TreeWrite for BTree<C> where
    C: InsertableCollection<Output = BTreeNode<'t, BST<I>>>,
    C::Output: PartialOrd + Sized,
    I: InsertableCollection + 't,
    I::Output: PartialOrd<C::Output> + Sized
{
    fn insert(&mut self, node: Self::Node) -> Result<Option<Self::Node>, Self::Error> {
        let mut subtree_index: usize = usize::from(self.root_tree.allocate(&node)?).max(self.length());

        //let mut subtree: &mut BST<I> = &mut self.inner.inner_mut()[subtree_index];
        //if subtree.length() < MAX_ELEMENTS {
            //return Ok(BST::<I>::insert(subtree, node)?);
        //}

        //let new_right_subtree: I = subtree.inner_mut().split_off(subtree.height());
        //self.inner.inner_mut().insert(subtree_index + 1, new_right_subtree);
        //self.length += 1;

        //self.rebalance(subtree_index, subtree_index + 1)?;
        Ok(None)
    }

    fn delete(&mut self, node: &Self::Node) -> Result<Self::Node, Self::Error> {
        todo!()
    }

    fn pop(&mut self) -> Result<Self::Node, Self::Error> {
        todo!()
    }
}

pub type BTreeWalker<'w, C> = BSTWalker<'w, C>;

impl<'w, C> BTreeWalker<'w, C> where
    C: IndexedCollection,
    C::Output: PartialOrd + Sized
{
    
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
        self.length() == 0 || other.eq(&self.inner[self.inner.length()])
    }
}

impl<I> PartialOrd<I::Output> for BST<I> where
    I: InsertableCollection,
    I::Output: PartialOrd
{
    fn partial_cmp(&self, other: &I::Output) -> Option<std::cmp::Ordering> {
        let length: usize = self.length();
        if length == 0 {
            return None;
        }

        other.partial_cmp(&self.inner[length])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BTreeError {
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