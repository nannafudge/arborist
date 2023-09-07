
use arborist_proc::impl_test;

use arborist::bst::BSTError;
use arborist_core::fenwick::FenwickTreeError;

struct ArgGen {
    arg: usize
}

impl ArgGen {
    fn new() -> Self {
        Self {
            arg: 0
        }
    }

    fn gen(&mut self) -> usize {
        self.arg += 1;
        self.arg
    }
}

macro_rules! impl_bst_test {
    (insert($bst:ty, $fn:ident args = $($args:tt)+)) => {
        let mut bst: $bst = <$bst>::new();

        for _ in 1..16 {
            assert_eq!(bst.insert($($args)+), Ok(None));
        }

        assert_eq!(bst.height(), 4);
    };
    (insert_const($bst:ty, $fn:ident args = $($args:tt)+)) => {
        let mut bst: $bst = <$bst>::new();

        for _ in 1..16 {
            assert_eq!(bst.insert($($args)+), Ok(None));
        }

        assert_eq!(bst.insert($($args)+), Err(BSTError::Inner(FenwickTreeError::Full)));
    };
}

impl_test!{
    bstset.insert for BSTSet<usize>.insert;
    use impl_bst_test(args.gen());
    setup = {
        use arborist::bst::bstset::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstmap.insert for BSTMap<usize, usize>.insert;
    use impl_bst_test(args.gen(), args.arg);
    setup = {
        use arborist::bst::bstmap::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstset.insert_const for BSTSetConst<usize, 16>.insert;
    use impl_bst_test(args.gen());
    setup = {
        use arborist::bst::bstset::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstmap.insert_const for BSTMapConst<usize, usize, 16>.insert;
    use impl_bst_test(args.gen(), args.arg);
    setup = {
        use arborist::bst::bstmap::*;

        let mut args: ArgGen = ArgGen::new();
    }
}