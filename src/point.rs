use crate::{vector, Vector, EPSILON};
use approx::AbsDiffEq;
use std::ops::{Add, Sub};

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn iter(&self) -> PointIter {
        PointIter {
            point: *self,
            index: 0,
        }
    }
}

impl AbsDiffEq for Point {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon)
            && f64::abs_diff_eq(&self.y, &other.y, epsilon)
            && f64::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl IntoIterator for Point {
    type Item = f64;
    type IntoIter = PointIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct PointIter {
    point: Point,
    index: u8,
}

impl Iterator for PointIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.index {
            0 => self.point.x,
            1 => self.point.y,
            2 => self.point.z,
            _ => return None,
        };

        self.index += 1;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector;

    #[test]
    fn subtracting_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn adding_vector_to_point() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);
        assert_eq!(p + v, point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }
}
