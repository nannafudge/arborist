pub mod fenwick;

/*################################
          Common Macros
################################*/
mod macros {
    #[macro_export]
    macro_rules! unwrap_enum {
        ($var:expr, $return:expr, $default:expr, $( $subcases:pat ),+) => {
            match $var {
                $($subcases)|+ => $return,
                _ => $default
            }
        };
        ($var:expr, $default:expr, $( $arm:pat => $body:expr ),+) => {
            match $var {
                $($arm => $body,)+
                _ => $default
            }
        };
    }
}

/*################################
           Tree Traits
################################*/

// Awaiting chalk support for nested associated type expansion...
pub trait Tree {
    type Key;
    type Value;

    fn size(&self) -> usize;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn contains(&self, key: &Self::Key) -> Option<&Self::Value>;
}

pub trait TreeMut: Tree {
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Value>;
    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Value>;
    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Option<&mut Self::Value>;

    fn push(&mut self, value: Self::Value) -> Option<&mut Self::Value>;
    fn pop(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;
}

// Generic defined to tightly bind Walkers to their respective Tree declarations
pub trait TreeWalker<T: Tree + ?Sized> {
    type Path;

    // Traversal methods
    fn up(&mut self) -> Option<&T::Value>;
    fn down(&mut self, side: NodeSide) -> Option<&T::Value>;
    fn seek(&mut self, path: Self::Path) -> Option<&T::Value>;
    fn reset(&mut self);
    // Node-related methods
    fn current(&self) -> Option<&T::Value>;
    fn sibling(&self) -> Option<&T::Value>;
    fn type_(&self) -> NodeType;
    fn side(&self) -> NodeSide;
}

/*################################
         Node Definitions
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