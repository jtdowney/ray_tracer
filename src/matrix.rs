use crate::{Point, Scalar, Vector, Vector3};
use generic_array::{ArrayLength, GenericArray, GenericArrayIter};
use itertools::iproduct;
use num_traits::{Float, One, Zero};
use std::error::Error;
use std::fmt::{self, Display};
use std::iter::{FromIterator, Sum};
use std::marker::PhantomData;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};
use typenum::{Prod, U2, U3, U4};

#[derive(Debug)]
pub struct NotInvertableError;

impl Error for NotInvertableError {}

impl Display for NotInvertableError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Matrix cannot be inverted")
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix<T, N, S = Prod<N, N>>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
{
    pub values: GenericArray<T, S>,
    _phantom: PhantomData<N>,
}

pub type Matrix2<T> = Matrix<T, U2>;
pub type Matrix3<T> = Matrix<T, U3>;
pub type Matrix4<T> = Matrix<T, U4>;

impl<T, N, S> Matrix<T, N, S>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    pub fn new(values: &[T]) -> Self {
        values.iter().cloned().collect()
    }

    pub fn row(&self, row: usize) -> Vector<T, N> {
        let offset = row * N::to_usize();
        let values = self.values[offset..]
            .iter()
            .take(N::to_usize())
            .cloned()
            .collect();
        Vector { values }
    }

    pub fn column(&self, column: usize) -> Vector<T, N> {
        let values = self.values[column..]
            .iter()
            .step_by(N::to_usize())
            .cloned()
            .collect();
        Vector { values }
    }

    pub fn transpose(&self) -> Self {
        (0..N::to_usize()).flat_map(|i| self.column(i)).collect()
    }
}

impl<T, N, S> Matrix<T, N, S>
where
    T: Scalar + One,
    N: ArrayLength<T> + Default,
    S: ArrayLength<T> + Default,
    S::ArrayType: Copy,
{
    pub fn identity() -> Self {
        let mut output = Matrix::default();
        for i in 0..N::to_usize() {
            output[(i, i)] = T::one();
        }

        output
    }
}

impl<T, N, S> FromIterator<T> for Matrix<T, N, S>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Matrix {
            values: iter.into_iter().collect(),
            _phantom: PhantomData,
        }
    }
}

impl<T, N, S> Index<(usize, usize)> for Matrix<T, N, S>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let offset = i * N::to_usize() + j;
        &self.values[offset]
    }
}

impl<T, N, S> IndexMut<(usize, usize)> for Matrix<T, N, S>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let offset = i * N::to_usize() + j;
        &mut self.values[offset]
    }
}

impl<T, N, S> IntoIterator for Matrix<T, N, S>
where
    T: Scalar,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
{
    type Item = T;
    type IntoIter = GenericArrayIter<T, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<T, N, S> Mul<Matrix<T, N, S>> for Matrix<T, N, S>
where
    T: Scalar + Mul<Output = T> + Sum<T>,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    type Output = Matrix<T, N, S>;

    fn mul(self, other: Matrix<T, N, S>) -> Self::Output {
        iproduct!(0..N::to_usize(), 0..N::to_usize())
            .map(|(i, j)| self.row(i).dot(other.column(j)))
            .collect()
    }
}

impl<T, N, S> Mul<Vector<T, N>> for Matrix<T, N, S>
where
    T: Scalar + Mul<Output = T> + Sum<T> + Default,
    N: ArrayLength<T> + Copy,
    S: ArrayLength<T>,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, other: Vector<T, N>) -> Self::Output {
        (0..N::to_usize()).map(|i| self.row(i).dot(other)).collect()
    }
}

impl<T> Mul<Point<T>> for Matrix4<T>
where
    T: Scalar + Mul<Output = T> + Add<Output = T>,
{
    type Output = Point<T>;

    fn mul(self, Point { x, y, z }: Point<T>) -> Self::Output {
        let row = self.row(0);
        let t0 = row[0] * x + row[1] * y + row[2] * z + row[3];
        let row = self.row(1);
        let t1 = row[0] * x + row[1] * y + row[2] * z + row[3];
        let row = self.row(2);
        let t2 = row[0] * x + row[1] * y + row[2] * z + row[3];

        Point::new(t0, t1, t2)
    }
}

impl<T> Mul<Vector3<T>> for Matrix4<T>
where
    T: Scalar + Mul<Output = T> + Add<Output = T>,
{
    type Output = Vector3<T>;

    fn mul(self, vector: Vector3<T>) -> Self::Output {
        let row = self.row(0);
        let t0 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];
        let row = self.row(1);
        let t1 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];
        let row = self.row(2);
        let t2 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];

        Vector3::new(t0, t1, t2)
    }
}

impl<T, N, S> PartialEq for Matrix<T, N, S>
where
    T: Scalar + Sub<Output = T>,
    N: ArrayLength<T>,
    S: ArrayLength<T>,
    S::ArrayType: Copy,
    f64: From<T>,
{
    fn eq(&self, other: &Matrix<T, N, S>) -> bool {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .all(|(&a, &b)| f64::from(a - b).abs() <= 0.00001)
    }
}

