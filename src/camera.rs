use crate::{Matrix4, Point, Ray};

pub struct Camera {
    pub horizontal_size: u16,
    pub vertical_size: u16,
    pub transform: Matrix4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(horizontal_size: u16, vertical_size: u16, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(horizontal_size) / f64::from(vertical_size);

        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };

        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };

        let pixel_size = half_width * 2.0 / f64::from(horizontal_size);

        Camera {
            horizontal_size,
            vertical_size,
            half_height,
            half_width,
            pixel_size,
            transform: Matrix4::identity(),
        }
    }

    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Ray {
        let x_offset = (f64::from(px) + 0.5) * self.pixel_size;
        let y_offset = (f64::from(py) + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse();
        let pixel = transform_inverse * Point::new(world_x, world_y, -1.0);
        let origin = transform_inverse * Point::default();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn pixels(&self) -> PixelsIter {
        PixelsIter {
            width: self.horizontal_size,
            height: self.vertical_size,
            x: 0,
            y: 0,
        }
    }
}

pub struct PixelsIter {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Iterator for PixelsIter {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        let pixel = (self.x, self.y);
        self.x += 1;

        Some(pixel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, Vector3, EPSILON};
    use std::f64::consts::PI;

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!((0.01 - c.pixel_size).abs() < EPSILON);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!((0.01 - c.pixel_size).abs() < EPSILON);
    }

    #[test]
    fn ray_for_pixel_at_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn ray_for_pixel_at_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_eq!(Vector3::new(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn ray_for_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = transforms::rotation_y(PI / 4.0) * transforms::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Point::new(0.0, 2.0, -5.0), r.origin);
        assert_eq!(
            Vector3::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0),
            r.direction
        );
    }

    #[test]
    fn pixels_iterator() {
        let c = Camera::new(3, 2, PI / 2.0);
        let mut pixels = c.pixels();
        assert_eq!(Some((0, 0)), pixels.next());
        assert_eq!(Some((1, 0)), pixels.next());
        assert_eq!(Some((2, 0)), pixels.next());
        assert_eq!(Some((0, 1)), pixels.next());
        assert_eq!(Some((1, 1)), pixels.next());
        assert_eq!(Some((2, 1)), pixels.next());
        assert_eq!(None, pixels.next());
    }
}
