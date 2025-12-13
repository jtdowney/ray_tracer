use std::ops::{Add, Sub};

use num_traits::AsPrimitive;
use wide::f32x4;

use crate::Vector;

pub const ORIGIN: Point = Point {
    data: f32x4::new([0.0, 0.0, 0.0, 1.0]),
};

pub fn point(
    x: impl AsPrimitive<f32>,
    y: impl AsPrimitive<f32>,
    z: impl AsPrimitive<f32>,
) -> Point {
    Point {
        data: f32x4::new([x.as_(), y.as_(), z.as_(), 1.0]),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub(crate) data: f32x4,
}

impl Default for Point {
    fn default() -> Self {
        ORIGIN
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Point {
    #[must_use]
    pub fn x(&self) -> f32 {
        self.data.as_array()[0]
    }

    #[must_use]
    pub fn y(&self) -> f32 {
        self.data.as_array()[1]
    }

    #[must_use]
    pub fn z(&self) -> f32 {
        self.data.as_array()[2]
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            data: self.data - rhs.data,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            data: self.data - rhs.data,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            data: self.data + rhs.data,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::vector;

    #[test]
    fn point_creates_point_with_coordinates() {
        let p = point(4.3, -4.2, 3.1);
        assert_relative_eq!(p.x(), 4.3);
        assert_relative_eq!(p.y(), -4.2);
        assert_relative_eq!(p.z(), 3.1);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(p1 - p2, vector(-2, -4, -6));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);
        assert_eq!(p - v, point(-2, -4, -6));
    }

    #[test]
    fn adding_vector_to_point() {
        let p = point(3, -2, 5);
        let v = vector(-2, 3, 1);
        assert_eq!(p + v, point(1, 1, 6));
    }
}
