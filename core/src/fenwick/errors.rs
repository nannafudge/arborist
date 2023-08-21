pub enum FenwickTreeError {
    OutOfBounds{index: usize, tree_len: usize},
    InvalidNodeSide(NodeSide)
}