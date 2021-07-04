use crate::{point, vector, Point, Vector, EPSILON};
use approx::AbsDiffEq;
use core::f64;
use generic_array::{ArrayLength, GenericArray};
use num::Integer;
use num::Zero;
use std::{
    iter::{self, FromIterator},
    marker::PhantomData,
    ops::{Index, IndexMut, Mul},
};
use typenum::{Prod, U2, U3, U4};

#[derive(Copy, Clone, Debug, PartialEq)]
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

pub fn matrix<N, S>(values: &[f64]) -> Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    values.iter().copied().collect()
}

impl<N, S> AbsDiffEq for Matrix<N, S>
where
    N: ArrayLength<f64> + PartialEq,
    S: ArrayLength<f64> + PartialEq,
    S::ArrayType: Copy,
{
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.values
            .iter()
            .copied()
            .zip(other.values)
            .all(|(a, b)| f64::abs_diff_eq(&a, &b, epsilon))
    }
}

impl<N, S> Default for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    fn default() -> Self {
        let values = Default::default();
        Matrix {
            values,
            _phantom: PhantomData,
        }
    }
}

impl<N, S> Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    pub fn identity() -> Self {
        (0..S::to_usize())
            .map(|s| {
                let (i, j) = s.div_rem(&N::to_usize());
                if i == j {
                    1.0
                } else {
                    0.0
                }
            })
            .collect()
    }

    pub fn transpose(&self) -> Self {
        (0..S::to_usize())
            .map(|s| {
                let (i, j) = s.div_rem(&N::to_usize());
                self[(j, i)]
            })
            .collect()
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

impl<N, S> Mul for Matrix<N, S>
where
    N: ArrayLength<f64>,
    S: ArrayLength<f64>,
    S::ArrayType: Copy,
{
    type Output = Matrix<N, S>;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: Check if there is a perf gain with using mem::zeroed()
        let mut result: Matrix<N, S> = Default::default();

        let n = N::to_usize();
        for i in 0..n {
            for j in 0..n {
                let value = (0..n).map(|x| self[(i, x)] * rhs[(x, j)]).sum();
                result[(i, j)] = value;
            }
        }

        result
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;

        vector(x, y, z)
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z + self[(0, 3)];
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z + self[(1, 3)];
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z + self[(2, 3)];

        point(x, y, z)
    }
}

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl Matrix3 {
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix2 {
        (0..3)
            .filter(|&x| i != x)
            .flat_map(|x| {
                let offset = x * 3;
                self.values[offset..]
                    .iter()
                    .take(3)
                    .copied()
                    .enumerate()
                    .filter(|&(index, _)| index != j)
                    .map(|(_, n)| n)
            })
            .collect()
    }

    pub fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
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
        let row = &self.values[0..3];
        (0..3).zip(row).map(|(i, &n)| n * self.cofactor(0, i)).sum()
    }
}

// TODO: Look for a way to make this generic over the size of the matrix
impl Matrix4 {
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix3 {
        (0..4)
            .filter(|&x| i != x)
            .flat_map(|x| {
                let offset = x * 4;
                self.values[offset..]
                    .iter()
                    .take(4)
                    .copied()
                    .enumerate()
                    .filter(|&(index, _)| index != j)
                    .map(|(_, n)| n)
            })
            .collect()
    }