impl<T> Matrix2<T>
where
    T: Scalar + Mul<Output = T> + Sub<Output = T>,
{
    pub fn determinant(&self) -> T {
        self.values[0] * self.values[3] - self.values[1] * self.values[2]
    }
}

// TODO: Find a way to make these generic
impl<T> Matrix3<T>
where
    T: Scalar + Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Sum<T>,
{
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2<T> {
        (0..3)
            .filter(|&i| i != row)
            .flat_map(|i| {
                self.row(i)
                    .into_iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != col)
                    .map(|(_, n)| n)
            })
            .collect()
    }

    pub fn minor(&self, row: usize, col: usize) -> T {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let value = self.minor(row, col);
        if row + col % 2 == 0 {
            value
        } else {
            -value
        }
    }

    pub fn determinant(&self) -> T {
        let row = self.row(0);
        (0..3).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }
}

impl<T> Matrix4<T>
where
    T: Scalar + Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Sum<T> + Zero,
{
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3<T> {
        (0..4)
            .filter(|&i| i != row)
            .flat_map(|i| {
                self.row(i)
                    .into_iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != col)
                    .map(|(_, n)| n)
            })
            .collect()
    }

    pub fn minor(&self, row: usize, col: usize) -> T {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let value = self.minor(row, col);
        if (row + col) % 2 == 0 {
            value
        } else {
            -value
        }
    }

    pub fn determinant(&self) -> T {
        let row = self.row(0);
        (0..4).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }
}

