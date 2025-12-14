use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

use anyhow::Result;
use ray_tracer::{
    Material, Shape, World, camera, color,
    pattern::checkers_pattern,
    point, point_light,
    shape::{cone, cylinder, plane},
    transform, vector,
};

fn build_floor() -> Shape {
    plane()
        .material(
            Material::builder()
                .pattern(
                    checkers_pattern(color(0.15, 0.15, 0.15), color(0.85, 0.85, 0.85))
                        .transform(transform::scaling(0.5, 0.5, 0.5))
                        .build(),
                )
                .reflective(0.2)
                .specular(0.0),
        )
        .build()
}

fn build_pillars() -> Vec<Shape> {
    let pillar_material = Material::builder()
        .color(color(0.8, 0.8, 0.85))
        .diffuse(0.7)
        .specular(0.3)
        .reflective(0.1)
        .build();

    vec![
        cylinder()
            .minimum(0.0)
            .maximum(3.0)
            .closed(true)
            .transform(transform::translation(-3.0, 0.0, 3.0) * transform::scaling(0.4, 1.0, 0.4))
            .material(pillar_material.clone())
            .build(),
        cylinder()
            .minimum(0.0)
            .maximum(3.0)
            .closed(true)
            .transform(transform::translation(3.0, 0.0, 3.0) * transform::scaling(0.4, 1.0, 0.4))
            .material(pillar_material.clone())
            .build(),
        cylinder()
            .minimum(0.0)
            .maximum(2.5)
            .closed(true)
            .transform(
                transform::translation(-3.0, 0.0, -2.0) * transform::scaling(0.35, 1.0, 0.35),
            )
            .material(pillar_material.clone())
            .build(),
        cylinder()
            .minimum(0.0)
            .maximum(2.5)
            .closed(true)
            .transform(transform::translation(3.0, 0.0, -2.0) * transform::scaling(0.35, 1.0, 0.35))
            .material(pillar_material)
            .build(),
    ]
}

fn build_cones() -> Vec<Shape> {
    vec![
        cone()
            .minimum(-1.0)
            .maximum(0.0)
            .closed(true)
            .transform(transform::translation(0.0, 1.0, 0.0))
            .material(
                Material::builder()
                    .color(color(0.9, 0.2, 0.1))
                    .diffuse(0.8)
                    .specular(0.5),
            )
            .build(),
        cone()
            .minimum(0.0)
            .maximum(0.5)
            .closed(true)
            .transform(transform::translation(-1.5, 0.5, 1.0) * transform::scaling(1.0, 1.0, 1.0))
            .material(
                Material::builder()
                    .color(color(0.1, 0.6, 0.2))
                    .diffuse(0.7)
                    .specular(0.4),
            )
            .build(),
        cone()
            .minimum(-0.5)
            .maximum(0.5)
            .closed(true)
            .transform(transform::translation(1.5, 0.5, 1.0) * transform::scaling(0.8, 1.0, 0.8))
            .material(
                Material::builder()
                    .color(color(0.1, 0.3, 0.8))
                    .diffuse(0.7)
                    .specular(0.4),
            )
            .build(),
    ]
}

fn build_decorative_cylinders() -> Vec<Shape> {
    vec![
        cylinder()
            .minimum(0.0)
            .maximum(0.3)
            .closed(true)
            .transform(
                transform::translation(0.0, 0.0, 2.0)
                    * transform::rotation_z(FRAC_PI_2)
                    * transform::scaling(0.2, 2.0, 0.2),
            )
            .material(
                Material::builder()
                    .color(color(0.6, 0.4, 0.1))
                    .diffuse(0.6)
                    .specular(0.3),
            )
            .build(),
        cylinder()
            .minimum(0.0)
            .maximum(1.5)
            .closed(false)
            .transform(
                transform::translation(-2.0, 0.0, -1.0) * transform::scaling(0.15, 1.0, 0.15),
            )
            .material(
                Material::builder()
                    .color(color(0.7, 0.5, 0.8))
                    .diffuse(0.8)
                    .specular(0.2)
                    .transparency(0.5)
                    .refractive_index(1.5),
            )
            .build(),
        cylinder()
            .minimum(0.0)
            .maximum(1.0)
            .closed(true)
            .transform(transform::translation(2.0, 0.0, -1.0) * transform::scaling(0.3, 1.0, 0.3))
            .material(
                Material::builder()
                    .color(color(0.3, 0.7, 0.7))
                    .diffuse(0.7)
                    .specular(0.6)
                    .reflective(0.3),
            )
            .build(),
    ]
}

fn build_glass_cylinder() -> Shape {
    cylinder()
        .minimum(0.0)
        .maximum(2.0)
        .closed(true)
        .transform(transform::translation(0.0, 0.0, -2.5) * transform::scaling(0.5, 1.0, 0.5))
        .material(
            Material::builder()
                .color(color(0.1, 0.1, 0.1))
                .diffuse(0.1)
                .specular(0.9)
                .shininess(300.0)
                .transparency(0.9)
                .refractive_index(1.5)
                .reflective(0.9),
        )
        .build()
}

fn main() -> Result<()> {
    let mut objects = vec![build_floor(), build_glass_cylinder()];
    objects.extend(build_pillars());
    objects.extend(build_cones());
    objects.extend(build_decorative_cylinders());

    let world = World::builder()
        .lights(vec![point_light(point(-5, 5, -5), color(1, 1, 1))])
        .objects(objects)
        .build();

    let camera = camera(1000, 500)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0, 3, -6),
            point(0, 1, 0),
            vector(0, 1, 0),
        ))
        .parallel(true)
        .build();

    let canvas = camera.render(&world);
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
