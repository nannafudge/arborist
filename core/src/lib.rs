pub mod macros;
pub mod fenwick;

pub use tree::*;

/*################################
              Tree
################################*/

pub trait Height {
    fn height(&self) -> usize;
}

pub mod tree {
    pub trait TreeRead<T, E> {
        fn get(&self, node: &T) -> Result<&T, E>;
        fn contains(&self, node: &T) -> Result<bool, E>;
    }

    pub trait TreeReadMut<T, E>: TreeRead<T, E> {
        fn get_mut(&mut self, node: &T) -> Result<&mut T, E>;
    }

    pub trait TreeWrite<T, E>: TreeReadMut<T, E> {
        fn insert(&mut self, node: T) -> Result<&T, E>;
        fn update(&mut self, node: T) -> Result<&T, E>;
        fn delete(&mut self, node: &T) -> Result<T, E>;

        fn push(&mut self, node: T) -> Result<&T, E>;
        fn pop(&mut self) -> Result<T, E>;
    }

    pub trait TreeWriteMut<T, E>: TreeWrite<T, E> {
        fn insert_mut(&mut self, node: T) -> Result<&mut T, E>;
        fn update_mut(&mut self, node: T) -> Result<&mut T, E>;
        fn push_mut(&mut self, node: T) -> Result<&mut T, E>;
    }
}

/*################################
            Tree KV
################################*/

pub mod tree_kv {
    use super::tree::{
        TreeRead, TreeReadMut,
        TreeWrite, TreeWriteMut
    };

    use core::convert::Into;
    use core::cmp::Ordering;

    #[derive(Debug)]
    pub enum NodeKV<'a, K: PartialEq, V> {
        Occupied(K, V),
        Search(&'a K),
        None
    }

    impl<'a, K, V> PartialEq<K> for NodeKV<'a, K, V> where
        K: PartialEq
    {
        fn eq(&self, other: &K) -> bool {
            match self {
                NodeKV::Occupied(k, _) => other.eq(k),
                NodeKV::Search(k) => other.eq(*k),
                NodeKV::None => false
            }
        }
    }

    impl<'a, K, V> PartialEq for NodeKV<'a, K, V> where
        K: PartialEq
    {
        fn eq(&self, other: &Self) -> bool {
            match self {
                NodeKV::Occupied(k, _) => other.eq(k),
                NodeKV::Search(k) => other.eq(*k),
                NodeKV::None => false
            }
        }
    }

    impl<'a, K, V> PartialOrd<K> for NodeKV<'a, K, V> where
        K: PartialOrd
    {
        fn partial_cmp(&self, other: &K) -> Option<Ordering> {
            match self {
                NodeKV::Occupied(k, _) => other.partial_cmp(k),
                NodeKV::Search(k) => other.partial_cmp(*k),
                NodeKV::None => None
            }
        }
    }

