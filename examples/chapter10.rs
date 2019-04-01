use ray_tracer::{
    color, transforms, Camera, CheckersPattern, Color, MaterialBuilder, PlaneBuilder, Point,
    PointLight, Shape, SolidPattern, SphereBuilder, Vector3, World,
};
use std::f64::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let floor = PlaneBuilder::default()
        .material(
            MaterialBuilder::default()
                .pattern(CheckersPattern::new(color::WHITE, color::BLACK))
                .specular(0.0)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let middle = SphereBuilder::default()
        .transform(transforms::translation(-0.5, 1.0, 0.5))
        .material(
            MaterialBuilder::default()
                .pattern(SolidPattern::new(Color::new(0.1, 1.0, 0.5)))
                .diffuse(0.7)
                .specular(0.3)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let right = SphereBuilder::default()
        .transform(transforms::translation(1.5, 0.5, -0.5) * transforms::scaling(0.5, 0.5, 0.5))
        .material(
            MaterialBuilder::default()
                .pattern(SolidPattern::new(Color::new(0.5, 1.0, 0.1)))
                .diffuse(0.7)
                .specular(0.3)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let left = SphereBuilder::default()
        .transform(
            transforms::translation(-1.5, 0.33, -0.75) * transforms::scaling(0.33, 0.33, 0.33),
        )
        .material(
            MaterialBuilder::default()
                .pattern(SolidPattern::new(Color::new(1.0, 0.8, 0.1)))
                .diffuse(0.7)
                .specular(0.3)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
    let objects = vec![
        Box::new(floor) as Box<Shape>,
        Box::new(left) as Box<Shape>,
        Box::new(middle) as Box<Shape>,
        Box::new(right) as Box<Shape>,
    ];
    let world = World::new(light, objects);

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
