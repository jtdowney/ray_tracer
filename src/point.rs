use crate::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, other: f32) -> Self::Output {
        Point::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, other: f32) -> Self::Output {
        Point::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        const EPSILON: f32 = 0.00001;
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_points() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let p2 = Point::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), p1 + p2);
    }

    #[test]
    fn test_adding_point_and_vector() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), p + v);
    }

    #[test]
    fn test_subtracting_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(Point::new(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn test_negating_point() {
        let p = Point::new(1.0, -2.0, 3.0);
        assert_eq!(Point::new(-1.0, 2.0, -3.0), -p);
    }

    #[test]
    fn test_multiplying_point_by_scalar() {
        let p = Point::new(1.0, -2.0, 3.0);
        assert_eq!(Point::new(3.5, -7.0, 10.5), p * 3.5);
    }

    #[test]
    fn test_multiplying_point_by_fraction() {
        let p = Point::new(1.0, -2.0, 3.0);
        assert_eq!(Point::new(0.5, -1.0, 1.5), p * 0.5);
    }

    #[test]
    fn test_dividing_point_by_scalar() {
        let p = Point::new(1.0, -2.0, 3.0);
        assert_eq!(Point::new(0.5, -1.0, 1.5), p / 2.0);
    }
}
