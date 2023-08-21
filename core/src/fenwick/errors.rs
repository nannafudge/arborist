use crate::NodeSide;

#[derive(Debug, Copy, Clone)]
pub enum FenwickTreeError {
    OutOfBounds{index: usize, tree_len: usize},
    InvalidNodeSide(NodeSide)
}