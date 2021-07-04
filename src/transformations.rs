use crate::Matrix4;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut transform = Matrix4::identity();
    transform[(0, 3)] = x;
    transform[(1, 3)] = y;
    transform[(2, 3)] = z;
    transform
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut transform = Matrix4::identity();
    transform[(0, 0)] = x;
    transform[(1, 1)] = y;
    transform[(2, 2)] = z;
    transform
}

pub fn rotation_x(theta: f64) -> Matrix4 {
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(1, 1)] = theta_cos;
    transform[(1, 2)] = -theta_sin;
    transform[(2, 1)] = theta_sin;
    transform[(2, 2)] = theta_cos;
    transform
}

pub fn rotation_y(theta: f64) -> Matrix4 {
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(0, 0)] = theta_cos;
    transform[(0, 2)] = theta_sin;
    transform[(2, 0)] = -theta_sin;
    transform[(2, 2)] = theta_cos;
    transform
}

pub fn rotation_z(theta: f64) -> Matrix4 {
    let mut transform = Matrix4::identity();
    let (theta_sin, theta_cos) = theta.sin_cos();
    transform[(0, 0)] = theta_cos;
    transform[(0, 1)] = -theta_sin;
    transform[(1, 0)] = theta_sin;
    transform[(1, 1)] = theta_cos;
    transform
}

pub fn shearing(x1: f64, x2: f64, y1: f64, y2: f64, z1: f64, z2: f64) -> Matrix4 {
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
    use crate::{point, vector};
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0))
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
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, vector(-8.0, 18.0, 32.0));
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
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(-2.0, 3.0, 4.0))
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        assert_abs_diff_eq!(
            half_quarter * p,
            point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
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
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_proportional_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_proportional_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_proportional_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_proportional_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_proportional_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_abs_diff_eq!(p2, point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_abs_diff_eq!(p3, point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_abs_diff_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let transform = c * b * a;
        assert_abs_diff_eq!(transform * p, point(15.0, 0.0, 7.0));
    }
}
