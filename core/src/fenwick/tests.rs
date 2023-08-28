use crate::{NodeSide, NodeType, TreeWalker};
use crate::fenwick::{
    VirtualTreeWalker, StatefulTreeWalker, StatefulTreeWalkerMut,
    IndexView, Direction, Length, lsb
};

macro_rules! impl_tests {
    (peek($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();
        
        // Up from 1 = 2
        assert_eq!(walker.peek(Direction::Up), Some($($ref$($mut)?)? 2));
        // No elements beneath 1
        assert_eq!(walker.peek(Direction::Down), None);
        // 1 last in array, no elements to left
        assert_eq!(walker.peek(Direction::Left), None);
        // Right should be sibling
        assert_eq!(walker.peek(Direction::Right), Some($($ref$($mut)?)? 3));
        
        // Shift to index 4
        walker.view.index = 4;
        walker.view.lsb = 4;

        assert_eq!(walker.peek(Direction::Up), Some($($ref$($mut)?)? 8));
        assert_eq!(walker.peek(Direction::Down), Some($($ref$($mut)?)? 2));
        assert_eq!(walker.peek(Direction::Left), None);
        assert_eq!(walker.peek(Direction::Right), Some($($ref$($mut)?)? 12));
    };
    (probe($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let walker: $tw = <$tw>::new(collection, 1).unwrap();

        assert_eq!(walker.probe(4), Some($($ref$($mut)?)? 4));
        assert_eq!(walker.probe(9), Some($($ref$($mut)?)? 9));
        assert_eq!(walker.probe(0), None);
        assert_eq!(walker.probe(walker.inner.length()), None);
    };
    (traverse($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });

        assert_eq!(walker.traverse(Direction::Right), Some($($ref$($mut)?)? 3));
        assert_eq!(walker.traverse(Direction::Up), Some($($ref$($mut)?)? 2));
        assert_eq!(walker.traverse(Direction::Right), Some($($ref$($mut)?)? 6));
        assert_eq!(walker.traverse(Direction::Up), Some($($ref$($mut)?)? 4));
        assert_eq!(walker.traverse(Direction::Up), Some($($ref$($mut)?)? 8));
        assert_eq!(walker.traverse(Direction::Up), None);

        assert_eq!(walker.traverse(Direction::Down), Some($($ref$($mut)?)? 8));
        assert_eq!(walker.traverse(Direction::Down), Some($($ref$($mut)?)? 4));
        assert_eq!(walker.traverse(Direction::Right), Some($($ref$($mut)?)? 12));
        assert_eq!(walker.traverse(Direction::Down), Some($($ref$($mut)?)? 10));
        assert_eq!(walker.traverse(Direction::Right), Some($($ref$($mut)?)? 14));
        assert_eq!(walker.traverse(Direction::Right), None);

        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 14));
        assert_eq!(walker.traverse(Direction::Down), Some($($ref$($mut)?)? 15));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 13));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 11));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 9));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 7));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 5));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 3));
        assert_eq!(walker.traverse(Direction::Left), Some($($ref$($mut)?)? 1));

        assert_eq!(walker.traverse(Direction::Down), None);
        assert_eq!(walker.traverse(Direction::Left), None);
        assert_eq!(walker.traverse(Direction::Right), None);
        assert_eq!(walker.traverse(Direction::Up), None);
    };
    (seek($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        for i in 1..collection.len() {
            assert_eq!(walker.seek(i), Some($($ref$($mut)?)? i));
        }

        assert_eq!(walker.seek(0), None);
        assert_eq!(walker.seek(walker.inner.length()), None);
    };
    (current($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        assert_eq!(walker.current(), Some($($ref$($mut)?)? 1));

        walker.view += 1;
        assert_eq!(walker.current(), Some($($ref$($mut)?)? 2));

        walker.view += 12;
        assert_eq!(walker.current(), Some($($ref$($mut)?)? 14));

        walker.view.index = walker.inner.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.current(), None);

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.current(), None);
    };
    (sibling($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();

        // Sibling at 1 should be 3
        assert_eq!(walker.sibling(), Some($($ref$($mut)?)? 3));

        walker.view += 1; // Navigate to 2
        assert_eq!(walker.sibling(), Some($($ref$($mut)?)? 6));
        
        walker.view += 11; // Navigate to 13
        assert_eq!(walker.sibling(), Some($($ref$($mut)?)? 15));

        walker.view += 1; // Navigate to 14
        assert_eq!(walker.sibling(), Some($($ref$($mut)?)? 10));

        walker.view.index = walker.inner.length();
        walker.view.lsb = lsb(walker.view.index);
        assert_eq!(walker.sibling(), None);

        walker.view.index = 0;
        walker.view.lsb = 0;
        assert_eq!(walker.sibling(), None);
    };
    (@test $name:ident($fn:ident, $tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        #[test]
        fn $fn() {
            impl_tests!{$name($tw $(,$ref$($mut)?)?)}
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

impl_tests!{@test peek(test_virtual_walker_peek, VirtualTreeWalker<[usize]>)}
impl_tests!{@test peek(test_stateful_walker_peek, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test peek(test_stateful_walker_mut_peek, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test probe(test_virtual_walker_probe, VirtualTreeWalker<[usize]>)}
impl_tests!{@test probe(test_stateful_walker_probe, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test probe(test_stateful_walker_mut_probe, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test traverse(test_virtual_walker_traverse, VirtualTreeWalker<[usize]>)}
impl_tests!{@test traverse(test_stateful_walker_traverse, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test traverse(test_stateful_walker_mut_traverse, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test seek(test_virtual_walker_seek, VirtualTreeWalker<[usize]>)}
impl_tests!{@test seek(test_stateful_walker_seek, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test seek(test_stateful_walker_mut_seek, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test current(test_virtual_walker_current, VirtualTreeWalker<[usize]>)}
impl_tests!{@test current(test_stateful_walker_current, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test current(test_stateful_walker_mut_current, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test sibling(test_virtual_walker_sibling, VirtualTreeWalker<[usize]>)}
impl_tests!{@test sibling(test_stateful_walker_sibling, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test sibling(test_stateful_walker_mut_sibling, StatefulTreeWalker<[usize]>, &)}

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