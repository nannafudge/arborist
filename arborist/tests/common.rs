use arborist_proc::impl_mock;
pub use arborist_core::fenwick::Length;
use rand::{
    SeedableRng, RngCore
};
use rand::rngs::{
    SmallRng, OsRng
};

impl_mock!(MockCollection);

pub struct ArgGen {
    seed: u64,
    rng: SmallRng,
    arg: usize
}

impl ArgGen {
    pub fn new() -> Self {
        let seed: u64 = OsRng.next_u64();

        Self {
            seed: seed,
            rng: SmallRng::seed_from_u64(seed),
            arg: 0
        }
    }

    pub fn gen(&mut self) {
        self.arg = self.rng.next_u64() as usize;
    }

    pub fn reset(&mut self) {
        self.rng = SmallRng::seed_from_u64(self.seed);
        self.arg = 0;
    }

    pub fn arg(&self) -> usize {
        self.arg
    }
}

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
