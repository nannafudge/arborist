#[macro_export]
macro_rules! impl_op {
    ($fn:ident, $op:tt, $rhs:ty) => {
        type Output = usize;

        fn $fn(self, rhs: $rhs) -> Self::Output {
            self.index $op rhs
        }
    };
}
#[macro_export]
macro_rules! impl_op_assign {
    ($fn:ident, $op:tt, $rhs:ty) => {
        fn $fn(&mut self, rhs: $rhs) {
            self.index $op rhs;
            self.lsb = lsb(self.index);
        }
    };
}

#[macro_export]
macro_rules! safe_tree_select {
    ($self:tt, $index:expr) => {
        unsafe {
            *const_time_select(
                &Ok(&$self.tree[$index]),
                &Err(FenwickTreeError::OutOfBounds{index: $index, tree_len: $self.tree.length()}),
                ($index == 0 || $index > $self.tree.length()) as usize
            )
        }
    };
}