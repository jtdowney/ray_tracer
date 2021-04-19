use approx::AbsDiffEq;
use num::Float;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn vector<N>(x: N, y: N, z: N) -> Vector<N>
where
    N: Copy,
{
    Vector::new(x, y, z)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector<N>
where
    N: Copy,
{
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Vector<N>
where
    N: Mul<Output = N> + Add<Output = N> + Copy,
{
    pub fn dot(self, other: Vector<N>) -> N {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<N> Vector<N>
where
    N: Mul<Output = N> + Sub<Output = N> + Copy,
{
    pub fn cross(self, other: Vector<N>) -> Vector<N> {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<N> Vector<N>
where
    N: Float + Copy,
{
    pub fn magnitude(self) -> N {
        let value = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        value.sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl<N: AbsDiffEq> AbsDiffEq for Vector<N>
where
    N: Copy,
    N::Epsilon: Copy,
{
    type Epsilon = N::Epsilon;

    fn default_epsilon() -> N::Epsilon {
        N::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: N::Epsilon) -> bool {
        N::abs_diff_eq(&self.x, &other.x, epsilon)
            && N::abs_diff_eq(&self.y, &other.y, epsilon)
            && N::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<N> Vector<N>
where
    N: Copy,
{
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N> Add for Vector<N>
where
    N: Add<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<N> Sub for Vector<N>
where
    N: Sub<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<N> Neg for Vector<N>
where
    N: Neg<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl<N> Mul<N> for Vector<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn mul(self, rhs: N) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<N> Div<N> for Vector<N>
where
    N: Div<Output = N> + Copy,
{
    type Output = Vector<N>;

    fn div(self, rhs: N) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPSILON;
    use approx::assert_abs_diff_eq;

    #[test]
    fn adding_vectors() {
        let v1 = vector(3, -2, 5);
        let v2 = vector(-2, 3, 1);
        assert_eq!(v1 + v2, vector(1, 1, 6))
    }

    #[test]
    fn subtracting_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(v1 - v2, vector(-2, -4, -6))
    }

    #[test]
    fn negating_vectors() {
        let v = vector(1, -2, 3);
        assert_eq!(-v, vector(-1, 2, -3));
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
        assert_abs_diff_eq!(
            v.normalize(),
            vector(0.26726, 0.53452, 0.80178),
            epsilon = EPSILON
        );
    }

    #[test]
    fn vector_dot_product() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.dot(b), 20);
    }

    #[test]
    fn vector_cross_product() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(b), vector(-1, 2, -1));
        assert_eq!(b.cross(a), vector(1, -2, 1));
    }
}
