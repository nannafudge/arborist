use crate::{NodeSide, NodeType, TreeWalker};
use crate::fenwick::{
    VirtualTreeWalker, StatefulTreeWalker, StatefulTreeWalkerMut,
    IndexView, Direction, Length
};

macro_rules! impl_tests {
    (peek($tw:ty $(,$ref:tt$($mut:tt)?)?)) => {
        let collection: &[usize] = &gen_collection();
        let mut walker: $tw = <$tw>::new(collection, 1).unwrap();
        assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });
        
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

    // Test decimal arithmetic assignment ops
    assert_eq!(view.index, 2);
    assert_eq!(view.lsb, 2);
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
}

impl_tests!{@test peek(test_virtual_walker_peek, VirtualTreeWalker<[usize]>)}
impl_tests!{@test peek(test_stateful_walker_peek, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test peek(test_stateful_walker_mut_peek, StatefulTreeWalker<[usize]>, &)}

impl_tests!{@test seek(test_virtual_walker_seek, VirtualTreeWalker<[usize]>)}
impl_tests!{@test seek(test_stateful_walker_seek, StatefulTreeWalker<[usize]>, &)}
impl_tests!{@test seek(test_stateful_walker_mut_seek, StatefulTreeWalker<[usize]>, &)}
