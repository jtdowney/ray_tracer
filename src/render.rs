use crate::{matrix, Camera, Canvas, World};

pub fn render(camera: Camera, world: World) -> Result<Canvas, matrix::NotInvertableError> {
    let mut canvas = Canvas::new(camera.horizontal_size, camera.vertical_size);

    for y in 0..camera.vertical_size {
        for x in 0..camera.horizontal_size {
            let ray = camera.ray_for_pixel(x, y)?;
            let color = world.color_at(ray)?;
            canvas.write_pixel(x, y, color);
        }
    }

    Ok(canvas)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, Color, Point, Vector3};
    use std::f64::consts::PI;

    #[test]
    fn test_rendering_world_with_camera() {
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::default();
        let up = Vector3::new(0.0, 1.0, 0.0);

        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.transform = transforms::view(from, to, up);

        let image = render(c, w).unwrap();
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5));
    }
}
