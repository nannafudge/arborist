#[macro_export]
macro_rules! impl_op {
    ($trait:ty, $target:ty, $fn:ident, $op:tt, $rhs:ty) => {
        impl $trait for $target {
            type Output = $rhs;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                self.index $op rhs
            }
        }
    };
}

#[macro_export]
macro_rules! impl_op_assign {
    ($trait:ty, $target:ty, $fn:ident, $op:tt, $rhs:ty) => {
        impl $trait for $target {
            fn $fn(&mut self, rhs: $rhs) {
                self.index $op rhs;
                if self.index == 0 { self.index = 1 };
                self.lsb = lsb(self.index);
            }
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

#[macro_export]
macro_rules! ensure_index_bounds {
    ($index:expr, $tree_len:expr) => {
        if $index == 0 || $index > $tree_len {
            return Err(FenwickTreeError::OutOfBounds{ index: $index, tree_len: $tree_len });
        }
    };
}