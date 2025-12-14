use std::f32::consts::FRAC_PI_3;

use anyhow::Result;
use ray_tracer::{
    Color, Material, Shape, World, camera, color,
    pattern::checkers_pattern,
    point, point_light,
    shape::{CsgOperation, csg, cube, sphere},
    transform, vector,
};

const PIP_RADIUS: f32 = 0.2;
const PIP_OFFSET: f32 = 0.5;
const CORNER_RADIUS: f32 = 1.7;

fn pip_sphere(pip_color: Color) -> Shape {
    sphere()
        .material(
            Material::builder()
                .color(pip_color)
                .ambient(0.3)
                .diffuse(0.7),
        )
        .build()
}

fn combine_shapes(shapes: Vec<Shape>) -> Shape {
    shapes
        .into_iter()
        .reduce(|acc, s| csg(CsgOperation::Union, &acc, &s))
        .expect("at least one shape")
}

fn face_pips(pip_color: Color, positions: &[(f32, f32, f32)]) -> Vec<Shape> {
    positions
        .iter()
        .map(|&(x, y, z)| {
            let pip = pip_sphere(pip_color);
            pip.set_transform(
                transform::translation(x, y, z)
                    * transform::scaling(PIP_RADIUS, PIP_RADIUS, PIP_RADIUS),
            );
            pip
        })
        .collect()
}

fn face_1_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(pip_color, &[(0.0, 1.1, 0.0)])
}

fn face_6_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(
        pip_color,
        &[
            (-PIP_OFFSET, -1.1, -PIP_OFFSET),
            (-PIP_OFFSET, -1.1, 0.0),
            (-PIP_OFFSET, -1.1, PIP_OFFSET),
            (PIP_OFFSET, -1.1, -PIP_OFFSET),
            (PIP_OFFSET, -1.1, 0.0),
            (PIP_OFFSET, -1.1, PIP_OFFSET),
        ],
    )
}

fn face_2_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(
        pip_color,
        &[
            (-PIP_OFFSET, -PIP_OFFSET, 1.1),
            (PIP_OFFSET, PIP_OFFSET, 1.1),
        ],
    )
}

fn face_5_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(
        pip_color,
        &[
            (-PIP_OFFSET, -PIP_OFFSET, -1.1),
            (-PIP_OFFSET, PIP_OFFSET, -1.1),
            (0.0, 0.0, -1.1),
            (PIP_OFFSET, -PIP_OFFSET, -1.1),
            (PIP_OFFSET, PIP_OFFSET, -1.1),
        ],
    )
}

fn face_3_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(
        pip_color,
        &[
            (1.1, -PIP_OFFSET, -PIP_OFFSET),
            (1.1, 0.0, 0.0),
            (1.1, PIP_OFFSET, PIP_OFFSET),
        ],
    )
}

fn face_4_pips(pip_color: Color) -> Vec<Shape> {
    face_pips(
        pip_color,
        &[
            (-1.1, -PIP_OFFSET, -PIP_OFFSET),
            (-1.1, -PIP_OFFSET, PIP_OFFSET),
            (-1.1, PIP_OFFSET, -PIP_OFFSET),
            (-1.1, PIP_OFFSET, PIP_OFFSET),
        ],
    )
}

fn rounded_cube(die_color: Color) -> Shape {
    let die_material = Material::builder()
        .color(die_color)
        .diffuse(0.6)
        .specular(0.6)
        .shininess(100.0)
        .reflective(0.1)
        .build();

    let body = cube().material(die_material.clone()).build();

    let rounder = sphere().material(die_material).build();
    rounder.set_transform(transform::scaling(
        CORNER_RADIUS,
        CORNER_RADIUS,
        CORNER_RADIUS,
    ));

    csg(CsgOperation::Intersection, &body, &rounder)
}

fn die(die_color: Color, pip_color: Color) -> Shape {
    let body = rounded_cube(die_color);

    let all_pips = [
        face_1_pips(pip_color),
        face_2_pips(pip_color),
        face_3_pips(pip_color),
        face_4_pips(pip_color),
        face_5_pips(pip_color),
        face_6_pips(pip_color),
    ]
    .into_iter()
    .flatten()
    .collect();

    let combined_pips = combine_shapes(all_pips);
    csg(CsgOperation::Difference, &body, &combined_pips)
}

fn build_floor() -> Shape {
    ray_tracer::shape::plane()
        .material(
            Material::builder()
                .pattern(
                    checkers_pattern(color(0.3, 0.3, 0.3), color(0.7, 0.7, 0.7))
                        .transform(transform::scaling(0.5, 0.5, 0.5))
                        .build(),
                )
                .reflective(0.1)
                .specular(0.0),
        )
        .build()
}

fn main() -> Result<()> {
    // Bottom-left die (blue)
    let blue_die = die(color(0.1, 0.2, 0.6), color(1, 1, 1));
    blue_die.set_transform(
        transform::translation(-0.75, 0.5, 0.25)
            * transform::rotation_y(0.5)
            * transform::scaling(0.5, 0.5, 0.5),
    );

    // Bottom-right die (maroon)
    let maroon_die = die(color(0.5, 0.1, 0.15), color(1, 1, 1));
    maroon_die.set_transform(
        transform::translation(0.75, 0.5, 0.15)
            * transform::rotation_y(-0.4)
            * transform::scaling(0.5, 0.5, 0.5),
    );

    // Top die (green) - resting on top of the other two, nearly level
    let green_die = die(color(0.1, 0.4, 0.2), color(1, 1, 1));
    green_die.set_transform(
        transform::translation(0.0, 1.5, 0.45)
            * transform::rotation_y(0.3)
            * transform::rotation_x(0.05)
            * transform::rotation_z(0.05)
            * transform::scaling(0.5, 0.5, 0.5),
    );

    let world = World::builder()
        .light(point_light(point(-5, 8, -5), color(1, 1, 1)))
        .objects(vec![build_floor(), blue_die, maroon_die, green_die])
        .build();

    let camera = camera(1000, 500)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0, 4, -4),
            point(0, 0.5, 0.5),
            vector(0, 1, 0),
        ))
        .parallel(true)
        .build();

    let canvas = camera.render(&world);
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
