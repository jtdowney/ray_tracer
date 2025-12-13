use std::ops::{Index, IndexMut, Mul};

use num_traits::AsPrimitive;
use wide::f32x4;

use crate::{Point, Vector, point, vector};

pub fn matrix<const N: usize, T: AsPrimitive<f32> + Copy>(data: [[T; N]; N]) -> Matrix<N> {
    let mut values = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            values[i][j] = data[i][j].as_();
        }
    }

    Matrix { values }
}

#[must_use]
pub fn identity_matrix<const N: usize>() -> Matrix<N> {
    let mut values = [[0.0; N]; N];
    for (i, row) in values.iter_mut().enumerate() {
        row[i] = 1.0;
    }

    Matrix { values }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const N: usize> {
    values: [[f32; N]; N],
}

pub type Matrix2 = Matrix<2>;
pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        let values = [[0.0; N]; N];
        Self { values }
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.values[row][col]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.values[row][col]
    }
}

impl<const N: usize> FromIterator<f32> for Matrix<N> {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut result = Matrix::default();
        for (offset, value) in iter.into_iter().enumerate() {
            let i = offset / N;
            let j = offset % N;
            result[(i, j)] = value;
        }

        result
    }
}

impl<const N: usize> Matrix<N> {
    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut values = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                values[i][j] = self[(j, i)];
            }
        }

        Self { values }
    }
}

impl Matrix2 {
    #[must_use]
    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl Matrix3 {
    #[must_use]
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        (0..3)
            .filter(|&x| x != row)
            .flat_map(|x| (0..3).filter(|&y| y != col).map(move |y| self[(x, y)]))
            .collect()
    }

    #[must_use]
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    #[must_use]
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col).is_multiple_of(2) {
            minor
        } else {
            -minor
        }
    }

    #[must_use]
    pub fn determinant(&self) -> f32 {
        let row = self.values[0];
        (0..3).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }
}

