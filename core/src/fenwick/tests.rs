use arborist_proc::{impl_mock, interpolate};

use crate::{
    NodeSide, NodeType,
    TreeWalker, TreeWalkerMut
};
use crate::fenwick::{
    VirtualTreeView, StatefulTreeView, StatefulTreeViewMut,
    IndexView, Direction, Length, FenwickTreeError, lsb,
    traits::*
};

impl_mock!(MockCollection);

// I really should make this interpolate library less shit to use...
// aka. actually implement boolean logic
macro_rules! assert_length_calls {
    // Boolean selector based on TreeView type
    // If type is VirtualTreeView, no length assert needed.
    (VirtualTreeView) => {};
    (StatefulTreeView<[usize]>) => {0};
    (StatefulTreeViewMut<[usize]>) => {0};
    // Ensures length() correctly proxies to the underlying collection implementation,
    // VirtualTreeView caches the length, therefore calls length() 0 times.
    ($tw:ty, $mock_ref:ident, $calls:literal) => {
        interpolate!{
            a => {
                select(
                    left => {}
                    right => {assert_eq!{MockCollection::length_calls($mock_ref), $calls}}
                    selector => {assert_length_calls!{$tw}}
                )
            },
            #[a]
        }
    };
}

macro_rules! impl_tests {
    (length($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: MockCollection = MockCollection::new(32);
        let _mock_ref: *const MockCollection = &collection as *const MockCollection;

        let walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.length(), 32);
        assert_length_calls!($tw, _mock_ref, 1);
    };
    // True testing of height() is performed in proptests
    (height($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let mut collection: MockCollection = MockCollection::new(32);
        let _mock_ref: *mut MockCollection = &mut collection as *mut MockCollection;

        let walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.height(), 5);
        assert_length_calls!($tw, _mock_ref, 1);
    };
    (new($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();

        for i in 1..16 {
            let walker: $tw = <$tw>::new(&$($inner_mods)? collection, i).unwrap();
            assert_eq!(walker.curr, IndexView { index: i, lsb: lsb(i) });
        }

        assert_eq!(<$tw>::new(&$($inner_mods)? collection, 0), Err(FenwickTreeError::OutOfBounds{ index: 0, length: 16 }))
    };
    // Peeks in a given direction without modifying the walker's internal index
    (peek($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let collection_length: usize = collection.length();
        // Starting at index 6
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 6).unwrap();
        assert_eq!(walker.curr, IndexView { index: 6, lsb: 2 });

        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 5));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Ok($($ref$($mut)?)? 7));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 2));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 10));

        walker.curr.update(12);

        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 8));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 10));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Ok($($ref$($mut)?)? 14));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Right), Err(FenwickTreeError::OutOfBounds{index: 20, length: collection_length}));

        walker.curr.update(8);

        assert_eq!(walker.$fn_ident(Direction::Up), Err(FenwickTreeError::OutOfBounds{index: 16, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Ok($($ref$($mut)?)? 12));
        assert_eq!(walker.$fn_ident(Direction::Left), Err(FenwickTreeError::OutOfBounds{index: 0, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Right), Err(FenwickTreeError::OutOfBounds{index: 24, length: collection_length}));

        walker.curr.update(32);

        assert_eq!(walker.$fn_ident(Direction::Up), Err(FenwickTreeError::OutOfBounds{index: 64, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Err(FenwickTreeError::OutOfBounds{index: 16, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Err(FenwickTreeError::OutOfBounds{index: 48, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Left), Err(FenwickTreeError::OutOfBounds{index: 0, length: collection_length}));
        assert_eq!(walker.$fn_ident(Direction::Right), Err(FenwickTreeError::OutOfBounds{index: 96, length: collection_length}));
    };
    (probe($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let collection_length: usize = collection.length();
        let $($inner_mods)? walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(4), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(9), Ok($($ref$($mut)?)? 9));
        assert_eq!(
            walker.$fn_ident(0),
            Err(FenwickTreeError::OutOfBounds{ index: 0, length: collection_length })
        );
        assert_eq!(
            walker.$fn_ident(walker.length()),
            Err(FenwickTreeError::OutOfBounds{ index: collection_length, length: collection_length })
        );
    };
    (traverse($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        walker.$fn_ident(Direction::Up);
        assert_eq!(walker.curr, IndexView { index: 2, lsb: 2 });
        walker.$fn_ident(Direction::Up);
        assert_eq!(walker.curr, IndexView { index: 4, lsb: 4 });

        walker.$fn_ident(Direction::Right);
        assert_eq!(walker.curr, IndexView { index: 12, lsb: 4 });
        walker.$fn_ident(Direction::Down(NodeSide::Left));
        assert_eq!(walker.curr, IndexView { index: 10, lsb: 2 });
        walker.$fn_ident(Direction::Down(NodeSide::Right));
        assert_eq!(walker.curr, IndexView { index: 11, lsb: 1 });
        walker.$fn_ident(Direction::Left);
        assert_eq!(walker.curr, IndexView { index: 9, lsb: 1 });

        walker.curr.update(8);

        walker.$fn_ident(Direction::Up);
        assert_eq!(walker.curr, IndexView { index: 16, lsb: 16 });
        walker.$fn_ident(Direction::Right);
        assert_eq!(walker.curr, IndexView { index: 48, lsb: 16 });
        walker.$fn_ident(Direction::Down(NodeSide::Left));
        assert_eq!(walker.curr, IndexView { index: 40, lsb: 8 });
        walker.$fn_ident(Direction::Down(NodeSide::Right));
        assert_eq!(walker.curr, IndexView { index: 44, lsb: 4 });
        walker.$fn_ident(Direction::Left);
        assert_eq!(walker.curr, IndexView { index: 36, lsb: 4 });
    };
    (seek($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        for $($($mut)?)? i in 1..16 {
            walker.$fn_ident(i);
            assert_eq!(walker.current(), Ok($($ref$($mut)?)? i));
        }

        walker.$fn_ident(0);
        assert_eq!(
            walker.current(),
            Err(FenwickTreeError::OutOfBounds{ index: 0, length: walker.length() })
        );
        walker.$fn_ident(walker.length());
        assert_eq!(
            walker.current(),
            Err(FenwickTreeError::OutOfBounds{ index: walker.length(), length: walker.length() })
        );
    };
    (current($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let collection_length: usize = collection.length();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 1));

        walker.curr.update(2);
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 2));

        walker.curr.update(14);
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 14));

        walker.curr.update(walker.length());
        assert_eq!(
            walker.$fn_ident(),
            Err(FenwickTreeError::OutOfBounds{ index: collection_length, length: collection_length })
        );

        walker.curr.update(0);
        assert_eq!(
            walker.$fn_ident(),
            Err(FenwickTreeError::OutOfBounds{ index: 0, length: collection_length })
        );
    };
    (sibling($fn_ident:ident, $tw:ty) $(collection = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let collection_length: usize = collection.length();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        // Sibling at 1 should be 3
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 3));

        walker.curr.update(2); // Navigate to 2
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 6));
        
        walker.curr.update(13); // Navigate to 13
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 15));

        assert_eq!(walker.length(), 16);
        walker.curr.update(walker.length());
        assert_eq!(
            walker.$fn_ident(),
            Err(FenwickTreeError::OutOfBounds{ index: 48, length: collection_length })
        );

        walker.curr.update(0);
        assert_eq!(
            walker.$fn_ident(),
            Err(FenwickTreeError::OutOfBounds{ index: 0, length: collection_length })
        );
    };
    (impl $test_name:ident.$subtest:ident for $tw:ty: $fn_ident:ident
    $(where $(collection = $inner_mods:tt)? return = $ret_ref:tt$($ret_mut:tt)?)?) => {
        #[test]
        fn $test_name() {
            impl_tests!{
                $subtest($fn_ident, $tw)
                $($(collection = $inner_mods)?)?
                $(modifiers = $ret_ref$($ret_mut)?)?
            }
        }
    };
    ($name:tt for $tw:ident$(<$generics:ty>)?
    $(where $(collection = $inner_mods:tt)? $(return = $ret_ref:tt $($ret_mut:tt)?)?)?) => {
        interpolate!{
            test_fn => { 
                select(
                    left => {format("#[fn_name]_mut" fn_name => {$name})}
                    right => {$name}
                    selector => { $($($($ret_mut)?)?)? }
                )
            }
            test_name => {
                format(
                    "test_#[ty]_#[method]" ty => {$tw}
                    method => {
                        select(
                            left => {format("#[a]_mut" a => {$name})}
                            right => {$name}
                            selector => { $($($($ret_mut)?)?)? }
                        )
                    }
                )
            }
            walker_type => {
                select(
                    left => {format("#[a]<#[b]>" a => {$tw} b => {$($generics)?})}
                    right => {$tw}
                    selector => { $($generics)? }
                )
            },
            impl_tests!{
                impl #[test_name].$name
                for #[walker_type]: #[test_fn]
                $(where $(collection = $inner_mods)? $(return = $ret_ref $($ret_mut)?)?)?
            }
        }
    };
}

