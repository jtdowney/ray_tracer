use ray_tracer::{Canvas, Color, Point, Ray, Shape, Sphere, World};
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / f64::from(canvas_pixels);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::default();
    let world = World::default();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * f64::from(y);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * f64::from(x);
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(ray, &world);
            if xs.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
