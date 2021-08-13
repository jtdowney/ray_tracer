use ray_tracer::{
    color, point, point_light, transformations, vector, Camera, CheckersPatternBuilder,
    ConeBuilder, CubeBuilder, CylinderBuilder, MaterialBuilder, WorldBuilder,
};
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    let world = WorldBuilder::default()
        .object(
            CubeBuilder::default() // room
                .transform(
                    transformations::rotation_y(PI / 3.5)
                        * transformations::translation(0.0, 12.0, 0.0)
                        * transformations::scaling(15.0, 12.0, 15.0),
                )
                .material(
                    MaterialBuilder::default()
                        .pattern(
                            CheckersPatternBuilder::default()
                                .a(color::WHITE)
                                .b(color::BLACK)
                                .transform(transformations::scaling(
                                    1.0 / 12.0,
                                    1.0 / 12.0,
                                    1.0 / 12.0,
                                ))
                                .build()
                                .unwrap(),
                        )
                        .specular(0.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            CylinderBuilder::default() // middle
                .transform(transformations::translation(-0.5, 0.0, 0.5))
                .closed(true)
                .maximum(1.0)
                .minimum(0.0)
                .material(
                    MaterialBuilder::default()
                        .color(color(0.8, 0.8, 0.4))
                        .diffuse(0.3)
                        .ambient(0.2)
                        .specular(0.2)
                        .reflective(0.9)
                        .shininess(100.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            CubeBuilder::default() //back cube
                .transform(
                    transformations::translation(-2.7, 0.5, 3.0)
                        * transformations::scaling(0.5, 0.5, 0.5)
                        * transformations::rotation_y(PI / 5.0),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(0.1, 0.1, 0.6))
                        .diffuse(0.6)
                        .specular(0.4)
                        .reflective(0.2)
                        .shininess(200.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            ConeBuilder::default() //right
                .minimum(-3.0)
                .maximum(0.0)
                .transform(
                    transformations::translation(1.5, 0.75, -0.5)
                        * transformations::scaling(0.25, 0.25, 0.25),
                )
                .material(
                    MaterialBuilder::default()
                        .color(color(0.5, 1.0, 0.1))
                        .diffuse(0.2)
                        .specular(0.2)
                        .reflective(0.4)
                        .shininess(200.0)
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