impl<T> Matrix4<T>
where
    T: Scalar + Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Sum<T> + Zero + Float,
{
    pub fn inverse(&self) -> Result<Self, NotInvertableError> {
        let determinant = self.determinant();
        if determinant == T::zero() {
            return Err(NotInvertableError);
        }

        let output = iproduct!(0..4, 0..4)
            .map(|(i, j)| self.cofactor(i, j))
            .collect::<Matrix4<T>>()
            .transpose()
            .into_iter()
            .map(|n| n / determinant)
            .collect();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector4;

    #[test]
    fn test_constructing_2x2_matrix() {
        let m = Matrix2::new(&[-3, 5, 1, -2]);

        assert_eq!(-3, m[(0, 0)]);
        assert_eq!(5, m[(0, 1)]);
        assert_eq!(1, m[(1, 0)]);
        assert_eq!(-2, m[(1, 1)]);
    }

    #[test]
    fn test_constructing_3x3_matrix() {
        let m = Matrix3::new(&[-3, 5, 0, 1, -2, -7, 0, 1, 1]);

        assert_eq!(-3, m[(0, 0)]);
        assert_eq!(-2, m[(1, 1)]);
        assert_eq!(1, m[(2, 2)]);
    }

    #[test]
    fn test_constructing_4x4_matrix() {
        let m = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(1.0, m[(0, 0)]);
        assert_eq!(4.0, m[(0, 3)]);
        assert_eq!(5.5, m[(1, 0)]);
        assert_eq!(7.5, m[(1, 2)]);
        assert_eq!(11.0, m[(2, 2)]);
        assert_eq!(13.5, m[(3, 0)]);
        assert_eq!(15.5, m[(3, 2)]);
    }

    #[test]
    fn test_matrix_row() {
        let m = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);

        assert_eq!(Vector4::new(5, 6, 7, 8), m.row(1));
    }

    #[test]
    fn test_matrix_column() {
        let m = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);

        assert_eq!(Vector4::new(2, 6, 8, 4), m.column(1));
    }

    #[test]
    fn test_matrix_equality() {
        let a = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);
        let b = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_matrix_inequality() {
        let a = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);
        let b = Matrix4::new(&[2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

        assert_ne!(a, b);
    }

    #[test]
    fn test_multiplying_matrices() {
        let a = Matrix4::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);
        let b = Matrix4::new(&[-2, 1, 2, 3, 3, 2, 1, -1, 4, 3, 6, 5, 1, 2, 7, 8]);

        assert_eq!(
            Matrix4::new(&[20, 22, 50, 48, 44, 54, 114, 108, 40, 58, 110, 102, 16, 26, 46, 42]),
            a * b
        );
    }

    #[test]
    fn test_multiplying_matrix_by_vector() {
        let a = Matrix4::new(&[1, 2, 3, 4, 2, 4, 4, 2, 8, 6, 4, 1, 0, 0, 0, 1]);
        let b = Vector4::new(1, 2, 3, 1);

        assert_eq!(Vector4::new(18, 24, 33, 1), a * b);
    }

    #[test]
    fn test_identity() {
        assert_eq!(Matrix2::new(&[1, 0, 0, 1]), Matrix::identity());
        assert_eq!(
            Matrix3::new(&[1, 0, 0, 0, 1, 0, 0, 0, 1]),
            Matrix::identity()
        );
        assert_eq!(
            Matrix4::new(&[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]),
            Matrix::identity()
        );
    }

    #[test]
    fn test_multiplying_matrix_by_identity() {
        let m = Matrix4::new(&[0, 1, 2, 4, 1, 2, 4, 8, 2, 4, 8, 16, 4, 8, 16, 32]);
        assert_eq!(m, m * Matrix::identity());
    }

    #[test]
    fn test_transposing_matrix() {
        let m = Matrix4::new(&[0, 9, 3, 0, 9, 8, 0, 8, 1, 8, 5, 3, 0, 0, 5, 8]);
        assert_eq!(
            Matrix4::new(&[0, 9, 1, 0, 9, 8, 8, 0, 3, 0, 5, 5, 0, 8, 3, 8]),
            m.transpose()
        );
    }

    #[test]
    fn test_transposing_identity_matrix() {
        let m = Matrix4::<i8>::identity();
        assert_eq!(Matrix4::identity(), m.transpose());
    }

    #[test]
    fn test_determinant_of_2x2_matrix() {
        let m = Matrix2::new(&[1, 5, -3, 2]);
        assert_eq!(17, m.determinant());
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let m = Matrix3::new(&[1, 5, 0, -3, 2, 7, 0, 6, -3]);
        assert_eq!(Matrix2::new(&[-3, 2, 0, 6]), m.submatrix(0, 2));
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let m = Matrix4::new(&[-6, 1, 1, 6, -8, 5, 8, 6, -1, 0, 8, 2, -7, 1, -1, 1]);
        assert_eq!(
            Matrix3::new(&[-6, 1, 6, -8, 8, 6, -7, -1, 1]),
            m.submatrix(2, 1)
        );
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        let m = Matrix3::new(&[3, 5, 0, 2, -1, -7, 6, -1, 5]);
        assert_eq!(25, m.minor(1, 0));
    }

    #[test]
    fn test_cofactor_of_3x3_matrix() {
        let m = Matrix3::new(&[3, 5, 0, 2, -1, -7, 6, -1, 5]);
        assert_eq!(-12, m.cofactor(0, 0));
        assert_eq!(-25, m.cofactor(1, 0));
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let m = Matrix3::new(&[1, 2, 6, -5, 8, -4, 2, 6, 4]);
        assert_eq!(56, m.cofactor(0, 0));
        assert_eq!(12, m.cofactor(0, 1));
        assert_eq!(-46, m.cofactor(0, 2));
        assert_eq!(-196, m.determinant());
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let m = Matrix4::new(&[-2, -8, 3, 5, -3, 1, 7, 3, 1, 2, -9, 6, -6, 7, 7, -9]);
        assert_eq!(690, m.cofactor(0, 0));
        assert_eq!(447, m.cofactor(0, 1));
        assert_eq!(210, m.cofactor(0, 2));
        assert_eq!(51, m.cofactor(0, 3));
        assert_eq!(-4071, m.determinant());
    }

    #[test]
    fn test_invertable_matrix() {
        let m = Matrix4::new(&[
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        ]);
        assert_eq!(-2120.0, m.determinant());
        assert!(m.inverse().is_ok());
    }

    #[test]
    fn test_noninvertable_matrix() {
        let m = Matrix4::new(&[
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(0.0, m.determinant());
        assert!(m.inverse().is_err());
    }

    #[test]
    fn test_calculating_inverse_matrix() {
        let a = Matrix4::new(&[
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);
        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(105.0, a.cofactor(3, 2));

        let b = a.inverse().unwrap();
        assert_eq!(-160.0 / 532.0, b[(3, 2)]);
        assert_eq!(105.0 / 532.0, b[(2, 3)]);
        assert_eq!(
            Matrix4::new(&[
                0.218045, 0.451128, 0.240602, -0.045113, -0.808271, -1.456767, -0.443609, 0.520677,
                -0.078947, -0.223684, -0.052632, 0.197368, -0.522556, -0.813910, -0.300752,
                0.306391
            ]),
            b
        );

        let m = Matrix4::new(&[
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        ]);
        assert_eq!(
            Matrix4::new(&[
                -0.153846, -0.153846, -0.282051, -0.538462, -0.076923, 0.123077, 0.025641,
                0.030769, 0.358974, 0.358974, 0.435897, 0.923077, -0.692308, -0.692308, -0.769231,
                -1.923077,
            ]),
            m.inverse().unwrap()
        );

        let m = Matrix4::new(&[
            9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
        ]);
        assert_eq!(
            Matrix4::new(&[
                -0.040741, -0.077778, 0.144444, -0.222222, -0.077778, 0.033333, 0.366667,
                -0.333333, -0.029012, -0.146296, -0.109259, 0.129630, 0.177778, 0.066667,
                -0.266667, 0.333333,
            ]),
            m.inverse().unwrap()
        );
    }

    #[test]
    fn test_multiplying_matrix_by_inverse() {
        let a = Matrix4::new(&[
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = Matrix4::new(&[
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        ]);
        let c = a * b;
        assert_eq!(a, c * b.inverse().unwrap());
    }
}
