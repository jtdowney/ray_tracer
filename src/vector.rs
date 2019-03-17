use generic_array::{ArrayLength, GenericArray};
use num_traits::{Float, Num};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};
use typenum::U3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector<T, N>
where
    T: Default + Num + PartialEq + PartialOrd,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub values: GenericArray<T, N>,
}

pub type Vector3<T> = Vector<T, U3>;

impl<T, N> Vector<T, N>
where
    T: Clone + Default + Num + PartialEq + PartialOrd,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub fn new(values: &[T]) -> Vector<T, N> {
        Vector {
            values: GenericArray::clone_from_slice(values),
        }
    }
}

impl<T, N> Vector<T, N>
where
    T: Copy + Default + Num + PartialEq + PartialOrd + Sum,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    pub fn dot(&self, other: Vector<T, N>) -> T {
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
    T: Copy + Default + Num + PartialEq + PartialOrd,
{
    pub fn cross(&self, other: &Vector<T, U3>) -> Vector<T, U3> {
        Vector::<T, U3>::new(&[
            self.values[1] * other.values[2] - self.values[2] * other.values[1],
            self.values[2] * other.values[0] - self.values[0] * other.values[2],
            self.values[0] * other.values[1] - self.values[1] * other.values[0],
        ])
    }
}

impl<T, N> Vector<T, N>
where
    T: Default + Float + Num + PartialEq + PartialOrd + Sum,
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
    T: Default + Num + PartialEq + PartialOrd,
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
    T: Copy + Default + Num + PartialEq + PartialOrd,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn div(self, other: T) -> Self::Output {
        let values = self.values.into_iter().map(|n| n / other).collect();
        Vector { values }
    }
}

impl<T, N> Mul<T> for Vector<T, N>
where
    T: Copy + Default + Num + PartialEq + PartialOrd,
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
    T: Default + Neg<Output = T> + Num + PartialEq + PartialOrd,
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
    T: Copy + Default + Num + PartialEq + PartialOrd,
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
    T: Default + Num + PartialEq + PartialOrd,
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
        let v1 = Vector3::new(&[3, -2, 5]);
        let v2 = Vector3::new(&[-2, 3, 1]);
        assert_eq!(Vector::new(&[1, 1, 6]), v1 + v2);
    }

    #[test]
    fn test_subtracting_vectors() {
        let v1 = Vector3::new(&[3, 2, 1]);
        let v2 = Vector3::new(&[5, 6, 7]);
        assert_eq!(Vector::new(&[-2, -4, -6]), v1 - v2);
    }

    #[test]
    fn test_subtracting_vector_from_zero() {
        let zero = Vector3::default();
        let v = Vector3::new(&[1, -2, 3]);
        assert_eq!(Vector::new(&[-1, 2, -3]), zero - v);
    }

    #[test]
    fn test_negating_vector() {
        let v = Vector3::new(&[1, -2, 3]);
        assert_eq!(Vector::new(&[-1, 2, -3]), -v);
    }

    #[test]
    fn test_multiplying_vector_by_scalar() {
        let v = Vector3::new(&[1.0, -2.0, 3.0]);
        assert_eq!(Vector::new(&[3.5, -7.0, 10.5]), v * 3.5);
    }

    #[test]
    fn test_multiplying_vector_by_fraction() {
        let v = Vector3::new(&[1.0, -2.0, 3.0]);
        assert_eq!(Vector::new(&[0.5, -1.0, 1.5]), v * 0.5);
    }

    #[test]
    fn test_dividing_vector_by_scalar() {
        let v = Vector3::new(&[1.0, -2.0, 3.0]);
        assert_eq!(Vector::new(&[0.5, -1.0, 1.5]), v / 2.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3::new(&[1.0, 0.0, 0.0]);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(&[0.0, 1.0, 0.0]);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(&[0.0, 0.0, 1.0]);
        assert_eq!(1.0, v.magnitude());
        let v = Vector3::new(&[1.0, 2.0, 3.0]);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
        let v = Vector3::new(&[-1.0, -2.0, -3.0]);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector3::new(&[4.0, 0.0, 0.0]);
        assert_eq!(Vector::new(&[1.0, 0.0, 0.0]), v.normalize());
        let v = Vector3::new(&[1.0, 2.0, 3.0]);
        assert_eq!(Vector::new(&[0.26726, 0.53452, 0.80178]), v.normalize());
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3::new(&[1, 2, 3]);
        let v2 = Vector3::new(&[2, 3, 4]);
        assert_eq!(20, v1.dot(v2));
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(&[1, 2, 3]);
        let v2 = Vector3::new(&[2, 3, 4]);
        assert_eq!(Vector::new(&[-1, 2, -1]), v1.cross(&v2));
        assert_eq!(Vector::new(&[1, -2, 1]), v2.cross(&v1));
    }
}
