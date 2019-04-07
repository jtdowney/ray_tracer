use indicatif::ProgressBar;
use ray_tracer::{
    color, transforms, Camera, CheckersPatternBuilder, Color, ConeBuilder, CubeBuilder,
    CylinderBuilder, MaterialBuilder, Point, PointLight, Vector3, WorldBuilder,
};
use std::f64::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let world = WorldBuilder::default()
        .object(
            CubeBuilder::default() // room
                .transform(
                    transforms::rotation_y(PI / 3.5)
                        * transforms::translation(0.0, 12.0, 0.0)
                        * transforms::scaling(15.0, 12.0, 15.0),
                )
                .material(
                    MaterialBuilder::default()
                        .pattern(
                            CheckersPatternBuilder::default()
                                .first(color::WHITE)
                                .second(color::BLACK)
                                .transform(transforms::scaling(1.0 / 12.0, 1.0 / 12.0, 1.0 / 12.0))
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
                .transform(transforms::translation(-0.5, 0.0, 0.5))
                .closed(true)
                .maximum(1.0)
                .minimum(0.0)
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(0.8, 0.8, 0.4))
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
                    transforms::translation(-2.7, 0.5, 3.0)
                        * transforms::scaling(0.5, 0.5, 0.5)
                        * transforms::rotation_y(PI / 5.0),
                )
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(0.1, 0.1, 0.6))
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
                    transforms::translation(1.5, 0.75, -0.5)
                        * transforms::scaling(0.25, 0.25, 0.25),
                )
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(0.5, 1.0, 0.1))
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
        .light(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            color::WHITE,
        ))
        .build();

    let width = 2000;
    let height = 1000;
    let mut camera = Camera::new(width as u16, height as u16, PI / 3.0);
    camera.transform = transforms::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let bar = ProgressBar::new(width * height);
    let canvas = ray_tracer::render_parallel(camera, world, || bar.inc(1));
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
