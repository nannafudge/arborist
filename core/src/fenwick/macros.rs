macro_rules! impl_op {
    ($trait:ty, $fn:ident, $op:tt, $rhs:ty) => {
        impl $trait for IndexView {
            type Output = $rhs;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                self.index $op rhs
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($trait:ty, $fn:ident, $op:tt, $rhs:ty) => {
        impl $trait for IndexView {
            fn $fn(&mut self, rhs: $rhs) {
                self.index $op rhs;
                self.lsb = lsb(self.index);
            }
        }
    };
}

/*################################
            Tree Walker
################################*/

macro_rules! impl_walker {
    (@up($self:ident, $op_left:tt, $op_right:tt)) => {
        // Current LSB + 1 cannot exceed height of the tree (lsb <> log2(tree_len))
        require!($self.inner.lsb < $self.length() >> 1, FenwickTreeError::OutOfBounds);
        // Transition upward to next 'lsb namespace'
        match NodeSide::from($self.inner.index) {
            NodeSide::Left => $self.inner $op_left $self.inner.lsb,
            NodeSide::Right => $self.inner $op_right $self.inner.lsb
        }
    };
    (@down($self:ident, $side:ident, $op_left:tt, $op_right:tt)) => {
        // Current LSB - 1 cannot be zero, lowest height level of a tree is 1
        require!($self.inner.lsb > 1, FenwickTreeError::OutOfBounds);
        // Transition downward to next 'lsb namespace'
        match $side {
            NodeSide::Left => $self.inner $op_left ($self.inner.lsb >> 1),
            NodeSide::Right => $self.inner $op_right ($self.inner.lsb >> 1)
        }
    };
    (@left($self:ident, $op:tt)) => {
        // LSB is equivalent to furthermost left node of the tree
        require!($self.inner.index != $self.inner.lsb, FenwickTreeError::OutOfBounds);
        $self.inner.index $op ($self.inner.lsb << 1)
    };
    (@right($self:ident, $op:tt)) => {
        // Index + LSB cannot be greater than the length of the tree
        require!($self.length() - $self.inner.index > $self.inner.lsb, FenwickTreeError::OutOfBounds);
        $self.inner $op ($self.inner.lsb << 1)
    };
    (@peek($fn:ident, $($mut:ident,)? $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w $($mut)? self, direction: Direction) -> $output {
            let index: usize = match direction {
                Direction::Up => {
                    impl_walker!{@up(self, +, -)}
                },
                Direction::Down(side) => {
                    impl_walker!{@down(self, side, -, +)}
                },
                Direction::Left => {
                    impl_walker!{@left(self, -)}
                },
                Direction::Right => {
                    impl_walker!{@right(self, +)}
                }
            };

            interpolate!(ret => {index}, $($wrap_ret)+)
        }
    };
    (@probe($fn:ident, $($mut:ident,)? $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w $($mut)? self, path: Self::Path) -> $output {
            require!(path > 0 && path < self.length(), FenwickTreeError::OutOfBounds);
            interpolate!(ret => {path}, $($wrap_ret)+)
        }
    };
    (@traverse($fn:ident, $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w mut self, direction: Direction) -> $output {
            match direction {
                Direction::Up => {
                    impl_walker!(@up(self, +=, -=));
                },
                Direction::Down(side) => {
                    impl_walker!(@down(self, side, -=, +=));
                },
                Direction::Left => {
                    impl_walker!(@left(self, -=));
                },
                Direction::Right => {
                    impl_walker!(@right(self, +=));
                },
            };

            interpolate!(ret => {self.inner.index}, $($wrap_ret)+)
        }
    };
    (@seek($fn:ident, $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w mut self, path: Self::Path) -> $output {
            require!(path > 0 && path < self.length(), FenwickTreeError::OutOfBounds);

            self.inner.index = path;
            self.inner.lsb = lsb(path);
            interpolate!(ret => {self.inner.index}, $($wrap_ret)+)
        }
    };
    (@current($fn:ident, $($mut:ident,)? $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w $($mut)? self) -> $output {
            require!(self.inner.index > 0 && self.inner.index < self.length(), FenwickTreeError::OutOfBounds);
            interpolate!(ret => {self.inner.index}, $($wrap_ret)+)
        }
    };
    (@sibling($fn:ident, $($mut:ident,)? $output:ty, $($wrap_ret:tt)+)) => {
        fn $fn(&'w $($mut)? self) -> $output {
            let sibling: usize = self.inner.index ^ self.inner.lsb << 1;
            require!(sibling > 0 && sibling < self.length(), FenwickTreeError::OutOfBounds);
            interpolate!(ret => {sibling}, $($wrap_ret)+)
        }
    };
    (body(output = $output:ty, return_wrapper = $($wrap_ret:tt)+)) => {
        type Path = usize;
        type Output = $output;

        impl_walker!{@peek(peek, $output, $($wrap_ret)+)}
        impl_walker!{@probe(probe, $output, $($wrap_ret)+)}
        impl_walker!{@traverse(traverse, $output, $($wrap_ret)+)}
        impl_walker!{@seek(seek, $output, $($wrap_ret)+)}
        impl_walker!{@current(current, $output, $($wrap_ret)+)}
        impl_walker!{@sibling(sibling, $output, $($wrap_ret)+)}

        fn reset(&mut self) {
            self.inner.index = self.length();
        }

        fn type_(&self) -> NodeType {
            NodeType::from(&self.inner)
        }

        fn side(&self) -> NodeSide {
            NodeSide::from(&self.inner)
        }
    };
    (type = VirtualTreeView, output = $output:ty, return_wrapper = $($wrap_ret:tt)+) => {
        impl<'w> TreeWalker<'w> for VirtualTreeView {
            impl_walker!{body(output = $output, return_wrapper = $($wrap_ret)+)}
        }
    };
    (type = $target_type:ident, output = $output:ty, return_wrapper = $($wrap_ret:tt)+) => {
        impl<'t, 'w, C> TreeWalker<'w> for $target_type<'t, C> where
            C: ?Sized + IndexedCollection,
            C::Output: Sized,
            't: 'w
        {
            impl_walker!{body(output = $output, return_wrapper = $($wrap_ret)+)}
        }
    };
    (@mut(type = $target_type:ident, output = $output:ty, return_wrapper = $($wrap_ret:tt)+)) => {
        impl<'t, 'w, C> TreeWalkerMut<'w> for $target_type<'t, C> where
            C: ?Sized + IndexedCollectionMut,
            C::Output: Sized,
            't: 'w
        {
            type OutputMut = $output;

            impl_walker!{@peek(peek_mut, mut, $output, $($wrap_ret)+)}
            impl_walker!{@probe(probe_mut, mut, $output, $($wrap_ret)+)}
            impl_walker!{@traverse(traverse_mut, $output, $($wrap_ret)+)}
            impl_walker!{@seek(seek_mut, $output, $($wrap_ret)+)}
            impl_walker!{@current(current_mut, mut, $output, $($wrap_ret)+)}
            impl_walker!{@sibling(sibling_mut, mut, $output, $($wrap_ret)+)}
        }
    };
}