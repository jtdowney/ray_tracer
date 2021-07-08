use ray_tracer::{
    color, point, point_light, transformations, vector, Camera, MaterialBuilder, SphereBuilder,
    WorldBuilder,
};
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    let world = WorldBuilder::default()
        .object(
            SphereBuilder::default() // floor
                .transform(transformations::scaling(10.0, 0.01, 10.0))
                .material(
                    MaterialBuilder::default()
                        .color(color(1.0, 0.9, 0.9))
                        .specular(0.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() // left wall
                .transform(
                    transformations::translation(0.0, 0.0, 5.0)
                        * transformations::rotation_y(-PI / 4.0)
                        * transformations::rotation_x(PI / 2.0)
                        * transformations::scaling(10.0, 0.01, 10.0),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(1.0, 0.9, 0.9))
                        .specular(0.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() // right wall
                .transform(
                    transformations::translation(0.0, 0.0, 5.0)
                        * transformations::rotation_y(PI / 5.0)
                        * transformations::rotation_x(PI / 2.0)
                        * transformations::scaling(10.0, 0.01, 10.0),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(1.0, 0.9, 0.9))
                        .specular(0.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() // middle
                .transform(transformations::translation(-0.5, 1.0, 0.5))
                .material(
                    MaterialBuilder::default()
                        .color(color(0.1, 1.0, 0.5))
                        .diffuse(0.7)
                        .specular(0.3)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() //right
                .transform(
                    transformations::translation(1.5, 0.5, -0.5)
                        * transformations::scaling(0.5, 0.5, 0.5),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(0.5, 1.0, 0.1))
                        .diffuse(0.7)
                        .specular(0.3)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() // left
                .transform(
                    transformations::translation(-1.5, 0.33, -0.75)
                        * transformations::scaling(0.33, 0.33, 0.33),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(1.0, 0.8, 0.1))
                        .diffuse(0.7)
                        .specular(0.3)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .light(point_light(point(-10.0, 10.0, -10.0), color::WHITE))
        .build()
        .unwrap();

    let width = 1024;
    let height = 768;
    let mut camera = Camera::new(width, height, PI / 3.0);
    camera.transform = transformations::view(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world)?;
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
