mod common;
use common::*;

use arborist_core::fenwick::FenwickTreeError;
use arborist::bst::{
    BSTWalker, BSTWalkerResult,
    BSTError
};

const BST_SIZE: usize = 16;

macro_rules! impl_tests {
    (@construct($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        {
            let mut bst: $bst = <$bst>::new();
            for _ in 1..BST_SIZE {
                $arg_gen.gen();
                assert_eq!(bst.insert($($args ,)? $arg), Ok(None));
            }

            bst
        }
    };
    (insert($bst:ident, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        for _ in 1..BST_SIZE {
            $arg_gen.gen();
            assert_eq!($bst.insert($($args ,)? $arg), Ok(None));
            assert_eq!($bst.insert($($args ,)? $arg), Ok(Some($arg)));
        }

        assert_eq!($bst.length(), BST_SIZE - 1);
        assert_eq!($bst.height(), (BST_SIZE - 1 as usize).ilog2() as usize);

        let inner = $bst.inner();
        for i in 1..inner.length() - 1 {
            assert!(inner[i] < inner[i + 1]);
        }
    };
    (insert_const($bst:ident, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        impl_tests!{insert($bst, $arg_gen, $arg $(, $args)?)};

        $arg_gen.gen();
        assert_eq!($bst.insert($($args ,)? $arg), Err(BSTError::Inner(FenwickTreeError::Full)));
    };
    (delete($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        let mut bst: $bst = impl_tests!{@construct($bst, $arg_gen, $arg $(, $args)?)};

        $arg_gen.reset();

        for _ in 1..BST_SIZE {
            $arg_gen.gen();
            assert_eq!(bst.delete(&$arg), Ok($arg));
        }

        assert_eq!(bst.delete(&0), Err(BSTError::KeyNotFound));
        assert_eq!(bst.length(), 0);
        assert_eq!(bst.height(), 0);
    };
    (contains($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        let bst: $bst = impl_tests!{@construct($bst, $arg_gen, $arg $(, $args)?)};

        $arg_gen.reset();

        for _ in 1..BST_SIZE {
            $arg_gen.gen();
            assert_eq!(bst.contains(&$arg), Ok(true));
        }

        assert_eq!(bst.contains(&0), Ok(false));
        $arg_gen.gen();
        assert_eq!(bst.contains(&$arg), Ok(false));
    };
    (pop($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        let mut bst: $bst = impl_tests!{@construct($bst, $arg_gen, $arg $(, $args)?)};
        let mut inner = bst.inner_mut().clone();

        for _ in 1..BST_SIZE {
            assert_eq!(bst.pop(), Ok(inner.pop().unwrap()));
        }

        assert_eq!(bst.pop(), Err(BSTError::Inner(FenwickTreeError::Empty)));
    };
    (@get($bst:ty, $fn_ident:ident, $arg_gen:ident, $arg:expr $(, $args:expr)?) $(: $mut:tt)?) => {
        let $($mut)? bst: $bst = impl_tests!{@construct($bst, $arg_gen, $arg $(, $args)?)};

        $arg_gen.reset();

        for _ in 1..BST_SIZE {
            $arg_gen.gen();
            assert_eq!(bst.$fn_ident(&$arg), Ok(&$($mut)? $arg));
        }

        assert_eq!(bst.$fn_ident(&0), Err(BSTError::KeyNotFound));
        $arg_gen.gen();
        assert_eq!(bst.$fn_ident(&$arg), Err(BSTError::KeyNotFound));
    };
    (get($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        impl_tests!{@get($bst, get, $arg_gen, $arg $(, $args)?)}
    };
    (get_mut($bst:ty, $arg_gen:ident, $arg:expr $(, $args:expr)?)) => {
        impl_tests!{@get($bst, get_mut, $arg_gen, $arg $(, $args)?): mut}
    };
}

#[test]
fn bstwalker_new() {
    {
        let empty_collection: MockCollection = MockCollection::new(0);
        let walker: BSTWalker<MockCollection> = BSTWalker::new(&empty_collection).unwrap();

        assert_eq!(walker.view.index(), 1);
    }

    {
        let collection: MockCollection = MockCollection::new(9);
        let walker: BSTWalker<MockCollection> = BSTWalker::new(&collection).unwrap();

        assert_eq!(walker.view.index(), 8);
    }

    {
        let collection: MockCollection = MockCollection::new(16);
        let walker: BSTWalker<MockCollection> = BSTWalker::new(&collection).unwrap();

        assert_eq!(walker.view.index(), 16);
    }
}

#[test]
fn bstwalker_allocate() {
    // Generate a collection of powers of two, starting from 4
    let mut generator_multiplier: usize = 2;
    let collection: [usize; BST_SIZE] = gen_collection_with(|| {
        let out: usize = 1 << generator_multiplier;
        generator_multiplier += 1;

        out
    });

    let mut walker: BSTWalker<[usize; BST_SIZE]> = BSTWalker::new(&collection).unwrap();

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
    let collection: [usize; BST_SIZE] = gen_collection();
    let mut walker: BSTWalker<[usize; BST_SIZE]> = BSTWalker::new(&collection).unwrap();

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

mod bstset {
    use arborist::bst::bstset::*;
    use super::*;

    #[test]
    fn insert() {
        for _ in 0..5 {
            let mut args: ArgGen = ArgGen::new();
            let mut bst: BSTSet<usize> = BSTSet::new();

            impl_tests!{insert(bst, args, args.arg())}
        }
    }

    #[test]
    fn delete() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{delete(BSTSet<usize>, args, args.arg())}
    }

    #[test]
    fn get() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get(BSTSet<usize>, args, args.arg())}
    }

    #[test]
    fn get_mut() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get_mut(BSTSet<usize>, args, args.arg())}
    }

    #[test]
    fn contains() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{contains(BSTSet<usize>, args, args.arg())}
    }

    #[test]
    fn pop() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{pop(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }
}

mod bstset_const {
    use arborist::bst::bstset::*;
    use super::*;

    #[test]
    fn insert() {
        for _ in 0..5 {
            let mut args: ArgGen = ArgGen::new();
            let mut bst: BSTSetConst<usize, BST_SIZE> = BSTSetConst::new();

            impl_tests!{insert_const(bst, args, args.arg())}
        }
    }

    #[test]
    fn delete() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{delete(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }

    #[test]
    fn get() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }

    #[test]
    fn get_mut() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get_mut(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }

    #[test]
    fn contains() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{contains(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }

    #[test]
    fn pop() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{pop(BSTSetConst<usize, BST_SIZE>, args, args.arg())}
    }
}

mod bstmap {
    use arborist::bst::bstmap::*;
    use super::*;

    #[test]
    fn insert() {
        for _ in 0..5 {
            let mut args: ArgGen = ArgGen::new();
            let mut bst: BSTMap<usize, usize> = BSTMap::new();

            impl_tests!{insert(bst, args, args.arg(), args.arg() + 1)}
        }
    }

    #[test]
    fn delete() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{delete(BSTMap<usize, usize>, args, args.arg(), args.arg())}
    }

    #[test]
    fn get() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get(BSTMap<usize, usize>, args, args.arg(), args.arg())}
    }

    #[test]
    fn get_mut() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get_mut(BSTMap<usize, usize>, args, args.arg(), args.arg())}
    }

    #[test]
    fn contains() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{contains(BSTMap<usize, usize>, args, args.arg(), args.arg())}
    }

    #[test]
    fn pop() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{pop(BSTMap<usize, usize>, args, args.arg(), args.arg())}
    }
}
mod bstmap_const {
    use arborist::bst::bstmap::*;
    use super::*;

    #[test]
    fn insert() {
        for _ in 0..5 {
            let mut args: ArgGen = ArgGen::new();
            let mut bst: BSTMapConst<usize, usize, BST_SIZE> = BSTMapConst::new();

            impl_tests!{insert_const(bst, args, args.arg(), args.arg() + 1)}
        }
    }

    #[test]
    fn delete() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{delete(BSTMapConst<usize, usize, BST_SIZE>, args, args.arg(), args.arg())}
    }

    #[test]
    fn get() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get(BSTMapConst<usize, usize, BST_SIZE>, args, args.arg(), args.arg())}
    }

    #[test]
    fn get_mut() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{get_mut(BSTMapConst<usize, usize, BST_SIZE>, args, args.arg(), args.arg())}
    }

    #[test]
    fn contains() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{contains(BSTMapConst<usize, usize, BST_SIZE>, args, args.arg(), args.arg())}
    }

    #[test]
    fn pop() {
        let mut args: ArgGen = ArgGen::new();
        impl_tests!{pop(BSTMapConst<usize, usize, BST_SIZE>, args, args.arg(), args.arg())}
    }
}