#[macro_use]
pub mod macros;
pub mod errors;

// Test trait impls (otherwise E0117)
#[cfg(test)]
pub mod test;

use arborist_proc::impl_length;

pub use crate::fenwick::errors::*;
use crate::{
    Tree, TreeWalker,
    Node, NodeSide, NodeType,
    unwrap_enum, const_time_select, const_time_select_mut
};

use core::ops::{
    Index, IndexMut,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    BitAnd, BitAndAssign,
    Add, AddAssign,
    Sub, SubAssign,
};

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
            Index View
################################*/
pub struct FenwickIndexView {
    index: usize,
    lsb: usize
}

impl FenwickIndexView {
    fn new(index: usize) -> Self {
        FenwickIndexView {
            index: index,
            lsb: lsb(index)
        }
    }

    fn split(&self) -> (&usize, &usize) {
        (&self.index, &self.lsb)
    }

    fn split_mut(&mut self) -> (&mut usize, &mut usize) {
        (&mut self.index, &mut self.lsb)
    }
}

impl<R> BitOr<R> for FenwickIndexView where usize: BitOr<R> {
    impl_op!{bitor, |, R}
}
impl<R> BitOrAssign<R> for FenwickIndexView where usize: BitOr<R> {
    impl_op_assign!{bitor_assign, |, R}
}
impl<R> BitXor<R> for FenwickIndexView where usize: BitXor<R> {
    impl_op!{bitxor, ^, R}
}
impl<R> BitXorAssign<R> for FenwickIndexView where usize: BitXor<R> {
    impl_op_assign!{bitxor_assign, ^, R}
}
impl<R> BitAnd<R> for FenwickIndexView where usize: BitAnd<R> {
    impl_op!{bitand, &, R}
}
impl<R> BitAndAssign<R> for FenwickIndexView where usize: BitAnd<R> {
    impl_op_assign!{bitand_assign, &, R}
}
impl<R> AddAssign<R> for FenwickIndexView where usize: Add<R> {
    impl_op_assign!{add_assign, +, R}
}
impl<R> SubAssign<R> for FenwickIndexView where usize: Sub<R> {
    impl_op_assign!{sub_assign, -, R}
}

/*################################
            Tree View
################################*/

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
            view: FenwickIndexView::new(index)
        }
    }
}

/*################################
            Tree Walker
################################*/

impl<'tree, C> TreeWalker<C> for FenwickTreeView<'tree, C> where
    C: IndexedCollection + Tree<Value = C::Output, Error = FenwickTreeError>
{
    type Path = usize;

    // TODO: Investigate whether such may be performed more naturally via. cyclic codes
    fn up(&mut self) -> Result<&C::Output, FenwickTreeError> {
        // Transition upward to next 'lsb namespace'
        match NodeSide::from(self.view.index) {
            NodeSide::Left => self.view += self.view.lsb,
            NodeSide::Right => self.view -= self.view.lsb,
            NodeSide::Null => {
                return Err(FenwickTreeError::InvalidNodeSide(NodeSide::Null));
            }
        }

        safe_tree_select!(self, self.view.index)
    }

    fn down(&mut self, side: NodeSide) -> Result<&C::Output, FenwickTreeError> {
        match side {
            NodeSide::Left => self.view -= self.view.lsb >> 1,
            NodeSide::Right => self.view += self.view.lsb >> 1,
            _ => {
                return Err(FenwickTreeError::InvalidNodeSide(side));
            }
        };

        safe_tree_select!(self, self.view.index)
    }

    fn seek(&mut self, path: usize) -> Result<&C::Output, FenwickTreeError> {
        self.view.index ^= path;

        safe_tree_select!(self, self.view.index)
    }

    fn reset(&mut self) {
        self.view.index = self.tree.length();
        self.view.lsb = lsb(self.view.index);
    }

    fn sibling(&self) -> Result<&C::Output, FenwickTreeError> {
        safe_tree_select!(self, self.view.index ^ self.view.lsb << 1)
    }

    fn type_(&self) -> NodeType {
        NodeType::from(&self.view)
    }

    fn side(&self) -> NodeSide {
        NodeSide::from(&self.view)
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

impl From<&FenwickIndexView> for NodeSide {
    fn from(view: &FenwickIndexView) -> Self {
        unsafe {
            *const_time_select(
                &NodeSide::Left,
                &NodeSide::Right,
                view.index & (view.lsb | view.lsb << 1)
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

impl From<&FenwickIndexView> for NodeType {
    fn from(view: &FenwickIndexView) -> Self {
        unsafe {
            *const_time_select(
                &NodeType::Leaf,
                &NodeType::Node,
                view.index & 1
            )
        }
    }
}