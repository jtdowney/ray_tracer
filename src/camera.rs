use std::f64::consts::FRAC_PI_2;

use bon::builder;

use crate::{
    Canvas, Matrix4, ORIGIN, REFLECTION_DEPTH, Ray, World, canvas, identity_matrix, point, ray,
};

#[must_use]
#[builder(finish_fn = build, on(f64, into))]
pub fn camera(
    #[builder(start_fn)] horizontal_size: u16,
    #[builder(start_fn)] vertical_size: u16,
    #[builder(default = FRAC_PI_2)] field_of_view: f64,
    #[builder(default = identity_matrix())] transform: Matrix4,
) -> Camera {
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
        transform,
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
    /// # Panics
    /// Panics if the camera's transform matrix is not invertible.
    #[must_use]
    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Ray {
        let x_offset = (f64::from(px) + 0.5) * self.pixel_size;
        let y_offset = (f64::from(py) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse = self
            .transform
            .inverse()
            .expect("camera transform is not invertible");
        let pixel = inverse * point(world_x, world_y, -1.0);
        let origin = inverse * ORIGIN;
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    /// # Errors
    /// Returns an error if writing a pixel to the canvas fails.
    pub fn render(&self, world: &World) -> anyhow::Result<Canvas> {
        let mut canvas = canvas(self.width as usize, self.height as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, REFLECTION_DEPTH);
                canvas.write_pixel(x as usize, y as usize, color)?;
            }
        }

        Ok(canvas)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, default_world, point, transform, vector};

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = FRAC_PI_2;
        let c = camera(hsize, vsize).field_of_view(field_of_view).build();
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_relative_eq!(c.field_of_view, FRAC_PI_2, epsilon = EPSILON);
        assert_eq!(c.transform, identity_matrix());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = camera(200, 125).field_of_view(FRAC_PI_2).build();
        assert_relative_eq!(c.pixel_size, 0.01, epsilon = EPSILON);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = camera(125, 200).field_of_view(FRAC_PI_2).build();
        assert_relative_eq!(c.pixel_size, 0.01, epsilon = EPSILON);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = camera(201, 101).field_of_view(FRAC_PI_2).build();
        let r = c.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.z, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.z, -1.0, epsilon = EPSILON);
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = camera(201, 101).field_of_view(FRAC_PI_2).build();
        let r = c.ray_for_pixel(0, 0);
        assert_relative_eq!(r.origin.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.z, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.x, 0.66519, epsilon = EPSILON);
        assert_relative_eq!(r.direction.y, 0.33259, epsilon = EPSILON);
        assert_relative_eq!(r.direction.z, -0.66851, epsilon = EPSILON);
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let c = camera(201, 101)
            .field_of_view(FRAC_PI_2)
            .transform(transform::rotation_y(FRAC_PI_4) * transform::translation(0, -2, 5))
            .build();
        let r = c.ray_for_pixel(100, 50);
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        assert_relative_eq!(r.origin.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.y, 2.0, epsilon = EPSILON);
        assert_relative_eq!(r.origin.z, -5.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.x, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(r.direction.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(r.direction.z, -sqrt2_over_2, epsilon = EPSILON);
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = default_world();
        let c = camera(11, 11)
            .field_of_view(FRAC_PI_2)
            .transform(transform::view_transform(
                point(0, 0, -5),
                point(0, 0, 0),
                vector(0, 1, 0),
            ))
            .build();
        let image = c.render(&w).unwrap();
        let pixel = image.pixel_at(5, 5).unwrap();
        assert_relative_eq!(pixel.red, 0.38066, epsilon = EPSILON);
        assert_relative_eq!(pixel.green, 0.47583, epsilon = EPSILON);
        assert_relative_eq!(pixel.blue, 0.2855, epsilon = EPSILON);
    }
}
