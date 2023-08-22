use core::ops::{Index, IndexMut};
use arborist_proc::impl_length;

// Define as `length()` to avoid the fn
// sig clashing with normal len() impls
pub trait Length {
    fn length(&self) -> usize;
}

pub trait IndexedCollection: Index<usize> + Length {}
pub trait IndexedCollectionMut: IndexMut<usize> + IndexedCollection {}

impl<C> IndexedCollection for C where C: Index<usize> + Length + ?Sized {}
impl<C> IndexedCollectionMut for C where C: IndexMut<usize> + IndexedCollection {}

impl_length!(<T>, [T]);
impl_length!(<T>, &[T]);
impl_length!(<T>, &mut [T]);
impl_length!(<T>, Vec<T>);