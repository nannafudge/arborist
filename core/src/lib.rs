pub mod macros;
pub mod fenwick;

pub use tree::*;

/*################################
              Tree
################################*/

pub mod tree {
    pub trait Height {
        fn height(&self) -> usize;
    }

    pub trait TreeRead<T, E> {
        fn get(&self, node: &T) -> Result<&T, E>;
        fn contains(&self, node: &T) -> Result<bool, E>;
    }

    pub trait TreeReadMut<T, E>: TreeRead<T, E> {
        fn get_mut(&mut self, node: &T) -> Result<&mut T, E>;
    }

    pub trait TreeWrite<T, E>: TreeReadMut<T, E> {
        fn insert(&mut self, node: T) -> Result<Option<T>, E>;
        fn delete(&mut self, node: &T) -> Result<T, E>;

        fn pop(&mut self) -> Result<T, E>;
    }
}

/*################################
            Tree KV
################################*/

pub mod tree_kv {
    use super::tree::{
        TreeRead, TreeReadMut,
        TreeWrite
    };

    pub use core::convert::Into;
    use core::cmp::Ordering;

    #[derive(Debug)]
    pub enum NodeKV<'a, K: PartialEq, V> {
        Occupied(K, V),
        Search(&'a K),
        None
    }

    impl<'a, K: PartialEq, V> Default for NodeKV<'a, K, V> {
        fn default() -> Self {
            NodeKV::None
        }
    }

    impl<'a, K: PartialEq, V> NodeKV<'a, K, V> {
        pub fn unwrap(self: Self) -> V {
            match self {
                NodeKV::Occupied(_, v) => v,
                _ => panic!("Invalid tree NodeKV returned from search")
            }
        }

        pub fn inner(self: &Self) -> &V {
            match self {
                NodeKV::Occupied(_, v) => v,
                _ => panic!("Invalid tree NodeKV returned from search")
            }
        }

        pub fn inner_mut(self: &mut Self) -> &mut V {
            match self {
                NodeKV::Occupied(_, v) => v,
                _ => panic!("Invalid tree NodeKV returned from search")
            }
        }
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

    pub trait TreeReadKV<'a, 'b, K, V, E>: TreeRead<NodeKV<'b, K, V>, E> where
        K: PartialEq + 'b,
        'b: 'a
    {
        fn get(&'a self, key: &'b K) -> Result<&'a V, E> {
            Ok(TreeRead::get(self, &NodeKV::<'b, K, V>::Search(key))?.inner())
        }

        fn contains(&self, key: &'b K) -> Result<bool, E> {
            TreeRead::contains(self, &NodeKV::Search(key))
        }
    }

    pub trait TreeReadKVMut<'a, 'b, K, V, E>: TreeReadMut<NodeKV<'b, K, V>, E> where
        K: PartialEq + 'b,
        'b: 'a
    {
        fn get_mut(&'a mut self, key: &'b K) -> Result<&'a mut V, E>  {
            Ok(TreeReadMut::get_mut(self, &NodeKV::<'b, K, V>::Search(key))?.inner_mut())
        }
    }

    pub trait TreeWriteKV<'a, K, V, E>: TreeWrite<NodeKV<'a, K, V>, E> where
        K: PartialEq + 'a
    {
        fn insert(&mut self, key: K, value: V) -> Result<Option<V>, E> {
            Ok(
                TreeWrite::insert(self, NodeKV::Occupied(key, value))?
                    .map(| kv  | kv.unwrap())
            )
        }

        fn delete(&mut self, key: &'a K) -> Result<V, E> {
            Ok(TreeWrite::delete(self, &NodeKV::Search(key))?.unwrap())
        }

        fn pop(&'a mut self) -> Result<NodeKV<K, V>, E> {
            TreeWrite::pop(self)
        }
    }

    impl<'a, 'b: 'a, K: PartialEq + 'b, V, E, T> TreeReadKV<'a, 'b, K, V, E> for T where T: TreeRead<NodeKV<'b, K, V>, E> {}
    impl<'a, 'b: 'a, K: PartialEq + 'b, V, E, T> TreeReadKVMut<'a, 'b, K, V, E> for T where T: TreeReadMut<NodeKV<'b, K, V>, E> {}
    impl<'a, K: PartialEq + 'a, V, E, T> TreeWriteKV<'a, K, V, E> for T where T: TreeWrite<NodeKV<'a, K, V>, E> + 'a {}
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