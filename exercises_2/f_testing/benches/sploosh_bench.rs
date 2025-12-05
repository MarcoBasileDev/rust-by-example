use std::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main};
use f_testing::*;

fn bench_sploosh(c: &mut Criterion) {
    c.bench_function("sploosh 8 9 10", |b| {
        b.iter(|| sploosh(black_box(8), black_box(9), black_box(10)))
    });
}

criterion_group!(benches, bench_sploosh);
criterion_main!(benches);
