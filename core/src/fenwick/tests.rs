use arborist_proc::interpolate;
use crate::{
    NodeSide, NodeType,
    TreeWalker, TreeWalkerMut
};
use crate::fenwick::{
    FenwickTreeError, lsb,
    Direction, Height, Length, IndexView,
    VirtualTreeView, StatefulTreeView, StatefulTreeViewMut
};

use core::cell::RefCell;

struct MockCollection {
    len: usize,
    length_calls: RefCell<usize>
}

impl Length for MockCollection {
    fn length(&self) -> usize {
        unsafe { *self.length_calls.as_ptr() += 1; }
        self.len
    }
}

impl core::ops::Index<usize> for MockCollection {
    type Output = usize;

    fn index(&self, _: usize) -> &Self::Output {
        &self.len
    }
}

impl core::ops::IndexMut<usize> for MockCollection {
    fn index_mut(&mut self, _: usize) -> &mut Self::Output {
        &mut self.len
    }
}

impl MockCollection {
    fn new(len: usize) -> Self {
        Self { len, length_calls: RefCell::new(0) }
    }

    fn set_length(collection: *mut MockCollection, length: usize) {
        unsafe { *(&mut (*collection).len) = length }
    }

    fn length_calls(collection: *const MockCollection) -> usize {
        unsafe { *(*collection).length_calls.as_ptr() }
    }
}

