#[macro_use]
pub mod macros;
pub mod errors;
pub mod traits;

#[cfg(test)]
mod tests;

use crate::{
    Tree, TreeWalker,
    NodeSide, NodeType
};
use core::ops::{
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    BitAnd, BitAndAssign,
    AddAssign, SubAssign,
};

pub use crate::fenwick::{errors::*, traits::*};

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
            Index View
################################*/
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct FenwickIndexView {
    pub index: usize,
    pub lsb: usize
}

impl FenwickIndexView {
    fn new(index: usize) -> Self {
        FenwickIndexView {
            index: index,
            lsb: lsb(index)
        }
    }
}

// Assignment operands update index/view lsb upon assignment
impl_op!{BitOr<usize>, FenwickIndexView, bitor, |, usize}
impl_op!{BitXor<usize>, FenwickIndexView, bitxor, ^, usize}
impl_op!{BitAnd<usize>, FenwickIndexView, bitand, &, usize}
impl_op_assign!{BitOrAssign<usize>, FenwickIndexView, bitor_assign, |=, usize}
impl_op_assign!{BitXorAssign<usize>, FenwickIndexView, bitxor_assign, ^=, usize}
impl_op_assign!{BitAndAssign<usize>, FenwickIndexView, bitand_assign, &=, usize}
impl_op_assign!{AddAssign<usize>, FenwickIndexView, add_assign, +=, usize}
impl_op_assign!{SubAssign<usize>, FenwickIndexView, sub_assign, -=, usize}

/*################################
            Tree View
################################*/

#[derive(Debug, Clone, PartialEq)]
pub struct FenwickTreeView<'tree, C: ?Sized> {
    tree: &'tree C,
    pub view: FenwickIndexView
}

impl<'tree, C> FenwickTreeView<'tree, C> where
    C: ?Sized + IndexedCollection + Tree<Value = C::Output> 
{
    pub fn new(tree: &'tree C, index: usize) -> Result<FenwickTreeView<C>, FenwickTreeError> {
        require!(index > 0, FenwickTreeError::ZeroIndex);

        Ok(Self {
            tree: tree,
            view: FenwickIndexView::new(index)
        })
    }
}

/*################################
            Tree Walker
################################*/

impl<'tree, C> TreeWalker<C> for FenwickTreeView<'tree, C> where
    C: ?Sized + IndexedCollection + Tree<Value = C::Output>,
{
    type Path = usize;

    // TODO: Investigate whether such may be performed more naturally via. cyclic codes
    fn up(&mut self) -> Option<&C::Output> {
        // Transition upward to next 'lsb namespace'
        match NodeSide::from(self.view.index) {
            NodeSide::Left => self.view += self.view.lsb,
            NodeSide::Right => self.view -= self.view.lsb
        }

        safe_tree_select!(self, self.view.index);
    }

    fn down(&mut self, side: NodeSide) -> Option<&C::Output> {
        match side {
            NodeSide::Left => self.view -= self.view.lsb >> 1,
            NodeSide::Right => self.view += self.view.lsb >> 1
        };

        safe_tree_select!(self, self.view.index);
    }

    fn seek(&mut self, path: usize) -> Option<&C::Output> {
        self.view ^= path;

        safe_tree_select!(self, self.view.index);
    }

    fn reset(&mut self) {
        self.view.index = self.tree.length();
        self.view.lsb = lsb(self.view.index);
    }

    fn current(&self) -> Option<&C::Output> {
        safe_tree_select!(self, self.view.index);
    }

    fn sibling(&self) -> Option<&C::Output> {
        safe_tree_select!(self, self.view.index ^ self.view.lsb << 1);
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
        match index >> index_lsb & 1 {
            0 => NodeSide::Left,
            1 => NodeSide::Right,
            _ => panic!("Invariant")
        }
    }
}

impl From<&FenwickIndexView> for NodeSide {
    fn from(view: &FenwickIndexView) -> Self {
        match view.index >> view.lsb & 1 {
            0 => NodeSide::Left,
            1 => NodeSide::Right,
            _ => panic!("Invariant")
        }
    }
}

impl From<usize> for NodeType {
    fn from(index: usize) -> Self {
        match index & 1 {
            0 => NodeType::Node,
            1 => NodeType::Leaf,
            _ => panic!("Invariant")
        }
    }
}

impl From<&FenwickIndexView> for NodeType {
    fn from(view: &FenwickIndexView) -> Self {
        match view.index & 1 {
            0 => NodeType::Node,
            1 => NodeType::Leaf,
            _ => panic!("Invariant")
        }
    }
}