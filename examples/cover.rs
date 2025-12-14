use std::f32::consts::FRAC_PI_2;

use anyhow::Result;
use ray_tracer::{
    Material, Matrix4, Shape, World, camera, color, point, point_light,
    shape::{cube, plane, sphere},
    transform, vector,
};

fn standard_transform() -> Matrix4 {
    transform::translation(1, -1, 1) * transform::scaling(0.5, 0.5, 0.5)
}

fn large_object() -> Matrix4 {
    standard_transform() * transform::scaling(3.5, 3.5, 3.5)
}

fn medium_object() -> Matrix4 {
    standard_transform() * transform::scaling(3, 3, 3)
}

fn small_object() -> Matrix4 {
    standard_transform() * transform::scaling(2, 2, 2)
}

fn white_material() -> Material {
    Material::builder()
        .color(color(1, 1, 1))
        .diffuse(0.7)
        .ambient(0.1)
        .specular(0.0)
        .reflective(0.1)
        .build()
}

fn blue_material() -> Material {
    Material::builder()
        .color(color(0.537, 0.831, 0.914))
        .diffuse(0.7)
        .ambient(0.1)
        .specular(0.0)
        .reflective(0.1)
        .build()
}

fn red_material() -> Material {
    Material::builder()
        .color(color(0.941, 0.322, 0.388))
        .diffuse(0.7)
        .ambient(0.1)
        .specular(0.0)
        .reflective(0.1)
        .build()
}

fn purple_material() -> Material {
    Material::builder()
        .color(color(0.373, 0.404, 0.550))
        .diffuse(0.7)
        .ambient(0.1)
        .specular(0.0)
        .reflective(0.1)
        .build()
}

fn backdrop() -> Shape {
    plane()
        .material(
            Material::builder()
                .color(color(1, 1, 1))
                .ambient(1.0)
                .diffuse(0.0)
                .specular(0.0),
        )
        .transform(transform::translation(0, 0, 500) * transform::rotation_x(FRAC_PI_2))
        .build()
}

fn glass_sphere() -> Shape {
    sphere()
        .material(
            Material::builder()
                .color(color(0.373, 0.404, 0.550))
                .diffuse(0.2)
                .ambient(0.0)
                .specular(1.0)
                .shininess(200.0)
                .reflective(0.7)
                .transparency(0.7)
                .refractive_index(1.5),
        )
        .transform(large_object())
        .build()
}

fn main() -> Result<()> {
    // Main light
    let light = point_light(point(50, 100, -50), color(1, 1, 1));
    // Secondary light (not supported yet): point(-400, 50, -10), intensity (0.2, 0.2, 0.2)

    let objects = vec![
        backdrop(),
        glass_sphere(),
        // Row 1 cubes
        cube()
            .material(white_material())
            .transform(transform::translation(4, 0, 0) * medium_object())
            .build(),
        cube()
            .material(blue_material())
            .transform(transform::translation(8.5, 1.5, -0.5) * large_object())
            .build(),
        cube()
            .material(red_material())
            .transform(transform::translation(0, 0, 4) * large_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(4, 0, 4) * small_object())
            .build(),
        cube()
            .material(purple_material())
            .transform(transform::translation(7.5, 0.5, 4) * medium_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(-0.25, 0.25, 8) * medium_object())
            .build(),
        cube()
            .material(blue_material())
            .transform(transform::translation(4, 1, 7.5) * large_object())
            .build(),
        cube()
            .material(red_material())
            .transform(transform::translation(10, 2, 7.5) * medium_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(8, 2, 12) * small_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(20, 1, 9) * small_object())
            .build(),
        // Lower cubes
        cube()
            .material(blue_material())
            .transform(transform::translation(-0.5, -5, 0.25) * large_object())
            .build(),
        cube()
            .material(red_material())
            .transform(transform::translation(4, -4, 0) * large_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(8.5, -4, 0) * large_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(0, -4, 4) * large_object())
            .build(),
        cube()
            .material(purple_material())
            .transform(transform::translation(-0.5, -4.5, 8) * large_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(0, -8, 4) * large_object())
            .build(),
        cube()
            .material(white_material())
            .transform(transform::translation(-0.5, -8.5, 8) * large_object())
            .build(),
    ];

    let world = World::builder().light(light).objects(objects).build();

    let camera = camera(800, 800)
        .field_of_view(0.785)
        .transform(transform::view_transform(
            point(-6, 6, -10),
            point(6, 0, 6),
            vector(-0.45, 1, 0),
        ))
        .parallel(true)
        .build();

    let canvas = camera.render(&world);
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
