use std::hint;

use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{Material, color, material, point, point_light, shape, vector};

fn lighting_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("lighting");

    let m = material();
    let object = shape::sphere().build();
    let position = point(0, 0, 0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 10.0, -10.0), color(1, 1, 1));

    group.bench_function("phong_not_shadowed", |b| {
        b.iter(|| {
            m.lighting(
                hint::black_box(&object),
                hint::black_box(&light),
                hint::black_box(position),
                hint::black_box(eyev),
                hint::black_box(normalv),
                hint::black_box(false),
            )
        });
    });

    group.bench_function("phong_shadowed", |b| {
        b.iter(|| {
            m.lighting(
                hint::black_box(&object),
                hint::black_box(&light),
                hint::black_box(position),
                hint::black_box(eyev),
                hint::black_box(normalv),
                hint::black_box(true),
            )
        });
    });

    let m_shiny = Material::builder().shininess(400.0).build();
    group.bench_function("phong_high_shininess", |b| {
        b.iter(|| {
            m_shiny.lighting(
                hint::black_box(&object),
                hint::black_box(&light),
                hint::black_box(position),
                hint::black_box(eyev),
                hint::black_box(normalv),
                hint::black_box(false),
            )
        });
    });

    group.finish();
}

criterion_group!(benches, lighting_benchmarks);
criterion_main!(benches);
