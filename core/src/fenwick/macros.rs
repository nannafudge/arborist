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
        ct_select_safe(
            &|| -> Result<&C::Output, FenwickTreeError> { 
                Ok(&$self.tree[$index])
            },
            &|| -> Result<&C::Output, FenwickTreeError> { 
                Err(FenwickTreeError::OutOfBounds{ index: $index })
            },
            ($index == 0 || $index > $self.tree.length() - 1) as usize
        )
    };
}

#[macro_export]
macro_rules! require {
    ($($clause:expr)+, $err:expr) => {
        if !($($clause)+) {
            return Err($err);
        }
    };
}