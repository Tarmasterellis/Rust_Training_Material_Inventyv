use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, rng};

fn sort_stable(c: &mut Criterion) {
    let mut rng = rng();
    let data: Vec<u32> = (0..10_000).map(|_| rng.random()).collect();

    c.bench_function("stable_sort", |b| {
        b.iter(|| {
            let mut cloned_data = data.clone();
            cloned_data.sort(); // Stable sort
        })
    });
}

fn sort_unstable(c: &mut Criterion) {
    let mut rng = rng();
    let data: Vec<u32> = (0..10_000).map(|_| rng.random()).collect();

    c.bench_function("unstable_sort", |b| {
        b.iter(|| {
            let mut cloned_data = data.clone();
            cloned_data.sort_unstable(); // Unstable sort
        })
    });
}

criterion_group!(benches, sort_stable, sort_unstable);
criterion_main!(benches);
