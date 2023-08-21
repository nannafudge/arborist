use arborist_core::TreeWalker;
use arborist_core::fenwick::*;

#[test]
fn basic() {
    let collection: [usize; 4] = [0, 1, 2, 3];

    let mut walker: FenwickTreeView<[usize]> = FenwickTreeView::new(&collection, 1);
    assert_eq!(walker.view, FenwickIndexView { index: 1, lsb: 1 });
    walker.up();
    assert_eq!(walker.view, FenwickIndexView { index: 2, lsb: 2 });
}