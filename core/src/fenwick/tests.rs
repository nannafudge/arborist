use crate::{NodeSide, NodeType, TreeWalker};
use crate::fenwick::{
    VirtualTreeWalker, StatefulTreeWalker, StatefulTreeWalkerMut,
    IndexView, Direction
};

macro_rules! impl_tests {
    (@peek($tw:ty $(,$ret_mods:tt)?)) => {
        let collection: &[usize] = &gen_collection();
        let walker: $tw = <$tw>::new(collection, 1).unwrap();
        assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });
        
        // Up from 1 = 2
        assert_eq!(walker.peek(Direction::Up), Some($($ret_mods)? 2));
        // Down from 1 = 1
        assert_eq!(walker.peek(Direction::Down), None);
        assert_eq!(walker.peek(Direction::Left), None);
        assert_eq!(walker.peek(Direction::Right), Some($($ret_mods)? 3));
    };
    ($tw:ty, $ret_mods:tt) => {
        #[test]
        fn test_$tw_peek() {

        }
        #[test]
        fn test_$tw_probe() {

        }
        #[test]
        fn test_$tw_traverse() {

        }
        #[test]
        fn test_$tw_seek() {

        }
        #[test]
        fn test_$tw_reset() {

        }
        #[test]
        fn test_$tw_current() {

        }
        #[test]
        fn test_$tw_sibling() {

        }
        #[test]
        fn test_$tw_type_() {

        }
        #[test]
        fn test_$tw_side() {

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

#[test]
fn test_virtual_walker_construction() {
    let collection: &[usize] = &gen_collection();

    let walker: VirtualTreeWalker<[usize]> = VirtualTreeWalker::new(collection, 1).unwrap();
    assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });
}

#[test]
fn test_virtual_walker_peek() {
    impl_tests!{@peek(VirtualTreeWalker<[usize]>)};
}

#[test]
fn test_stateful_walker_construction() {
    let collection: &[usize] = &gen_collection();

    let walker: StatefulTreeWalker<[usize]> = StatefulTreeWalker::new(collection, 1).unwrap();
    assert_eq!(walker.view, IndexView { index: 1, lsb: 1 });
}

