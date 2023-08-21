use forest_rs::{
    Tree, TreeWalker, Node, NodeSide, NodeType,
    fenwick::{FenwickTreeError, FenwickTreeView}
};

#[test]
fn basic() {
    let collection: [Node<usize>; 3] = [
        Node::Occupied(0),
        Node::Occupied(1), 
        Node::Occupied(2)
    ];
    
    let view: FenwickTreeView = FenwickTreeView::from(&collection);
}