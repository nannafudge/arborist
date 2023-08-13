use crate::{IndexedCollection, Tree, TreeView};

pub struct BTreeMap<C:  Sized> {
    inner: C
}

pub struct BTreeNode<K, V> {
    key: K,
    value: V
}

/*impl<T> Tree for BTreeMap<T> where
    T: IndexedCollection,
    T::Output: Sized
{
    type Key = isize;
    type Value = T::Output;
    type Node = BTreeNode<Self::Key, Self::Value>;

    fn size(&self) -> usize {
        todo!()
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Node> {
        todo!()
    }

    fn contains(&self, key: &Self::Key) -> Option<&Self::Node> {
        todo!()
    }

    fn walk<TV: crate::TreeView>(&self, key: &Self::Key) -> Option<TV> {
        todo!()
    }
}*/