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
    (@virtual(self = $self:tt, item = $eval:expr)) => {
        if $eval == 0 || $eval >= $self.inner.length() {
            None
        } else {
            Some($eval)
        }
    };
    // wrap_ret for StatefulTreeWalkers
    (@stateful(self = $self:tt, index = $index:expr, mutators = $($ref:tt$($mut:tt)?)?)) => {
        if $index == 0 || $index >= $self.inner.length() {
            None
        } else {
            println!("{:?}", $index);
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
            NodeSide::Left => $self.view.index $op_left ($self.view.lsb >> 1).max(1),
            NodeSide::Right => $self.view.index $op_right ($self.view.lsb >> 1).max(1)
        }
    };
    (@left($self:ident)) => {
        $self.view.index - ($self.view.lsb << 1).min($self.view.index)
    };
    (@left_mut($self:tt)) => {
        $self.view.index = impl_walker!(@left($self));
        $self.view.lsb = lsb($self.view.index);
    };
    (@right($self:tt, $op:tt)) => {
        $self.view.index $op ($self.view.lsb << 1)
    };
    (@peek($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker $($mut)? self, direction: Direction) -> Option<$output> {
            let index: usize = match direction {
                Direction::Up => impl_walker!(@up(self, +, -)),
                Direction::Down => impl_walker!(@down(self, -, +)),
                Direction::Left => impl_walker!(@left(self)),
                Direction::Right => impl_walker!(@right(self, +)),
            };

            interpolate_expr!($wrap_ret, ret => {index})
        }
    };
    (@probe($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker $($mut)? self, path: Self::Path) -> Option<$output> {
            let expr: usize = self.view.index ^ path;
            interpolate_expr!($wrap_ret, ret => {expr})
        }
    };
    (@traverse($fn:ident, $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker mut self, direction: Direction) -> Option<$output> {
            match direction {
                Direction::Up => {
                    impl_walker!(@up(self, +=, -=));
                },
                Direction::Down => {
                    impl_walker!(@down(self, -=, +=));
                },
                Direction::Left => {
                    impl_walker!(@left_mut(self));
                },
                Direction::Right => {
                    impl_walker!(@right(self, +=));
                },
            };
            
            interpolate_expr!($wrap_ret, ret => {self.view.index})
        }
    };
    (@seek($fn:ident, $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker mut self, path: Self::Path) -> Option<$output> {
            self.view.index = path;
            self.view.lsb = lsb(path);
            interpolate_expr!($wrap_ret, ret => {self.view.index})
        }
    };
    (@current($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker $($mut)? self) -> Option<$output> {
            let expr: usize = self.view.index ^ self.view.lsb;
            interpolate_expr!($wrap_ret, ret => {expr})
        }
    };
    (@sibling($fn:ident, $($mut:ident,)? $output:ty, $wrap_ret:expr)) => {
        fn $fn(&'walker $($mut)? self) -> Option<$output> {
            let expr: usize = self.view.index ^ self.view.lsb;
            interpolate_expr!($wrap_ret, ret => {expr})
        }
    };
    (output = $output:ty, return_wrapper = $wrap_ret:expr) => {
        type Path = usize;
        type Output = $output;

        impl_walker!{@peek(peek, $output, $wrap_ret)}
        impl_walker!{@probe(probe, $output, $wrap_ret)}
        impl_walker!{@traverse(traverse, $output, $wrap_ret)}
        impl_walker!{@seek(seek, $output, $wrap_ret)}
        impl_walker!{@current(current, $output, $wrap_ret)}
        impl_walker!{@sibling(sibling, $output, $wrap_ret)}

        fn reset(&'walker mut self) {
            self.view.index = self.inner.length();
        }

        fn type_(&'walker self) -> NodeType {
            NodeType::from(&self.view)
        }

        fn side(&'walker self) -> NodeSide {
            NodeSide::from(&self.view)
        }
    };
    (@mut(output = $output:ty, return_wrapper = $wrap_ret:expr)) => {
        type MutOutput = $output;

        impl_walker!{@peek(peek_mut, mut, $output, $wrap_ret)}
        impl_walker!{@probe(probe_mut, mut, $output, $wrap_ret)}
        impl_walker!{@traverse(traverse_mut, $output, $wrap_ret)}
        impl_walker!{@seek(seek_mut, $output, $wrap_ret)}
        impl_walker!{@current(current_mut, mut, $output, $wrap_ret)}
        impl_walker!{@sibling(sibling_mut, mut, $output, $wrap_ret)}
    };
}
