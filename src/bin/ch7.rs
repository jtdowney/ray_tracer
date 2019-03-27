use failure::Error;
use ray_tracer::{world, Camera, Color, Matrix4, Point, PointLight, Sphere, Vector3, World};
use std::f32::consts::PI;

fn main() -> Result<(), Error> {
    let mut floor = Sphere::default();
    floor.transform = Matrix4::scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(-PI / 4.0)
        * Matrix4::rotation_x(PI / 2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(PI / 4.0)
        * Matrix4::rotation_x(PI / 2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.transform = Matrix4::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform = Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let objects = vec![floor, left_wall, right_wall, left, middle, right];
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let world = World::new(light, objects);

    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = world::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let canvas = ray_tracer::render(camera, world)?;
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
