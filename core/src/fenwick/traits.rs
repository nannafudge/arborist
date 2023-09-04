use core::ops::{Index, IndexMut};
use arborist_proc::{
    impl_insertable_collection,
    impl_length
};

#[cfg(feature = "no_std")]
use bumpalo::Vec;
#[cfg(not(feature = "no_std"))]
use std::vec::Vec;

// Define as `length()` to avoid the fn
// sig clashing with normal len() impls
pub trait Length {
    fn length(&self) -> usize;
}

pub trait IndexedCollection: Index<usize> + Length {}
pub trait IndexedCollectionMut: IndexMut<usize> + IndexedCollection {}

impl<C> IndexedCollection for C where C: Index<usize> + Length + ?Sized {}
impl<C> IndexedCollectionMut for C where C: IndexMut<usize> + IndexedCollection + Length +  ?Sized {}

pub trait InsertableCollection: IndexedCollection {
    fn insert(&mut self, index: usize, item: Self::Output);
    fn remove(&mut self, index: usize) -> Self::Output;
}

#[cfg(feature = "const_vec")]
mod const_vec {
    use tinyvec::{Array, ArrayVec, SliceVec};
    use arborist_proc::{impl_length, impl_insertable_collection};
    use super::{InsertableCollection, IndexedCollection, Length};

    impl_length!(<C: Array + IndexedCollection> ArrayVec<C>);
    impl_length!(<'a, C: Array + IndexedCollection> SliceVec<'a, C>);
    impl_insertable_collection!(<C: Array + IndexedCollection> ArrayVec<C>);
    impl_insertable_collection!(<'a, C: Array + IndexedCollection + Default> SliceVec<'a, C>);
}

impl_length!(<T> [T]);
impl_length!(<T, const N: usize> [T; N]);
impl_length!(<T> &[T]);
impl_length!(<T, const N: usize> &[T; N]);
impl_length!(<T> &mut [T]);
impl_length!(<T, const N: usize> &mut [T; N]);
impl_length!(<T> Vec<T>);
impl_length!(<T> &Vec<T>);
impl_length!(<T> &mut Vec<T>);

impl_insertable_collection!(<T> Vec<T>);

#[cfg(any(feature = "proptest", feature = "bench"))]
pub mod test_impls {
    use crate::fenwick::traits::Length;

    // For test purposes, namespaced to avoid conflicts
    impl Length for usize {
        fn length(&self) -> usize {
            *self
        }
    }
}