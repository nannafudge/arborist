#![cfg(feature = "bench")]
use criterion::{
    black_box, criterion_group, criterion_main,
    Criterion
};
use rand::{SeedableRng, RngCore};
use rand::rngs::{SmallRng, OsRng};

const USIZE_MIDPOINT: usize = (usize::BITS >> 1) as usize;

#[inline(always)]
pub fn lsb(i: &usize) -> usize {
    i & (!i).overflowing_add(1).0
}

#[inline(always)]
fn height(length: &usize) -> usize {
    let mut mid: usize = USIZE_MIDPOINT;
    let mut cur: usize = USIZE_MIDPOINT;

    while mid > 1 {
        match length >> cur {
            1 => break,
            0 => cur -= { mid >>= 1; mid },
            _ => cur += { mid >>= 1; mid },
        }
    }

    cur + (&lsb(length) != length) as usize
}

fn bench_fenwick<const N: usize>(b: &mut Criterion) {
    let mut group = b.benchmark_group("Fenwick");
    let mut randomness: SmallRng = SmallRng::seed_from_u64(OsRng.next_u64());

    for _ in 0..N {
        let next_val: usize = randomness.next_u32() as usize;

        group.bench_with_input("log2_ceil", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box((*value as f64).log2().ceil() as usize)
            });
        });
    
        group.bench_with_input("log2_next_pow2_ilog2", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box((value.next_power_of_two()).ilog2() as usize)
            });
        });

        
        group.bench_with_input("log2_next_pow2_trailing_zeros", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box((value.next_power_of_two()).trailing_zeros() as usize)
            });
        });

        group.bench_with_input("log2_leading_zeros_invert", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box(usize::MAX - value.leading_zeros() as usize)
            });
        });

        group.bench_with_input("log2_bin_search", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box(height(value))
            });
        });

        group.bench_with_input("lsb_cast", &next_val, | bencher, value | {
            bencher.iter(|| {
                let raw = *value as isize;
                black_box((raw & -raw) as usize)
            });
        });
    
        group.bench_with_input("lsb_overflow", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box(value & (!value).overflowing_add(1).0)
            });
        });

        group.bench_with_input("lsb_xor", &next_val, | bencher, value | {
            bencher.iter(|| {
                black_box(value & (value ^ (value.min(&1) - 1)))
            });
        });
    }

    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().without_plots().sample_size(10);
    targets = bench_fenwick::<8>,
}
criterion_main!{benches}