use std::ops::{Add, Sub};

use crate::vector::Vector;

pub fn point<N>(x: N, y: N, z: N) -> Point<N>
where
    N: Copy,
{
    Point::new(x, y, z)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point<N>
where
    N: Copy,
{
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Point<N>
where
    N: Copy,
{
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N> Sub for Point<N>
where
    N: Sub<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<N> Add<Vector<N>> for Point<N>
where
    N: Add<Output = N> + Copy,
{
    type Output = Point<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<N> Sub<Vector<N>> for Point<N>
where
    N: Sub<Output = N> + Copy,
{
    type Output = Point<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
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
