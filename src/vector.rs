use crate::Scalar;
use generic_array::{ArrayLength, GenericArray, GenericArrayIter};
use num_traits::Float;
use std::iter::{FromIterator, Sum};
use std::ops::{Add, Div, Mul, Neg, Sub};
use typenum::{U3, U4};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector<T, N>
where
    T: Scalar,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub values: GenericArray<T, N>,
}

pub type Vector3<T> = Vector<T, U3>;
pub type Vector4<T> = Vector<T, U4>;

impl<T, N> Vector<T, N>
where
    T: Scalar + Mul<Output = T> + Sum,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub fn dot(&self, other: &Vector<T, N>) -> T {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .map(|(&a, &b)| a * b)
            .sum::<T>()
    }
}

impl<T> Vector3<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        [x, y, z].iter().cloned().collect()
    }
}

impl<T> Vector4<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        [x, y, z, w].iter().cloned().collect()
    }
}

impl<T> Vector3<T>
where
    T: Scalar + Mul<Output = T> + Sub<Output = T>,
{
    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            self.values[1] * other.values[2] - self.values[2] * other.values[1],
            self.values[2] * other.values[0] - self.values[0] * other.values[2],
            self.values[0] * other.values[1] - self.values[1] * other.values[0],
        )
    }
}

impl<T, N> Vector<T, N>
where
    T: Scalar + Float + Sum,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub fn magnitude(&self) -> T {
        self.values
            .as_slice()
            .iter()
            .map(|n| n.powi(2))
            .sum::<T>()
            .sqrt()
    }

    pub fn normalize(&self) -> Vector<T, N> {
        let magnitude = self.magnitude();
        let values = self
            .values
            .as_slice()
            .iter()
            .map(|&n| n / magnitude)
            .collect();
        Vector { values }
    }
}

impl<T, N> Add<Vector<T, N>> for Vector<T, N>
where
    T: Scalar + Add<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        let values = self
            .values
            .into_iter()
            .zip(other.values)
            .map(|(a, b)| a + b)
            .collect();
        Vector { values }
    }
}

impl<T, N> Div<T> for Vector<T, N>
where
    T: Scalar + Div<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn div(self, other: T) -> Self::Output {
        let values = self.values.into_iter().map(|n| n / other).collect();
        Vector { values }
    }
}

impl<T, N> FromIterator<T> for Vector<T, N>
where
    T: Scalar,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Vector {
            values: iter.into_iter().collect(),
        }
    }
}

impl<T, N> IntoIterator for Vector<T, N>
where
    T: Scalar,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Item = T;
    type IntoIter = GenericArrayIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<T, N> Mul<T> for Vector<T, N>
where
    T: Scalar + Mul<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, other: T) -> Self::Output {
        let values = self.values.into_iter().map(|n| n * other).collect();
        Vector { values }
    }
}

impl<T, N> Neg for Vector<T, N>
where
    T: Scalar + Neg<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        let values = self.values.into_iter().map(|v| -v).collect();
        Vector { values }
    }
}

impl<T, N> PartialEq for Vector<T, N>
where
    T: Scalar + Sub<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
    f64: From<T>,
{
    fn eq(&self, other: &Vector<T, N>) -> bool {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .all(|(&a, &b)| f64::from(a - b) <= 0.00001)
    }
}

impl<T, N> Sub<Vector<T, N>> for Vector<T, N>
where
    T: Scalar + Sub<Output = T>,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn sub(self, other: Vector<T, N>) -> Self::Output {
        let values = self
            .values
            .into_iter()
            .zip(other.values)
            .map(|(a, b)| a - b)
            .collect();
        Vector { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_vectors() {
        let v1 = Vector3::new(3, -2, 5);
        let v2 = Vector3::new(-2, 3, 1);
        assert_eq!(Vector3::new(1, 1, 6), v1 + v2);
    }

    #[test]
    fn test_subtracting_vectors() {
        let v1 = Vector3::new(3, 2, 1);
        let v2 = Vector3::new(5, 6, 7);
        assert_eq!(Vector3::new(-2, -4, -6), v1 - v2);
    }

    #[test]
    fn test_subtracting_vector_from_zero() {
        let zero = Vector3::default();
        let v = Vector3::new(1, -2, 3);
        assert_eq!(Vector3::new(-1, 2, -3), zero - v);
    }

    #[test]
    fn test_negating_vector() {
        let v = Vector3::new(1, -2, 3);
        assert_eq!(Vector3::new(-1, 2, -3), -v);
    }

    #[test]
    fn test_multiplying_vector_by_scalar() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(Vector3::new(3.5, -7.0, 10.5), v * 3.5);
    }

    #[test]
    fn test_multiplying_vector_by_fraction() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(Vector3::new(0.5, -1.0, 1.5), v * 0.5);
    }

    #[test]
    fn test_dividing_vector_by_scalar() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(Vector3::new(0.5, -1.0, 1.5), v / 2.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
        let v = Vector3::new(-1.0, -2.0, -3.0);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector3::new(4.0, 0.0, 0.0);
        assert_eq!(Vector3::new(1.0, 0.0, 0.0), v.normalize());
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(Vector3::new(0.26726, 0.53452, 0.80178), v.normalize());
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(2, 3, 4);
        assert_eq!(20, v1.dot(&v2));
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(2, 3, 4);
        assert_eq!(Vector3::new(-1, 2, -1), v1.cross(&v2));
        assert_eq!(Vector3::new(1, -2, 1), v2.cross(&v1));
    }
}
