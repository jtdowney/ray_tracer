use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::AsPrimitive;
use wide::f32x4;

pub fn vector(
    x: impl AsPrimitive<f32>,
    y: impl AsPrimitive<f32>,
    z: impl AsPrimitive<f32>,
) -> Vector {
    Vector {
        data: f32x4::new([x.as_(), y.as_(), z.as_(), 0.0]),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub(crate) data: f32x4,
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            data: f32x4::splat(0.0),
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Vector {
    #[must_use]
    pub fn x(&self) -> f32 {
        self.data.as_array()[0]
    }

    #[must_use]
    pub fn y(&self) -> f32 {
        self.data.as_array()[1]
    }

    #[must_use]
    pub fn z(&self) -> f32 {
        self.data.as_array()[2]
    }

    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    #[must_use]
    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - *normal * 2.0 * self.dot(normal)
    }

    #[must_use]
    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            data: self.data / f32x4::splat(mag),
        }
    }

    #[must_use]
    pub fn dot(&self, other: &Vector) -> f32 {
        let product = self.data * other.data;
        let arr = product.as_array();
        arr[0] + arr[1] + arr[2]
    }

    #[must_use]
    pub fn cross(&self, other: &Vector) -> Vector {
        let (ax, ay, az) = (self.x(), self.y(), self.z());
        let (bx, by, bz) = (other.x(), other.y(), other.z());
        Vector {
            data: f32x4::new([
                ay * bz - az * by,
                az * bx - ax * bz,
                ax * by - ay * bx,
                0.0,
            ]),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            data: self.data + rhs.data,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            data: self.data - rhs.data,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector { data: -self.data }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            data: self.data * f32x4::splat(rhs),
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            data: self.data / f32x4::splat(rhs),
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
        assert_relative_eq!(v.x(), 4.3);
        assert_relative_eq!(v.y(), -4.2);
        assert_relative_eq!(v.z(), 3.1);
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
        assert_relative_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = vector(-1, -2, -3);
        assert_relative_eq!(v.magnitude(), 14.0_f32.sqrt());
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
        let sqrt14 = 14.0_f32.sqrt();
        assert_relative_eq!(norm.x(), 1.0 / sqrt14);
        assert_relative_eq!(norm.y(), 2.0 / sqrt14);
        assert_relative_eq!(norm.z(), 3.0 / sqrt14);
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
        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let n = vector(sqrt2_over_2, sqrt2_over_2, 0);
        let r = v.reflect(&n);
        assert_relative_eq!(r.x(), 1.0, epsilon = crate::EPSILON);
        assert_relative_eq!(r.y(), 0.0, epsilon = crate::EPSILON);
        assert_relative_eq!(r.z(), 0.0, epsilon = crate::EPSILON);
    }
}
