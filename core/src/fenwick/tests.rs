use quote::format_ident;
use crate::{NodeSide, NodeType, TreeWalker, escape_syntax};
use crate::fenwick::{
    FenwickTreeWalker, VirtualTreeWalker, StatefulTreeWalker, StatefulTreeWalkerMut,
    IndexView, Direction, Length, lsb
};

macro_rules! impl_tests {
    (peek($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let $($($mut)?)? collection: [usize; 16] = gen_collection();
        let mut walker: $tw = <$tw>::new(&$($($mut)?)? collection, 1).unwrap();
        
        // Up from 1 = 2
        assert_eq!(walker.$fn_ident(Direction::Up), Some($($ref)? 2));
        // No elements beneath 1
        assert_eq!(walker.$fn_ident(Direction::Down), None);
        // 1 last in array, no elements to left
        assert_eq!(walker.$fn_ident(Direction::Left), None);
        // Right should be sibling
        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref)? 3));
        
        // Shift to index 4
        walker.view.index = 4;
        walker.view.lsb = 4;

        assert_eq!(walker.$fn_ident(Direction::Up), Some($($ref)? 8));
        assert_eq!(walker.$fn_ident(Direction::Down), Some($($ref)? 2));
        assert_eq!(walker.$fn_ident(Direction::Left), None);
        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref)? 12));
    };
    (probe($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &$($($mut)?)?[usize] = &$($($mut)?)?gen_collection();
        let walker: $tw = <$tw>::new($($($mut)?)? collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(4), Some($($ref)? 4));
        assert_eq!(walker.$fn_ident(9), Some($($ref)? 9));
        assert_eq!(walker.$fn_ident(0), None);
        assert_eq!(walker.$fn_ident(walker.inner.length()), None);
    };
    (traverse($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });

        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref$($mut)?)? 3));
        assert_eq!(walker.$fn_ident(Direction::Up), Some($($ref$($mut)?)? 2));
        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref$($mut)?)? 6));
        assert_eq!(walker.$fn_ident(Direction::Up), Some($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Up), Some($($ref$($mut)?)? 8));
        assert_eq!(walker.$fn_ident(Direction::Up), None);

        assert_eq!(walker.$fn_ident(Direction::Down), Some($($ref$($mut)?)? 8));
        assert_eq!(walker.$fn_ident(Direction::Down), Some($($ref$($mut)?)? 4));
        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref$($mut)?)? 12));
        assert_eq!(walker.$fn_ident(Direction::Down), Some($($ref$($mut)?)? 10));
        assert_eq!(walker.$fn_ident(Direction::Right), Some($($ref$($mut)?)? 14));
        assert_eq!(walker.$fn_ident(Direction::Right), None);

        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 14));
        assert_eq!(walker.$fn_ident(Direction::Down), Some($($ref$($mut)?)? 15));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 13));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 11));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 9));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 7));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 5));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 3));
        assert_eq!(walker.$fn_ident(Direction::Left), Some($($ref$($mut)?)? 1));

        assert_eq!(walker.$fn_ident(Direction::Down), None);
        assert_eq!(walker.$fn_ident(Direction::Left), None);
        assert_eq!(walker.$fn_ident(Direction::Right), None);
        assert_eq!(walker.$fn_ident(Direction::Up), None);
    };
    (seek($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &$($($mut)?)?[usize] = &$($($mut)?)? gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        for i in 1..collection.len() {
            assert_eq!(walker.$fn_ident(i), Some($($ref$($mut)?)? i));
        }

        assert_eq!(walker.$fn_ident(0), None);
        assert_eq!(walker.$fn_ident(walker.inner.length()), None);
    };
    (current($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 1));

        walker.view += 1;
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 2));

        walker.view += 12;
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 14));

        walker.view.index = walker.inner.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.$fn_ident(), None);

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.$fn_ident(), None);
    };
    (sibling($fn_ident:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        // Sibling at 1 should be 3
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 3));

        walker.view += 1; // Navigate to 2
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 6));
        
        walker.view += 11; // Navigate to 13
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 15));

        walker.view += 1; // Navigate to 14
        assert_eq!(walker.$fn_ident(), Some($($ref$($mut)?)? 10));

        walker.view.index = walker.inner.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.$fn_ident(), None);

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.sibling(), None);
    };
    (impl $test_name:ident.$subtest:ident for $tw:ty: $fn_ident:ident $(where return = $ref:tt$($mut:tt)?)?) => {
        #[test]
        fn $test_name() {
            impl_tests!{$subtest($fn_ident, $tw $(,$ref$($mut)?)?)}
        }
    };
    ($name:tt for $tw:ident<$generics:ty> $(where return = $ref:tt $($mut:tt)?)?) => {
        arborist_proc::interpolate!{
            walker_type => { $tw<$generics> }
            test_fn => { 
                select(l => {$name} r => {
                    format("#[a]_mut" a => {$name})
                } s => { $($($mut)?)? })
            }
            test_name => { format("test_#[a]_#[b]_#[c]" a => {$tw} b => {$name} c => {$($($mut)?)?}) },
            impl_tests!{
                impl #[test_name].$name for #[walker_type]: #[test_fn] $(where return = $ref$($mut)?)?
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

impl_tests!{peek for VirtualTreeWalker<[usize]>}
impl_tests!{peek for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{peek for StatefulTreeWalkerMut<[usize]> where return = &}
impl_tests!{peek for StatefulTreeWalkerMut<[usize]> where return = &mut}

/*impl_tests!{probe for VirtualTreeWalker<[usize]>}
impl_tests!{probe for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{probe for StatefulTreeWalkerMut<[usize]> where return = &mut}

impl_tests!{traverse for VirtualTreeWalker<[usize]>}
impl_tests!{traverse for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{traverse for StatefulTreeWalkerMut<[usize]> where return = &mut}

impl_tests!{seek for VirtualTreeWalker<[usize]>}
impl_tests!{seek for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{seek for StatefulTreeWalkerMut<[usize]> where return = &mut}

impl_tests!{current for VirtualTreeWalker<[usize]>}
impl_tests!{current for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{current for StatefulTreeWalkerMut<[usize]> where return = &mut}

impl_tests!{sibling for VirtualTreeWalker<[usize]>}
impl_tests!{sibling for StatefulTreeWalker<[usize]> where return = &}
impl_tests!{sibling for StatefulTreeWalkerMut<[usize]> where return = &mut}*/

#[cfg(feature = "fuzz")]
mod fuzz {
    use proptest::prelude::*;
    use crate::fenwick::{Height, test_impls::*};

    proptest! {
        #[test]
        fn test_height(s in 0..usize::MAX) {
            prop_assert_eq!(Height::height(&s), (s as f64).log(2.0).ceil() as usize);
        }
    }
}