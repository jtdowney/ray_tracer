use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Vector {
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
    #[must_use]
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[must_use]
    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - *normal * 2.0 * self.dot(normal)
    }

    #[must_use]
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    #[must_use]
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[must_use]
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn vector_creates_vector_with_coordinates() {
        let v = vector(4.3, -4.2, 3.1);
        assert_relative_eq!(v.x, 4.3);
        assert_relative_eq!(v.y, -4.2);
        assert_relative_eq!(v.z, 3.1);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = vector(3, -2, 5);
        let v2 = vector(-2, 3, 1);
        assert_eq!(v1 + v2, vector(1, 1, 6));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(v1 - v2, vector(-2, -4, -6));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = vector(0, 0, 0);
        let v = vector(1, -2, 3);
        assert_eq!(zero - v, vector(-1, 2, -3));
    }

    #[test]
    fn negating_a_vector() {
        let v = vector(1, -2, 3);
        assert_eq!(-v, vector(-1, 2, -3));
    }

    #[test]
    fn multiplying_vector_by_scalar() {
        let v = vector(1, -2, 3);
        assert_eq!(v * 3.5, vector(3.5, -7, 10.5));
    }

    #[test]
    fn multiplying_vector_by_fraction() {
        let v = vector(1, -2, 3);
        assert_eq!(v * 0.5, vector(0.5, -1, 1.5));
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let v = vector(1, -2, 3);
        assert_eq!(v / 2.0, vector(0.5, -1, 1.5));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = vector(1, 0, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = vector(0, 1, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = vector(0, 0, 1);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = vector(1, 2, 3);
        assert_relative_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = vector(-1, -2, -3);
        assert_relative_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = vector(4, 0, 0);
        assert_eq!(v.normalize(), vector(1, 0, 0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = vector(1, 2, 3);
        let norm = v.normalize();
        let sqrt14 = 14.0_f64.sqrt();
        assert_relative_eq!(norm.x, 1.0 / sqrt14);
        assert_relative_eq!(norm.y, 2.0 / sqrt14);
        assert_relative_eq!(norm.z, 3.0 / sqrt14);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector(1, 2, 3);
        let norm = v.normalize();
        assert_relative_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_relative_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(&b), vector(-1, 2, -1));
        assert_eq!(b.cross(&a), vector(1, -2, 1));
    }

    #[test]
    fn reflecting_vector_approaching_at_45_degrees() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1, 1, 0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0, -1, 0);
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let n = vector(sqrt2_over_2, sqrt2_over_2, 0);
        let r = v.reflect(&n);
        assert_relative_eq!(r.x, 1.0, epsilon = crate::EPSILON);
        assert_relative_eq!(r.y, 0.0, epsilon = crate::EPSILON);
        assert_relative_eq!(r.z, 0.0, epsilon = crate::EPSILON);
    }
}
