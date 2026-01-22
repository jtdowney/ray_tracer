use std::hint;

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::vector;

fn vector_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector");

    let v = vector(1.0, 2.0, 3.0);
    let v2 = vector(4.0, 5.0, 6.0);
    let normal = vector(0.0, 1.0, 0.0);

    group.bench_function("magnitude", |b| b.iter(|| hint::black_box(v).magnitude()));

    group.bench_function("normalize", |b| b.iter(|| hint::black_box(v).normalize()));

    group.bench_function("dot", |b| {
        b.iter(|| hint::black_box(v).dot(&hint::black_box(v2)));
    });

    group.bench_function("cross", |b| {
        b.iter(|| hint::black_box(v).cross(&hint::black_box(v2)));
    });

    group.bench_function("reflect", |b| {
        b.iter(|| hint::black_box(v).reflect(&hint::black_box(normal)));
    });

    group.bench_function("add", |b| {
        b.iter(|| hint::black_box(v) + hint::black_box(v2));
    });

    group.bench_function("sub", |b| {
        b.iter(|| hint::black_box(v) - hint::black_box(v2));
    });

    group.bench_function("mul_scalar", |b| {
        b.iter(|| hint::black_box(v) * hint::black_box(2.5));
    });

    group.bench_function("div_scalar", |b| {
        b.iter(|| hint::black_box(v) / hint::black_box(2.5));
    });

    group.bench_function("neg", |b| b.iter(|| -hint::black_box(v)));

    group.finish();
}

criterion_group!(benches, vector_benchmarks);
criterion_main!(benches);
