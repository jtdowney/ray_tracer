use std::f32::consts::PI;

use anyhow::Result;
use ray_tracer::{canvas, color::RED, point, transform::rotation_y};

fn main() -> Result<()> {
    let size: usize = 200;
    let mut c = canvas(size, size);

    #[allow(clippy::cast_precision_loss)]
    let (center, radius) = {
        let size = size as f32;
        (size / 2.0, size * 3.0 / 8.0)
    };

    let twelve = point(0, 0, 1);

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    for hour in 0..12 {
        let rotation = rotation_y(hour as f32 * PI / 6.0);
        let position = rotation * twelve;

        let x = (position.x * radius + center).round() as usize;
        let y = (position.z * radius + center).round() as usize;

        if x < c.width && y < c.height {
            c.write_pixel(x, y, RED)?;
        }
    }

    let ppm = c.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