    pub fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let value = self.minor(i, j);
        if (i + j) % 2 == 0 {
            value
        } else {
            -value
        }
    }

    pub fn determinant(&self) -> f64 {
        let row = &self.values[0..4];
        (0..4).zip(row).map(|(i, &n)| n * self.cofactor(0, i)).sum()
    }

    pub fn inverse(&self) -> Matrix4 {
        let determinant = self.determinant();
        if determinant.is_zero() {
            // TODO: Figure out if we should panic or just return Result
            panic!("matrix is not invertable");
        }

        let mut inverse: Matrix4 = iter::repeat(0.0).collect();
        for i in 0..4 {
            for j in 0..4 {
                let cofactor = self.cofactor(i, j);
                inverse[(j, i)] = cofactor / determinant;
            }
        }

        inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;
    use approx::assert_abs_diff_eq;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl<N, S> Arbitrary for Matrix<N, S>
    where
        N: ArrayLength<f64> + 'static,
        S: ArrayLength<f64> + 'static,
        S::ArrayType: Copy,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            iter::repeat_with(|| f64::arbitrary(g))
                .filter(|n| n.is_normal())
                .take(S::to_usize())
                .collect()
        }
    }

    #[test]
    fn constructing_and_inspecting_2x2() {
        let m: Matrix2 = matrix(&[-3.0, 5.0, 1.0, -2.0]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn constructing_and_inspecting_3x3() {
        let m: Matrix3 = matrix(&[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn constructing_and_inspecting_4x4() {
        let m: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn matrix_equality() {
        let a: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_inequality() {
        let a: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b: Matrix4 = matrix(&[
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_matrices() {
        let a: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b: Matrix4 = matrix(&[
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);

        assert_eq!(
            a * b,
            matrix(&[
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ]),
        );
    }

    #[test]
    fn multiplying_matrix_by_point() {
        let a: Matrix4 = matrix(&[
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let b = point(1.0, 2.0, 3.0);

        assert_eq!(a * b, point(18.0, 24.0, 33.0));
    }

    #[quickcheck]
    fn multiplying_matrix_by_identity_matrix(m: Matrix4) {
        assert_eq!(m * Matrix4::identity(), m);
    }

    #[quickcheck]
    fn multiplying_vector_by_identity_matrix(v: Vector) {
        assert_eq!(Matrix4::identity() * v, v);
    }

    #[test]
    fn transposing_matrix() {
        let m: Matrix4 = matrix(&[
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);
        assert_eq!(
            m.transpose(),
            matrix(&[
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            ]),
        );
    }

    #[test]
    fn transposing_identity_matrix() {
        let m = Matrix4::identity();
        assert_eq!(m.transpose(), Matrix4::identity());
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let m: Matrix2 = matrix(&[1.0, 5.0, -3.0, 2.0]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        let m: Matrix3 = matrix(&[1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        assert_eq!(m.submatrix(0, 2), matrix(&[-3.0, 2.0, 0.0, 6.0]));
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        let m: Matrix4 = matrix(&[
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);
        assert_eq!(
            m.submatrix(2, 1),
            matrix(&[-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
        );
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let m: Matrix3 = matrix(&[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let m: Matrix3 = matrix(&[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let m: Matrix3 = matrix(&[1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let m: Matrix4 = matrix(&[
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn invertable_matrix() {
        let m: Matrix4 = matrix(&[
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        ]);
        assert_eq!(m.determinant(), -2120.0);
    }

    #[test]
    fn calculating_inverse_matrix() {
        let a: Matrix4 = matrix(&[
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(a.cofactor(3, 2), 105.0);

        let b = a.inverse();
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);
        assert_abs_diff_eq!(
            b,
            matrix(&[
                0.218045, 0.451128, 0.240602, -0.045113, -0.808271, -1.456767, -0.443609, 0.520677,
                -0.078947, -0.223684, -0.052632, 0.197368, -0.522556, -0.813910, -0.300752,
                0.306391
            ])
        );

        let m = matrix(&[
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        ]);
        assert_abs_diff_eq!(
            m.inverse(),
            matrix(&[
                -0.153846, -0.153846, -0.282051, -0.538462, -0.076923, 0.123077, 0.025641,
                0.030769, 0.358974, 0.358974, 0.435897, 0.923077, -0.692308, -0.692308, -0.769231,
                -1.923077,
            ])
        );

        let m = matrix(&[
            9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
        ]);
        assert_abs_diff_eq!(
            m.inverse(),
            matrix(&[
                -0.040741, -0.077778, 0.144444, -0.222222, -0.077778, 0.033333, 0.366667,
                -0.333333, -0.029012, -0.146296, -0.109259, 0.129630, 0.177778, 0.066667,
                -0.266667, 0.333333,
            ])
        );
    }

    #[test]
    fn multiplying_matrix_by_inverse() {
        let a = matrix(&[
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = matrix(&[
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        ]);
        let c = a * b;
        assert_abs_diff_eq!(c * b.inverse(), a);
    }
}
