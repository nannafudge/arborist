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
        Add, Sub
    }
};
use arborist_proc::interpolate_expr;

pub use crate::fenwick::{errors::*, traits::*};

/*################################
            Functions
################################*/

#[inline(always)]
pub fn lsb(i: usize) -> usize {
    let _i: isize = i as isize;
    (_i & -_i) as usize
}

impl<C> Height for C where C: Length + ?Sized {
    #[cfg(not(target_pointer_width = "64"))]
    fn height(&self) -> usize {
        (self.length() as f32).log2().ceil() as usize
    }

    #[cfg(target_pointer_width = "64")]
    fn height(&self) -> usize {
        (self.length() as f64).log2().ceil() as usize
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
impl_op!{BitOr<usize>, bitor, |, usize}
impl_op!{BitXor<usize>, bitxor, ^, usize}
impl_op!{BitAnd<usize>, bitand, &, usize}
impl_op!{Add<usize>, add, +, usize}
impl_op!{Sub<usize>, sub, -, usize}
impl_op_assign!{BitOrAssign<usize>, bitor_assign, |=, usize}
impl_op_assign!{BitXorAssign<usize>, bitxor_assign, ^=, usize}
impl_op_assign!{BitAndAssign<usize>, bitand_assign, &=, usize}
impl_op_assign!{AddAssign<usize>, add_assign, +=, usize}
impl_op_assign!{SubAssign<usize>, sub_assign, -=, usize}

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

impl_walker!{
    type = VirtualTreeWalker,
    output = usize,
    return_wrapper = safe_tree_select!(
        @virtual(
            self = self,
            item = $[ret]
        )
    )
}

impl_walker!{
    type = StatefulTreeWalker,
    output = &'walker C::Output,
    return_wrapper = safe_tree_select!(
        @stateful(
            self = self,
            index = $[ret],
            mutators = &
        )
    )
}

impl_walker!{
    type = StatefulTreeWalkerMut,
    output = &'walker C::Output,
    return_wrapper = safe_tree_select!(
        @stateful(
            self = self,
            index = $[ret],
            mutators = &
        )
    )
}

impl_walker!{
    @mut(
        type = StatefulTreeWalkerMut,
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