    impl<'a, K, V> PartialOrd for NodeKV<'a, K, V> where
        K: PartialOrd
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self {
                NodeKV::Occupied(k, _) => other.partial_cmp(k),
                NodeKV::Search(k) => other.partial_cmp(*k),
                NodeKV::None => None
            }
        }
    }

    pub trait TreeReadKV<'a, K, V, E>: TreeRead<NodeKV<'a, K, V>, E> where
        &'a NodeKV<'a, K, V>: Into<&'a V>,
        K: PartialEq + 'a,
        V: 'a
    {
        fn get(&'a self, key: &'a K) -> Result<&V, E> {
            Ok(TreeRead::get(self, &NodeKV::Search(key))?.into())
        }
        fn contains(&'a self, key: &'a K) -> Result<bool, E> {
            TreeRead::contains(self, &NodeKV::Search(key))
        }
    }

    pub trait TreeReadKVMut<'a, K, V, E>: TreeReadMut<NodeKV<'a, K, V>, E> where
        &'a mut NodeKV<'a, K, V>: Into<&'a mut V>,
        K: PartialEq + 'a,
        V: 'a
    {
        fn get_mut(&'a mut self, key: &'a K) -> Result<&mut V, E> {
            Ok(TreeReadMut::get_mut(self, &NodeKV::Search(key))?.into())
        }
    }

    pub trait TreeWriteKV<'a, K, V, E>: TreeWrite<NodeKV<'a, K, V>, E> where
        &'a NodeKV<'a, K, V>: Into<&'a V>,
        NodeKV<'a, K, V>: Into<V>,
        K: PartialEq + 'a,
        V: 'a,
    {
        fn insert(&'a mut self, key: K, value: V) -> Result<&V, E> {
            Ok(TreeWrite::insert(self, NodeKV::Occupied(key, value))?.into())
        }

        fn update(&'a mut self, key: &'a K, value: V) -> Result<&V, E> {
            match TreeReadMut::get_mut(self, &NodeKV::Search(key))? {
                NodeKV::Occupied(_, v) => {
                    *v = value;
                    Ok(v)
                },
                _ => panic!("Unexpected result in update: get() should not return unoccupied nodes")
            }
        }

        fn delete(&'a mut self, key: &'a K) -> Result<V, E> {
            Ok(TreeWrite::delete(self, &NodeKV::Search(key))?.into())
        }

        fn push(&'a mut self, key: K, value: V) -> Result<&V, E> {
            Ok(TreeWrite::push(self, NodeKV::Occupied(key, value))?.into())
        }

        fn pop(&'a mut self) -> Result<NodeKV<K, V>, E> {
            TreeWrite::pop(self)
        }
    }

    pub trait TreeWriteMutKV<'a, K, V, E>: TreeWriteMut<NodeKV<'a, K, V>, E> where
        &'a mut NodeKV<'a, K, V>: Into<&'a mut V>,
        NodeKV<'a, K, V>: Into<V>,
        K: PartialEq + 'a,
        V: 'a,
    {
        fn insert_mut(&'a mut self, key: K, value: V) -> Result<&mut V, E> {
            Ok(TreeWriteMut::insert_mut(self, NodeKV::Occupied(key, value))?.into())
        }

        fn update_mut(&'a mut self, key: &'a K, value: V) -> Result<&mut V, E> {
            match TreeReadMut::get_mut(self, &NodeKV::Search(key))? {
                NodeKV::Occupied(_, v) => {
                    *v = value;
                    Ok(v)
                },
                _ => panic!("Unexpected result in update: get() should not return unoccupied nodes")
            }
        }

        fn push_mut(&'a mut self, key: K, value: V) -> Result<&mut V, E> {
            Ok(TreeWriteMut::push_mut(self, NodeKV::Occupied(key, value))?.into())
        }
    }
}

/*################################
           Tree Walker 
################################*/

pub trait TreeWalker<'w> {
    type Path;
    type Output;

    fn peek(&'w self, direction: Direction) -> Self::Output;
    fn probe(&'w self, path: Self::Path) -> Self::Output;
    fn traverse(&'w mut self, direction: Direction) -> Self::Output;
    fn seek(&'w mut self, path: Self::Path) -> Self::Output;
    fn reset(&'w mut self);

    fn current(&'w self) -> Self::Output;
    fn sibling(&'w self) -> Self::Output;
    fn type_(&'w self) -> NodeType;
    fn side(&'w self) -> NodeSide;
}

pub trait TreeWalkerMut<'w>: TreeWalker<'w> {
    type OutputMut;

    fn peek_mut(&'w mut self, direction: Direction) -> Self::OutputMut;
    fn probe_mut(&'w mut self, path: Self::Path) -> Self::OutputMut;
    fn traverse_mut(&'w mut self, direction: Direction) -> Self::OutputMut;
    fn seek_mut(&'w mut self, path: Self::Path) -> Self::OutputMut;

    fn current_mut(&'w mut self) -> Self::OutputMut;
    fn sibling_mut(&'w mut self) -> Self::OutputMut;

    //fn iter_mut(&'w self, direction: Direction, callback: &'w dyn FnMut(&'w mut Self));
}

pub enum Direction {
    Up,
    Down(NodeSide),
    Left,
    Right
}

/*################################
            Tree Node
################################*/

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeSide {
    Left,
    Right
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeType {
    Node,
    Leaf
}

/*################################
              Errors
################################*/

pub struct TreeError<T>(T);