use crate::{point, ray, world::World, Canvas, CanvasError, Matrix4, Ray};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub horizontal_size: u16,
    pub vertical_size: u16,
    pub field_of_view: f64,
    pub transform: Matrix4,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(horizontal_size: u16, vertical_size: u16, field_of_view: f64) -> Self {
        let transform = Matrix4::identity();
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

        Self {
            horizontal_size,
            vertical_size,
            field_of_view,
            transform,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Ray {
        let x_offset = (f64::from(px) + 0.5) * self.pixel_size;
        let y_offset = (f64::from(py) + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse();
        let pixel = transform_inverse * point(world_x, world_y, -1.0);
        let origin = transform_inverse * point::ORIGIN;
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    pub fn render(&self, world: &World) -> Result<Canvas, CanvasError> {
        let mut canvas = Canvas::new(self.horizontal_size, self.vertical_size);
        for y in 0..self.vertical_size {
            for x in 0..self.horizontal_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.write_pixel(x, y, color)?;
            }
        }

        Ok(canvas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point, transformations, vector, world::default_world};
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.horizontal_size, 160);
        assert_eq!(c.vertical_size, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix4::identity());
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn ray_for_pixel_at_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(point(0.0, 0.0, 0.0), r.origin);
        assert_abs_diff_eq!(vector(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn ray_for_pixel_at_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_abs_diff_eq!(point(0.0, 0.0, 0.0), r.origin);
        assert_abs_diff_eq!(vector(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn ray_for_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform =
            transformations::rotation_y(PI / 4.0) * transformations::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(point(0.0, 2.0, -5.0), r.origin);
        assert_abs_diff_eq!(
            vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0),
            r.direction
        );
    }

    #[test]
    fn render_a_world_with_a_camera() {
        let w = default_world();
        let from = point(0.0, 0.0, -5.0);
        let to = point::ORIGIN;
        let up = vector(0.0, 1.0, 0.0);
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.transform = transformations::view(from, to, up);
        let canvas = c.render(&w).unwrap();
        assert_abs_diff_eq!(
            canvas.pixel_at(5, 5).unwrap(),
            color(0.38066, 0.47583, 0.2855)
        );
    }
}
