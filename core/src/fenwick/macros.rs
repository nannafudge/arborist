#[macro_export]
macro_rules! impl_bitwise {
    ($fn:ident, $op:tt, $rhs:ty) => {
        type Output = Self;

        fn $fn(self, rhs: $rhs) -> Self::Output {
            match self {
                Self::Valid { index, .. } => {
                    let new_index: usize = rhs $op index;
                    Self::Valid { index: new_index, lsb: lsb(new_index) }
                },
                Self::Invalid(index) => {
                    Self::Invalid(rhs $op index)
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_bitwise_assign {
    ($fn:ident, $op:tt, $rhs:ty) => {
        fn $fn(&mut self, rhs: $rhs) {
            match self {
                Self::Valid { index, lsb } => {
                    *index = rhs $op *index;
                    *lsb = crate::fenwick::lsb(*index); // To avoid shadowing issues
                }
                Self::Invalid(index) => {
                    *index = rhs $op *index;
                }
            }
        }
    };
}