fn gen_collection() -> [usize; 16] {
    let mut out: [usize; 16] = [0; 16];
    for i in 0..16 {
        out[i] = i;
    }
    out
}

#[test]
fn test_indexview_ops() {
    let mut view: IndexView = IndexView::new(2);
    assert_eq!(view.index, 2);
    assert_eq!(view.lsb, 2);

    // Test decimal arithmetic ops
    assert_eq!(view + 1, 3);
    assert_eq!(view - 1, 1);

    // Test decimal arithmetic assignment ops
    view += 2;
    assert_eq!(view.index, 4);
    assert_eq!(view.lsb, 4);
    view -= 3;
    assert_eq!(view.index, 1);
    assert_eq!(view.lsb, 1);
    view -= 1;
    assert_eq!(view.index, 0);
    assert_eq!(view.lsb, 0);

    // Test binary assignment operators
    view ^= 3;
    assert_eq!(view.index, 3);
    assert_eq!(view.lsb, 1);
    view &= 1;
    assert_eq!(view.index, 1);
    assert_eq!(view.lsb, 1);
    view |= 2;
    assert_eq!(view.index, 3);
    assert_eq!(view.lsb, 1);

    // Test binary (non-assignment) operators
    assert_eq!(view ^ 2, 1);
    assert_eq!(view & 2, 2);
    assert_eq!(view | 2, 3);
}

