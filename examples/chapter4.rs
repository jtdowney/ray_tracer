use std::f64::consts::PI;

use ray_tracer::{color, transform, Canvas, ORIGIN};

fn main() -> anyhow::Result<()> {
    let width = 500;
    let half_width = f64::from(width as i16 / 2);
    let mut canvas = Canvas::new(width, width);
    let mut p = transform::translation(0.0, -215.0, 0.0) * ORIGIN;
    let transform = transform::rotation_z(PI / 6.0);

    for _ in 0..12 {
        canvas.write_pixel(
            (p.x + half_width).round() as u16,
            (p.y + half_width).round() as u16,
            color(1.0, 0.5, 0.0),
        )?;

        p = transform * p;
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