impl Matrix4 {
    #[must_use]
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        (0..4)
            .filter(|&x| x != row)
            .flat_map(|x| (0..4).filter(|&y| y != col).map(move |y| self[(x, y)]))
            .collect()
    }

    #[must_use]
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    #[must_use]
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col).is_multiple_of(2) {
            minor
        } else {
            -minor
        }
    }

    #[must_use]
    pub fn determinant(&self) -> f32 {
        let row = self.values[0];
        (0..4).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }

    #[must_use]
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    #[must_use]
    pub fn inverse(&self) -> Option<Self> {
        if !self.is_invertible() {
            return None;
        }

        let mut inverse = Self::default();
        let determinant = self.determinant();
        for i in 0..4 {
            for j in 0..4 {
                let cofactor = self.cofactor(i, j);
                inverse[(j, i)] = cofactor / determinant;
            }
        }
        Some(inverse)
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        result
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let row0 = f32x4::new(self.values[0]);
        let row1 = f32x4::new(self.values[1]);
        let row2 = f32x4::new(self.values[2]);

        let p0 = row0 * rhs.data;
        let p1 = row1 * rhs.data;
        let p2 = row2 * rhs.data;

        let a0 = p0.as_array();
        let a1 = p1.as_array();
        let a2 = p2.as_array();

        let x = a0[0] + a0[1] + a0[2] + a0[3];
        let y = a1[0] + a1[1] + a1[2] + a1[3];
        let z = a2[0] + a2[1] + a2[2] + a2[3];

        point(x, y, z)
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let row0 = f32x4::new(self.values[0]);
        let row1 = f32x4::new(self.values[1]);
        let row2 = f32x4::new(self.values[2]);

        let p0 = row0 * rhs.data;
        let p1 = row1 * rhs.data;
        let p2 = row2 * rhs.data;

        let a0 = p0.as_array();
        let a1 = p1.as_array();
        let a2 = p2.as_array();

        let x = a0[0] + a0[1] + a0[2];
        let y = a1[0] + a1[1] + a1[2];
        let z = a2[0] + a2[1] + a2[2];

        vector(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, vector};

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let m = matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_relative_eq!(m[(0, 0)], 1.0, epsilon = EPSILON);
        assert_relative_eq!(m[(0, 3)], 4.0, epsilon = EPSILON);
        assert_relative_eq!(m[(1, 0)], 5.5, epsilon = EPSILON);
        assert_relative_eq!(m[(1, 2)], 7.5, epsilon = EPSILON);
        assert_relative_eq!(m[(2, 2)], 11.0, epsilon = EPSILON);
        assert_relative_eq!(m[(3, 0)], 13.5, epsilon = EPSILON);
        assert_relative_eq!(m[(3, 2)], 15.5, epsilon = EPSILON);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m: Matrix2 = matrix([[-3, 5], [1, -2]]);
        assert_relative_eq!(m[(0, 0)], -3.0, epsilon = EPSILON);
        assert_relative_eq!(m[(0, 1)], 5.0, epsilon = EPSILON);
        assert_relative_eq!(m[(1, 0)], 1.0, epsilon = EPSILON);
        assert_relative_eq!(m[(1, 1)], -2.0, epsilon = EPSILON);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m: Matrix3 = matrix([[-3, 5, 0], [1, -2, -7], [0, 1, 1]]);
        assert_relative_eq!(m[(0, 0)], -3.0, epsilon = EPSILON);
        assert_relative_eq!(m[(1, 1)], -2.0, epsilon = EPSILON);
        assert_relative_eq!(m[(2, 2)], 1.0, epsilon = EPSILON);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = matrix([[2, 3, 4, 5], [6, 7, 8, 9], [8, 7, 6, 5], [4, 3, 2, 1]]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = matrix([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);
        let expected = matrix([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn matrix_multiplied_by_point() {
        let a = matrix([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
        let b = point(1, 2, 3);
        assert_eq!(a * b, point(18, 24, 33));
    }

    #[test]
    fn matrix_multiplied_by_vector() {
        let a = matrix([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
        let b = vector(1, 2, 3);
        assert_eq!(a * b, vector(14, 22, 32));
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let a = matrix([[0, 1, 2, 4], [1, 2, 4, 8], [2, 4, 8, 16], [4, 8, 16, 32]]);
        assert_eq!(a * identity_matrix(), a);
    }

    #[test]
    fn multiplying_identity_matrix_by_point() {
        let a = point(1, 2, 3);
        assert_eq!(identity_matrix() * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = matrix([[0, 9, 3, 0], [9, 8, 0, 8], [1, 8, 5, 3], [0, 0, 5, 8]]);
        let expected = matrix([[0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 8, 3, 8]]);
        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn transposing_identity_matrix() {
        assert_eq!(identity_matrix::<4>().transpose(), identity_matrix());
    }

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let a: Matrix2 = matrix([[1, 5], [-3, 2]]);
        assert_relative_eq!(a.determinant(), 17.0, epsilon = EPSILON);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a: Matrix3 = matrix([[1, 5, 0], [-3, 2, 7], [0, 6, -3]]);
        let expected: Matrix2 = matrix([[-3, 2], [0, 6]]);
        assert_eq!(a.submatrix(0, 2), expected);
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = matrix([[-6, 1, 1, 6], [-8, 5, 8, 6], [-1, 0, 8, 2], [-7, 1, -1, 1]]);
        let expected: Matrix3 = matrix([[-6, 1, 6], [-8, 8, 6], [-7, -1, 1]]);
        assert_eq!(a.submatrix(2, 1), expected);
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let a: Matrix3 = matrix([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        let b = a.submatrix(1, 0);
        assert_relative_eq!(b.determinant(), 25.0, epsilon = EPSILON);
        assert_relative_eq!(a.minor(1, 0), 25.0, epsilon = EPSILON);
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let a: Matrix3 = matrix([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        assert_relative_eq!(a.minor(0, 0), -12.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 0), -12.0, epsilon = EPSILON);
        assert_relative_eq!(a.minor(1, 0), 25.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(1, 0), -25.0, epsilon = EPSILON);
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let a: Matrix3 = matrix([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
        assert_relative_eq!(a.cofactor(0, 0), 56.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 1), 12.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 2), -46.0, epsilon = EPSILON);
        assert_relative_eq!(a.determinant(), -196.0, epsilon = EPSILON);
    }

    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        let a = matrix([[-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]]);
        assert_relative_eq!(a.cofactor(0, 0), 690.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 1), 447.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 2), 210.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(0, 3), 51.0, epsilon = EPSILON);
        assert_relative_eq!(a.determinant(), -4071.0, epsilon = EPSILON);
    }

    #[test]
    fn testing_invertible_matrix_for_invertibility() {
        let a = matrix([[6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]]);
        assert_relative_eq!(a.determinant(), -2120.0, epsilon = EPSILON);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_noninvertible_matrix_for_invertibility() {
        let a = matrix([[-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]]);
        assert_relative_eq!(a.determinant(), 0.0, epsilon = EPSILON);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let a = matrix([[-5, 2, 6, -8], [1, -5, 1, 8], [7, 7, -6, -7], [1, -3, 7, 4]]);
        let b = a.inverse().unwrap();
        assert_relative_eq!(a.determinant(), 532.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(2, 3), -160.0, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 2)], -160.0 / 532.0, epsilon = EPSILON);
        assert_relative_eq!(a.cofactor(3, 2), 105.0, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 3)], 105.0 / 532.0, epsilon = EPSILON);

        assert_relative_eq!(b[(0, 0)], 0.21805, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 1)], 0.45113, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 2)], 0.24060, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 3)], -0.04511, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 0)], -0.80827, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 1)], -1.45677, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 2)], -0.44361, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 3)], 0.52068, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 0)], -0.07895, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 1)], -0.22368, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 2)], -0.05263, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 3)], 0.19737, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 0)], -0.52256, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 1)], -0.81391, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 2)], -0.30075, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 3)], 0.30639, epsilon = EPSILON);
    }

    #[test]
    fn calculating_inverse_of_another_matrix() {
        let a = matrix([[8, -5, 9, 2], [7, 5, 6, 1], [-6, 0, 9, 6], [-3, 0, -9, -4]]);
        let b = a.inverse().unwrap();
        assert_relative_eq!(b[(0, 0)], -0.15385, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 1)], -0.15385, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 2)], -0.28205, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 3)], -0.53846, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 0)], -0.07692, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 1)], 0.12308, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 2)], 0.02564, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 3)], 0.03077, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 0)], 0.35897, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 1)], 0.35897, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 2)], 0.43590, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 3)], 0.92308, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 0)], -0.69231, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 1)], -0.69231, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 2)], -0.76923, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 3)], -1.92308, epsilon = EPSILON);
    }

    #[test]
    fn calculating_inverse_of_third_matrix() {
        let a = matrix([[9, 3, 0, 9], [-5, -2, -6, -3], [-4, 9, 6, 4], [-7, 6, 6, 2]]);
        let b = a.inverse().unwrap();
        assert_relative_eq!(b[(0, 0)], -0.04074, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 1)], -0.07778, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 2)], 0.14444, epsilon = EPSILON);
        assert_relative_eq!(b[(0, 3)], -0.22222, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 0)], -0.07778, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 1)], 0.03333, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 2)], 0.36667, epsilon = EPSILON);
        assert_relative_eq!(b[(1, 3)], -0.33333, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 0)], -0.02901, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 1)], -0.14630, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 2)], -0.10926, epsilon = EPSILON);
        assert_relative_eq!(b[(2, 3)], 0.12963, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 0)], 0.17778, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 1)], 0.06667, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 2)], -0.26667, epsilon = EPSILON);
        assert_relative_eq!(b[(3, 3)], 0.33333, epsilon = EPSILON);
    }

    #[test]
    fn multiplying_product_by_its_inverse() {
        let a = matrix([[3, -9, 7, 3], [3, -8, 2, -9], [-4, 4, 4, 1], [-6, 5, -1, 1]]);
        let b = matrix([[8, 2, 2, 2], [3, -1, 7, 0], [7, 0, 5, 4], [6, -2, 0, 5]]);
        let c = a * b;
        let result = c * b.inverse().unwrap();
        for row in 0..4 {
            for col in 0..4 {
                assert_relative_eq!(result[(row, col)], a[(row, col)], epsilon = EPSILON);
            }
        }
    }
}
