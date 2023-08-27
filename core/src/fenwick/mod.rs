#[macro_use]
pub mod macros;
pub mod errors;
pub mod traits;

#[cfg(test)]
mod tests;

use crate::{
    Height,  Direction,
    NodeSide, NodeType,
    TreeWalker, TreeWalkerMut,
    require
};
use core::{
    marker::PhantomData,
    ops::{
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        BitAnd, BitAndAssign,
        AddAssign, SubAssign,
    }
};
use arborist_proc::interpolate_expr;

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

impl<C> Height for C where C: Length + ?Sized {
    #[cfg(target_pointer_width = "32")]
    fn height(&self) -> usize {
        (self.length() as f32).log(2.0).ceil() as usize
    }

    #[cfg(target_pointer_width = "64")]
    fn height(&self) -> usize {
        (self.length() as f64).log(2.0).ceil() as usize
    }
}

/*################################
            Index View
################################*/

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct IndexView {
    pub index: usize,
    pub lsb: usize
}

impl IndexView {
    fn new(index: usize) -> Self {
        IndexView {
            index: index,
            lsb: lsb(index)
        }
    }
}

// Assignment operands update index/view lsb upon assignment
impl_op!{BitOr<usize>, IndexView, bitor, |, usize}
impl_op!{BitXor<usize>, IndexView, bitxor, ^, usize}
impl_op!{BitAnd<usize>, IndexView, bitand, &, usize}
impl_op_assign!{BitOrAssign<usize>, IndexView, bitor_assign, |=, usize}
impl_op_assign!{BitXorAssign<usize>, IndexView, bitxor_assign, ^=, usize}
impl_op_assign!{BitAndAssign<usize>, IndexView, bitand_assign, &=, usize}
impl_op_assign!{AddAssign<usize>, IndexView, add_assign, +=, usize}
impl_op_assign!{SubAssign<usize>, IndexView, sub_assign, -=, usize}

/*################################
           Tree Walkers
################################*/

#[derive(Debug, Clone, PartialEq)]
pub struct FenwickTreeWalker<C, O> {
    inner: C,
    pub view: IndexView,
    marker: PhantomData<O>
}

pub type VirtualTreeWalker<'tree, C> = FenwickTreeWalker<&'tree C, usize>;
pub type StatefulTreeWalker<'tree, C> = FenwickTreeWalker<&'tree C, &'tree C>;
pub type StatefulTreeWalkerMut<'tree, C> = FenwickTreeWalker<&'tree mut C, &'tree mut C>;

impl<C: Length, O> FenwickTreeWalker<C, O> {
    fn new(inner: C, index: usize) -> Result<FenwickTreeWalker<C, O>, FenwickTreeError> {
        require!(index > 0, FenwickTreeError::OutOfBounds(index, inner.length()));

        Ok(Self {
            inner: inner,
            view: IndexView::new(index),
            marker: PhantomData
        })
    }
}

impl<'walker, 'tree, C> TreeWalker<'walker> for VirtualTreeWalker<'tree, C> where
    C: ?Sized + IndexedCollection,
    'tree: 'walker
{
    impl_walker!{
        output = usize,
        return_wrapper = safe_tree_select!(
            @virtual(
                self = self,
                item = $[ret]
            )
        )
    }
}

impl<'walker, 'tree, C> TreeWalker<'walker> for StatefulTreeWalker<'tree, C> where
    C: ?Sized + IndexedCollection,
    'tree: 'walker
{
    impl_walker!{
        output = &'walker C::Output,
        return_wrapper = safe_tree_select!(
            @stateful(
                self = self,
                index = $[ret],
                mutators = &
            )
        )
    }
}

impl<'walker, 'tree, C> TreeWalker<'walker> for StatefulTreeWalkerMut<'tree, C> where
    C: ?Sized + IndexedCollectionMut,
    'tree: 'walker
{
    impl_walker!{
        output = &'walker C::Output,
        return_wrapper = safe_tree_select!(
            @stateful(
                self = self,
                index = $[ret],
                mutators = &
            )
        )
    }
}

impl<'walker, 'tree, C> TreeWalkerMut<'walker> for StatefulTreeWalkerMut<'walker, C> where
    C: ?Sized + IndexedCollectionMut
{
    impl_walker!{
        @mut(
            output = &'walker mut C::Output,
            return_wrapper = safe_tree_select!(
                @stateful(
                    self = self,
                    index = $[ret],
                    mutators = &mut
                )
            )
        )
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

impl From<&IndexView> for NodeSide {
    fn from(view: &IndexView) -> Self {
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

impl From<&IndexView> for NodeType {
    fn from(view: &IndexView) -> Self {
        match view.index & 1 {
            0 => NodeType::Node,
            1 => NodeType::Leaf,
            _ => panic!("Invariant")
        }
    }
}