use std::ops::{Index, IndexMut, Mul};

use approx::AbsDiffEq;

use crate::{EPSILON, Point, Vector, point, vector};

pub fn matrix<const N: usize, T: Into<f64> + Copy>(data: [[T; N]; N]) -> Matrix<N> {
    let mut values = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            values[i][j] = data[i][j].into();
        }
    }

    Matrix { values }
}

pub fn identity_matrix<const N: usize>() -> Matrix<N> {
    let mut result = Matrix::<N>::default();
    for i in 0..N {
        result[(i, i)] = 1.0;
    }

    result
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const N: usize> {
    values: [[f64; N]; N],
}

pub type Matrix2 = Matrix<2>;
pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

impl Matrix2 {
    fn determinant(self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl Matrix3 {
    fn submatrix(self, i: usize, j: usize) -> Matrix2 {
        (0..3)
            .filter(|&x| x != i)
            .flat_map(|x| (0..3).filter(|&y| y != j).map(move |y| self[(x, y)]))
            .collect()
    }

    fn minor(self, i: usize, j: usize) -> f64 {
        let submatrix = self.submatrix(i, j);
        submatrix.determinant()
    }

    fn cofactor(self, i: usize, j: usize) -> f64 {
        let minor = self.minor(i, j);
        if (i + j) % 2 == 0 { minor } else { -minor }
    }

    fn determinant(self) -> f64 {
        let row = self.values[0];
        (0..3).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }
}

impl Matrix4 {
    fn submatrix(self, i: usize, j: usize) -> Matrix3 {
        (0..4)
            .filter(|&x| x != i)
            .flat_map(|x| (0..4).filter(|&y| y != j).map(move |y| self[(x, y)]))
            .collect()
    }

    fn minor(self, i: usize, j: usize) -> f64 {
        let submatrix = self.submatrix(i, j);
        submatrix.determinant()
    }

    fn cofactor(self, i: usize, j: usize) -> f64 {
        let minor = self.minor(i, j);
        if (i + j) % 2 == 0 { minor } else { -minor }
    }

    fn determinant(self) -> f64 {
        let row = self.values[0];
        (0..4).zip(row).map(|(i, n)| n * self.cofactor(0, i)).sum()
    }

    pub fn inverse(self) -> Matrix4 {
        let determinant = self.determinant();
        debug_assert!(determinant != 0.0, "matrix is not invertable");

        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                let cofactor = self.cofactor(i, j);
                result[(j, i)] = cofactor / determinant;
            }
        }

        result
    }
}

impl<const N: usize> Matrix<N> {
    pub fn transpose(self) -> Self {
        let mut result = Self::default();
        for i in 0..N {
            for j in 0..N {
                result[(j, i)] = self[(i, j)];
            }
        }

        result
    }
}

impl<const N: usize> AbsDiffEq for Matrix<N> {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.into_iter()
            .zip(*other)
            .all(|(a, b)| f64::abs_diff_eq(&a, &b, epsilon))
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        let values = [[0.0; N]; N];
        Self { values }
    }
}

impl<const N: usize> IntoIterator for Matrix<N> {
    type Item = f64;
    type IntoIter = Iter<N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            values: self.values,
            i: 0,
            j: 0,
        }
    }
}

impl<const N: usize> FromIterator<f64> for Matrix<N> {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut result = Matrix::default();
        for (offset, value) in iter.into_iter().enumerate() {
            let i = offset / N;
            let j = offset % N;
            result[(i, j)] = value;
        }

        result
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.values[i][j]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.values[i][j]
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = Matrix::<N>::default();
        for i in 0..N {
            for j in 0..N {
                let value = (0..N).map(|x| self[(i, x)] * other[(x, j)]).sum();
                result[(i, j)] = value;
            }
        }

        result
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        let x =
            self[(0, 0)] * other.x + self[(0, 1)] * other.y + self[(0, 2)] * other.z + self[(0, 3)];
        let y =
            self[(1, 0)] * other.x + self[(1, 1)] * other.y + self[(1, 2)] * other.z + self[(1, 3)];
        let z =
            self[(2, 0)] * other.x + self[(2, 1)] * other.y + self[(2, 2)] * other.z + self[(2, 3)];

        point(x, y, z)
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        let x = self[(0, 0)] * other.x + self[(0, 1)] * other.y + self[(0, 2)] * other.z;
        let y = self[(1, 0)] * other.x + self[(1, 1)] * other.y + self[(1, 2)] * other.z;
        let z = self[(2, 0)] * other.x + self[(2, 1)] * other.y + self[(2, 2)] * other.z;

        vector(x, y, z)
    }
}

pub struct Iter<const N: usize> {
    values: [[f64; N]; N],
    i: usize,
    j: usize,
}

impl<const N: usize> Iterator for Iter<N> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= N {
            return None;
        }

        let result = self.values[self.i][self.j];

        self.j += 1;
        if self.j >= N {
            self.j = 0;
            self.i += 1;
        }

        Some(result)
    }
}

