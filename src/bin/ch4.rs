use ray_tracer::{transforms, Canvas, Color, Point};
use std::f32::consts::PI;
use std::fmt;

fn main() -> Result<(), fmt::Error> {
    let mut canvas = Canvas::new(500, 500);
    let mut p = transforms::translation(0.0f32, -215.0, 0.0) * Point::new(0.0, 0.0, 0.0);
    let transform = transforms::rotation_z(PI / 6.0);

    for _ in 0..12 {
        canvas.write_pixel(
            (p.x + 250.0).round() as u16,
            (p.y + 250.0).round() as u16,
            Color::new(1.0, 0.5, 0.0),
        );

        p = transform * p;
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
