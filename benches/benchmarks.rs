use criterion::{criterion_group, criterion_main, Criterion};

use rust_advent_of_code_2022::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Day 1
    c.bench_function("1-1", |b| b.iter(p1_1));
    c.bench_function("1-2", |b| b.iter(p1_2));
    // Day 2
    c.bench_function("2-1", |b| b.iter(p2_1));
    c.bench_function("2-2", |b| b.iter(p2_2));
    // Day 3
    c.bench_function("3-1", |b| b.iter(p3_1));
    c.bench_function("3-2", |b| b.iter(p3_2));
    // Day 4
    c.bench_function("4-1", |b| b.iter(p4_1));
    c.bench_function("4-2", |b| b.iter(p4_2));
    // Day 5
    c.bench_function("5-1", |b| b.iter(p5_1));
    c.bench_function("5-2", |b| b.iter(p5_2));
    // Day 6
    c.bench_function("6-1", |b| b.iter(p6_1));
    c.bench_function("6-2", |b| b.iter(p6_2));
    // Day 7
    c.bench_function("7-1", |b| b.iter(p7_1));
    c.bench_function("7-2", |b| b.iter(p7_2));
    // Day 8
    c.bench_function("8-1", |b| b.iter(p8_1));
    c.bench_function("8-2", |b| b.iter(p8_2));
    // Day 9
    c.bench_function("9-1", |b| b.iter(p9_1));
    c.bench_function("9-2", |b| b.iter(p9_2));
    // Day 10
    c.bench_function("10-1", |b| b.iter(p10_1));
    c.bench_function("10-2", |b| b.iter(p10_2));
    // Day 11
    c.bench_function("11-1", |b| b.iter(p11_1));
    c.bench_function("11-2", |b| b.iter(p11_2));
    // Day 12
    c.bench_function("12-1", |b| b.iter(p12_1));
    c.bench_function("12-2", |b| b.iter(p12_2));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
