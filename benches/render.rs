use std::{f32::consts::FRAC_PI_3, hint};

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{Material, World, camera, color, point, point_light, shape, transform, vector};

fn build_test_world() -> World {
    let floor = shape::plane()
        .material(Material::builder().color(color(1.0, 0.9, 0.9)))
        .build();

    let sphere = shape::sphere()
        .transform(transform::translation(-0.5, 1.0, 0.5))
        .material(
            Material::builder()
                .color(color(0.1, 1.0, 0.5))
                .diffuse(0.7)
                .specular(0.3),
        )
        .build();

    World::builder()
        .objects(vec![floor, sphere])
        .light(point_light(point(-10.0, 10.0, -10.0), color(1, 1, 1)))
        .build()
}

fn build_camera(width: u16, height: u16, parallel: bool) -> ray_tracer::Camera {
    camera(width, height)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0),
        ))
        .parallel(parallel)
        .build()
}

fn sequential_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_sequential");

    let world = build_test_world();

    group.sample_size(10);
    let cam_small = build_camera(100, 50, false);
    group.bench_function("small_100x50", |b| {
        b.iter(|| cam_small.render(hint::black_box(&world)));
    });

    group.sample_size(10);
    let cam_medium = build_camera(400, 200, false);
    group.bench_function("medium_400x200", |b| {
        b.iter(|| cam_medium.render(hint::black_box(&world)));
    });

    group.sample_size(10);
    let cam_large = build_camera(1000, 500, false);
    group.bench_function("large_1000x500", |b| {
        b.iter(|| cam_large.render(hint::black_box(&world)));
    });

    group.finish();
}

fn parallel_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_parallel");

    let world = build_test_world();

    group.sample_size(10);
    let cam_small = build_camera(100, 50, true);
    group.bench_function("small_100x50", |b| {
        b.iter(|| cam_small.render(hint::black_box(&world)));
    });

    group.sample_size(10);
    let cam_medium = build_camera(400, 200, true);
    group.bench_function("medium_400x200", |b| {
        b.iter(|| cam_medium.render(hint::black_box(&world)));
    });

    group.sample_size(10);
    let cam_large = build_camera(1000, 500, true);
    group.bench_function("large_1000x500", |b| {
        b.iter(|| cam_large.render(hint::black_box(&world)));
    });

    group.finish();
}

criterion_group!(benches, sequential_benchmarks, parallel_benchmarks);
criterion_main!(benches);
