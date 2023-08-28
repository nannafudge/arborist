use core::ops::{Index, IndexMut};
use arborist_proc::impl_length;

#[cfg(feature = "no_std")]
use bumpalo::Vec;
#[cfg(not(feature = "no_std"))]
use std::vec::Vec;

// Define as `length()` to avoid the fn
// sig clashing with normal len() impls
pub trait Length {
    fn length(&self) -> usize;
}

impl_length!(<T>, [T]);
impl_length!(<T, const N: usize>, [T; N]);
impl_length!(<T>, &[T]);
impl_length!(<T, const N: usize>, &[T; N]);
impl_length!(<T>, &mut [T]);
impl_length!(<T, const N: usize>, &mut [T; N]);
impl_length!(<T>, Vec<T>);
impl_length!(<T>, &Vec<T>);
impl_length!(<T>, &mut Vec<T>);

pub trait IndexedCollection: Index<usize> + Length {}
pub trait IndexedCollectionMut: IndexMut<usize> + IndexedCollection {}

impl<C> IndexedCollection for C where C: Index<usize> + Length + ?Sized {}
impl<C> IndexedCollectionMut for C where C: IndexMut<usize> + IndexedCollection + ?Sized {}

#[cfg(any(feature = "fuzz", feature = "bench"))]
pub mod test_impls {
    use crate::fenwick::traits::Length;

    // For test purposes, namespaced to avoid conflicts
    impl Length for usize {
        fn length(&self) -> usize {
            *self
        }
    }
}