use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// ✅ Correct benchmark function
fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// ✅ Use the correct function name inside `criterion_group!`
criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);