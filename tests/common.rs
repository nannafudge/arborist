use arborist_proc::impl_mock;
pub use arborist_core::fenwick::Length;

impl_mock!(MockCollection);

pub fn gen_collection<const N: usize>() -> [usize; N] {
    let mut out: [usize; N] = [0; N];
    for i in 0..N {
        out[i] = i;
    }
    out
}

pub fn gen_collection_with<const N: usize, G: FnMut() -> usize>(mut generator: G) -> [usize; N] {
    let mut out: [usize; N] = [0; N];

    for i in 0..N {
        out[i] = generator();
    }
    out
}
