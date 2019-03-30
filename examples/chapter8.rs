use ray_tracer::{transforms, Camera, Color, Point, PointLight, Shape, Sphere, Vector3, World};
use std::error;
use std::f64::consts::PI;
use std::fmt::Display;

#[derive(Debug)]
struct Error {
    error: String,
}

impl<T: Display + error::Error> From<T> for Error {
    fn from(error: T) -> Self {
        Error {
            error: format!("Error: {}", error),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut floor = Sphere::default();
    floor.transform = transforms::scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.transform = transforms::translation(0.0, 0.0, 5.0)
        * transforms::rotation_y(-PI / 4.0)
        * transforms::rotation_x(PI / 2.0)
        * transforms::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.transform = transforms::translation(0.0, 0.0, 5.0)
        * transforms::rotation_y(PI / 4.0)
        * transforms::rotation_x(PI / 2.0)
        * transforms::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.transform = transforms::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = transforms::translation(1.5, 0.5, -0.5) * transforms::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform =
        transforms::translation(-1.5, 0.33, -0.75) * transforms::scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let objects = [floor, left_wall, right_wall, left, middle, right]
        .iter()
        .cloned()
        .map(|s| Box::new(s) as Box<Shape + Send + Sync>)
        .collect::<Vec<_>>();
    let world = World::new(light, objects);

    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = transforms::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let canvas = ray_tracer::render(camera, world)?;
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