// I really should make this interpolate library less shit to use...
// aka. actually implement boolean logic
macro_rules! assert_length_calls {
    (VirtualTreeView) => {};
    (StatefulTreeView<[usize]>) => {0};
    (StatefulTreeViewMut<[usize]>) => {0};
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
    (length($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: MockCollection = MockCollection::new(32);
        let mock_ref: *const MockCollection = &collection as *const MockCollection;

        let walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();
        assert_eq!(MockCollection::length_calls(mock_ref), 1);

        assert_eq!(walker.length(), 32);
        assert_length_calls!($tw, mock_ref, 2);
    };
    // True testing of height() is performed in proptests
    (height($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let mut collection: MockCollection = MockCollection::new(32);
        let mock_ref: *mut MockCollection = &mut collection as *mut MockCollection;

        let walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();
        assert_eq!(MockCollection::length_calls(mock_ref), 1);

        assert_eq!(walker.height(), 5);
        assert_length_calls!($tw, mock_ref, 2);
    };
    // Peeks in a given direction without modifying the walker's internal index
    (peek($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        // Index = 1
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Left), Err(FenwickTreeError::OutOfBounds));

        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 2));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 3));
        
        walker.view.index = 4;
        walker.view.lsb = 4;

        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 8));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 2));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Ok($($ref$($mut)?)? 6));
        assert_eq!(walker.$fn_ident(Direction::Left), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 12));
    };
    (probe($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let $($inner_mods)? walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(4), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(9), Ok($($ref$($mut)?)? 9));
        assert_eq!(walker.$fn_ident(0), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(walker.length()), Err(FenwickTreeError::OutOfBounds));
    };
    (traverse($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });

        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 3));
        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 2));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 6));
        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 8));
        assert_eq!(walker.$fn_ident(Direction::Up), Err(FenwickTreeError::OutOfBounds));

        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Ok($($ref$($mut)?)? 12));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 10));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 14));
        assert_eq!(walker.$fn_ident(Direction::Right), Err(FenwickTreeError::OutOfBounds));

        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Ok($($ref$($mut)?)? 13));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 11));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 9));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 7));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 5));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 3));
        assert_eq!(walker.$fn_ident(Direction::Left), Ok($($ref$($mut)?)? 1));

        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Left)), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Down(NodeSide::Right)), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Left), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(Direction::Right), Ok($($ref$($mut)?)? 3));
        assert_eq!(walker.$fn_ident(Direction::Up), Ok($($ref$($mut)?)? 2));
    };
    (seek($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        for $($($mut)?)? i in 1..16 {
            assert_eq!(walker.$fn_ident(i), Ok($($ref$($mut)?)? i));
        }

        assert_eq!(walker.$fn_ident(0), Err(FenwickTreeError::OutOfBounds));
        assert_eq!(walker.$fn_ident(walker.length()), Err(FenwickTreeError::OutOfBounds));
    };
    (current($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 1));

        walker.view += 1;
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 2));

        walker.view += 12;
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 14));

        walker.view.index = walker.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.$fn_ident(), Err(FenwickTreeError::OutOfBounds));

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.$fn_ident(), Err(FenwickTreeError::OutOfBounds));
    };
    (sibling($fn_ident:ident, $tw:ty) $(inner = $inner_mods:tt)? $(modifiers = $ref:tt$($mut:tt)?)?) => {
        let $($inner_mods)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($inner_mods)? collection, 1).unwrap();

        // Sibling at 1 should be 3
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 3));

        walker.view += 1; // Navigate to 2
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 6));
        
        walker.view += 11; // Navigate to 13
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 15));

        walker.view += 1; // Navigate to 14
        assert_eq!(walker.$fn_ident(), Ok($($ref$($mut)?)? 10));

        walker.view.index = walker.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.$fn_ident(), Err(FenwickTreeError::OutOfBounds));

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.sibling(), Err(FenwickTreeError::OutOfBounds));
    };
    (impl $test_name:ident.$subtest:ident for $tw:ty: $fn_ident:ident
    $(where $(inner = $inner_mods:tt)? return = $ret_ref:tt$($ret_mut:tt)?)?) => {
        #[test]
        fn $test_name() {
            impl_tests!{
                $subtest($fn_ident, $tw)
                $($(inner = $inner_mods)?)?
                $(modifiers = $ret_ref$($ret_mut)?)?
            }
        }
    };
    ($name:tt for $tw:ident$(<$generics:ty>)?
    $(where $(inner = $inner_mods:tt)? $(return = $ret_ref:tt $($ret_mut:tt)?)?)?) => {
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
                $(where $(inner = $inner_mods)? $(return = $ret_ref $($ret_mut)?)?)?
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
impl_tests!{length for StatefulTreeViewMut<MockCollection> where inner = mut return = &}

impl_tests!{height for VirtualTreeView}
impl_tests!{height for StatefulTreeView<MockCollection>}
impl_tests!{height for StatefulTreeViewMut<MockCollection> where inner = mut return = &}

impl_tests!{peek for VirtualTreeView}
impl_tests!{peek for StatefulTreeView<[usize]> where return = &}
impl_tests!{peek for StatefulTreeViewMut<[usize]> where inner = mut return = &}

impl_tests!{probe for VirtualTreeView}
impl_tests!{probe for StatefulTreeView<[usize]> where return = &}
impl_tests!{probe for StatefulTreeViewMut<[usize]> where inner = mut return = &}

impl_tests!{traverse for VirtualTreeView}
impl_tests!{traverse for StatefulTreeView<[usize]> where return = &}
impl_tests!{traverse for StatefulTreeViewMut<[usize]> where inner = mut return = &}

impl_tests!{seek for VirtualTreeView}
impl_tests!{seek for StatefulTreeView<[usize]> where return = &}
impl_tests!{seek for StatefulTreeViewMut<[usize]> where inner = mut return = &}

impl_tests!{current for VirtualTreeView}
impl_tests!{current for StatefulTreeView<[usize]> where return = &}
impl_tests!{current for StatefulTreeViewMut<[usize]> where inner = mut return = &}

impl_tests!{sibling for VirtualTreeView}
impl_tests!{sibling for StatefulTreeView<[usize]> where return = &}
impl_tests!{sibling for StatefulTreeViewMut<[usize]> where inner = mut return = &}

/*################################
        TreeWalkerMut Tests
################################*/

impl_tests!{peek for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}
impl_tests!{probe for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}
impl_tests!{traverse for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}
impl_tests!{seek for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}
impl_tests!{current for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}
impl_tests!{sibling for StatefulTreeViewMut<[usize]> where inner = mut return = &mut}

#[cfg(feature = "proptest")]
mod proptest {
    use proptest::prelude::*;
    use crate::fenwick::{compat::log2_bin, Height};

    proptest! {
        #[test]
        fn test_collection_height(s in 0..usize::MAX) {
            prop_assert_eq!(super::MockCollection::new(s).height(), (s as f64).log(2.0).ceil() as usize);
        }
        #[test]
        fn test_log2_bin_height(s in 0..usize::MAX) {
            prop_assert_eq!(log2_bin(&s), (s as f64).log(2.0).ceil() as usize);
        }
    }
}