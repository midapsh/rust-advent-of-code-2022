use criterion::{criterion_group, criterion_main, Criterion};

use rust_advent_of_code_2022::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1-1", |b| b.iter(p1_1));
    c.bench_function("1-2", |b| b.iter(p1_2));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);