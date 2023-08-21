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
         Common Functions
################################*/

//TODO: Create safe & tested interface around these enforcing use of Pin
#[inline(always)]
pub(crate) unsafe fn const_time_select<T: Unpin + Sized>(a: *const T, b: *const T, selector: usize) -> *const T {
    assert!(selector < 2); // TODO: Create Selector type to artifically constrain (or only accept bool)
    a.offset(b.offset_from(a) * (selector & 0x1) as isize)
}

#[inline(always)]
pub(crate) unsafe fn const_time_select_mut<T: Unpin + Sized>(a: *mut T, b: *mut T, selector: usize) -> *mut T {
    assert!(selector < 2);
    a.offset(b.offset_from(a) * (selector & 0x1) as isize)
}

/*################################
           Tree Traits
################################*/

// Awaiting chalk support for nested associated type expansion...
pub trait Tree {
    type Key;
    type Value;
    type Error;

    fn size(&self) -> usize;

    fn get(&self, key: &Self::Key) -> Result<&Self::Value, Self::Error>;
    fn contains(&self, key: &Self::Key) -> Result<&Self::Value, Self::Error>;
}

pub trait TreeMut: Tree {
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> Result<&mut Self::Value, Self::Error>;
    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Result<&mut Self::Value, Self::Error>;
    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Result<&mut Self::Value, Self::Error>;

    fn push(&mut self, value: Self::Value) -> Result<&Self::Value, Self::Error>;
    fn pop(&mut self, key: &Self::Key) -> Result<&mut Self::Value, Self::Error>;

    fn get_mut(&mut self, key: &Self::Key) -> Result<&mut Self::Value, Self::Error>;
}

// Generic defined to tightly bind Walkers to their respective Tree declarations
pub trait TreeWalker<T: Tree> {
    type Path;

    // Traversal methods
    fn up(&mut self) -> Result<&T::Value, T::Error>;
    fn down(&mut self, side: NodeSide) -> Result<&T::Value, T::Error>;
    fn seek(&mut self, path: Self::Path) -> Result<&T::Value, T::Error>;
    fn reset(&mut self);
    // Node-related methods
    fn sibling(&self) -> Result<&T::Value, T::Error>;
    fn type_(&self) -> NodeType;
    fn side(&self) -> NodeSide;
}

/*################################
         Node Definitions
################################*/

#[derive(Debug, Copy, Clone)]
pub enum NodeSide {
    Left,
    Right,
    Null
}

#[derive(Debug, Copy, Clone)]
pub enum NodeType {
    Node,
    Leaf,
    Null
}

#[derive(Debug, Copy, Clone)]
pub enum Node<T> {
    Occupied(T),
    Empty
}

impl<T> Into<Option<T>> for Node<T> {
    fn into(self) -> Option<T> {
        unwrap_enum!(self, Some(v), None, Node::Occupied(v))
    }
}

impl<'a, T> Into<Option<&'a T>> for &'a Node<T> {
    fn into(self) -> Option<&'a T> {
        unwrap_enum!(self, Some(v), None, Node::Occupied(v))
    }
}