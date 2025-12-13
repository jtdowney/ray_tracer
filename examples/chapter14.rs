use std::f32::consts::{FRAC_PI_3, FRAC_PI_6};

use anyhow::Result;
use ray_tracer::{
    Material, Shape, World, camera, color,
    pattern::checkers_pattern,
    point, point_light,
    shape::{group, plane, sphere},
    transform, vector,
};

fn hexagon_corner() -> Shape {
    sphere()
        .transform(transform::translation(0, 0, -1) * transform::scaling(0.25, 0.25, 0.25))
        .material(
            Material::builder()
                .color(color(1, 0.2, 0.2))
                .diffuse(0.8)
                .specular(0.5),
        )
        .build()
}

fn hexagon_edge() -> Shape {
    sphere()
        .transform(
            transform::translation(0, 0, -1)
                * transform::rotation_y(-FRAC_PI_6)
                * transform::translation(0, 0, -0.5)
                * transform::scaling(0.1, 0.1, 0.4),
        )
        .material(
            Material::builder()
                .color(color(0.2, 0.8, 0.2))
                .diffuse(0.7)
                .specular(0.3),
        )
        .build()
}

fn hexagon_side() -> Shape {
    let side = group().build();
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());
    side
}

fn hexagon() -> Shape {
    let hex = group().build();

    #[allow(clippy::cast_precision_loss)]
    for n in 0..6 {
        let side = hexagon_side();
        side.set_transform(transform::rotation_y(n as f32 * FRAC_PI_3));
        hex.add_child(side);
    }

    hex
}

fn build_floor() -> Shape {
    plane()
        .material(
            Material::builder()
                .pattern(
                    checkers_pattern(color(0.2, 0.2, 0.2), color(0.8, 0.8, 0.8))
                        .transform(transform::scaling(0.5, 0.5, 0.5))
                        .build(),
                )
                .reflective(0.3)
                .specular(0.0),
        )
        .build()
}

fn main() -> Result<()> {
    let hex1 = hexagon();
    hex1.set_transform(transform::translation(-1.5, 1, 0));

    let hex2 = hexagon();
    hex2.set_transform(transform::translation(1.5, 1, 0) * transform::rotation_x(FRAC_PI_6));

    let hex3 = hexagon();
    hex3.set_transform(
        transform::translation(0, 1, 2.5)
            * transform::rotation_z(FRAC_PI_6)
            * transform::scaling(0.75, 0.75, 0.75),
    );

    let world = World::builder()
        .light(point_light(point(-5, 5, -5), color(1, 1, 1)))
        .objects(vec![build_floor(), hex1, hex2, hex3])
        .build();

    let camera = camera(1000, 500)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0, 2.5, -5),
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
