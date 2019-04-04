use crate::{Camera, Canvas, Color, World};
use rayon::prelude::*;

pub fn render(camera: Camera, world: World) -> Canvas {
    let mut canvas = Canvas::new(camera.horizontal_size, camera.vertical_size);

    for (x, y) in camera.pixels() {
        let color = render_pixel(&camera, &world, x, y);
        canvas.write_pixel(x, y, color);
    }

    canvas
}

pub fn render_parallel(camera: Camera, world: World) -> Canvas {
    let pixels: Vec<(u16, u16)> = camera.pixels().collect();
    let rendered_pixels: Vec<Color> = pixels
        .into_par_iter()
        .map(|(x, y)| render_pixel(&camera, &world, x, y))
        .collect();

    Canvas::from_pixels(
        camera.horizontal_size,
        camera.vertical_size,
        rendered_pixels,
    )
}

fn render_pixel(camera: &Camera, world: &World, x: u16, y: u16) -> Color {
    let ray = camera.ray_for_pixel(x, y);
    world.color_at(ray, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, Color, Point, Vector3};
    use std::f64::consts::PI;

    #[test]
    fn rendering_world_with_camera() {
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::default();
        let up = Vector3::new(0.0, 1.0, 0.0);

        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.transform = transforms::view(from, to, up);

        let image = render(c, w);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5));
    }
}
