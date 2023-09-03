#[macro_use]
pub mod macros;
pub mod errors;
pub mod traits;

#[cfg(test)]
mod tests;

use core::marker::PhantomData;
use core::ops::{
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    BitAnd, BitAndAssign,
    AddAssign, SubAssign,
    Add, Sub
};
use crate::{
    Height,  Direction,
    NodeSide, NodeType,
    TreeWalker, TreeWalkerMut,
    Tree, TreeMut, require
};
use arborist_proc::{
    Length, interpolate, length_method
};

pub use crate::fenwick::{errors::*, traits::*};

/*################################
            Functions
################################*/

#[cfg(any(feature = "no_float", feature = "fuzz"))]
pub(crate) mod compat {
    const USIZE_MIDPOINT: usize = (usize::BITS >> 1) as usize;

    #[inline(always)]
    pub(crate) fn log2_bin(length: &usize) -> usize {
        let mut mid: usize = USIZE_MIDPOINT;
        let mut cur: usize = USIZE_MIDPOINT;
    
        while mid > 1 {
            match length >> cur {
                1 => break,
                0 => cur -= { mid >>= 1; mid },
                _ => cur += { mid >>= 1; mid },
            }
        }
    
        cur + (&crate::fenwick::lsb(*length) != length) as usize
    }
}

#[inline(always)]
pub fn lsb(i: usize) -> usize {
    let _i: isize = i as isize;
    (_i & -_i) as usize
}

#[cfg(not(feature = "no_float"))]
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

#[cfg(feature = "no_float")]
impl<C> Height for C where C: Length + ?Sized {
    fn height(&self) -> usize {
        log2_bin(self.length())
    }
}

/*################################
            Index View
################################*/

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct IndexView {
    pub(crate) index: usize,
    pub(crate) lsb: usize
}

impl IndexView {
    fn new(index: usize) -> Self {
        IndexView {
            index: index,
            lsb: lsb(index)
        }
    }
}

/*################################
           Tree Walkers
################################*/

#[derive(Debug, Clone, PartialEq, Length)]
#[length_method("self.length")]
pub struct VirtualTreeView {
    length: usize,
    pub view: IndexView
}

#[derive(Debug, Clone, PartialEq, Length)]
#[length_method("self.inner.length()")]
pub struct StatefulTreeView<'a, C: ?Sized + Length> {
    inner: &'a C,
    pub view: IndexView
}

#[derive(Debug, PartialEq, Length)]
#[length_method("self.inner.length()")]
pub struct StatefulTreeViewMut<'a, C: ?Sized + Length> {
    inner: &'a mut C,
    pub view: IndexView
}

impl VirtualTreeView {
    pub fn new(inner: &impl Length, index: usize) -> Result<Self, FenwickTreeError> {
        let length: usize = inner.length();
        require!(index > 0 && index < length, FenwickTreeError::OutOfBounds);

        Ok(Self {
            length: length,
            view: IndexView::new(index)
        })
    }
}

impl<'a, C> StatefulTreeView<'a, C> where
    C: ?Sized + IndexedCollection,
    C::Output: Sized
{
    pub fn new(inner: &'a C, index: usize) -> Result<Self, FenwickTreeError> {
        require!(index > 0 && index < inner.length(), FenwickTreeError::OutOfBounds);

        Ok(Self {
            inner: inner,
            view: IndexView::new(index)
        })
    }
}

impl<'a, C> StatefulTreeViewMut<'a, C> where
    C: ?Sized + IndexedCollectionMut,
    C::Output: Sized
{
    pub fn new(inner: &'a mut C, index: usize) -> Result<Self, FenwickTreeError> {
        require!(index > 0 && index < inner.length(), FenwickTreeError::OutOfBounds);

        Ok(Self {
            inner: inner,
            view: IndexView::new(index)
        })
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

/*################################
           Macro Impls
################################*/

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

impl_walker!{
    type = VirtualTreeView,
    output = Result<usize, FenwickTreeError>,
    return_wrapper = Ok(#[ret])
}

impl_walker!{
    type = StatefulTreeView,
    output = Result<&'w C::Output, FenwickTreeError>,
    return_wrapper = Ok(&self.inner[#[ret]])
}

impl_walker!{
    type = StatefulTreeViewMut,
    output = Result<&'w C::Output, FenwickTreeError>,
    return_wrapper = Ok(&self.inner[#[ret]])
}

impl_walker!{
    @mut(
        type = StatefulTreeViewMut,
        output = Result<&'w mut C::Output, FenwickTreeError>,
        return_wrapper = Ok(&mut self.inner[#[ret]])
    )
}