use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6},
    fs,
};

use anyhow::Result;
use ray_tracer::{
    Material, ObjParser, World, camera, color, pattern::checkers_pattern, point, point_light,
    shape::plane, transform, vector,
};

fn build_floor() -> ray_tracer::Shape {
    plane()
        .material(
            Material::builder()
                .pattern(
                    checkers_pattern(color(0.2, 0.2, 0.2), color(0.8, 0.8, 0.8))
                        .transform(transform::scaling(0.5, 0.5, 0.5))
                        .build(),
                )
                .reflective(0.2)
                .specular(0.0),
        )
        .build()
}

fn main() -> Result<()> {
    let obj_content = fs::read_to_string("assets/teapot.obj")?;
    let parser: ObjParser = obj_content.parse().expect("Failed to parse OBJ file");

    let teapot = parser.as_ref().clone();
    teapot.set_transform(
        transform::rotation_y(FRAC_PI_6)
            * transform::rotation_x(-FRAC_PI_2)
            * transform::scaling(0.08, 0.08, 0.08),
    );
    teapot.set_material(
        Material::builder()
            .color(color(0.9, 0.2, 0.2))
            .diffuse(0.7)
            .specular(0.3)
            .shininess(50.0)
            .build(),
    );

    let world = World::builder()
        .light(point_light(point(-5, 5, -5), color(1, 1, 1)))
        .objects(vec![build_floor(), teapot])
        .build();

    let camera = camera(1000, 500)
        .field_of_view(FRAC_PI_3)
        .transform(transform::view_transform(
            point(0, 2, -4),
            point(0, 0.5, 0),
            vector(0, 1, 0),
        ))
        .parallel(true)
        .build();

    let canvas = camera.render(&world);
    let ppm = canvas.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
