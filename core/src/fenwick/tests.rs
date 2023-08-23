use crate::{NodeSide, NodeType};
use crate::{Tree, TreeWalker};
use crate::fenwick::{FenwickTreeError, FenwickTreeView, FenwickIndexView};

impl Tree for [usize] {
    type Key = usize;
    type Value = usize;

    fn size(&self) -> usize {
        todo!()
    }

    fn get(&self, _key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }

    fn contains(&self, _key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }
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
    let mut view: FenwickIndexView = FenwickIndexView::new(2);

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
    assert_eq!(NodeSide::from(&FenwickIndexView::new(1)), NodeSide::Left);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(2)), NodeSide::Left);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(3)), NodeSide::Right);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(4)), NodeSide::Left);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(5)), NodeSide::Left);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(6)), NodeSide::Right);
    assert_eq!(NodeSide::from(&FenwickIndexView::new(7)), NodeSide::Right);
}

#[test]
fn test_nodetype_conversion() {
    /*        4 <--------> 12
           2 <-----> 6
        1 <-> 3   5 <-> 7
    */
    assert_eq!(NodeType::from(&FenwickIndexView::new(1)), NodeType::Leaf);
    assert_eq!(NodeType::from(&FenwickIndexView::new(2)), NodeType::Node);
    assert_eq!(NodeType::from(&FenwickIndexView::new(3)), NodeType::Leaf);
    assert_eq!(NodeType::from(&FenwickIndexView::new(4)), NodeType::Node);
    assert_eq!(NodeType::from(&FenwickIndexView::new(5)), NodeType::Leaf);
    assert_eq!(NodeType::from(&FenwickIndexView::new(6)), NodeType::Node);
    assert_eq!(NodeType::from(&FenwickIndexView::new(7)), NodeType::Leaf);
}

#[test]
fn test_tv_construction() {
    let collection: &[usize] = &gen_collection();

    let walker: FenwickTreeView::<[usize]> = FenwickTreeView::<[usize]>::new(collection, 1).unwrap();
    assert_eq!(walker.view, FenwickIndexView { index: 1, lsb: 1 });
}

#[test]
fn test_tv_construction_zero_index() {
    let collection: &[usize] = &gen_collection();

    // Zero'd index
    assert_eq!(
        FenwickTreeView::new(collection, 0),
        Err(FenwickTreeError::ZeroIndex)
    );
}

#[test]
fn test_traverse_up() {
    // [usize; 16] where each index corresponds to itself
    // i.e. c[0] = 0; c[1] = 1; c[2] = 2, etc...
    let collection: &[usize] = &gen_collection();

    // Start from index 1
    let mut walker: FenwickTreeView<[usize]> = FenwickTreeView::new(collection, 1).unwrap();
    // Fenwick: parent of 1 is 2
    assert_eq!(walker.up(), Some(&collection[2]));
    assert_eq!(walker.view, FenwickIndexView { index: 2, lsb: 2 });
    // Fenwick: parent of 2 is 4
    assert_eq!(walker.up(), Some(&collection[4]));
    assert_eq!(walker.view, FenwickIndexView { index: 4, lsb: 4 });
    // Fenwick: parent of 4 is 8
    assert_eq!(walker.up(), Some(&collection[8]));
    assert_eq!(walker.view, FenwickIndexView { index: 8, lsb: 8 });
    // Fenwick: parent of 8 is 32, Out of bounds
    assert_eq!(walker.up(), None);
    assert_eq!(walker.view, FenwickIndexView { index: 16, lsb: 16 });
}

#[test]
fn test_traverse_down() {
    // [usize; 16] where each index corresponds to itself
    // i.e. c[0] = 0; c[1] = 1; c[2] = 2, etc...
    let collection: &[usize] = &gen_collection();
    // Start from index 8 (midpoint)
    let mut walker: FenwickTreeView<[usize]> = FenwickTreeView::new(collection, 8).unwrap();

    // Fenwick: Children of 8 are (4, 12), LSB = 4
    assert_eq!(walker.down(NodeSide::Left), Some(&collection[4]));
    assert_eq!(walker.view, FenwickIndexView { index: 4, lsb: 4 });

    // Fenwick: Children of 4 are (2, 6), LSB = 2
    assert_eq!(walker.down(NodeSide::Right), Some(&collection[6]));
    assert_eq!(walker.view, FenwickIndexView { index: 6, lsb: 2 });

    // Fenwick: Children of 6 are (5, 7), LSB = 1
    assert_eq!(walker.down(NodeSide::Left), Some(&collection[5]));
    assert_eq!(walker.view, FenwickIndexView { index: 5, lsb: 1 });

    // Fenwick: Cannot proceed further
    assert_eq!(walker.down(NodeSide::Left), Some(&collection[5]));
    assert_eq!(walker.down(NodeSide::Right), Some(&collection[5]));
    assert_eq!(walker.view, FenwickIndexView { index: 5, lsb: 1 });
}