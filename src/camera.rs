use itertools::iproduct;
use rayon::prelude::*;

use crate::{
    canvas::canvas, identity_matrix, point, ray, Canvas, Matrix4, Ray, World, ORIGIN,
    REFLECTION_DEPTH,
};

pub fn camera(horizontal_size: u16, vertical_size: u16, field_of_view: f64) -> Camera {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = f64::from(horizontal_size) / f64::from(vertical_size);

    let half_width;
    let half_height;
    if aspect >= 1.0 {
        half_width = half_view;
        half_height = half_view / aspect;
    } else {
        half_width = half_view * aspect;
        half_height = half_view;
    }

    let pixel_size = (half_width * 2.0) / f64::from(horizontal_size);

    Camera {
        width: horizontal_size,
        height: vertical_size,
        field_of_view,
        transform: identity_matrix(),
        pixel_size,
        half_width,
        half_height,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub width: u16,
    pub height: u16,
    pub field_of_view: f64,
    pub transform: Matrix4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn ray_for_pixel(&self, x: u16, y: u16) -> Ray {
        let xoffset = (f64::from(x) + 0.5) * self.pixel_size;
        let yoffset = (f64::from(y) + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv = self.transform.inverse();
        let pixel = inv * point(world_x, world_y, -1.0);
        let origin = inv * ORIGIN;
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    pub fn render(&self, world: World) -> anyhow::Result<Canvas> {
        let pixels = iproduct!(0..self.width, 0..self.height)
            .par_bridge()
            .map(|(x, y)| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, REFLECTION_DEPTH);
                (x, y, color)
            })
            .collect::<Vec<_>>();

        let mut canvas = canvas(self.width, self.height);
        for (x, y, pixel) in pixels {
            canvas.write_pixel(x, y, pixel)?;
        }

        Ok(canvas)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use crate::{
        color,
        transform::{rotation_y, translation, view_transform},
        vector,
        world::default_world,
        ORIGIN,
    };

    use super::*;

    #[test]
    fn pixel_size_landscape() {
        let c = camera(200, 125, PI / 2.0);
        assert_abs_diff_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn pixel_size_portrait() {
        let c = camera(125, 200, PI / 2.0);
        assert_abs_diff_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn constructing_ray_through_canvas_center() {
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(ORIGIN, r.origin);
        assert_abs_diff_eq!(vector(0, 0, -1), r.direction);
    }

    #[test]
    fn constructing_ray_through_canvas_corner() {
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(ORIGIN, r.origin);
        assert_abs_diff_eq!(vector(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut c = camera(201, 101, PI / 2.0);
        c.transform = rotation_y(PI / 4.0) * translation(0, -2, 5);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(point(0, 2, -5), r.origin);
        assert_abs_diff_eq!(
            vector(2.0_f64.sqrt() / 2.0, 0.0, -(2.0_f64.sqrt()) / 2.0),
            r.direction
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = default_world();
        let mut c = camera(11, 11, PI / 2.0);
        let from = point(0, 0, -5);
        let to = ORIGIN;
        let up = vector(0, 1, 0);
        c.transform = view_transform(from, to, up);
        let image = c.render(w).unwrap();
        assert_abs_diff_eq!(
            color(0.38066, 0.47583, 0.2855),
            image.pixel_at(5, 5).unwrap()
        );
    }
}
