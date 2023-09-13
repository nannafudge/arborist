mod common;
use common::*;

use arborist_proc::impl_test;

use arborist_core::fenwick::FenwickTreeError;
use arborist::bst::{
    BSTWalker, BSTWalkerResult,
    BSTError
};

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

    fn reset(&mut self) {
        self.arg = 0;
    }
}

macro_rules! impl_bst_test {
    (insert($bst:ty, $fn:ident args = $reset_args:expr, $arg:expr $(, $args:expr)?)) => {
        let mut bst: $bst = <$bst>::new();

        for _ in 1..16 {
            assert_eq!(bst.insert($arg $(, $args)?), Ok(None));
        }

        $reset_args;

        for i in 1..16 {
            assert_eq!(bst.insert($arg $(, $args)?), Ok(Some(i)));
        }

        assert_eq!(bst.height(), 3);
    };
    (insert_const($bst:ty, $fn:ident args = $reset_args:expr, $arg:expr $(, $args:expr)?)) => {
        let mut bst: $bst = <$bst>::new();

        for _ in 1..16 {
            assert_eq!(bst.insert($arg $(, $args)?), Ok(None));
        }

        assert_eq!(bst.insert($arg $(, $args)?), Err(BSTError::Inner(FenwickTreeError::Full)));

        $reset_args;

        for i in 1..16 {
            assert_eq!(bst.insert($arg $(, $args)?), Ok(Some(i)));
        }

        assert_eq!(bst.height(), 3);
    };
}

#[test]
fn bstwalker_allocate() {
    // Generate a collection of powers of two, starting from 4
    let mut generator_multiplier: usize = 2;
    let collection: [usize; 16] = gen_collection_with(|| {
        let out: usize = 1 << generator_multiplier;
        generator_multiplier += 1;

        out
    });

    let mut walker: BSTWalker<[usize; 16]> = BSTWalker::new(&collection).unwrap();

    let mut index: usize = 1;
    // Skip first element as index 0 is unreachable in implementation
    for element in &collection[1..collection.len()] {
        assert_eq!(walker.allocate(element), BSTWalkerResult::Existing(index));
        walker.reset();
        assert_eq!(walker.allocate(&(element - 1)), BSTWalkerResult::New(index));
        walker.reset();
        assert_eq!(walker.allocate(&(element + 1)), BSTWalkerResult::New(index + 1));
        walker.reset();
        index += 1;
    }
}

#[test]
fn bstwalker_find() {
    let collection: [usize; 16] = gen_collection();
    let mut walker: BSTWalker<[usize; 16]> = BSTWalker::new(&collection).unwrap();

    let mut index: usize = 1;
    // Skip first element as index 0 is unreachable in implementation
    for element in &collection[1..collection.len()] {
        assert_eq!(walker.find(element), Ok(index));
        walker.reset();
        index += 1;
    }

    assert_eq!(walker.find(&0), Err(BSTError::KeyNotFound));
    walker.reset();
    assert_eq!(walker.find(&(collection.last().unwrap() + 1)), Err(BSTError::KeyNotFound));
}

impl_test!{
    bstset.insert for BSTSet<usize>.insert;
    use impl_bst_test(args.reset(), args.gen());
    setup = {
        use arborist::bst::bstset::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstmap.insert for BSTMap<usize, usize>.insert;
    use impl_bst_test(args.reset(), args.gen(), args.arg);
    setup = {
        use arborist::bst::bstmap::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstset.insert_const for BSTSetConst<usize, 16>.insert;
    use impl_bst_test(args.reset(), args.gen());
    setup = {
        use arborist::bst::bstset::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

impl_test!{
    bstmap.insert_const for BSTMapConst<usize, usize, 16>.insert;
    use impl_bst_test(args.reset(), args.gen(), args.arg);
    setup = {
        use arborist::bst::bstmap::*;

        let mut args: ArgGen = ArgGen::new();
    }
}

#[cfg(feature = "proptest")]
mod proptest {
    use proptest::prelude::*;
    use super::{
        BSTWalker, BSTWalkerResult,
        common::*
    };

    // For testing for regressions
    proptest!{
        #[test]
        fn proptest_bstwalker_create(i in 0..usize::MAX) {
            let mock_collection: MockCollection = MockCollection::new(i);
            let walker: BSTWalker<MockCollection> = BSTWalker::new(&mock_collection).unwrap();

            prop_assert_eq!(walker.view.index(), 1 << height(i as usize));
        }
    }
}