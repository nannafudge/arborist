use core::ops::{Index, IndexMut};
use arborist_proc::{
    impl_insertable_collection,
    impl_length
};

pub use crate::tree::Height;

#[cfg(any(feature = "no_float", test))]
pub(crate) mod compat {
    const USIZE_MIDPOINT: usize = (usize::BITS >> 1) as usize;

    pub fn height(length: usize) -> usize {
        let mut mid: usize = USIZE_MIDPOINT;
        let mut cur: usize = USIZE_MIDPOINT;
    
        while mid > 1 {
            match length >> cur {
                1 => break,
                0 => cur -= { mid >>= 1; mid },
                _ => cur += { mid >>= 1; mid },
            }
        }
    
        cur
    }
}

#[cfg(feature = "no_float")]
pub use compat::*;

#[cfg(all(not(feature = "no_float"), not(target_pointer_width = "64")))]
#[inline(always)]
pub fn height(length: usize) -> usize {
    (length as f32).log2().floor() as usize
}

#[cfg(all(not(feature = "no_float"), target_pointer_width = "64"))]
#[inline(always)]
pub fn height(length: usize) -> usize {
    (length as f64).log2().floor() as usize
}

pub trait IndexedCollection: Index<usize> + Length {}
pub trait IndexedCollectionMut: IndexMut<usize> + IndexedCollection {}

pub trait InsertableCollection: IndexedCollection {
    fn new() -> Self;
    fn insert(&mut self, index: usize, item: Self::Output);
    fn remove(&mut self, index: usize) -> Self::Output;
    fn set_length(&mut self, length: usize);
    fn has_capacity(&self) -> bool;
}

// Define as `length()` to avoid the fn
// sig clashing with normal len() impls
pub trait Length {
    fn length(&self) -> usize;
}

#[cfg(feature = "const_vec")]
mod const_vec {
    use tinyvec::{Array, ArrayVec, SliceVec};
    use super::{
        InsertableCollection, IndexedCollection, Length,
        impl_length, impl_insertable_collection
    };

    impl_length!(<C: Array + IndexedCollection> ArrayVec<C>);
    impl_length!(<'a, C: Array + IndexedCollection> SliceVec<'a, C>);
    impl_insertable_collection!(<C: Array + IndexedCollection> ArrayVec<C>);
    impl_insertable_collection!(<'a, C: Array + IndexedCollection + Default> SliceVec<'a, C>);
}

#[cfg(feature = "bumpalo_vec")]
mod bumpalo_vec {
    use bumpalo::collections::Vec;
    use super::{
        InsertableCollection, Length,
        impl_length
    };

    impl_length!(<T> Vec<T>);
    impl_length!(<T> &Vec<T>);
    impl_length!(<T> &mut Vec<T>);

    // TODO: MACRO
    impl<T> InsertableCollection for Vec<T> {
        fn new() -> Self {
            Vec::with_capacity(1)
        }

        fn insert(&mut self, index: usize, item: Self::Output) {
            Vec::insert(self, index, item)
        }
    
        fn remove(&mut self, index: usize) -> Self::Output {
            Vec::remove(self, index)
        }

        fn set_length(&mut self, length: usize) {
            unsafe { Vec::set_len(self, length) }
        }

        // Ideally Vec types should *always* have capacity
        fn has_capacity(&self) -> bool {
            true
        }
    }
}

#[cfg(feature = "std_vec")]
mod std_vec {
    use std::vec::Vec;
    use super::{
        InsertableCollection, Length,
        impl_length
    };

    impl_length!(<T> Vec<T>);
    impl_length!(<T> &Vec<T>);
    impl_length!(<T> &mut Vec<T>);

    // TODO: MACRO
    impl<T> InsertableCollection for Vec<T> {
        fn new() -> Self {
            Vec::with_capacity(1)
        }

        fn insert(&mut self, index: usize, item: Self::Output) {
            Vec::insert(self, index, item)
        }
    
        fn remove(&mut self, index: usize) -> Self::Output {
            Vec::remove(self, index)
        }

        fn set_length(&mut self, length: usize) {
            unsafe { Vec::set_len(self, length) }
        }

        // Ideally Vec types should *always* have capacity
        fn has_capacity(&self) -> bool {
            true
        }
    }
}

impl<C> Height for C where C: Length + ?Sized {
    fn height(&self) -> usize {
        height(self.length())
    }
}

impl_length!(<T> [T]);
impl_length!(<T, const N: usize> [T; N]);
impl_length!(<T> &[T]);
impl_length!(<T, const N: usize> &[T; N]);
impl_length!(<T> &mut [T]);
impl_length!(<T, const N: usize> &mut [T; N]);

impl<C> IndexedCollection for C where C: Index<usize> + Length + ?Sized {}
impl<C> IndexedCollectionMut for C where C: IndexMut<usize> + IndexedCollection + Length +  ?Sized {}