#[test]
fn test_nodeside_conversion() {
    /*        4 <--------> 12
           2 <-----> 6
        1 <-> 3   5 <-> 7
    */
    assert_eq!(NodeSide::from(&IndexView::new(1)), NodeSide::Left);
    assert_eq!(NodeSide::from(&IndexView::new(2)), NodeSide::Left);
    assert_eq!(NodeSide::from(&IndexView::new(3)), NodeSide::Right);
    assert_eq!(NodeSide::from(&IndexView::new(4)), NodeSide::Left);
    assert_eq!(NodeSide::from(&IndexView::new(5)), NodeSide::Left);
    assert_eq!(NodeSide::from(&IndexView::new(6)), NodeSide::Right);
    assert_eq!(NodeSide::from(&IndexView::new(7)), NodeSide::Right);
    assert_eq!(NodeSide::from(&IndexView::new(8)), NodeSide::Left);
}

#[test]
fn test_nodetype_conversion() {
    /*        4 <--------> 12
           2 <-----> 6
        1 <-> 3   5 <-> 7
    */
    assert_eq!(NodeType::from(&IndexView::new(1)), NodeType::Leaf);
    assert_eq!(NodeType::from(&IndexView::new(2)), NodeType::Node);
    assert_eq!(NodeType::from(&IndexView::new(3)), NodeType::Leaf);
    assert_eq!(NodeType::from(&IndexView::new(4)), NodeType::Node);
    assert_eq!(NodeType::from(&IndexView::new(5)), NodeType::Leaf);
    assert_eq!(NodeType::from(&IndexView::new(6)), NodeType::Node);
    assert_eq!(NodeType::from(&IndexView::new(7)), NodeType::Leaf);
    assert_eq!(NodeType::from(&IndexView::new(8)), NodeType::Node);
}

/*################################
         TreeWalker Tests
################################*/

impl_tests!{length for VirtualTreeView}
impl_tests!{length for StatefulTreeView<MockCollection>}
impl_tests!{length for StatefulTreeViewMut<MockCollection> where collection = mut return = &}

impl_tests!{height for VirtualTreeView}
impl_tests!{height for StatefulTreeView<MockCollection>}
impl_tests!{height for StatefulTreeViewMut<MockCollection> where collection = mut return = &}

impl_tests!{new for VirtualTreeView}
impl_tests!{new for StatefulTreeView<[usize]> where return = &}
impl_tests!{new for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{peek for VirtualTreeView}
impl_tests!{peek for StatefulTreeView<[usize]> where return = &}
impl_tests!{peek for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{probe for VirtualTreeView}
impl_tests!{probe for StatefulTreeView<[usize]> where return = &}
impl_tests!{probe for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{traverse for VirtualTreeView}
impl_tests!{traverse for StatefulTreeView<[usize]> where return = &}
impl_tests!{traverse for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{seek for VirtualTreeView}
impl_tests!{seek for StatefulTreeView<[usize]> where return = &}
impl_tests!{seek for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{current for VirtualTreeView}
impl_tests!{current for StatefulTreeView<[usize]> where return = &}
impl_tests!{current for StatefulTreeViewMut<[usize]> where collection = mut return = &}

impl_tests!{sibling for VirtualTreeView}
impl_tests!{sibling for StatefulTreeView<[usize]> where return = &}
impl_tests!{sibling for StatefulTreeViewMut<[usize]> where collection = mut return = &}

/*################################
        TreeWalkerMut Tests
################################*/

impl_tests!{peek for StatefulTreeViewMut<[usize]> where collection = mut return = &mut}
impl_tests!{probe for StatefulTreeViewMut<[usize]> where collection = mut return = &mut}
impl_tests!{current for StatefulTreeViewMut<[usize]> where collection = mut return = &mut}
impl_tests!{sibling for StatefulTreeViewMut<[usize]> where collection = mut return = &mut}

#[cfg(feature = "proptest")]
mod proptest {
    use proptest::prelude::*;
    use crate::fenwick::traits::*;

    proptest! {
        #[test]
        fn proptest_default_height_impl(s in 0..usize::MAX) {
            prop_assert_eq!(height(s), (s as f64).log(2.0).floor() as usize);
        }
        #[test]
        fn proptest_dfs_height_impl(s in 0..usize::MAX) {
            prop_assert_eq!(compat::height(&s), (s as f64).log(2.0).floor() as usize);
        }
    }
}