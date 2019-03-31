use ray_tracer::{
    color, transforms, Camera, Color, Pattern, Point, PointLight, Shape, SolidPattern, Sphere,
    Vector3, World,
};
use std::f64::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let mut floor = Sphere::default();
    floor.transform = transforms::scaling(10.0, 0.01, 10.0);
    floor.material.pattern =
        Box::new(SolidPattern::new(Color::new(1.0, 0.9, 0.9))) as Box<Pattern + Send + Sync>;
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.transform = transforms::translation(0.0, 0.0, 5.0)
        * transforms::rotation_y(-PI / 4.0)
        * transforms::rotation_x(PI / 2.0)
        * transforms::scaling(10.0, 0.01, 10.0);
    left_wall.material.pattern =
        Box::new(SolidPattern::new(Color::new(1.0, 0.9, 0.9))) as Box<Pattern + Send + Sync>;
    left_wall.material.specular = 0.0;

    let mut right_wall = Sphere::default();
    right_wall.transform = transforms::translation(0.0, 0.0, 5.0)
        * transforms::rotation_y(PI / 4.0)
        * transforms::rotation_x(PI / 2.0)
        * transforms::scaling(10.0, 0.01, 10.0);
    right_wall.material.pattern =
        Box::new(SolidPattern::new(Color::new(1.0, 0.9, 0.9))) as Box<Pattern + Send + Sync>;
    right_wall.material.specular = 0.0;

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
    let objects = vec![floor, left_wall, right_wall, left, middle, right]
        .into_iter()
        .map(|s| Box::new(s) as Box<Shape + Send + Sync>)
        .collect::<Vec<Box<Shape + Send + Sync>>>();
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
