use crate::EPSILON;
use approx::AbsDiffEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(self, normal: Vector) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn cross(self, other: Vector) -> Self {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(self) -> f64 {
        let value = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        value.sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        vector(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl AbsDiffEq for Vector {
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

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        vector(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        vector(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        vector(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        vector(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;
    use approx::assert_abs_diff_eq;
    use quickcheck::Arbitrary;

    impl Arbitrary for Vector {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let values = iter::repeat_with(|| f64::arbitrary(g))
                .filter(|n| n.is_normal())
                .take(3)
                .collect::<Vec<f64>>();
            let x = values[0];
            let y = values[1];
            let z = values[2];

            vector(x, y, z)
        }
    }

    #[test]
    fn adding_vectors() {
        let v1 = vector(3.0, -2.0, 5.0);
        let v2 = vector(-2.0, 3.0, 1.0);
        assert_eq!(v1 + v2, vector(1.0, 1.0, 6.0))
    }

    #[test]
    fn subtracting_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn negating_vectors() {
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(-v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scalar_multiplication_vector() {
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(v * 3.5, vector(3.5, -7.0, 10.5));
    }

    #[test]
    fn scalar_fraction_multiplication_vector() {
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(v * 0.5, vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn scalar_division_vector() {
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn vector_magnitude() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn vector_normalization() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
        let v = vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn vector_dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn vector_cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_approaching_at_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1.0, 1.0, 0.0))
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        let r = v.reflect(n);
        assert_abs_diff_eq!(r, vector(1.0, 0.0, 0.0))
    }
}
