use arborist_proc::{impl_test, interpolate};

use arborist::bst::BSTError;
use arborist::bst::bstmap::*;
use arborist::bst::bstset::*;

use arborist_core::fenwick::FenwickTreeError;

static mut ARG_GEN: usize = 0;

/*macro_rules! construct_bst_args {
    ($ty:ty) => {
        interpolate!{
            a => {
                format("#[a]" a => {$ty})
            },
            construct_bst_args!{where #[a]}
        }
    };
    (where BSTSet<usize>) => {
        unsafe { ARG_GEN }
    };
    (where BSTSetConst<usize; 32>) => {
        unsafe { ARG_GEN }
    };
    (where BSTMap<usize, usize>) => {
        unsafe { ARG_GEN }, unsafe  { ARG_GEN }
    };
    (where BSTMapConst<usize, usize; 32>) => {
        unsafe { ARG_GEN }, unsafe { ARG_GEN }
    };
}

macro_rules! impl_bst_test {
    (insert($bst:ty, $fn:ident)) => {
        let mut bst: $bst = <$bst>::new();

        for i in 1..bst.length() - 1 {
            assert_eq!(bst.insert(construct_bst_args!{$bst}), Ok(&i));
        }

        assert_eq!(bst.insert(construct_bst_args!{$bst}), Err(BSTError::Inner(FenwickTreeError::Full)));
    };
}

impl_test!{
    bstset.insert for BSTSet<usize>.insert;
    use impl_bst_test;
    setup = unsafe { ARG_GEN = 0; }
}

impl_test!{
    bstmap.insert for BSTMap<usize, usize>.insert;
    use impl_bst_test;
    setup = unsafe { ARG_GEN = 0; }
}*/