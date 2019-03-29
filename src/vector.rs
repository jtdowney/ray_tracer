use generic_array::{ArrayLength, GenericArray, GenericArrayIter};
use std::iter::FromIterator;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use typenum::{U3, U4};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    values: GenericArray<f32, N>,
}

pub type Vector3 = Vector<U3>;
pub type Vector4 = Vector<U4>;

impl<N> Vector<N>
where
    N: ArrayLength<f32> + Copy,
    N::ArrayType: Copy,
{
    pub fn dot(self, other: Vector<N>) -> f32 {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub fn reflect(self, other: Vector<N>) -> Self {
        self - other * 2.0 * self.dot(other)
    }

    pub fn magnitude(&self) -> f32 {
        self.values
            .as_slice()
            .iter()
            .map(|n| n.powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn normalize(&self) -> Vector<N> {
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

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        [x, y, z].iter().cloned().collect()
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        [x, y, z, w].iter().cloned().collect()
    }
}

impl<N> Add<Vector<N>> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Self::Output {
        let values = self
            .values
            .into_iter()
            .zip(other.values)
            .map(|(a, b)| a + b)
            .collect();
        Vector { values }
    }
}

impl<N> Div<f32> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn div(self, other: f32) -> Self::Output {
        let values = self.values.into_iter().map(|n| n / other).collect();
        Vector { values }
    }
}

impl<N> Index<usize> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<N> FromIterator<f32> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = f32>,
    {
        Vector {
            values: iter.into_iter().collect(),
        }
    }
}

impl<N> IntoIterator for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Item = f32;
    type IntoIter = GenericArrayIter<f32, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<N> Mul<f32> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn mul(self, other: f32) -> Self::Output {
        let values = self.values.into_iter().map(|n| n * other).collect();
        Vector { values }
    }
}

impl<N> Neg for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        let values = self.values.into_iter().map(|v| -v).collect();
        Vector { values }
    }
}

impl<N> PartialEq for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    fn eq(&self, other: &Vector<N>) -> bool {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .all(|(&a, &b)| (a - b).abs() <= 0.00001)
    }
}

impl<N> Sub<Vector<N>> for Vector<N>
where
    N: ArrayLength<f32>,
    N::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn sub(self, other: Vector<N>) -> Self::Output {
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
        let v1 = Vector3::new(3.0, -2.0, 5.0);
        let v2 = Vector3::new(-2.0, 3.0, 1.0);
        assert_eq!(Vector3::new(1.0, 1.0, 6.0), v1 + v2);
    }

    #[test]
    fn test_subtracting_vectors() {
        let v1 = Vector3::new(3.0, 2.0, 1.0);
        let v2 = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(Vector3::new(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn test_subtracting_vector_from_zero() {
        let zero = Vector3::default();
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(Vector3::new(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn test_negating_vector() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(Vector3::new(-1.0, 2.0, -3.0), -v);
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
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        assert_eq!(20.0, v1.dot(v2));
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        assert_eq!(Vector3::new(-1.0, 2.0, -1.0), v1.cross(v2));
        assert_eq!(Vector3::new(1.0, -2.0, 1.0), v2.cross(v1));
    }

    #[test]
    fn test_reflecting_vector_approaching_45() {
        let v = Vector3::new(1.0, -1.0, 0.0);
        let n = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(Vector3::new(1.0, 1.0, 0.0), v.reflect(n));
    }

    #[test]
    fn test_reflecting_vector_off_slanted_surface() {
        let v = Vector3::new(0.0, -1.0, 0.0);
        let n = Vector3::new(f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0);
        assert_eq!(Vector3::new(1.0, 0.0, 0.0), v.reflect(n));
    }
}
