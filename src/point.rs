use std::ops::{Add, Sub};

use crate::{vector, Vector};

pub fn point<T: Into<f64>>(x: T, y: T, z: T) -> Point {
    Point {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        vector(x, y, z)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Point {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let x = f64::from(i32::arbitrary(g));
        let y = f64::from(i32::arbitrary(g));
        let z = f64::from(i32::arbitrary(g));
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn adding_point_and_vector(a: Point, b: Vector) {
        let c = a + b;
        assert_eq!(a.x + b.x, c.x);
        assert_eq!(a.y + b.y, c.y);
        assert_eq!(a.z + b.z, c.z);
    }

    #[quickcheck]
    fn subtracting_points(a: Point, b: Point) {
        let c = a - b;
        assert_eq!(a.x - b.x, c.x);
        assert_eq!(a.y - b.y, c.y);
        assert_eq!(a.z - b.z, c.z);
    }
}
