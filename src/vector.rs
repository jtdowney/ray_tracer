use approx::AbsDiffEq;
use num::Float;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn vector<T>(x: T, y: T, z: T) -> Vector<T>
where
    T: Copy,
{
    Vector::new(x, y, z)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub fn dot(self, other: Vector<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vector<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    pub fn cross(self, other: Vector<T>) -> Vector<T> {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T> Vector<T>
where
    T: Float + Copy,
{
    pub fn magnitude(self) -> T {
        let value = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        value.sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl<T> AbsDiffEq for Vector<T>
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

impl<T> Vector<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPSILON;
    use approx::assert_abs_diff_eq;
    use quickcheck::Arbitrary;

    impl<T> Arbitrary for Vector<T>
    where
        T: Arbitrary + Copy + Clone,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let x = T::arbitrary(g);
            let y = T::arbitrary(g);
            let z = T::arbitrary(g);

            Vector::new(x, y, z)
        }
    }

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
