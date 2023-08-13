mod btree;
mod fenwick;
pub use crate::{btree::*, fenwick::*};

use core::ops::{Index, IndexMut};

#[macro_export]
macro_rules! impl_length {
    ($generics: tt, $type:ty) => {
        impl<$generics> Length for $type {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    ($type:ty) => {
        impl Length for $type {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_length!(T, [T]);
impl_length!(T, &[T]);
impl_length!(T, &mut [T]);
impl_length!(T, Vec<T>);
impl_length!(T, &Vec<T>);
impl_length!(T, &mut Vec<T>);

pub trait Length {
    fn length(&self) -> usize;
}

pub trait IndexedCollection<T>: Index<usize, Output = T> + Length {}
pub trait IndexedCollectionMut<T>: IndexedCollection<T> + IndexMut<usize> {}
impl<T, C> IndexedCollection<T> for C where C: Index<usize, Output = T> + Length {}
impl<T, C> IndexedCollectionMut<T> for C where C: IndexedCollection<T> + IndexMut<usize> {}

/*pub trait TreeNode<K, V> {
    fn up(&self) -> Option<&Self>;
    fn down(&self) -> Option<&Self>;
    fn sibling(&self) -> Option<&Self>;
}

// Awaiting chalk support for nested asc type expansion...
pub trait Tree<K, V, N: TreeNode<K, V>>: Sized {
    fn size(&self) -> usize;

    fn get(&self, key: &K) -> Option<&N>;
    fn contains(&self, key: &K) -> Option<&N>;
}

pub trait TreeMut<K, V, N: TreeNode<K, V>>: Tree<K, V, N> {
    fn insert(&mut self, key: &K, value: &V) -> Option<&mut N>;
    fn update(&mut self, key: &K, value: V) -> Option<&mut N>;
    fn delete(&mut self, key: &K, value: V) -> Option<&mut N>;

    fn push(&mut self, value: V) -> Option<&N>;
    fn pop(&mut self, key: &K) -> Option<&mut N>;

    fn get_mut(&mut self, key: &K) -> Option<&mut N>;
}*/

pub trait TreePath {}

pub trait TreeView {
    fn up(&mut self, levels: usize) -> &Self;
    fn down(&mut self, levels: usize) -> &Self;
    fn sibling(&mut self) -> &Self;
    fn seek<P: TreePath>(&mut self, path: P);
}

// Awaiting chalk support for nested asc type expansion...
pub trait Tree: Sized {
    type Key;
    type Value;
    type Node;

    fn size(&self) -> usize;

    fn get(&self, key: &Self::Key) -> Option<&Self::Node>;
    fn contains(&self, key: &Self::Key) -> Option<&Self::Node>;
    fn walk<TV: TreeView>(&self, key: &Self::Key) -> Option<TV>;
}

pub trait TreeMut: Tree {
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Node>;
    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Node>;
    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Node>;

    fn push(&mut self, value: Self::Value) -> Option<&Self::Node>;
    fn pop(&mut self, key: &Self::Key) -> Option<&mut Self::Node>;

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Node>;
    fn walk_mut<TV: TreeView>(&mut self, key: &Self::Key) -> Option<TV>;
}