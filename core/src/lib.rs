pub mod fenwick;
pub mod macros;

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
    type Error;

    fn get(&self, key: &Self::Key) -> Result<&Self::Value, Self::Error>;
    fn contains(&self, key: &Self::Key) -> Result<bool, Self::Error>;
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> Result<&Self::Value, Self::Error>;
    fn update(&mut self, key: &Self::Key, value: Self::Value) -> Result<&Self::Value, Self::Error>;
    fn delete(&mut self, key: &Self::Key, value: Self::Value) -> Result<&Self::Value, Self::Error>;

    fn push(&mut self, value: Self::Value) -> Result<&Self::Value, Self::Error>;
    fn pop(&mut self, key: &Self::Key) -> Result<&Self::Value, Self::Error>;
}

pub trait TreeMut: Tree {
    fn get_mut(&mut self, key: &Self::Key) -> Result<&mut Self::Value, Self::Error>;
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

    //fn iter(&'w self, direction: Direction, callback: &'w dyn Fn(&'w Self));
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