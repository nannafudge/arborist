use crate::NodeSide;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FenwickTreeError {
    InvalidNodeSide(NodeSide),
    ZeroIndex,
}