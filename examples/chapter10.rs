use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use anyhow::Result;
use ray_tracer::{
    Material, World, camera, color,
    pattern::{checkers_pattern, gradient_pattern, ring_pattern, stripe_pattern},
    point, point_light,
    shape::{plane, sphere},
    transform, vector,
};

fn main() -> Result<()> {
    let floor = plane().material(
        Material::builder()
            .pattern(
                checkers_pattern(color(0.9, 0.9, 0.9), color(0.1, 0.1, 0.1))
                    .transform(transform::scaling(0.5, 0.5, 0.5))
                    .build(),
            )
            .specular(0.0),
    );

    let backdrop = plane()
        .transform(transform::translation(0, 0, 5) * transform::rotation_x(FRAC_PI_2))
        .material(
            Material::builder()
                .pattern(
                    stripe_pattern(color(0.8, 0.8, 1), color(0.6, 0.6, 0.9))
                        .transform(transform::scaling(0.25, 0.25, 0.25))
                        .build(),
                )
                .specular(0.0),
        );

    let middle = sphere()
        .transform(transform::translation(-0.5, 1, 0.5))
        .material(
            Material::builder()
                .pattern(
                    stripe_pattern(color(0.1, 0.8, 0.2), color(0.1, 0.5, 0.1))
                        .transform(
                            transform::scaling(0.2, 0.2, 0.2) * transform::rotation_z(FRAC_PI_4),
                        )
                        .build(),
                )
                .diffuse(0.7)
                .specular(0.3),
        );

    let right = sphere()
        .transform(transform::translation(1.5, 0.5, -0.5) * transform::scaling(0.5, 0.5, 0.5))
        .material(
            Material::builder()
                .pattern(
                    gradient_pattern(color(1, 0.5, 0), color(0.5, 0, 0.5))
                        .transform(transform::translation(1, 0, 0) * transform::scaling(2, 2, 2))
                        .build(),
                )
                .diffuse(0.7)
                .specular(0.3),
        );

    let left = sphere()
        .transform(transform::translation(-1.5, 0.33, -0.75) * transform::scaling(0.33, 0.33, 0.33))
        .material(
            Material::builder()
                .pattern(
                    ring_pattern(color(0.9, 0.9, 0.1), color(0.4, 0.1, 0.1))
                        .transform(
                            transform::scaling(0.15, 0.15, 0.15) * transform::rotation_x(FRAC_PI_2),
                        )
                        .build(),
                )
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World::builder()
        .light(point_light(point(-10, 10, -10), color(1, 1, 1)))
        .objects(bon::vec![floor, backdrop, middle, right, left])
        .build();

    let camera = camera(1000, 500)
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
