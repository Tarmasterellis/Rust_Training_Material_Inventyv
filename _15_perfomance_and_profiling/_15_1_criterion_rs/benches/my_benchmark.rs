use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, rng};

fn sort_benchmark(c: &mut Criterion) {
    let mut rng = rng();
    let data: Vec<u32> = (0..10_000).map(|_| rng.random()).collect();

    c.bench_function("sorting", |b| {
        b.iter(|| {
            let mut cloned_data = data.clone();
            cloned_data.sort();
        })
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
