use subtle::{Choice, CtOption};

use crate::{Tree, TreeView, IndexedCollection, IndexedCollectionMut, TreePath, Length};
use core::ops::{Index, IndexMut};
use std::{ops::DerefMut, marker::PhantomData};

macro_rules! inner_enum {
    ($var:expr, $enum:path $(, $others:path )?) => {
        match $var {
            $enum(v) $(| $others(v))? => v,
            _ => panic!("Unable to unwrap enum")
        }
    };
}

fn lsb(i: isize) -> usize {
    (i & -i) as usize
}

//TODO: Create safe interface around these enforcing use of Pin
unsafe fn const_time_select<T: Sized>(a: *const T, b: *const T, selector: usize) -> *const T {
    a.offset(b.offset_from(a) * (selector & 0x1) as isize)
}

unsafe fn const_time_select_mut<T: Sized>(a: *mut T, b: *mut T, selector: usize) -> *mut T {
    a.offset(b.offset_from(a) * (selector & 0x1) as isize)
}

enum FenwickTreeError {
    OutOfBounds(usize, usize)
}

#[derive(Debug, Copy, Clone)]
enum Side {
    Left,
    Right
}

#[derive(Debug, Copy, Clone)]
enum NodeType {
    Node,
    Leaf
}

enum FenwickNode<T> {
    Occupied(T),
    Null,
    Err(FenwickTreeError)
}

impl From<usize> for Side {
    fn from(index: usize) -> Self {
        unsafe {
            *const_time_select(
                &Side::Left,
                &Side::Right,
                index & 2 >> 1 
            )
        }
    }
}

impl From<usize> for NodeType {
    fn from(index: usize) -> Self {
        unsafe {
            *const_time_select(
                &NodeType::Leaf,
                &NodeType::Node,
                index & 1
            )
        }
    }
}

trait FenwickCollection<T>: IndexedCollectionMut<FenwickNode<T>> {}

struct FenwickTree<T, C: FenwickCollection<T>> {
    inner: C,
    _marker: PhantomData<T>
}

impl<T, C: FenwickCollection<T>> Length for FenwickTree<T, C> {
    fn length(&self) -> usize {
        self.inner.length()
    }
}

impl<T, C: FenwickCollection<T>> Index<usize> for FenwickTree<T, C> {
    type Output = FenwickNode<T>;

    fn index(&self, index: usize) -> &Self::Output {
        // 1 = Positive Match (Err), else Ok
        let length: usize = self.inner.length();
        let error: u8 = u8::from(index == 0) | u8::from(index > length);

        unsafe {
            &*const_time_select(
                &self.inner[index],
                &self.inner[0],
                error as usize
            )
        }
    }
}

impl<T, C: FenwickCollection<T>> IndexMut<usize> for FenwickTree<T, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // 1 = Positive Match (Err), else Ok
        let length: usize = self.inner.length();
        let error: u8 = u8::from(index == 0) | u8::from(index > length);

        unsafe {
            &mut *const_time_select_mut(
                &mut self.inner[index],
                &mut self.inner[index],
                error as usize
            )
        }
    }
}

impl<T, C: FenwickCollection<T>> FenwickTree<T, C> {
    fn new(inner: C) -> FenwickTree<T, C> {
        FenwickTree {
            inner: inner,
            _marker: PhantomData
        }
    }
}

struct FenwickTreePath {
    index: usize,
    lsb: usize
}

impl TreePath for FenwickTreePath {}

struct FenwickTreeWalker<'a, T, C: FenwickCollection<T>> {
    tree: &'a FenwickTree<T, C>,
    index: usize,
    typ: NodeType,
    side: Side
}

impl<'a, T, C: FenwickCollection<T>> FenwickTreeWalker<'a, T, C> {
    fn new(tree: &FenwickTree<T, C>, index: usize) -> Result<Self, FenwickTreeError> {
        let length: usize = tree.length();
        if index < 0 || index > length {
            Err(FenwickTreeError::OutOfBounds(index, length))
        }

        Ok(Self {
            
        })
    }
}

/*impl<'a, T: IndexedCollection> TreeView for FenwickTreeWalker<'a, FenwickTree<T>> {
    fn up(&mut self, levels: usize) -> &Self {
        let index: Side = inner_enum!(&self.curr, &Side, NodeType::Leaf, NodeType::Node);

        self
    }

    fn down(&mut self, levels: usize) -> &Self {
        todo!()
    }

    fn sibling(&mut self) -> &Self {
        todo!()
    }

    fn seek<FenwickTreePath>(&mut self, path: FenwickTreePath) {
        todo!()
    }
}*/