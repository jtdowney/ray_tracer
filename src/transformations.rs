use num::{Float, Num};

use crate::Matrix4;

pub fn translation<T>(x: T, y: T, z: T) -> Matrix4<T>
where
    T: Num + Copy,
{
    let mut transform = Matrix4::identity();
    transform[(0, 3)] = x;
    transform[(1, 3)] = y;
    transform[(2, 3)] = z;
    transform
}

pub fn scaling<T>(x: T, y: T, z: T) -> Matrix4<T>
where
    T: Num + Copy,
{
    let mut transform = Matrix4::identity();
    transform[(0, 0)] = x;
    transform[(1, 1)] = y;
    transform[(2, 2)] = z;
    transform
}

pub fn rotation_x<T>(theta: T) -> Matrix4<T>
where
    T: Float + Copy,
{
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(1, 1)] = theta_cos;
    transform[(1, 2)] = -theta_sin;
    transform[(2, 1)] = theta_sin;
    transform[(2, 2)] = theta_cos;
    transform
}

pub fn rotation_y<T>(theta: T) -> Matrix4<T>
where
    T: Float + Copy,
{
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(0, 0)] = theta_cos;
    transform[(0, 2)] = theta_sin;
    transform[(2, 0)] = -theta_sin;
    transform[(2, 2)] = theta_cos;
    transform
}

pub fn rotation_z<T>(theta: T) -> Matrix4<T>
where
    T: Float + Copy,
{
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(0, 0)] = theta_cos;
    transform[(0, 1)] = -theta_sin;
    transform[(1, 0)] = theta_sin;
    transform[(1, 1)] = theta_cos;
    transform
}

pub fn shearing<T>(x1: T, x2: T, y1: T, y2: T, z1: T, z2: T) -> Matrix4<T>
where
    T: Num + Copy,
{
    let mut transform = Matrix4::identity();
    transform[(0, 1)] = x1;
    transform[(0, 2)] = x2;
    transform[(1, 0)] = y1;
    transform[(1, 2)] = y2;
    transform[(2, 0)] = z1;
    transform[(2, 1)] = z2;
    transform
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, vector, EPSILON};
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5, -3, 2);
        let p = point(-3, 4, 5);
        assert_eq!(transform * p, point(2, 1, 7))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(inverse * p, point(-8.0, 7.0, 3.0))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5, -3, 2);
        let v = vector(-3, 4, 5);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = scaling(2, 3, 4);
        let p = point(-4, 6, 8);
        assert_eq!(transform * p, point(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2, 3, 4);
        let v = vector(-4, 6, 8);
        assert_eq!(transform * v, vector(-8, 18, 32));
    }

    #[test]
    fn scaling_inverse() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(inverse * v, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection() {
        let transform = scaling(-1, 1, 1);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(-2, 3, 4))
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        assert_abs_diff_eq!(
            half_quarter * p,
            point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            epsilon = EPSILON
        );

        let full_quarter = rotation_x(PI / 2.0);
        assert_abs_diff_eq!(full_quarter * p, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert_abs_diff_eq!(
            inv * p,
            point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        assert_abs_diff_eq!(
            half_quarter * p,
            point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );

        let full_quarter = rotation_y(PI / 2.0);
        assert_abs_diff_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        assert_abs_diff_eq!(
            half_quarter * p,
            point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
        );

        let full_quarter = rotation_z(PI / 2.0);
        assert_abs_diff_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_proportional_to_y() {
        let transform = shearing(1, 0, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(5, 3, 4));
    }

    #[test]
    fn shearing_moves_x_proportional_to_z() {
        let transform = shearing(0, 1, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(6, 3, 4));
    }

    #[test]
    fn shearing_moves_y_proportional_to_x() {
        let transform = shearing(0, 0, 1, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 5, 4));
    }

    #[test]
    fn shearing_moves_y_proportional_to_z() {
        let transform = shearing(0, 0, 0, 1, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 7, 4));
    }

    #[test]
    fn shearing_moves_z_proportional_to_x() {
        let transform = shearing(0, 0, 0, 0, 1, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 6));
    }

    #[test]
    fn shearing_moves_z_proportional_to_y() {
        let transform = shearing(0, 0, 0, 0, 0, 1);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 7));
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_abs_diff_eq!(p2, point(1.0, -1.0, 0.0), epsilon = EPSILON);

        let p3 = b * p2;
        assert_abs_diff_eq!(p3, point(5.0, -5.0, 0.0), epsilon = EPSILON);

        let p4 = c * p3;
        assert_abs_diff_eq!(p4, point(15.0, 0.0, 7.0), epsilon = EPSILON);
    }

    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let transform = c * b * a;
        assert_abs_diff_eq!(transform * p, point(15.0, 0.0, 7.0), epsilon = EPSILON);
    }
}
