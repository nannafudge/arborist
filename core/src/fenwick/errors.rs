use crate::{NodeSide, unwrap_enum};
use core::fmt::{Write, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum FenwickTreeError {
    InvalidNodeSide(usize, NodeSide),
    OutOfBounds(usize, usize)
}

impl Write for FenwickTreeError {
    fn write_str(&mut self, s: &str) -> Result {
        unwrap_enum!(
            self,
            Self::InvalidNodeSide(index, side) => {
                format_args!("{}: Invalid node side {:?} for index {:?}", s, index, side);
                Ok(())
            },
            Self::OutOfBounds(index, tree_len) => {
                format_args!("{}: Index {:?} out of bounds: wanted 1-{:?}", s, index, tree_len);
                Ok(())
            }
        )
    }
}