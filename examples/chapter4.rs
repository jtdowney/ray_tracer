use ray_tracer::{transformations, Canvas, Color, Point};
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    let width = 500;
    let half_width = f64::from(width as i16 / 2);
    let mut canvas = Canvas::new(width, width);
    let mut p = transformations::translation(0.0, -215.0, 0.0) * Point::new(0.0, 0.0, 0.0);
    let transform = transformations::rotation_z(PI / 6.0);

    for _ in 0..12 {
        canvas.write_pixel(
            (p.x + half_width).round() as usize,
            (p.y + half_width).round() as usize,
            Color::new(1.0, 0.5, 0.0),
        )?;

        p = transform * p;
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
