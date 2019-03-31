use ray_tracer::{
    color, transforms, Camera, CheckersPattern, Color, Pattern, Plane, Point, PointLight, Shape,
    SolidPattern, Sphere, Vector3, World,
};
use std::f64::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let mut floor = Plane::default();
    floor.material.pattern =
        Box::new(CheckersPattern::new(color::WHITE, color::BLACK)) as Box<Pattern + Send + Sync>;
    floor.material.specular = 0.0;

    let mut middle = Sphere::default();
    middle.transform = transforms::translation(-0.5, 1.0, 0.5);
    middle.material.pattern =
        Box::new(SolidPattern::new(Color::new(0.1, 1.0, 0.5))) as Box<Pattern + Send + Sync>;
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = transforms::translation(1.5, 0.5, -0.5) * transforms::scaling(0.5, 0.5, 0.5);
    right.material.pattern =
        Box::new(SolidPattern::new(Color::new(0.5, 1.0, 0.1))) as Box<Pattern + Send + Sync>;
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform =
        transforms::translation(-1.5, 0.33, -0.75) * transforms::scaling(0.33, 0.33, 0.33);
    left.material.pattern =
        Box::new(SolidPattern::new(Color::new(1.0, 0.8, 0.1))) as Box<Pattern + Send + Sync>;
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
    let objects = vec![
        Box::new(floor) as Box<Shape + Send + Sync>,
        Box::new(left) as Box<Shape + Send + Sync>,
        Box::new(middle) as Box<Shape + Send + Sync>,
        Box::new(right) as Box<Shape + Send + Sync>,
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
