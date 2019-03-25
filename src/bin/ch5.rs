use ray_tracer::{Canvas, Color, Point, Ray, Sphere};
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::new();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());

            if let Some(xs) = r.intersect(&shape) {
                if xs.is_hit() {
                    canvas.write_pixel(x, y, color);
                }
            }
        }
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
