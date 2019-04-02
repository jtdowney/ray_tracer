use crate::{Vector3, EPSILON};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Self::Output {
        Point::new(self.x + other[0], self.y + other[1], self.z + other[2])
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Self::Output {
        Point::new(self.x - other[0], self.y - other[1], self.z - other[2])
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_points() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let p2 = Point::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), p1 + p2);
    }

    #[test]
    fn adding_point_and_vector() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector3::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), p + v);
    }

    #[test]
    fn subtracting_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(Vector3::new(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(Point::new(-2.0, -4.0, -6.0), p - v);
    }
}
