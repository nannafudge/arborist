#[derive(Debug, Clone, PartialEq)]
pub enum FenwickTreeError {
    OutOfBounds{index: usize, length: usize},
    Full,
    Empty
}