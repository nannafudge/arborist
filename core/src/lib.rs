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
        ($var:expr, $return:expr, $( $subcases:pat ),+) => {
            match $var {
                $($subcases)|+ => $return
            }
        };
        ($var:expr, $default:expr, $( $arm:pat => $body:expr ),+) => {
            match $var {
                $($arm => $body,)+
                _ => $default
            }
        };
        ($var:expr, $( $arm:pat => $body:expr ),+) => {
            match $var {
                $($arm => $body,)+
            }
        };
    }
    #[macro_export]
    macro_rules! require {
        ($($clause:expr)+, $err:expr) => {
            if !($($clause)+) {
                return Err($err);
            }
        };
    }
}

/*################################
              Tree
################################*/

pub trait Height {
    fn height(&self) -> usize;
}

// Awaiting chalk support for nested associated type expansion...
pub trait Tree: Height {
    type Key;
    type Value;

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

/*################################
           Tree Walker 
################################*/

pub trait TreeWalker<'a> {
    type Path;
    type Output;

    fn peek(&'a self, direction: Direction) -> Option<Self::Output>;
    fn probe(&'a self, path: Self::Path) -> Option<Self::Output>;
    fn traverse(&'a mut self, direction: Direction) -> Option<Self::Output>;
    fn seek(&'a mut self, path: Self::Path) -> Option<Self::Output>;
    fn reset(&'a mut self);

    fn current(&'a self) -> Option<Self::Output>;
    fn sibling(&'a self) -> Option<Self::Output>;
    fn type_(&'a self) -> NodeType;
    fn side(&'a self) -> NodeSide;
}

pub trait TreeWalkerMut<'a>: TreeWalker<'a> {
    type MutOutput;

    fn peek_mut(&'a mut self, direction: Direction) -> Option<Self::MutOutput>;
    fn probe_mut(&'a mut self, path: Self::Path) -> Option<Self::MutOutput>;
    fn traverse_mut(&'a mut self, direction: Direction) -> Option<Self::MutOutput>;
    fn seek_mut(&'a mut self, path: Self::Path) -> Option<Self::MutOutput>;

    fn current_mut(&'a mut self) -> Option<Self::MutOutput>;
    fn sibling_mut(&'a mut self) -> Option<Self::MutOutput>;
}

pub enum Direction {
    Up,
    Down,
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