use std::ops::{Add, Sub};

use crate::Vector;

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub fn point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Point {
    Point {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
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
        assert_relative_eq!(p.x, 4.3);
        assert_relative_eq!(p.y, -4.2);
        assert_relative_eq!(p.z, 3.1);
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
