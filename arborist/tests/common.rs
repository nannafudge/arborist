pub use arborist_core::fenwick::Length;
pub use std::io::Write;
use std::panic::PanicInfo;

use arborist_proc::impl_mock;
use ctor::ctor;
use rand::{
    SeedableRng, RngCore
};
use rand::rngs::{
    SmallRng, OsRng
};

pub static mut SEED: u64 = 0;

pub static RNG: SmallRng = {
    SmallRng::seed_from_u64(OsRng.next_u64())
};

pub struct RandomArgs {
    salt: u64,
    rng: SmallRng,
    arg: usize
}

impl RandomArgs {
    pub fn new() -> Self {
        Self {
            salt: 0,
            rng: SmallRng::seed_from_u64(seed()),
            arg: 0
        }
    }

    pub fn new_with_salt(salt: u64) -> Self {
        Self {
            salt: salt,
            rng: SmallRng::seed_from_u64(seed() ^ salt),
            arg: 0
        }
    }

    pub fn next(&mut self) {
        self.arg = self.rng.next_u64() as usize;
    }

    pub fn reset(&mut self) {
        self.rng = SmallRng::seed_from_u64(seed() ^ self.salt);
        self.arg = 0;
    }

    pub fn arg(&self) -> usize {
        self.arg
    }
}

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

#[inline]
fn seed() -> u64 {
    unsafe {
        SEED
    }
}

fn default_panic_handler(_: &PanicInfo<'_>) {
    let _ = std::io::stderr().lock().write_fmt(
        format_args!("Suite failed with seed: {}\n", seed())
    );
}

#[inline]
pub fn reset_panic_handler() {
    let _ = std::panic::take_hook();
    register_panic_handler(default_panic_handler);
}

pub fn register_panic_handler<F: Fn(&std::panic::PanicInfo<'_>) + Send + Sync + 'static>(handler: F) {
    let parent_fn = std::panic::take_hook();
    let handler_fn = Box::new(handler);

    std::panic::set_hook(Box::new( move | info | {
        handler_fn(info);
        parent_fn(info);
    }));
}

#[ctor]
fn init() {
    unsafe {
        SEED = OsRng.next_u64();
        register_panic_handler(default_panic_handler);
    }
}

macro_rules! test_with_harness {
    (for $i:ident in $rng_l:literal..$rng_h:literal {$($loop_body:tt)+}) => {
        let mut __count: usize = $rng_h;
        test_with_harness!("Failed at iteration: {}\n", __count);

        for $i in $rng_l..$rng_h {
            $($loop_body)+
            __count += 1;
        }

        reset_panic_handler();
    };
    (for $i:ident in $rng_l:literal..$rng_h:literal {$($loop_body:tt)+}, $($formatter:tt)+) => {
        let mut __count: usize = $rng_h;
        test_with_harness!($($formatter)+, __count);

        for $i in $rng_l..$rng_h {
            $($loop_body)+
            __count += 1;
        }

        reset_panic_handler();
    };
    ($($formatter:tt)+) => {
        common::register_panic_handler(move |_| {
            let _ = std::io::stderr().lock().write_fmt(
                format_args!($($formatter)+)
            );
        });
    };
}