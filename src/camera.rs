use crate::{matrix, Matrix4, Point, Ray, Scalar};
use num_traits::{Float, One};
use std::iter::Sum;

pub struct Camera<T: Scalar> {
    pub horizontal_size: u16,
    pub vertical_size: u16,
    pub transform: Matrix4<T>,
    pixel_size: T,
    half_width: T,
    half_height: T,
}

impl<T> Camera<T>
where
    T: Scalar + Float + From<u16> + One,
{
    pub fn new(horizontal_size: u16, vertical_size: u16, field_of_view: T) -> Self {
        let half_view = (field_of_view / 2.into()).tan();
        let aspect: T = Into::<T>::into(horizontal_size) / vertical_size.into();

        let half_width = if aspect >= T::one() {
            half_view
        } else {
            half_view * aspect
        };

        let half_height = if aspect >= T::one() {
            half_view / aspect
        } else {
            half_view
        };

        let pixel_size = half_width * 2.into() / horizontal_size.into();

        Camera {
            horizontal_size,
            vertical_size,
            half_height,
            half_width,
            pixel_size,
            transform: Matrix4::identity(),
        }
    }
}

impl<T> Camera<T>
where
    T: Scalar + Float + From<u16> + From<f32> + One + Sum<T>,
{
    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Result<Ray<T>, matrix::NotInvertableError> {
        let x_offset = (Into::<T>::into(px) + 0.5.into()) * self.pixel_size;
        let y_offset = (Into::<T>::into(py) + 0.5.into()) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse()?;
        let pixel = transform_inverse * Point::new(world_x, world_y, -T::one());
        let origin = transform_inverse * Point::default();
        let direction = (pixel - origin).normalize();

        Ok(Ray::new(origin, direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector3;
    use std::f32::consts::PI;

    #[test]
    fn test_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn test_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn test_ray_for_pixel_at_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn test_ray_for_pixel_at_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0).unwrap();
        assert_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_eq!(Vector3::new(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn test_ray_for_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix4::rotation_y(PI / 4.0) * Matrix4::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(Point::new(0.0, 2.0, -5.0), r.origin);
        assert_eq!(
            Vector3::new(2.0.sqrt() / 2.0, 0.0, -2.0.sqrt() / 2.0),
            r.direction
        );
    }
}
