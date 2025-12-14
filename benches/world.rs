use std::{f32::consts::FRAC_PI_3, hint};

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{REFLECTION_DEPTH, camera, default_world, point, ray, transform, vector};

fn world_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("world");

    let world = default_world();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

    group.bench_function("intersect", |b| {
        b.iter(|| world.intersect(hint::black_box(r)))
    });

    group.bench_function("color_at", |b| {
        b.iter(|| world.color_at(hint::black_box(r), hint::black_box(REFLECTION_DEPTH)));
    });

    group.bench_function("is_shadowed_for_light", |b| {
        b.iter(|| {
            world.is_shadowed_for_light(hint::black_box(point(0.0, 10.0, 0.0)), &world.lights[0])
        });
    });

    group.finish();
}

fn camera_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("camera");

    let cam = camera(100, 50).field_of_view(FRAC_PI_3).build();

    group.bench_function("ray_for_pixel_center", |b| {
        b.iter(|| cam.ray_for_pixel(hint::black_box(50), hint::black_box(25)));
    });

    group.bench_function("ray_for_pixel_corner", |b| {
        b.iter(|| cam.ray_for_pixel(hint::black_box(0), hint::black_box(0)));
    });

    let cam_transformed = camera(100, 50)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0),
        ))
        .build();

    group.bench_function("ray_for_pixel_transformed", |b| {
        b.iter(|| cam_transformed.ray_for_pixel(hint::black_box(50), hint::black_box(25)));
    });

    group.finish();
}

criterion_group!(benches, world_benchmarks, camera_benchmarks);
criterion_main!(benches);
