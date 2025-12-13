use anyhow::Result;
use ray_tracer::{canvas, color::RED, hit, point, ray, shape::sphere};

fn main() -> Result<()> {
    let ray_origin = point(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels: usize = 100;

    #[allow(clippy::cast_precision_loss)]
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut c = canvas(canvas_pixels, canvas_pixels);
    let shape = sphere().build();

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = point(world_x, world_y, wall_z);
            let direction = (position - ray_origin).normalize();
            let r = ray(ray_origin, direction);
            let xs = shape.intersect(r);

            if hit(xs).is_some() {
                c.write_pixel(x, y, RED)?;
            }
        }
    }

    let ppm = c.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
