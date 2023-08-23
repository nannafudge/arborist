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
                self.lsb = lsb(self.index);
            }
        }
    };
}

#[macro_export]
macro_rules! safe_tree_select {
    ($self:tt, $index:expr) => {
        if $index == 0 || $index >= $self.tree.length() {
            return None;
        }

        return Some(&$self.tree[$index]);
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

#[macro_export]
macro_rules! bool_to_choice {
    ($(($condition:expr) $($op:tt)?)+) => {
        $(Choice::from($condition as u8) $($op)?)+
    };
}