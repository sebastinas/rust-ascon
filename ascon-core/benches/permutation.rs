use ascon_core::State;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, RngCore, SeedableRng};

fn criterion_bench_permutation(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(0x0123456789abcdef);
    let mut state = State::new(
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
    );

    let mut c = c.benchmark_group("Permutation");
    c.bench_function("1 round", |b| {
        b.iter(|| {
            state.permute_1();
        })
    });

    c.bench_function("6 rounds", |b| {
        b.iter(|| {
            state.permute_6();
        })
    });

    c.bench_function("8 rounds", |b| {
        b.iter(|| {
            state.permute_8();
        })
    });

    c.bench_function("12 rounds", |b| {
        b.iter(|| {
            state.permute_12();
        })
    });
    c.finish();
}

criterion_group!(bench_permutation, criterion_bench_permutation);
criterion_main!(bench_permutation);
