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
    // wrap_ret for VirtualTreeWalker
    (@virtual($self:tt, $eval:expr)) => {
        if $eval == 0 || $eval >= $self.inner.length() {
            None
        } else {
            Some($eval)
        }
    };
    // wrap_ret for StatefulTreeWalkers
    (@stateful($self:tt, $index:expr, $($ref:tt$($mut:tt)?)? )) => {
        if $index == 0 || $index >= $self.inner.length() {
            None
        } else {
            Some($($ref$($mut)?)? $self.inner[$index])
        }
    };
}

/*################################
            Tree Walker
################################*/

#[macro_export]
macro_rules! impl_walker {
    (@up($self:tt, $op_left:tt, $op_right:tt)) => {
        // Transition upward to next 'lsb namespace'
        match NodeSide::from($self.view.index) {
            NodeSide::Left => $self.view.index $op_left $self.view.lsb,
            NodeSide::Right => $self.view.index $op_right $self.view.lsb
        }
    };
    (@down($self:tt, $op_left:tt, $op_right:tt)) => {
        // Transition downward to next 'lsb namespace'
         match NodeSide::from($self.view.index) {
            NodeSide::Left => $self.view.index $op_left $self.view.lsb >> 1,
            NodeSide::Right => $self.view.index $op_right $self.view.lsb >> 1
        }
    };
    (@left($self:tt)) => {
        ($self.view.index - $self.view.lsb).min($self.view.lsb)
    };
    (@left($self:tt, mut)) => {
        $self.view.index = impl_walker!(@left($self));
        $self.view.lsb = lsb($self.view.index);
    };
    (@right($self:tt, $op:tt)) => {
        $self.view.index $op $self.view.lsb
    };
    (@peek($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree $($mut)? self, direction: Direction) -> Option<$output> {
            let index: usize = match direction {
                Direction::Up => impl_walker!(@up(self, +, -)),
                Direction::Down => impl_walker!(@down(self, -, +)),
                Direction::Left => impl_walker!(@left(self)),
                Direction::Right => impl_walker!(@right(self, +)),
            };

            interpolate_expr!(ret => {index}, $wrap_ret)
        }
    };
    (@probe($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree $($mut)? self, path: Self::Path) -> Option<$output> {
            interpolate_expr!(ret => {self.view ^ path}, $wrap_ret)
        }
    };
    (@traverse($fn:ident, $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree mut self, direction: Direction) -> Option<$output> {
            match direction {
                Direction::Up => {
                    impl_walker!(@up(self, +=, -=));
                },
                Direction::Down => {
                    impl_walker!(@down(self, -=, +=));
                },
                Direction::Left => {
                    impl_walker!(@left(self, mut));
                },
                Direction::Right => {
                    impl_walker!(@right(self, +=));
                },
            };

            interpolate_expr!(ret => {self.view.index}, $wrap_ret)
        }
    };
    (@seek($fn:ident, $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree mut self, path: Self::Path) -> Option<$output> {
            self.view ^= path;
            interpolate_expr!(ret => {self.view.index}, $wrap_ret)
        }
    };
    (@current($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree $($mut)? self) -> Option<$output> {
            interpolate_expr!(ret => {self.view.index}, $wrap_ret)
        }
    };
    (@sibling($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'tree $($mut)? self) -> Option<$output> {
            interpolate_expr!(ret => {self.view.index ^ self.view.lsb}, $wrap_ret)
        }
    };
    ($output:ty, $wrap_ret:expr) => {
        type Path = usize;
        type Output = $output;

        impl_walker!{@peek(peek, $output, $wrap_ret)}
        impl_walker!{@probe(probe, $output, $wrap_ret)}
        impl_walker!{@traverse(traverse, $output, $wrap_ret)}
        impl_walker!{@seek(seek, $output, $wrap_ret)}
        impl_walker!{@current(current, $output, $wrap_ret)}
        impl_walker!{@sibling(sibling, $output, $wrap_ret)}

        fn reset(&'tree mut self) {
            self.view.index = self.inner.length();
        }

        fn type_(&'tree self) -> NodeType {
            NodeType::from(&self.view)
        }

        fn side(&'tree self) -> NodeSide {
            NodeSide::from(&self.view)
        }
    };
    (@mut($output:ty, $wrap_ret:expr)) => {
        type MutOutput = $output;

        impl_walker!{@peek(peek_mut, mut, $output, $wrap_ret)}
        impl_walker!{@probe(probe_mut, mut, $output, $wrap_ret)}
        impl_walker!{@traverse(traverse_mut, $output, $wrap_ret)}
        impl_walker!{@seek(seek_mut, $output, $wrap_ret)}
        impl_walker!{@current(current_mut, mut, $output, $wrap_ret)}
        impl_walker!{@sibling(sibling_mut, mut, $output, $wrap_ret)}
    };
}
