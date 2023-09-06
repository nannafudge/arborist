#[derive(Debug, Clone, PartialEq)]
pub enum FenwickTreeError {
    OutOfBounds,
    OutOfBoundsFor{index: usize, length: usize},
    Full,
    Empty
}