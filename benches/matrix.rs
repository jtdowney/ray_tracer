use std::{f64::consts::PI, hint};

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{point, transform, vector};

fn matrix_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix");

    let m1 = transform::rotation_x(PI / 4.0) * transform::translation(1.0, 2.0, 3.0);
    let m2 = transform::rotation_y(PI / 3.0) * transform::scaling(2.0, 2.0, 2.0);
    let p = point(1.0, 2.0, 3.0);
    let v = vector(1.0, 0.0, 0.0);

    group.bench_function("multiply", |b| {
        b.iter(|| hint::black_box(m1) * hint::black_box(m2))
    });

    group.bench_function("inverse", |b| b.iter(|| hint::black_box(m1).inverse()));

    group.bench_function("transpose", |b| b.iter(|| hint::black_box(m1).transpose()));

    group.bench_function("determinant", |b| {
        b.iter(|| hint::black_box(m1).determinant())
    });

    group.bench_function("transform_point", |b| {
        b.iter(|| hint::black_box(m1) * hint::black_box(p));
    });

    group.bench_function("transform_vector", |b| {
        b.iter(|| hint::black_box(m1) * hint::black_box(v));
    });

    group.finish();
}

fn transform_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform");

    group.bench_function("translation", |b| {
        b.iter(|| {
            transform::translation(
                hint::black_box(1.0),
                hint::black_box(2.0),
                hint::black_box(3.0),
            )
        });
    });

    group.bench_function("scaling", |b| {
        b.iter(|| {
            transform::scaling(
                hint::black_box(2.0),
                hint::black_box(2.0),
                hint::black_box(2.0),
            )
        });
    });

    group.bench_function("rotation_x", |b| {
        b.iter(|| transform::rotation_x(hint::black_box(PI / 4.0)));
    });

    group.bench_function("rotation_y", |b| {
        b.iter(|| transform::rotation_y(hint::black_box(PI / 4.0)));
    });

    group.bench_function("rotation_z", |b| {
        b.iter(|| transform::rotation_z(hint::black_box(PI / 4.0)));
    });

    group.bench_function("view_transform", |b| {
        b.iter(|| {
            transform::view_transform(
                hint::black_box(point(0.0, 1.5, -5.0)),
                hint::black_box(point(0.0, 1.0, 0.0)),
                hint::black_box(vector(0.0, 1.0, 0.0)),
            )
        });
    });

    group.finish();
}

criterion_group!(benches, matrix_benchmarks, transform_benchmarks);
criterion_main!(benches);
