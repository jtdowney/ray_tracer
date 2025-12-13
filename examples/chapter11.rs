use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

use anyhow::Result;
use ray_tracer::{
    Material, World, camera, color,
    pattern::checkers_pattern,
    point, point_light,
    shape::{plane, sphere},
    transform, vector,
};

fn main() -> Result<()> {
    let floor = plane()
        .material(
            Material::builder()
                .pattern(checkers_pattern(color(0.35, 0.35, 0.35), color(0.65, 0.65, 0.65)).build())
                .reflective(0.4)
                .specular(0.0),
        )
        .build();

    let backdrop = plane()
        .transform(transform::translation(0, 0, 10) * transform::rotation_x(FRAC_PI_2))
        .material(
            Material::builder()
                .pattern(checkers_pattern(color(0.15, 0.15, 0.15), color(0.85, 0.85, 0.85)).build())
                .specular(0.0),
        )
        .build();

    let glass_sphere = sphere()
        .transform(transform::translation(0, 1, 0))
        .material(
            Material::builder()
                .color(color(0.05, 0.05, 0.05))
                .diffuse(0.1)
                .specular(1.0)
                .shininess(300)
                .reflective(0.9)
                .transparency(0.9)
                .refractive_index(1.5),
        )
        .build();

    let inner_sphere = sphere()
        .transform(transform::translation(0, 1, 0) * transform::scaling(0.5, 0.5, 0.5))
        .material(
            Material::builder()
                .color(color(1, 0, 0))
                .ambient(0.5)
                .diffuse(0.9)
                .specular(0.9)
                .shininess(200),
        )
        .build();

    let right_sphere = sphere()
        .transform(transform::translation(2, 0.5, -1) * transform::scaling(0.5, 0.5, 0.5))
        .material(
            Material::builder()
                .color(color(0.1, 0.1, 0.8))
                .diffuse(0.9)
                .specular(0.9)
                .shininess(200)
                .reflective(0.1),
        )
        .build();

    let left_sphere = sphere()
        .transform(transform::translation(-2, 0.75, -0.5) * transform::scaling(0.75, 0.75, 0.75))
        .material(
            Material::builder()
                .color(color(0.1, 0.6, 0.1))
                .diffuse(0.9)
                .specular(0.9)
                .shininess(200)
                .reflective(0.1),
        )
        .build();

    let world = World::builder()
        .light(point_light(point(-10, 10, -10), color(1, 1, 1)))
        .objects(vec![
            floor,
            backdrop,
            glass_sphere,
            inner_sphere,
            right_sphere,
            left_sphere,
        ])
        .build();

    let camera = camera(1000, 400)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0, 1.5, -5),
            point(0, 1, 0),
            vector(0, 1, 0),
        ))
        .build();

    let canvas = camera.render(&world)?;
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
