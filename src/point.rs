use crate::{Scalar, Vector3};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Point<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Add<Vector3<T>> for Point<T>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: Vector3<T>) -> Self::Output {
        Point::new(
            self.x + other.values[0],
            self.y + other.values[1],
            self.z + other.values[2],
        )
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: Scalar + Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, other: Point<T>) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Sub<Vector3<T>> for Point<T>
where
    T: Scalar + Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, other: Vector3<T>) -> Self::Output {
        Point::new(
            self.x - other.values[0],
            self.y - other.values[1],
            self.z - other.values[2],
        )
    }
}

impl<T> PartialEq for Point<T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    fn eq(&self, other: &Point<T>) -> bool {
        const EPSILON: f64 = 0.00001;
        f64::from(self.x - other.x).abs() < EPSILON
            && f64::from(self.y - other.y).abs() < EPSILON
            && f64::from(self.z - other.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_points() {
        let p1 = Point::new(3, -2, 5);
        let p2 = Point::new(-2, 3, 1);
        assert_eq!(Point::new(1, 1, 6), p1 + p2);
    }

    #[test]
    fn test_adding_point_and_vector() {
        let p = Point::new(3, -2, 5);
        let v = Vector3::new(-2, 3, 1);
        assert_eq!(Point::new(1, 1, 6), p + v);
    }

    #[test]
    fn test_subtracting_points() {
        let p1 = Point::new(3, 2, 1);
        let p2 = Point::new(5, 6, 7);
        assert_eq!(Vector3::new(-2, -4, -6), p1 - p2);
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let p = Point::new(3, 2, 1);
        let v = Vector3::new(5, 6, 7);
        assert_eq!(Point::new(-2, -4, -6), p - v);
    }
}
