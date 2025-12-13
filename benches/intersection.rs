use std::{f32::consts::FRAC_1_SQRT_2, hint};

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{hit, intersection, point, ray, shape, transform, vector};

fn intersection_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersection");

    let sphere = shape::sphere()
        .transform(transform::translation(0.0, 1.0, 0.0))
        .build();
    let plane = shape::plane().build();
    let cube = shape::cube().build();
    let cylinder = shape::cylinder().build();
    let cone = shape::cone().build();

    let ray_hitting_sphere = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let ray_hitting_plane = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
    let ray_hitting_cube = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let ray_hitting_cylinder = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let ray_hitting_cone = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

    group.bench_function("sphere_intersect", |b| {
        b.iter(|| sphere.intersect(hint::black_box(ray_hitting_sphere)));
    });

    group.bench_function("plane_intersect", |b| {
        b.iter(|| plane.intersect(hint::black_box(ray_hitting_plane)));
    });

    group.bench_function("cube_intersect", |b| {
        b.iter(|| cube.intersect(hint::black_box(ray_hitting_cube)));
    });

    group.bench_function("cylinder_intersect", |b| {
        b.iter(|| cylinder.intersect(hint::black_box(ray_hitting_cylinder)));
    });

    group.bench_function("cone_intersect", |b| {
        b.iter(|| cone.intersect(hint::black_box(ray_hitting_cone)));
    });

    group.finish();
}

fn hit_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("hit");

    let sphere = shape::sphere().build();

    let intersections_few = vec![
        intersection(1.0, sphere.clone()),
        intersection(2.0, sphere.clone()),
    ];

    let intersections_many: Vec<_> = (0..10).map(|i| intersection(i, sphere.clone())).collect();

    group.bench_function("hit_few", |b| {
        b.iter(|| hit(hint::black_box(intersections_few.clone())));
    });

    group.bench_function("hit_many", |b| {
        b.iter(|| hit(hint::black_box(intersections_many.clone())));
    });

    group.finish();
}

fn normal_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal");

    let sphere = shape::sphere()
        .transform(transform::translation(0.0, 1.0, 0.0))
        .build();
    let plane = shape::plane().build();
    let cube = shape::cube().build();

    let sphere_point = point(0.0, 1.70711, -FRAC_1_SQRT_2);
    let plane_point = point(0.0, 0.0, 0.0);
    let cube_point = point(1.0, 0.5, -0.8);

    group.bench_function("sphere_normal", |b| {
        b.iter(|| sphere.normal_at(hint::black_box(sphere_point)));
    });

    group.bench_function("plane_normal", |b| {
        b.iter(|| plane.normal_at(hint::black_box(plane_point)));
    });

    group.bench_function("cube_normal", |b| {
        b.iter(|| cube.normal_at(hint::black_box(cube_point)));
    });

    group.finish();
}

criterion_group!(
    benches,
    intersection_benchmarks,
    hit_benchmarks,
    normal_benchmarks
);
criterion_main!(benches);
