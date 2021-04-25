use approx::AbsDiffEq;
use num::Num;

use crate::vector::Vector;
use std::ops::{Add, Sub};

pub fn point<T>(x: T, y: T, z: T) -> Point<T>
where
    T: Copy,
{
    Point::new(x, y, z)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Point<T>
where
    T: Num + Copy,
{
    pub fn origin() -> Self {
        let x = T::zero();
        let y = T::zero();
        let z = T::zero();
        Self { x, y, z }
    }
}

impl<T> AbsDiffEq for Point<T>
where
    T: AbsDiffEq + Copy,
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &other.x, epsilon)
            && T::abs_diff_eq(&self.y, &other.y, epsilon)
            && T::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Add<Vector<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub<Vector<T>> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector;

    #[test]
    fn subtracting_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(p1 - p2, vector(-2, -4, -6));
    }

    #[test]
    fn adding_vector_to_point() {
        let p = point(3, -2, 5);
        let v = vector(-2, 3, 1);
        assert_eq!(p + v, point(1, 1, 6));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);
        assert_eq!(p - v, point(-2, -4, -6));
    }
}
