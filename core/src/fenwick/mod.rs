#[macro_use]
pub mod macros;
pub mod errors;

// Test trait impls (otherwise E0117)
#[cfg(test)]
pub mod test;

use arborist_proc::{impl_length, impl_trait};

pub use crate::fenwick::errors::*;
use crate::{
    const_time_select,
    Tree, TreeWalker,
    Node, NodeSide, NodeType, unwrap_enum
};

use core::ops::{
    Index, IndexMut,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    BitAnd, BitAndAssign
};
use std::process::Output;

use generic_array::GenericArray;

/*################################
           Functions
################################*/

// Casting here should induce no overhead due to size equivalence
#[inline(always)]
pub fn lsb(i: usize) -> usize {
    let _i: isize = i as isize;
    (_i & -_i) as usize
}

/*################################
             Traits
################################*/

// Define as `length()` to avoid the fn
// sig clashing with normal len() impls
pub trait Length {
    fn length(&self) -> usize;
}

pub trait IndexedCollection: Index<usize> + Length {}
pub trait IndexedCollectionMut: IndexMut<usize> + IndexedCollection {}

impl<C> IndexedCollection for C where C: Index<usize> + Length {}
impl<C> IndexedCollectionMut for C where C: IndexMut<usize> + IndexedCollection {}

impl_length!(<T>, &[T]);
impl_length!(<T>, &mut [T]);
impl_length!(<T>, Vec<T>);

/*################################
          Concrete Types
################################*/
pub enum FenwickIndexView {
    Valid{index: usize, lsb: usize},
    Invalid(usize)
}

impl FenwickIndexView {
    fn new(index: usize, tree_len: usize) -> Self {
        if index == 0 || index > tree_len {
            FenwickIndexView::Invalid(index)
        } else {
            FenwickIndexView::Valid { index: index, lsb: lsb(index) }
        }
    }
}

impl_trait!(<R: BitOr<usize, Output = usize>>, BitOr<R>, FenwickIndexView, impl_bitwise!{bitor, |, R});
impl_trait!(<R: BitOr<usize, Output = usize>>, BitOrAssign<R>, FenwickIndexView, impl_bitwise_assign!{bitor_assign, |, R});

impl_trait!(<R: BitXor<usize, Output = usize>>, BitXor<R>, FenwickIndexView, impl_bitwise!{bitxor, ^, R});
impl_trait!(<R: BitXor<usize, Output = usize>>, BitXorAssign<R>, FenwickIndexView, impl_bitwise_assign!{bitxor_assign, ^, R});

impl_trait!(<R: BitAnd<usize, Output = usize>>, BitAnd<R>, FenwickIndexView, impl_bitwise!{bitand, &, R});
impl_trait!(<R: BitAnd<usize, Output = usize>>, BitAndAssign<R>, FenwickIndexView, impl_bitwise_assign!{bitand_assign, &, R});

pub struct FenwickTreeView<'tree, C> {
    tree: &'tree C,
    view: FenwickIndexView
}

impl<'tree, C> FenwickTreeView<'tree, C> where
    C: IndexedCollection + Tree<Value = C::Output>
{
    fn new(tree: &'tree C, index: usize) -> FenwickTreeView<C> {
        Self {
            tree: tree,
            view: FenwickIndexView::new(index, tree.length())
        }
    }
}

impl<'tree, C> From<&C> for FenwickTreeView<'tree, C> where
    C: IndexedCollection + Tree<Value = C::Output>
{
    fn from(tree: &C) -> Self {
        FenwickTreeView::new(tree, tree.length())
    }
}

/*################################
            Tree Impl
################################*/

impl<'tree, C> TreeWalker<C> for FenwickTreeView<'tree, C> where
    C: IndexedCollection + Tree<Value = C::Output>
{
    type Path = usize;

    fn up(&mut self) -> &C::Output {
        // Transition upward to next 'lsb namespace'
        self.view ^= self.curr ^ lsb(self.curr) << 1;
        // Too large an index will return TreeNode::Null
        &self.inner[self.curr]
    }

    fn down(&mut self, side: NodeSide) -> &C::Output {
        let lsb: usize = lsb(self.curr) >> 1;
        self.curr = match side {
            NodeSide::Left => lsb - lsb,
            NodeSide::Right => lsb + lsb,
            _ => 0
        };

        // 0 will return TreeNode::Null
        &self.inner[self.curr]
    }

    fn seek(&mut self, path: usize) -> &C::Output {
        self.curr ^= path;

        // Out of bounds index will return TreeNode::Null
        &self.inner[self.curr]
    }

    fn reset(&mut self) {
        self.curr = self.inner.length()
    }

    fn sibling(&self) -> &C::Output {
        &self.inner[self.curr ^ lsb(self.curr) << 1]
    }

    fn type_(&self) -> NodeType {
        NodeType::from(self.curr)
    }

    fn side(&self) -> NodeSide {
        NodeSide::from(self.curr)
    }
}

/*################################
            Node Impl
################################*/

impl From<usize> for NodeSide {
    fn from(index: usize) -> Self {
        let index_lsb: usize = lsb(index);
        unsafe {
            *const_time_select(
                &NodeSide::Left,
                &NodeSide::Right,
                // Bit n+1 determines the side of the node, create
                // a bitmask from the LSB that fetches such (i.e. 11(0<n>))
                index & (index_lsb | index_lsb << 1)
            )
        }
    }
}

impl From<usize> for NodeType {
    fn from(index: usize) -> Self {
        unsafe {
            *const_time_select(
                &NodeType::Leaf,
                &NodeType::Node,
                index & 1
            )
        }
    }
}

/*

impl<'a, T, C: FenwickCollection<T>> FenwickTreeWalker<'a, T, C> {
    fn new(tree: &'a FenwickTree<T, C>, index: usize) -> Result<Self, FenwickTreeError> {
        let length: usize = tree.length();
        if index < 0 || index > length {
            return Err(FenwickTreeError::OutOfBounds(index, length));
        }

        Ok(Self {
            tree: tree,
            index: index,
            typ: NodeType::from(index),
            side: Side::from(index)
        })
    }
}*/

/*impl<'a, T: IndexedCollection> TreeView for FenwickTreeWalker<'a, FenwickTree<T>> {
    fn up(&mut self, levels: usize) -> &Self {
        let index: Side = inner_enum!(&self.curr, &Side, NodeType::Leaf, NodeType::Node);

        self
    }

    fn down(&mut self, levels: usize) -> &Self {
        todo!()
    }

    fn sibling(&mut self) -> &Self {
        todo!()
    }

    fn seek<FenwickTreePath>(&mut self, path: FenwickTreePath) {
        todo!()
    }
}*/