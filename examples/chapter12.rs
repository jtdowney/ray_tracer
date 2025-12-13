use std::f32::consts::{FRAC_PI_3, FRAC_PI_4};

use anyhow::Result;
use ray_tracer::{
    Material, Shape, World, camera, color, pattern::checkers_pattern, point, point_light,
    shape::cube, transform, vector,
};

fn table_material() -> Material {
    Material::builder()
        .color(color(0.55, 0.35, 0.15))
        .diffuse(0.7)
        .specular(0.3)
        .build()
}

fn build_room() -> Vec<Shape> {
    let floor = cube()
        .transform(transform::scaling(20, 0.1, 20))
        .material(
            Material::builder()
                .pattern(
                    checkers_pattern(color(0.1, 0.1, 0.1), color(0.9, 0.9, 0.9))
                        .transform(transform::scaling(0.1, 0.1, 0.1))
                        .build(),
                )
                .reflective(0.3)
                .specular(0.8),
        )
        .build();

    let back_wall = cube()
        .transform(transform::translation(0, 5, 10) * transform::scaling(20, 10, 0.1))
        .material(
            Material::builder()
                .color(color(0.3, 0.25, 0.2))
                .specular(0.0),
        )
        .build();

    let left_wall = cube()
        .transform(transform::translation(-10, 5, 0) * transform::scaling(0.1, 10, 20))
        .material(
            Material::builder()
                .color(color(0.35, 0.3, 0.25))
                .specular(0.0),
        )
        .build();

    let right_wall = cube()
        .transform(transform::translation(10, 5, 0) * transform::scaling(0.1, 10, 20))
        .material(
            Material::builder()
                .color(color(0.35, 0.3, 0.25))
                .specular(0.0),
        )
        .build();

    let ceiling = cube()
        .transform(transform::translation(0, 10, 0) * transform::scaling(20, 0.1, 20))
        .material(
            Material::builder()
                .color(color(0.4, 0.35, 0.3))
                .specular(0.0),
        )
        .build();

    vec![floor, back_wall, left_wall, right_wall, ceiling]
}

fn build_table() -> Vec<Shape> {
    let tabletop = cube()
        .transform(transform::translation(0, 3, 0) * transform::scaling(3, 0.15, 1.5))
        .material(table_material())
        .build();

    let leg_scale = transform::scaling(0.15, 1.5, 0.15);
    let legs: Vec<Shape> = [(2.5, 1.0), (-2.5, 1.0), (2.5, -1.0), (-2.5, -1.0)]
        .into_iter()
        .map(|(x, z)| {
            cube()
                .transform(transform::translation(x, 1.5, z) * leg_scale)
                .material(table_material())
                .build()
        })
        .collect();

    let mut result = vec![tabletop];
    result.extend(legs);
    result
}

fn build_table_cubes() -> Vec<Shape> {
    vec![
        cube()
            .transform(
                transform::translation(-1.5, 3.65, 0.5)
                    * transform::rotation_y(FRAC_PI_4 * 0.5)
                    * transform::scaling(0.5, 0.5, 0.5),
            )
            .material(
                Material::builder()
                    .color(color(0.8, 0.1, 0.1))
                    .diffuse(0.8)
                    .specular(0.4),
            )
            .build(),
        cube()
            .transform(
                transform::translation(0.0, 3.5, -0.3)
                    * transform::rotation_y(FRAC_PI_4 * 0.3)
                    * transform::scaling(0.35, 0.35, 0.35),
            )
            .material(
                Material::builder()
                    .color(color(0.1, 0.7, 0.2))
                    .diffuse(0.8)
                    .specular(0.4),
            )
            .build(),
        cube()
            .transform(
                transform::translation(1.2, 3.55, 0.6)
                    * transform::rotation_y(-FRAC_PI_4 * 0.7)
                    * transform::scaling(0.4, 0.4, 0.4),
            )
            .material(
                Material::builder()
                    .color(color(0.1, 0.2, 0.8))
                    .diffuse(0.8)
                    .specular(0.4),
            )
            .build(),
        cube()
            .transform(
                transform::translation(2.0, 3.4, -0.5)
                    * transform::rotation_y(FRAC_PI_4 * 1.2)
                    * transform::scaling(0.25, 0.25, 0.25),
            )
            .material(
                Material::builder()
                    .color(color(0.9, 0.8, 0.1))
                    .diffuse(0.8)
                    .specular(0.4),
            )
            .build(),
    ]
}

fn build_floor_boxes() -> Vec<Shape> {
    vec![
        cube()
            .transform(
                transform::translation(-4.0, 0.4, -2.0)
                    * transform::rotation_y(FRAC_PI_4 * 0.8)
                    * transform::scaling(0.4, 0.4, 0.4),
            )
            .material(
                Material::builder()
                    .color(color(0.6, 0.3, 0.1))
                    .diffuse(0.7)
                    .specular(0.2),
            )
            .build(),
        cube()
            .transform(
                transform::translation(4.5, 0.6, 1.5)
                    * transform::rotation_y(-FRAC_PI_4 * 1.3)
                    * transform::scaling(0.6, 0.6, 0.6),
            )
            .material(
                Material::builder()
                    .color(color(0.4, 0.4, 0.5))
                    .diffuse(0.7)
                    .specular(0.2),
            )
            .build(),
        cube()
            .transform(
                transform::translation(-3.5, 0.25, 3.0)
                    * transform::rotation_y(FRAC_PI_4 * 2.1)
                    * transform::scaling(0.25, 0.25, 0.25),
            )
            .material(
                Material::builder()
                    .color(color(0.7, 0.5, 0.6))
                    .diffuse(0.7)
                    .specular(0.2),
            )
            .build(),
    ]
}

fn main() -> Result<()> {
    let mut objects = build_room();
    objects.extend(build_table());
    objects.extend(build_table_cubes());
    objects.extend(build_floor_boxes());

    let world = World::builder()
        .light(point_light(point(-8, 8, -8), color(1, 1, 1)))
        .objects(objects)
        .build();

    let camera = camera(1000, 500)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(-6, 5, -8),
            point(0, 3, 0),
            vector(0, 1, 0),
        ))
        .build();

    let canvas = camera.render(&world)?;
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
