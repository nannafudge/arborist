use sith::test_suite;
use arborist_proc::impl_mock;

use crate::*;
use crate::fenwick::*;

impl_mock!(MockCollection);

macro_rules! esc {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

macro_rules! generate_walker {
    ($walker:ty, $inner:ident, $index:expr) => {
        {
            let expected = $inner.clone();
            (<$walker>::new(&mut $inner, $index).unwrap(), expected)
        }
    };
}

#[test_suite]
mod auxiliary {
    use super::*;
    use sith::test_case;

    #[test_case]
    fn lsb_accepts_zero() {
        assert_eq!(lsb(0), 0);
    }

    #[test_case(index_zero, with(0, 0, 0))]
    #[test_case(index_one, with(1, 1, 1))]
    #[test_case(index_two, with(2, 2, 2))]
    #[test_case(index_twelve, with(12, 12, 4))]
    fn indexview_new(index: usize, expected_index: usize, expected_lsb: usize) {
        let view = IndexView::new(index);
        assert_eq!(view.index, expected_index);
        assert_eq!(view.lsb, expected_lsb);
    }

    #[test_case(add, with(IndexView::new(2), verbatim(+), 4))]
    #[test_case(sub, with(IndexView::new(2), verbatim(-), 0))]
    #[test_case(bit_or, with(IndexView::new(2), verbatim(|), 2))]
    #[test_case(bit_and, with(IndexView::new(2), verbatim(&), 2))]
    #[test_case(bit_xor, with(IndexView::new(2), verbatim(^), 0))]
    fn indexview_op(view: IndexView, r#op: _, expected: usize) {
        assert_eq!(view r#op 2, r#expected);
    }

    #[test_case(add, with(IndexView::new(2), verbatim(+=), 4))]
    #[test_case(sub, with(IndexView::new(2), verbatim(-=), 0))]
    #[test_case(bit_or, with(IndexView::new(2), verbatim(|=), 2))]
    #[test_case(bit_and, with(IndexView::new(2), verbatim(&=), 2))]
    #[test_case(bit_xor, with(IndexView::new(2), verbatim(^=), 0))]
    fn indexview_assign(mut view: IndexView, r#op: _, expected: usize) {
        esc!(view r#op 2);

        assert_eq!(view.index, expected);
        assert_eq!(view.lsb, expected);
    }

    #[test_case(with_index_two, with(2))]
    #[test_case(with_index_zero, with(0))]
    fn indexview_update(index: usize) {
        let mut view = IndexView::new(1);

        view.update(index);
        assert_eq!(view, IndexView { index: index, lsb: index })
    }

    #[test_case]
    fn nodeside_conversion() {
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
    
    #[test_case]
    fn nodetype_conversion() {
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
}

#[test_suite]
mod walkers {
    use super::*;
    use sith::test_case;

    #[test_case(virtual_view, with(verbatim(VirtualTreeView), verbatim()))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>), verbatim()))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(mut)))]
    fn new_errors_on_zero_index(r#walker: _, r#mut: _) {
        assert_eq!(
            esc!(<r#walker>::new(&r#mut MockCollection::new(32), 0)),
            Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 })
        );
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView)))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>)))]
    fn new(r#walker_ty: _) {
        let mut inner = MockCollection::new(32);
        for i in 1..16 {
            let walker = esc!(<r#walker_ty>::new(&mut inner, i).unwrap());
            assert_eq!(walker.curr, IndexView { index: i, lsb: lsb(i) });
        }
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView)))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>)))]
    fn length(r#walker_ty: _) {
        let mut inner = MockCollection::new(32);

        let walker = esc!(<r#walker_ty>::new(&mut inner, 1).unwrap());
        assert_eq!(walker.length(), 32);
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView), verbatim(peek), verbatim()))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>), verbatim(peek), verbatim(&)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(peek), verbatim(&)))]
    #[test_case(mut_stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(peek_mut), verbatim(&mut)))]
    fn peek(r#walker_ty: _, r#peek_fn: _, r#output_ref: _) {
        let mut inner: MockCollection = MockCollection::new(32);
        let (mut walker, mut expected) = generate_walker!(r#walker_ty, inner, 1);

        assert_eq!(walker.r#peek_fn(Direction::Left), Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 }));
        assert_eq!(walker.r#peek_fn(Direction::Down(NodeSide::Left)), Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 }));

        assert_eq!(walker.r#peek_fn(Direction::Right), Ok(r#output_ref expected[3]));
        assert_eq!(walker.r#peek_fn(Direction::Up), Ok(r#output_ref expected[2]));
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView), verbatim(probe), verbatim()))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>), verbatim(probe), verbatim(&)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(probe), verbatim(&)))]
    #[test_case(mut_stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(probe_mut), verbatim(&mut)))]
    fn probe(r#walker_ty: _, r#probe_fn: _, r#output_ref: _) {
        let mut inner: MockCollection = MockCollection::new(32);
        let (mut walker, mut expected) = generate_walker!(r#walker_ty, inner, 1);

        assert_eq!(walker.r#probe_fn(0), Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 }));
        assert_eq!(walker.r#probe_fn(32), Err(FenwickTreeError::OutOfBounds { index: 32, length: 32 }));

        assert_eq!(walker.r#probe_fn(1), Ok(r#output_ref expected[1]));
        assert_eq!(walker.r#probe_fn(31), Ok(r#output_ref expected[31]));

        assert_eq!(walker.curr, IndexView { index: 1, lsb: 1 });
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView)))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>)))]
    fn traverse_oob_left(r#walker_ty: _) {
        let mut inner: MockCollection = MockCollection::new(32);

        let (mut walker, _) = generate_walker!(r#walker_ty, inner, 1);
        walker.traverse(Direction::Left);

        assert_eq!(walker.curr, IndexView { index: 0, lsb: 1 });
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView)))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>)))]
    fn traverse_oob_right(r#walker_ty: _) {
        let mut inner: MockCollection = MockCollection::new(32);

        let (mut walker, _) = generate_walker!(r#walker_ty, inner, 31);
        walker.traverse(Direction::Right);

        assert_eq!(walker.curr, IndexView { index: 33, lsb: 1 });
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView)))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>)))]
    fn seek(r#walker_ty: _) {
        let mut inner: MockCollection = MockCollection::new(32);

        let (mut walker, _) = generate_walker!(r#walker_ty, inner, 31);

        walker.seek(0);
        assert_eq!(walker.curr, IndexView { index: 0, lsb: 0 });
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView), verbatim(current), verbatim()))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>), verbatim(current), verbatim(&)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(current), verbatim(&)))]
    #[test_case(mut_stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(current_mut), verbatim(&mut)))]
    fn current(r#walker_ty: _, r#current_fn: _, r#output_ref: _) {
        let mut inner: MockCollection = MockCollection::new(32);

        let (mut walker, _) = generate_walker!(r#walker_ty, inner, 1);
        assert_eq!(walker.r#current_fn(), Ok(r#output_ref 1));

        walker.curr = IndexView::new(0);
        assert_eq!(walker.r#current_fn(), Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 }));

        walker.curr = IndexView::new(32);
        assert_eq!(walker.r#current_fn(), Err(FenwickTreeError::OutOfBounds { index: 32, length: 32 }));
    }

    #[test_case(virtual_view, with(verbatim(VirtualTreeView), verbatim(sibling), verbatim()))]
    #[test_case(stateful_view, with(verbatim(StatefulTreeView<MockCollection>), verbatim(sibling), verbatim(&)))]
    #[test_case(stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(sibling), verbatim(&)))]
    #[test_case(mut_stateful_view_mut, with(verbatim(StatefulTreeViewMut<MockCollection>), verbatim(sibling_mut), verbatim(&mut)))]
    fn sibling(r#walker_ty: _, r#sibling_fn: _, r#output_ref: _) {
        let mut inner: MockCollection = MockCollection::new(32);

        let (mut walker, _) = generate_walker!(r#walker_ty, inner, 1);
        assert_eq!(walker.r#sibling_fn(), Ok(r#output_ref 3));

        walker.curr = IndexView::new(2);
        assert_eq!(walker.r#sibling_fn(), Ok(r#output_ref 6));

        walker.curr = IndexView::new(0);
        assert_eq!(walker.r#sibling_fn(), Err(FenwickTreeError::OutOfBounds { index: 0, length: 32 }));

        walker.curr = IndexView::new(32);
        assert_eq!(walker.r#sibling_fn(), Err(FenwickTreeError::OutOfBounds { index: 96, length: 32 }));
    }
}

mod helper_functions {
    use crate::fenwick;
    use rand::{
        SeedableRng, RngCore
    };
    use rand::rngs::{
        SmallRng, OsRng
    };

    use lazy_static::lazy_static;

    const ITERATIONS: usize = 128;
    lazy_static!{
        static ref SEED: u64 = OsRng.next_u64();
    }

    #[test]
    fn height_default() {
        let mut randomness: SmallRng = SmallRng::seed_from_u64(*SEED);
        for i in 0..ITERATIONS {
            let val: usize = randomness.next_u64() as usize;
            assert_eq!(
                fenwick::height(val), (val as f64).log(2.0).floor() as usize,
                "Failed at iteration {} with value: {}, seed: {}", i, val, *SEED
            );
        }
    }

    #[test]
    fn height_compat() {
        let mut randomness: SmallRng = SmallRng::seed_from_u64(*SEED);
        for i in 0..ITERATIONS {
            let val: usize = randomness.next_u64() as usize;
            assert_eq!(
                fenwick::compat::height(val), (val as f64).log(2.0).floor() as usize,
                "Failed at iteration {} with value: {}, seed: {}", i, val, *SEED
            );
        }
    }

    #[test]
    fn root() {
        let mut randomness: SmallRng = SmallRng::seed_from_u64(*SEED);
        for i in 0..ITERATIONS {
            let val: usize = randomness.next_u64() as usize;
            let height: usize = fenwick::height(val);

            assert_eq!(
                fenwick::root(&height), 2usize.pow(height as u32),
                "Failed at iteration {} with value: {}, seed: {}", i, val, *SEED
            );
        }
    }

    #[test]
    fn lsb() {
        assert_eq!(fenwick::lsb(0), 0);
        assert_eq!(fenwick::lsb(1), 1);
        assert_eq!(fenwick::lsb(2), 2);
        assert_eq!(fenwick::lsb(3), 1);
        assert_eq!(fenwick::lsb(4), 4);
    }
}