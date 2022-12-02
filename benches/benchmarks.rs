use criterion::{criterion_group, criterion_main, Criterion};

use rust_advent_of_code_2022::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Day 1
    c.bench_function("1-1", |b| b.iter(p1_1));
    c.bench_function("1-2", |b| b.iter(p1_2));
    // Day 2
    c.bench_function("2-1", |b| b.iter(p2_1));
    c.bench_function("2-2", |b| b.iter(p2_2));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);