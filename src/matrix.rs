use crate::{Point, Vector, Vector3, EPSILON};
use generic_array::{ArrayLength, GenericArray, GenericArrayIter};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Mul};
use typenum::{Prod, U2, U3, U4};

#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix<N, S = Prod<N, N>>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    values: GenericArray<f64, S>,
    _phantom: PhantomData<N>,
}

pub type Matrix2 = Matrix<U2>;
pub type Matrix3 = Matrix<U3>;
pub type Matrix4 = Matrix<U4>;

impl<N, S> Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    pub fn new(values: &[f64]) -> Self {
        values.iter().cloned().collect()
    }

    pub fn row(&self, row: usize) -> Vector<N> {
        let offset = row * N::to_usize();
        self.values[offset..]
            .iter()
            .take(N::to_usize())
            .cloned()
            .collect()
    }

    pub fn column(&self, column: usize) -> Vector<N> {
        self.values[column..]
            .iter()
            .step_by(N::to_usize())
            .cloned()
            .collect()
    }

    pub fn transpose(&self) -> Self {
        (0..N::to_usize()).flat_map(|i| self.column(i)).collect()
    }
}

impl<N, S> Matrix<N, S>
where
    N: ArrayLength<f64> + Default,
    S: ArrayLength<f64> + Default,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    pub fn identity() -> Self {
        let mut output = Matrix::default();
        for i in 0..N::to_usize() {
            output[(i, i)] = 1.0;
        }

        output
    }
}

impl<N, S> FromIterator<f64> for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        Matrix {
            values: iter.into_iter().collect(),
            _phantom: PhantomData,
        }
    }
}

impl<N, S> Index<(usize, usize)> for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let offset = i * N::to_usize() + j;
        &self.values[offset]
    }
}

impl<N, S> IndexMut<(usize, usize)> for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let offset = i * N::to_usize() + j;
        &mut self.values[offset]
    }
}

impl<N, S> IntoIterator for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    type Item = f64;
    type IntoIter = GenericArrayIter<f64, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<N, S> Mul<Matrix<N, S>> for Matrix<N, S>
where
    N: ArrayLength<f64> + Default + Copy,
    S: ArrayLength<f64> + Default,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    type Output = Matrix<N, S>;

    fn mul(self, other: Matrix<N, S>) -> Self::Output {
        let mut result = Matrix::<N, S>::default();

        for j in 0..N::to_usize() {
            let column = other.column(j);
            for i in 0..N::to_usize() {
                let row = self.row(i);
                result[(i, j)] = row.dot(column)
            }
        }

        result
    }
}

impl<N, S> Mul<Vector<N>> for Matrix<N, S>
where
    N: ArrayLength<f64> + Copy,
    S: ArrayLength<f64>,
    N::ArrayType: Copy,
    S::ArrayType: Copy,
{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Self::Output {
        (0..N::to_usize()).map(|i| self.row(i).dot(other)).collect()
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, Point { x, y, z }: Point) -> Self::Output {
        let row = self.row(0);
        let t0 = row[0] * x + row[1] * y + row[2] * z + row[3];
        let row = self.row(1);
        let t1 = row[0] * x + row[1] * y + row[2] * z + row[3];
        let row = self.row(2);
        let t2 = row[0] * x + row[1] * y + row[2] * z + row[3];

        Point::new(t0, t1, t2)
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Self::Output {
        let row = self.row(0);
        let t0 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];
        let row = self.row(1);
        let t1 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];
        let row = self.row(2);
        let t2 = row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2];

        Vector3::new(t0, t1, t2)
    }
}

impl<N, S> PartialEq for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    fn eq(&self, other: &Matrix<N, S>) -> bool {
        self.values
            .as_slice()
            .iter()
            .zip(other.values.as_slice())
            .all(|(&a, &b)| (a - b).abs() <= EPSILON)
    }
}

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self.values[0] * self.values[3] - self.values[1] * self.values[2]
    }
}

impl Matrix3 {
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
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

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let value = self.minor(row, col);
        if row + col % 2 == 0 {
            value
        } else {
            -value
        }
    }

    pub fn determinant(&self) -> f64 {
        let row = self.row(0);
        (0..3).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }
}

