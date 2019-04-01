use ray_tracer::{
    color, transforms, Camera, Color, MaterialBuilder, PlaneBuilder, Point, PointLight,
    SphereBuilder, Vector3, WorldBuilder,
};
use std::f64::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let world = WorldBuilder::default()
        .object(
            PlaneBuilder::default() // floor
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(1.0, 0.9, 0.9))
                        .specular(0.0)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default() // middle
                .transform(transforms::translation(-0.5, 1.0, 0.5))
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(0.1, 1.0, 0.5))
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
                    transforms::translation(1.5, 0.5, -0.5) * transforms::scaling(0.5, 0.5, 0.5),
                )
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(0.5, 1.0, 0.1))
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
                    transforms::translation(-1.5, 0.33, -0.75)
                        * transforms::scaling(0.33, 0.33, 0.33),
                )
                .material(
                    MaterialBuilder::default()
                        .color(Color::new(1.0, 0.8, 0.1))
                        .diffuse(0.7)
                        .specular(0.3)
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

    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = transforms::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let canvas = ray_tracer::render(camera, world);
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
