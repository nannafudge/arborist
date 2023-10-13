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

    pub trait TreeRead {
        type Node;
        type Error;
    
        fn get(&self, node: &Self::Node) -> Result<&Self::Node, Self::Error>;        
        fn first(&self) -> Result<&Self::Node, Self::Error>;
        fn last(&self) -> Result<&Self::Node, Self::Error>;
        fn root(&self) -> Result<&Self::Node, Self::Error>;

        fn contains(&self, node: &Self::Node) -> Result<bool, Self::Error>;
    }

    pub trait TreeReadMut: TreeRead {
        fn get_mut(&mut self, node: &Self::Node) -> Result<&mut Self::Node, Self::Error>;

        fn first_mut(&mut self) -> Result<&mut Self::Node, Self::Error>;
        fn last_mut(&mut self) -> Result<&mut Self::Node, Self::Error>;
        fn root_mut(&mut self) -> Result<&mut Self::Node, Self::Error>;
    }

    pub trait TreeWrite: TreeReadMut {
        fn insert(&mut self, node: Self::Node) -> Result<Option<Self::Node>, Self::Error>;
        fn delete(&mut self, node: &Self::Node) -> Result<Self::Node, Self::Error>;

        fn pop(&mut self) -> Result<Self::Node, Self::Error>;
    }
}

/*################################
            Tree KV
################################*/

pub mod tree_kv {
    use core::cmp::Ordering;
    use super::tree::{
        TreeRead, TreeReadMut,
        TreeWrite
    };

    fn construct_search<'c, 't, K: 't, V>(key: &'c K) -> NodeKV<'t, K, V> {
        unsafe {
            std::mem::transmute::<NodeKV::<'c, K, V>, NodeKV::<'t, K, V>>(
                NodeKV::<'c, K, V>::Search(key)
            )
        }
    }

    #[derive(Debug, Clone)]
    pub enum NodeKV<'a, K, V> {
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

    pub trait TreeReadKV<'t, K, V, E> where
        Self: TreeRead<Node = NodeKV<'t, K, V>, Error = E>,
        K: PartialEq + 't
    {
        fn get<'c>(&'c self, key: &'c K) -> Result<&V, E> where 't: 'c {
            Ok(TreeRead::get(self, &construct_search(key))?.inner())
        }

        fn first<'c>(&'c self) -> Result<&V, Self::Error> where 't: 'c {
            Ok(TreeRead::first(self)?.inner())
        }

        fn last<'c>(&'c self) -> Result<&V, Self::Error> where 't: 'c {
            Ok(TreeRead::last(self)?.inner())
        }

        fn root<'c>(&'c self) -> Result<&V, Self::Error> where 't: 'c {
            Ok(TreeRead::root(self)?.inner())
        }

        fn contains<'c>(&'t self, key: &'c K) -> Result<bool, E> {
            TreeRead::contains(self, &construct_search(key))
        }
    }

    pub trait TreeReadKVMut<'t, K, V, E> where
        Self: TreeReadMut<Node = NodeKV<'t, K, V>>,
        K: PartialEq + 't
    {
        fn get_mut<'c>(&'c mut self, key: &'c K) -> Result<&mut V, Self::Error> where 't: 'c {
            Ok(TreeReadMut::get_mut(self, &construct_search(key))?.inner_mut())
        }

        fn first_mut<'c>(&'c mut self) -> Result<&mut V, Self::Error> where 't: 'c {
            Ok(TreeReadMut::first_mut(self)?.inner_mut())
        }

        fn last_mut<'c>(&'c mut self) -> Result<&mut V, Self::Error> where 't: 'c {
            Ok(TreeReadMut::last_mut(self)?.inner_mut())
        }

        fn root_mut<'c>(&'c mut self) -> Result<&mut V, Self::Error> where 't: 'c {
            Ok(TreeReadMut::root_mut(self)?.inner_mut())
        }
    }

    pub trait TreeWriteKV<'t, K, V, E> where
        Self: TreeWrite<Node = NodeKV<'t, K, V>, Error = E>,
        K: PartialEq + 't
    {
        fn insert(&mut self, key: K, value: V) -> Result<Option<V>, E> {
            Ok(
                TreeWrite::insert(self, NodeKV::Occupied(key, value))?
                    .map(| kv  | kv.unwrap())
            )
        }

        fn delete<'c>(&mut self, key: &'c K) -> Result<V, E> {
            Ok(TreeWrite::delete(self, &construct_search(key))?.unwrap())
        }

        fn pop<'c>(&'c mut self) -> Result<NodeKV<K, V>, E> where 't: 'c {
            TreeWrite::pop(self)
        }
    }

    impl<'t, K: PartialEq + 't, V, E, T> TreeReadKV<'t, K, V, E> for T where
        T: TreeRead<Node = NodeKV<'t, K, V>, Error = E> {}
    impl<'t, K: PartialEq + 't, V, E, T> TreeReadKVMut<'t, K, V, E> for T where
        T: TreeReadMut<Node = NodeKV<'t, K, V>, Error = E> {}
    impl<'t, K: PartialEq + 't, V, E, T> TreeWriteKV<'t, K, V, E> for T where
        T: TreeWrite<Node = NodeKV<'t, K, V>, Error = E> {}
}

/*################################
           Tree Walker 
################################*/

pub trait TreeWalker<'w> {
    type Path;
    type Output;
    type Error;

    fn peek(&'w self, direction: Direction) -> Result<Self::Output, Self::Error>;
    fn probe(&'w self, path: Self::Path) -> Result<Self::Output, Self::Error>;
    fn current(&'w self) -> Result<Self::Output, Self::Error>;
    fn sibling(&'w self) -> Result<Self::Output, Self::Error>;

    fn traverse(&'w mut self, direction: Direction);
    fn seek(&'w mut self, path: Self::Path);
    fn reset(&'w mut self);

    fn node_type(&'w self) -> NodeType;
    fn node_side(&'w self) -> NodeSide;
}

pub trait TreeWalkerMut<'w>: TreeWalker<'w> {
    type OutputMut;

    fn peek_mut(&'w mut self, direction: Direction) -> Result<Self::OutputMut, Self::Error>;
    fn probe_mut(&'w mut self, path: Self::Path) -> Result<Self::OutputMut, Self::Error>;

    fn current_mut(&'w mut self) -> Result<Self::OutputMut, Self::Error>;
    fn sibling_mut(&'w mut self) -> Result<Self::OutputMut, Self::Error>;
}

#[repr(u8)]
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
#[repr(u8)]
pub enum NodeSide {
    Left = 0,
    Right = 1
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum NodeType {
    Node = 0,
    Leaf = 1
}