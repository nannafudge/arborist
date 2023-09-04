#[derive(Debug, Clone, PartialEq)]
pub enum FenwickTreeError {
    OutOfBounds,
    Full,
    Empty
}