use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum_numbers(n: usize) -> u64 {
    (0..=n as u64).sum()
}

// ✅ Correct benchmark function
fn sum_numbers_benchmark(c: &mut Criterion) {
    c.bench_function("sum_numbers 1000", |b| b.iter(|| sum_numbers(black_box(1000))));
}

// ✅ Use the correct function name inside `criterion_group!`
criterion_group!(benches, sum_numbers_benchmark);
criterion_main!(benches);