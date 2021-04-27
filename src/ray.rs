use crate::{Matrix4, Point, Vector};
use std::ops::{Add, Mul};

pub fn ray<T>(origin: Point<T>, direction: Vector<T>) -> Ray<T>
where
    T: Copy,
{
    Ray { origin, direction }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray<T>
where
    T: Copy,
{
    pub origin: Point<T>,
    pub direction: Vector<T>,
}

impl<T> Ray<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub fn position(&self, time: T) -> Point<T> {
        self.origin + self.direction * time
    }

    pub fn transform(&self, transform: Matrix4<T>) -> Ray<T> {
        Ray {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, transformations, vector};

    #[test]
    fn creating_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = ray(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = transformations::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(point(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(vector(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = transformations::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(point(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(vector(0.0, 3.0, 0.0), r2.direction);
    }
}