#[cfg(test)]
impl<const N: usize> quickcheck::Arbitrary for Matrix<N> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut values = [[0.0; N]; N];
        for row in values.iter_mut() {
            for value in row.iter_mut() {
                *value = f64::from(i32::arbitrary(g));
            }
        }

        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use crate::vector;

    use super::*;

    #[test]
    fn constructing_and_inspecting() {
        let m = matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
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
    fn matrix_multiplication_example() {
        let a = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let b = matrix([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);
        let c = matrix([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);
        assert_eq!(c, a * b);
    }

    #[quickcheck]
    fn matrix_multiplication(a: Matrix2, b: Matrix2) {
        let c = a * b;
        assert_eq!(a[(0, 0)] * b[(0, 0)] + a[(0, 1)] * b[(1, 0)], c[(0, 0)]);
        assert_eq!(a[(0, 0)] * b[(0, 1)] + a[(0, 1)] * b[(1, 1)], c[(0, 1)]);
        assert_eq!(a[(1, 0)] * b[(0, 0)] + a[(1, 1)] * b[(1, 0)], c[(1, 0)]);
        assert_eq!(a[(1, 0)] * b[(0, 1)] + a[(1, 1)] * b[(1, 1)], c[(1, 1)]);
    }

    #[test]
    fn matrix_point_multiplication_example() {
        let a = matrix([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
        let b = point(1, 2, 3);
        let c = a * b;
        assert_eq!(point(18, 24, 33), c);
    }

    #[test]
    fn matrix_vector_multiplication_example() {
        let a = matrix([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
        let b = vector(1, 2, 3);
        let c = a * b;
        assert_eq!(vector(14, 22, 32), c);
    }

    #[quickcheck]
    fn multiplying_by_identity_matrix(m: Matrix4) {
        assert_eq!(m, m * identity_matrix());
    }

    #[test]
    fn transposing_matrix() {
        let m = matrix([[0, 9, 3, 0], [9, 8, 0, 8], [1, 8, 5, 3], [0, 0, 5, 8]]);
        assert_eq!(
            matrix([[0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 8, 3, 8]]),
            m.transpose(),
        );
    }

    #[test]
    fn transposing_identity_matrix() {
        let m = identity_matrix::<4>();
        assert_eq!(m, m.transpose());
    }

    #[test]
    fn determinant_of_2x2() {
        let m = matrix([[1, 5], [-3, 2]]);
        assert_eq!(17.0, m.determinant());
    }

    #[quickcheck]
    fn submatrix_of_3x3(m: Matrix3) {
        let submatrix = m.submatrix(0, 2);
        assert_eq!(m[(1, 0)], submatrix[(0, 0)]);
        assert_eq!(m[(1, 1)], submatrix[(0, 1)]);
        assert_eq!(m[(2, 0)], submatrix[(1, 0)]);
        assert_eq!(m[(2, 1)], submatrix[(1, 1)]);
    }

    #[quickcheck]
    fn submatrix_of_4x4(m: Matrix4) {
        let submatrix = m.submatrix(2, 1);
        assert_eq!(m[(0, 0)], submatrix[(0, 0)]);
        assert_eq!(m[(0, 2)], submatrix[(0, 1)]);
        assert_eq!(m[(0, 3)], submatrix[(0, 2)]);
        assert_eq!(m[(1, 0)], submatrix[(1, 0)]);
        assert_eq!(m[(1, 2)], submatrix[(1, 1)]);
        assert_eq!(m[(1, 3)], submatrix[(1, 2)]);
        assert_eq!(m[(3, 0)], submatrix[(2, 0)]);
        assert_eq!(m[(3, 2)], submatrix[(2, 1)]);
        assert_eq!(m[(3, 3)], submatrix[(2, 2)]);
    }

    #[test]
    fn minor_of_3x3() {
        let a = matrix([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        let b = a.submatrix(1, 0);
        assert_eq!(25.0, b.determinant());
        assert_eq!(25.0, a.minor(1, 0));
    }

    #[test]
    fn cofactor_of_3x3() {
        let a = matrix([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn determinant_of_3x3() {
        let m = matrix([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
        assert_eq!(56.0, m.cofactor(0, 0));
        assert_eq!(12.0, m.cofactor(0, 1));
        assert_eq!(-46.0, m.cofactor(0, 2));
        assert_eq!(-196.0, m.determinant());
    }

    #[test]
    fn determinant_of_4x4() {
        let m = matrix([[-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]]);
        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.determinant());
    }

    #[test]
    fn invertable_4x4() {
        let m = matrix([[6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]]);
        assert_eq!(-2120.0, m.determinant());
    }

    #[test]
    fn noninvertable_4x4() {
        let m = matrix([[-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]]);
        assert_eq!(0.0, m.determinant());
    }

    #[test]
    fn inverse_of_4x4() {
        let a = matrix([[-5, 2, 6, -8], [1, -5, 1, 8], [7, 7, -6, -7], [1, -3, 7, 4]]);
        let b = a.inverse();
        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, b[(3, 2)]);
        assert_eq!(105.0, a.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, b[(2, 3)]);
        assert_abs_diff_eq!(
            matrix([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            ]),
            b
        );
    }

    #[quickcheck]
    fn multiplying_inverse_matricies(a: Matrix4, b: Matrix4) -> TestResult {
        if b.determinant() == 0.0 {
            return TestResult::discard();
        }

        let c = a * b;
        assert_abs_diff_eq!(a, c * b.inverse(), epsilon = 0.01);

        TestResult::passed()
    }
}
