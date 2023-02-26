use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn vector<T: Into<f64>>(x: T, y: T, z: T) -> Vector {
    Vector {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn magnitude(self) -> f64 {
        let Self { x, y, z } = self;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        let x = self.x / magnitude;
        let y = self.y / magnitude;
        let z = self.z / magnitude;
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Self { x, y, z }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Self::Output {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Self { x, y, z }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Self::Output {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;
        Self { x, y, z }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Vector {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let x = f64::from(i32::arbitrary(g));
        let y = f64::from(i32::arbitrary(g));
        let z = f64::from(i32::arbitrary(g));
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn adding_vectors(a: Vector, b: Vector) {
        let c = a + b;
        assert_eq!(a.x + b.x, c.x);
        assert_eq!(a.y + b.y, c.y);
        assert_eq!(a.z + b.z, c.z);
    }

    #[quickcheck]
    fn subtracting_vectors(a: Vector, b: Vector) {
        let c = a - b;
        assert_eq!(a.x - b.x, c.x);
        assert_eq!(a.y - b.y, c.y);
        assert_eq!(a.z - b.z, c.z);
    }

    #[quickcheck]
    fn negating_vector(v: Vector) {
        let negated = -v;
        assert_eq!(-v.x, negated.x);
        assert_eq!(-v.y, negated.y);
        assert_eq!(-v.z, negated.z);
    }

    #[quickcheck]
    fn scalar_multiplying_vector(v: Vector, scale: i32) {
        let scale = f64::from(scale);
        let scaled = v * scale;
        assert_eq!(v.x * scale, scaled.x);
        assert_eq!(v.y * scale, scaled.y);
        assert_eq!(v.z * scale, scaled.z);
    }

    #[quickcheck]
    fn scalar_dividing_vector(v: Vector, scale: i32) -> TestResult {
        if scale == 0 {
            return TestResult::discard();
        }

        let scale = f64::from(scale);
        let scaled = v / scale;
        assert_eq!(v.x / scale, scaled.x);
        assert_eq!(v.y / scale, scaled.y);
        assert_eq!(v.z / scale, scaled.z);

        TestResult::passed()
    }

    #[quickcheck]
    fn vector_magnitude(v @ Vector { x, y, z }: Vector) {
        let magnitude = v.magnitude();
        assert_eq!((x.powi(2) + y.powi(2) + z.powi(2)).sqrt(), magnitude);
    }

    #[quickcheck]
    fn vector_normalization(v: Vector) {
        let magnitude = v.magnitude();
        let normalized = v.normalize();
        assert_eq!(v.x / magnitude, normalized.x);
        assert_eq!(v.y / magnitude, normalized.y);
        assert_eq!(v.z / magnitude, normalized.z);
    }

    #[test]
    fn vector_dot_example() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(20.0, a.dot(b));
    }

    #[quickcheck]
    fn vector_dot_product(a: Vector, b: Vector) {
        let dot = a.dot(b);
        assert_eq!(a.x * b.x + a.y * b.y + a.z * b.z, dot);
    }

    #[test]
    fn vector_cross_example() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(vector(-1, 2, -1), a.cross(b));
        assert_eq!(vector(1, -2, 1), b.cross(a));
    }

    #[quickcheck]
    fn vector_cross_product(a: Vector, b: Vector) {
        let cross = a.cross(b);
        assert_eq!(a.y * b.z - a.z * b.y, cross.x);
        assert_eq!(a.z * b.x - a.x * b.z, cross.y);
        assert_eq!(a.x * b.y - a.y * b.x, cross.z);
    }
}