impl Matrix4 {
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
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

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let value = self.minor(row, col);
        if (row + col) % 2 == 0 {
            value
        } else {
            -value
        }
    }

    pub fn determinant(&self) -> f64 {
        let row = self.row(0);
        (0..4).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }

    pub fn inverse(&self) -> Self {
        let determinant = self.determinant();
        if determinant == 0.0 {
            panic!("Matrix not invertable: {:?}", self);
        }

        let mut output = Matrix4::default();
        for j in 0..4 {
            for i in 0..4 {
                let cofactor = self.cofactor(i, j);
                output[(j, i)] = cofactor / determinant;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector4;

    #[test]
    fn test_constructing_2x2_matrix() {
        let m = Matrix2::new(&[-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(-3.0, m[(0, 0)]);
        assert_eq!(5.0, m[(0, 1)]);
        assert_eq!(1.0, m[(1, 0)]);
        assert_eq!(-2.0, m[(1, 1)]);
    }

    #[test]
    fn test_constructing_3x3_matrix() {
        let m = Matrix3::new(&[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(-3.0, m[(0, 0)]);
        assert_eq!(-2.0, m[(1, 1)]);
        assert_eq!(1.0, m[(2, 2)]);
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
        let m = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(Vector4::new(5.0, 6.0, 7.0, 8.0), m.row(1));
    }

    #[test]
    fn test_matrix_column() {
        let m = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(Vector4::new(2.0, 6.0, 8.0, 4.0), m.column(1));
    }

    #[test]
    fn test_matrix_equality() {
        let a = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_matrix_inequality() {
        let a = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix4::new(&[
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn test_multiplying_matrices() {
        let a = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix4::new(&[
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);

        assert_eq!(
            Matrix4::new(&[
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ]),
            a * b
        );
    }

    #[test]
    fn test_multiplying_matrix_by_vector() {
        let a = Matrix4::new(&[
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let b = Vector4::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(Vector4::new(18.0, 24.0, 33.0, 1.0), a * b);
    }

    #[test]
    fn test_identity() {
        assert_eq!(Matrix2::new(&[1.0, 0.0, 0.0, 1.0]), Matrix::identity());
        assert_eq!(
            Matrix3::new(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]),
            Matrix::identity()
        );
        assert_eq!(
            Matrix4::new(&[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            ]),
            Matrix::identity()
        );
    }

    #[test]
    fn test_multiplying_matrix_by_identity() {
        let m = Matrix4::new(&[
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);
        assert_eq!(m, m * Matrix::identity());
    }

    #[test]
    fn test_transposing_matrix() {
        let m = Matrix4::new(&[
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);
        assert_eq!(
            Matrix4::new(&[
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            ]),
            m.transpose()
        );
    }

    #[test]
    fn test_transposing_identity_matrix() {
        let m = Matrix4::identity();
        assert_eq!(Matrix4::identity(), m.transpose());
    }

    #[test]
    fn test_determinant_of_2x2_matrix() {
        let m = Matrix2::new(&[1.0, 5.0, -3.0, 2.0]);
        assert_eq!(17.0, m.determinant());
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let m = Matrix3::new(&[1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        assert_eq!(Matrix2::new(&[-3.0, 2.0, 0.0, 6.0]), m.submatrix(0, 2));
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let m = Matrix4::new(&[
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);
        assert_eq!(
            Matrix3::new(&[-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]),
            m.submatrix(2, 1)
        );
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        let m = Matrix3::new(&[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(25.0, m.minor(1, 0));
    }

    #[test]
    fn test_cofactor_of_3x3_matrix() {
        let m = Matrix3::new(&[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(-12.0, m.cofactor(0, 0));
        assert_eq!(-25.0, m.cofactor(1, 0));
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let m = Matrix3::new(&[1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(56.0, m.cofactor(0, 0));
        assert_eq!(12.0, m.cofactor(0, 1));
        assert_eq!(-46.0, m.cofactor(0, 2));
        assert_eq!(-196.0, m.determinant());
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let m = Matrix4::new(&[
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);
        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.determinant());
    }

    #[test]
    fn test_invertable_matrix() {
        let m = Matrix4::new(&[
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        ]);
        assert_eq!(-2120.0, m.determinant());
        m.inverse();
    }

    #[test]
    #[should_panic]
    fn test_noninvertable_matrix() {
        let m = Matrix4::new(&[
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(0.0, m.determinant());
        m.inverse();
    }

    #[test]
    fn test_calculating_inverse_matrix() {
        let a = Matrix4::new(&[
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);
        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(105.0, a.cofactor(3, 2));

        let b = a.inverse();
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
            m.inverse()
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
            m.inverse()
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
        assert_eq!(a, c * b.inverse());
    }
}
