pub mod errors;
pub mod traits;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

use core::ops::{
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    BitAnd, BitAndAssign,
    AddAssign, SubAssign,
    Add, Sub
};
use crate::{
    NodeSide, NodeType, Direction,
    TreeWalker, TreeWalkerMut,
    require
};
use arborist_proc::{
    Length, interpolate, length_method
};

pub use self::{errors::*, traits::*};

/*################################
            Functions
################################*/

#[cfg(any(feature = "no_float", feature = "proptest"))]
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

impl PartialEq<usize> for IndexView {
    fn eq(&self, other: &usize) -> bool {
        &self.index == other
    }
}

/*################################
           Tree Walkers
################################*/

#[derive(Debug, Clone, PartialEq, Length)]
#[length_method(self.length)]
pub struct VirtualTreeView {
    length: usize,
    pub inner: IndexView
}

#[derive(Debug, Clone, PartialEq, Length)]
#[length_method(self.collection.length())]
pub struct StatefulTreeView<'a, C: ?Sized + Length> {
    collection: &'a C,
    pub inner: IndexView
}

#[derive(Debug, PartialEq, Length)]
#[length_method(self.collection.length())]
pub struct StatefulTreeViewMut<'a, C: ?Sized + Length> {
    collection: &'a mut C,
    pub inner: IndexView
}

impl VirtualTreeView {
    pub fn new(collection: &impl Length, index: usize) -> Result<Self, FenwickTreeError> {
        let length: usize = collection.length();
        require!(
            index > 0 && index < length,
            FenwickTreeError::OutOfBoundsFor{ index: index, length: length }
        );

        Ok(Self {
            length,
            inner: IndexView::new(index)
        })
    }
}

impl<'a, C> StatefulTreeView<'a, C> where
    C: ?Sized + IndexedCollection,
    C::Output: Sized
{
    pub fn new(collection: &'a C, index: usize) -> Result<Self, FenwickTreeError> {
        require!(
            index > 0 && index < collection.length(),
            FenwickTreeError::OutOfBoundsFor{ index: index, length: collection.length() }
        );

        Ok(Self {
            collection,
            inner: IndexView::new(index)
        })
    }
}

impl<'a, C> StatefulTreeViewMut<'a, C> where
    C: ?Sized + IndexedCollectionMut,
    C::Output: Sized
{
    pub fn new(collection: &'a mut C, index: usize) -> Result<Self, FenwickTreeError> {
        require!(
            index > 0 && index < collection.length(),
            FenwickTreeError::OutOfBoundsFor { index: index, length: collection.length() }
        );

        Ok(Self {
            collection,
            inner: IndexView::new(index)
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
    return_wrapper = Ok(&self.collection[#[ret]])
}

impl_walker!{
    type = StatefulTreeViewMut,
    output = Result<&'w C::Output, FenwickTreeError>,
    return_wrapper = Ok(&self.collection[#[ret]])
}

impl_walker!{
    @mut(
        type = StatefulTreeViewMut,
        output = Result<&'w mut C::Output, FenwickTreeError>,
        return_wrapper = Ok(&mut self.collection[#[ret]])
    )
}